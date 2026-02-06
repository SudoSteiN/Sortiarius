#!/usr/bin/env bash
# SteinBot Setup - Run once to make SteinBot your global Claude Code assistant
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
CLAUDE_DIR="$HOME/.claude"

# --- Helpers ---

die() { echo "FAIL: $*" >&2; exit 1; }
ok()  { echo "  [ok] $*"; }
skip() { echo "  [skip] $*"; }

detect_shell_profile() {
  local shell_name
  shell_name="$(basename "${SHELL:-bash}")"
  case "$shell_name" in
    zsh)  echo "$HOME/.zshrc" ;;
    bash)
      # Prefer .bashrc, fall back to .bash_profile (macOS default)
      if [ -f "$HOME/.bashrc" ]; then
        echo "$HOME/.bashrc"
      else
        echo "$HOME/.bash_profile"
      fi
      ;;
    *)    echo "$HOME/.profile" ;;
  esac
}

# Cross-platform readlink -f
resolve_path() {
  local target="$1"
  if readlink -f "$target" 2>/dev/null; then
    return
  fi
  # macOS fallback: python or manual resolution
  python3 -c "import os; print(os.path.realpath('$target'))" 2>/dev/null && return
  # Last resort: echo as-is
  echo "$target"
}

echo "=== SteinBot Global Setup ==="
echo "Repo: $REPO_DIR"
echo ""

# --- 1. Validate prerequisites ---

echo "Checking prerequisites..."

command -v git >/dev/null 2>&1 || die "git is not installed"
ok "git found"

if command -v claude >/dev/null 2>&1; then
  ok "claude CLI found"
else
  echo "  [warn] claude CLI not found - install before using SteinBot"
  echo "         https://docs.anthropic.com/en/docs/claude-code"
fi

[ -f "$REPO_DIR/CLAUDE.md" ] || die "CLAUDE.md not found in $REPO_DIR - is this the SteinBot repo?"
ok "CLAUDE.md found"

echo ""

# --- 2. Create ~/.claude and symlink global CLAUDE.md ---

echo "Installing global CLAUDE.md..."
mkdir -p "$CLAUDE_DIR"

GLOBAL_CLAUDE="$CLAUDE_DIR/CLAUDE.md"

if [ -L "$GLOBAL_CLAUDE" ]; then
  current_target="$(resolve_path "$GLOBAL_CLAUDE")"
  repo_target="$(resolve_path "$REPO_DIR/CLAUDE.md")"
  if [ "$current_target" = "$repo_target" ]; then
    skip "Already linked correctly"
  else
    ln -sf "$REPO_DIR/CLAUDE.md" "$GLOBAL_CLAUDE"
    ok "Updated symlink (was: $current_target)"
  fi
elif [ -f "$GLOBAL_CLAUDE" ]; then
  cp "$GLOBAL_CLAUDE" "$GLOBAL_CLAUDE.bak"
  ln -sf "$REPO_DIR/CLAUDE.md" "$GLOBAL_CLAUDE"
  ok "Backed up existing to CLAUDE.md.bak, created symlink"
else
  ln -sf "$REPO_DIR/CLAUDE.md" "$GLOBAL_CLAUDE"
  ok "Created symlink"
fi

# Verify symlink works
[ -f "$GLOBAL_CLAUDE" ] || die "Symlink creation failed - check permissions on $CLAUDE_DIR"

echo ""

# --- 3. Add bin/ to PATH ---

echo "Installing stein command..."
PROFILE="$(detect_shell_profile)"
BIN_DIR="$REPO_DIR/bin"

[ -f "$BIN_DIR/stein" ] || die "bin/stein not found in $REPO_DIR"
[ -x "$BIN_DIR/stein" ] || chmod +x "$BIN_DIR/stein"

if grep -qF "$BIN_DIR" "$PROFILE" 2>/dev/null; then
  skip "Already in PATH ($PROFILE)"
else
  {
    echo ""
    echo "# SteinBot - personal AI assistant"
    echo "export PATH=\"$BIN_DIR:\$PATH\""
  } >> "$PROFILE"
  ok "Added to PATH in $PROFILE"
fi

echo ""

# --- 4. Create projects directory ---

echo "Setting up projects directory..."
PROJECTS_DIR="$HOME/projects"
if [ -d "$PROJECTS_DIR" ]; then
  skip "Already exists at $PROJECTS_DIR"
else
  mkdir -p "$PROJECTS_DIR"
  ok "Created $PROJECTS_DIR"
fi

echo ""

# --- 5. Summary ---

echo "=== Setup Complete ==="
echo ""
echo "Activate now:"
echo "  source $PROFILE"
echo ""
echo "Then use from anywhere:"
echo "  stein              Claude Code + SteinBot in current dir"
echo "  stein new my-app   Create project at ~/projects/my-app"
echo "  stein home         Manage skills and memory"
echo "  stein sync         Git push workspace changes"
echo "  stein help         All commands"
echo "  claude             Also works - SteinBot loads globally"
echo ""
echo "Next step: Edit ~/SteinBot/workspace/MEMORY.md with your environment details"
echo "(Azure subscription, resource groups, SQL instances, etc.)"
