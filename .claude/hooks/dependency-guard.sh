#!/usr/bin/env bash
# Sortiarius Dependency Guard: Warn when installing unvetted packages
# Hook type: PreToolUse (matcher: Bash)
#
# When detecting package installation commands:
# - Logs the dependency addition
# - Warns about unvetted packages (allow but inject warning)
# - Blocks installation of known-malicious package patterns
#
# This catches supply chain risks and ensures conscious dependency decisions.
set -uo pipefail

INPUT="$(cat)"
COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // empty')"

# --- Detect package installation commands ---
IS_INSTALL=false
PKG_MANAGER=""
PACKAGES=""

# npm/yarn/pnpm install with specific packages
if echo "$COMMAND" | grep -qE '(npm|yarn|pnpm)\s+(install|add|i)\s+[a-zA-Z@]'; then
  IS_INSTALL=true
  PKG_MANAGER="npm"
  # Extract package names (everything after install/add that isn't a flag)
  PACKAGES="$(echo "$COMMAND" | sed -E 's/.*(install|add|i)\s+//' | tr ' ' '\n' | grep -v '^-' | tr '\n' ' ')"
fi

# pip install with specific packages (not -r requirements.txt)
if echo "$COMMAND" | grep -qE 'pip3?\s+install\s+[a-zA-Z]' && ! echo "$COMMAND" | grep -qE 'pip3?\s+install\s+-r'; then
  IS_INSTALL=true
  PKG_MANAGER="pip"
  PACKAGES="$(echo "$COMMAND" | sed -E 's/.*install\s+//' | tr ' ' '\n' | grep -v '^-' | tr '\n' ' ')"
fi

# cargo add
if echo "$COMMAND" | grep -qE 'cargo\s+add\s+[a-zA-Z]'; then
  IS_INSTALL=true
  PKG_MANAGER="cargo"
  PACKAGES="$(echo "$COMMAND" | sed -E 's/.*add\s+//' | tr ' ' '\n' | grep -v '^-' | tr '\n' ' ')"
fi

# go get
if echo "$COMMAND" | grep -qE 'go\s+get\s+'; then
  IS_INSTALL=true
  PKG_MANAGER="go"
  PACKAGES="$(echo "$COMMAND" | sed -E 's/.*get\s+//' | tr ' ' '\n' | grep -v '^-' | tr '\n' ' ')"
fi

# Not an install command, skip
$IS_INSTALL || exit 0

# --- Check for typosquat patterns (common attack vector) ---
SUSPICIOUS=""

# Known typosquat indicators: single-char differences from popular packages
# These are examples — in a real system this would be a maintained list
POPULAR_NPM="express react vue angular lodash axios moment webpack babel eslint prettier jest mocha"
for pkg in $PACKAGES; do
  # Flag packages with suspicious names (very short + similar to popular ones)
  pkg_clean="$(echo "$pkg" | sed 's/@.*//')"  # Remove version/scope

  # Flag packages that are just a dash/underscore variant of popular packages
  for popular in $POPULAR_NPM; do
    if [ "$pkg_clean" != "$popular" ] && [ "${#pkg_clean}" -ge 3 ]; then
      # Check Levenshtein-like similarity (simple: same length, 1 char diff)
      if [ "${#pkg_clean}" = "${#popular}" ]; then
        diff_count=0
        for (( i=0; i<${#pkg_clean}; i++ )); do
          [ "${pkg_clean:$i:1}" != "${popular:$i:1}" ] && diff_count=$((diff_count + 1))
        done
        if [ "$diff_count" -eq 1 ]; then
          SUSPICIOUS="${SUSPICIOUS}Package '$pkg_clean' is 1 character different from popular package '$popular'. Possible typosquat.\n"
        fi
      fi
    fi
  done
done

# --- Block known-malicious patterns ---
for pkg in $PACKAGES; do
  # Block packages with obviously suspicious names
  if echo "$pkg" | grep -qEi '(malware|keylogger|stealer|backdoor|reverse.shell|c2|rat\b)'; then
    cat << DENY
{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Package name '$pkg' matches known malicious patterns. Review this dependency carefully before installing."}}
DENY
    exit 0
  fi
done

# --- Inject warning for all new dependencies ---
WARNING="[Sortiarius Dependency Guard]\nInstalling new ${PKG_MANAGER} packages: ${PACKAGES}\n"

if [ -n "$SUSPICIOUS" ]; then
  WARNING="${WARNING}\nSUSPICIOUS PACKAGES DETECTED:\n${SUSPICIOUS}"
fi

WARNING="${WARNING}\nBefore adding dependencies, consider:\n- Is this package actively maintained?\n- Does it have a reasonable download count?\n- Could the functionality be implemented without adding a dependency?\n- Check: https://www.npmjs.com/package/<name> or https://pypi.org/project/<name>"

jq -n --arg msg "$(echo -e "$WARNING")" '{
  "systemMessage": $msg
}'

exit 0
