# Tools Guidance

## Shell Commands
- Default to PowerShell for Windows/Azure operations
- Use bash for general system tasks
- Always include error handling

## File Operations
- Create backups before modifying config files
- Use atomic writes where possible

## Browser
- Use for Azure Portal only when CLI isn't sufficient
- Prefer semantic snapshots over screenshots

## Dangerous Operations
These require explicit confirmation:
- Deleting resources (VMs, storage, databases)
- Modifying production configurations
- Running scripts without -WhatIf first
- Any operation affecting multiple resources
