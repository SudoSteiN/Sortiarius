# Memory

This file is SteinBot's persistent knowledge. It grows over time as the assistant
learns from interactions. Edit it directly or let SteinBot update it during sessions.
Run `stein sync` to push changes to git.

---

## Environment

Fill these in so SteinBot has context about your infrastructure.
Delete the example values and replace with your own.

```yaml
azure:
  subscription_id: ""           # e.g., "12345678-abcd-..."
  tenant_id: ""
  primary_region: "eastus"
  resource_groups:              # List your most-used RGs
    - name: ""
      purpose: ""
  key_vaults:
    - name: ""
      resource_group: ""
  sql_servers:
    - name: ""
      resource_group: ""
      has_ag: false             # Always On Availability Group?

alerts:
  channels: []                  # Slack/Teams channels for production alerts
```

---

## Learned Preferences

- Justin prefers PowerShell over Az CLI for complex automation
- Use -WhatIf before destructive operations
- Always generate rollback scripts for production changes

---

## Past Solutions

<!-- SteinBot adds entries here as problems get solved -->
<!-- Format: date, problem, solution, outcome -->

---

## Patterns That Work

<!-- Approaches and patterns that proved effective -->

---

## Things That Failed

<!-- Track what didn't work and WHY so we don't repeat mistakes -->
<!-- Format: date, what was tried, why it failed, what worked instead -->
