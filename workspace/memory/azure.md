# Azure Memory

Environment details and patterns for Azure infrastructure.

---

## Environment

Fill these in so SteinBot has context about your infrastructure.

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

## Patterns That Work

<!-- Format: <!-- learned: YYYY-MM-DD --> description -->

---

## Things That Failed

<!-- Format: <!-- learned: YYYY-MM-DD --> what was tried, why it failed, what worked instead -->
