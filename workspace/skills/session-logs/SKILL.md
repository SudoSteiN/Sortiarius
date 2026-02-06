---
name: session-logs
description: Query and analyze session logs from the learning-tracker hook. Surface usage patterns, trends, errors, and activity reports.
triggers:
  - session log
  - session history
  - command history
  - what did I do
  - analyze session
  - usage stats
  - activity log
  - what have I been working on
pipeline: []
---

# Session Logs Skill

Analyze `workspace/scratch/session-log.jsonl` to surface patterns, trends, and insights.

## Log Location

```bash
LOG="$HOME/Sortiarius/workspace/scratch/session-log.jsonl"
# Fallback: ./workspace/scratch/session-log.jsonl
```

Each line is a JSON object with fields: `timestamp`, `command`, `exit_code`, `domain`, `session_id`, `cwd`.

## Common Queries

### Recent Activity
```bash
tail -20 "$LOG" | jq -r '[.timestamp, .command] | @tsv'
# Today only
jq -r "select(.timestamp | startswith(\"$(date +%Y-%m-%d)\")) | [.timestamp, .command] | @tsv" "$LOG"
# This week
WEEK_START=$(date -d "last monday" +%Y-%m-%d 2>/dev/null || date -v-monday +%Y-%m-%d)
jq -r "select(.timestamp >= \"$WEEK_START\") | [.timestamp, .command] | @tsv" "$LOG"
```

### Filter by Domain
```bash
jq 'select(.domain == "azure")' "$LOG"
jq 'select(.command | test("pwsh|powershell"; "i"))' "$LOG"
jq 'select(.command | test("^git "))' "$LOG"
```

### Error Analysis
```bash
jq 'select(.exit_code != 0 and .exit_code != null)' "$LOG"
# Error rate by domain
jq -s 'group_by(.domain) | map({domain: .[0].domain, total: length, errors: [.[] | select(.exit_code != 0 and .exit_code != null)] | length})' "$LOG"
# Top 10 failures
jq -s '[.[] | select(.exit_code != 0)] | group_by(.command) | map({command: .[0].command, count: length}) | sort_by(-.count) | .[:10]' "$LOG"
```

### Frequency & Trends
```bash
# Most used commands
jq -r '.command' "$LOG" | sort | uniq -c | sort -rn | head -10
# Activity by hour
jq -r '.timestamp' "$LOG" | cut -dT -f2 | cut -d: -f1 | sort | uniq -c | sort -rn
# Domains this week
jq -r "select(.timestamp >= \"$WEEK_START\") | .domain // \"unknown\"" "$LOG" | sort | uniq -c | sort -rn
```

## Export to Markdown Report

When asked for a report, produce this structure:

```markdown
# Session Activity Report
**Period:** [start] to [end]
## Summary
- Total commands: X | Unique sessions: Y | Error rate: Z%
## Top Domains
| Domain | Commands | Errors |
|--------|----------|--------|
## Most Used Commands
1. `command` (N times)
## Error Hotspots
- [command] failed X times -- likely cause: [pattern]
## Work Patterns
- Most active hour: [hour] | Most active day: [day]
- Primary focus this week: [domain]
```

## Missing Logs

If the log file does not exist or is empty:
1. Check both `~/Sortiarius/workspace/scratch/` and local project scratch dir.
2. Inform Justin the `learning-tracker.sh` hook may not be active.
3. Verify: `cat ~/.claude/settings.json | jq '.hooks'`

## Changelog
- 2026-02-06: Initial creation
