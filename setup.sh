#!/usr/bin/env bash
# SteinBot Setup - Run once to configure your shell alias
set -e

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
SHELL_NAME="$(basename "$SHELL")"

echo "Setting up SteinBot from: $REPO_DIR"

# Detect shell profile
case "$SHELL_NAME" in
  zsh)  PROFILE="$HOME/.zshrc" ;;
  bash) PROFILE="$HOME/.bashrc" ;;
  *)    PROFILE="$HOME/.profile" ;;
esac

ALIAS_LINE="alias stein='cd $REPO_DIR && claude'"

# Check if alias already exists
if grep -q "alias stein=" "$PROFILE" 2>/dev/null; then
  echo "Alias 'stein' already exists in $PROFILE"
else
  echo "" >> "$PROFILE"
  echo "# SteinBot - personal AI assistant" >> "$PROFILE"
  echo "$ALIAS_LINE" >> "$PROFILE"
  echo "Added alias to $PROFILE"
fi

echo ""
echo "Done! Run this to activate:"
echo "  source $PROFILE"
echo ""
echo "Then just type:"
echo "  stein"
echo ""
echo "Optional: Edit workspace/MEMORY.md with your Azure environment details."
