#!/usr/bin/env bash
# SteinBot Setup - Run once to make SteinBot your global Claude Code assistant
set -e

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
SHELL_NAME="$(basename "$SHELL")"
CLAUDE_DIR="$HOME/.claude"

echo "=== SteinBot Global Setup ==="
echo "Repo: $REPO_DIR"
echo ""

# --- 1. Create ~/.claude if needed ---
mkdir -p "$CLAUDE_DIR"

# --- 2. Symlink global CLAUDE.md ---
GLOBAL_CLAUDE="$CLAUDE_DIR/CLAUDE.md"
if [ -L "$GLOBAL_CLAUDE" ]; then
  CURRENT_TARGET="$(readlink "$GLOBAL_CLAUDE")"
  if [ "$CURRENT_TARGET" = "$REPO_DIR/CLAUDE.md" ]; then
    echo "[ok] Global CLAUDE.md already linked"
  else
    echo "[update] Updating symlink (was: $CURRENT_TARGET)"
    ln -sf "$REPO_DIR/CLAUDE.md" "$GLOBAL_CLAUDE"
  fi
elif [ -f "$GLOBAL_CLAUDE" ]; then
  echo "[backup] Existing ~/.claude/CLAUDE.md backed up to ~/.claude/CLAUDE.md.bak"
  cp "$GLOBAL_CLAUDE" "$GLOBAL_CLAUDE.bak"
  ln -sf "$REPO_DIR/CLAUDE.md" "$GLOBAL_CLAUDE"
else
  ln -sf "$REPO_DIR/CLAUDE.md" "$GLOBAL_CLAUDE"
  echo "[ok] Linked ~/.claude/CLAUDE.md -> $REPO_DIR/CLAUDE.md"
fi

# --- 3. Add bin/ to PATH ---
detect_profile() {
  case "$SHELL_NAME" in
    zsh)  echo "$HOME/.zshrc" ;;
    bash) echo "$HOME/.bashrc" ;;
    *)    echo "$HOME/.profile" ;;
  esac
}

PROFILE="$(detect_profile)"
BIN_DIR="$REPO_DIR/bin"
PATH_LINE="export PATH=\"$BIN_DIR:\$PATH\""

if grep -q "$BIN_DIR" "$PROFILE" 2>/dev/null; then
  echo "[ok] bin/ already in PATH"
else
  echo "" >> "$PROFILE"
  echo "# SteinBot - personal AI assistant" >> "$PROFILE"
  echo "$PATH_LINE" >> "$PROFILE"
  echo "[ok] Added $BIN_DIR to PATH in $PROFILE"
fi

# --- 4. Create projects directory ---
PROJECTS_DIR="$HOME/projects"
mkdir -p "$PROJECTS_DIR"
echo "[ok] Projects directory: $PROJECTS_DIR"

# --- 5. Install memory sync hook ---
HOOKS_DIR="$CLAUDE_DIR/hooks"
mkdir -p "$HOOKS_DIR"

# Create the stop hook that syncs memory on session end
HOOK_FILE="$HOOKS_DIR/memory-sync.sh"
cat > "$HOOK_FILE" << 'HOOK_EOF'
#!/usr/bin/env bash
# Auto-sync SteinBot memory after Claude Code sessions
# Triggered by: claude code hook on stop
STEINBOT_HOME="${STEINBOT_HOME:-$HOME/SteinBot}"

cd "$STEINBOT_HOME" 2>/dev/null || exit 0

if ! git diff --quiet workspace/MEMORY.md 2>/dev/null; then
  git add workspace/MEMORY.md
  git commit -m "Auto-sync memory - $(date '+%Y-%m-%d %H:%M')" --no-verify 2>/dev/null
  git push 2>/dev/null || true
fi
HOOK_EOF
chmod +x "$HOOK_FILE"
echo "[ok] Memory sync hook installed"

# --- 6. Summary ---
echo ""
echo "=== Setup Complete ==="
echo ""
echo "What was done:"
echo "  1. Symlinked ~/.claude/CLAUDE.md -> SteinBot (global identity)"
echo "  2. Added bin/stein to your PATH"
echo "  3. Created ~/projects/ for new projects"
echo "  4. Installed memory sync hook"
echo ""
echo "Activate now:"
echo "  source $PROFILE"
echo ""
echo "Then use anywhere:"
echo "  stein              # SteinBot in current directory"
echo "  stein new my-app   # Create new project with SteinBot"
echo "  stein home         # Manage SteinBot skills/memory"
echo "  stein sync         # Push memory updates to git"
echo "  claude             # Also works - SteinBot identity loads globally"
echo ""
echo "Optional: Edit ~/SteinBot/workspace/MEMORY.md with your environment details."
