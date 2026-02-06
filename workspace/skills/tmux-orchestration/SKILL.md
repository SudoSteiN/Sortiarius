---
name: tmux-orchestration
description: Orchestrate tmux sessions for long-running tasks, parallel monitoring, and coordinating multiple Claude Code instances.
triggers:
  - tmux
  - terminal session
  - split pane
  - long running
  - monitor process
  - parallel terminal
  - orchestrate terminal
  - background task
pipeline: []
---

# Tmux Orchestration Skill

Use tmux to manage long-running processes, monitor multiple streams, and coordinate parallel work.

## Naming Convention

```bash
SESSION="sortiarius-$(echo '<task>' | tr ' ' '-')-$(date +%H%M%S)"
```

## Core Operations

```bash
# Create detached session
tmux new-session -d -s "$SESSION"
# With initial command
tmux new-session -d -s "$SESSION" "tail -f /var/log/syslog"

# Split panes: -v = top/bottom, -h = left/right
tmux split-window -v -t "$SESSION"
tmux split-window -h -t "$SESSION"

# Named windows (tabs)
tmux new-window -t "$SESSION" -n "logs"
tmux new-window -t "$SESSION" -n "monitoring"

# Send commands to specific panes
tmux send-keys -t "$SESSION:0.0" "top -b -n 1" Enter
tmux send-keys -t "$SESSION:monitoring" "htop" Enter
tmux send-keys -t "$SESSION:0.0" C-c   # Ctrl+C
```

## Wait-for-Text Pattern

Poll a pane until specific output appears. Essential for sequential coordination.

```bash
wait_for_text() {
  local session="$1" pane="$2" pattern="$3" timeout="${4:-120}" elapsed=0
  while [ $elapsed -lt $timeout ]; do
    if tmux capture-pane -t "$session:$pane" -p | grep -q "$pattern"; then return 0; fi
    sleep 2; elapsed=$((elapsed + 2))
  done
  echo "TIMEOUT waiting for '$pattern' in $session:$pane" >&2; return 1
}
# Example: wait for build
tmux send-keys -t "$SESSION:0.0" "npm run build" Enter
wait_for_text "$SESSION" "0.0" "Build complete" 300
```

## Capture Pane Output

```bash
tmux capture-pane -t "$SESSION:0.0" -p              # visible content
tmux capture-pane -t "$SESSION:0.0" -p -S -          # full scrollback
tmux capture-pane -t "$SESSION:0.0" -p -S - > /tmp/pane-output.txt  # save to file
```

## Common Workflows

### Deploy with Health Monitoring
```bash
SESSION="sortiarius-deploy-$(date +%H%M%S)"
tmux new-session -d -s "$SESSION" -n "deploy"
tmux split-window -v -t "$SESSION:deploy"
tmux send-keys -t "$SESSION:deploy.0" "./deploy.sh" Enter
tmux send-keys -t "$SESSION:deploy.1" "watch -n 5 'curl -s localhost:8080/health'" Enter
```

### Parallel Claude Code Agents
```bash
SESSION="sortiarius-parallel-$(date +%H%M%S)"
for agent in agent1 agent2 agent3; do
  tmux new-window -t "$SESSION" -n "$agent" 2>/dev/null || tmux new-session -d -s "$SESSION" -n "$agent"
done
tmux send-keys -t "$SESSION:agent1" "claude --print 'Audit Key Vault policies' > /tmp/agent1.txt" Enter
tmux send-keys -t "$SESSION:agent2" "claude --print 'Check AG replica status' > /tmp/agent2.txt" Enter
tmux send-keys -t "$SESSION:agent3" "claude --print 'Review firewall rules' > /tmp/agent3.txt" Enter
# Wait for all, then aggregate
for agent in agent1 agent2 agent3; do wait_for_text "$SESSION" "$agent" "\\$" 600; done
```

## Session Management
```bash
tmux ls 2>/dev/null | grep "^sortiarius-"                                    # list sessions
tmux kill-session -t "$SESSION"                                               # kill one
tmux ls 2>/dev/null | grep "^sortiarius-" | cut -d: -f1 | xargs -I{} tmux kill-session -t {}  # kill all
```

## Notes
- Always use `-d` (detached) from Claude Code -- you cannot attach interactively.
- Use `capture-pane` and `wait_for_text` to interact programmatically.
- Clean up sessions after completion to prevent sprawl.
- Install if missing: `sudo apt install tmux` or `sudo yum install tmux`.

## Changelog
- 2026-02-06: Initial creation
