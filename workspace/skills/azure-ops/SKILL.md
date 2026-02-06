---
name: azure-ops
description: Azure infrastructure operations using PowerShell and Az CLI
triggers:
  - azure
  - resource group
  - key vault
  - app service
  - sql azure
  - virtual machine
  - storage account
pipeline: []
---

# Azure Operations Skill

## Prerequisites Check
Before any Azure operation:
```powershell
# Verify connected
Get-AzContext

# Verify correct subscription
Get-AzContext | Select-Object -ExpandProperty Subscription
```

## Common Patterns

### Resource Group Operations
```powershell
# List resources in a group
Get-AzResource -ResourceGroupName "RG-NAME" | Select-Object Name, ResourceType

# Check resource group tags
Get-AzResourceGroup -Name "RG-NAME" | Select-Object -ExpandProperty Tags
```

### Key Vault Operations
```powershell
# List secrets (names only, not values)
Get-AzKeyVaultSecret -VaultName "VAULT-NAME" | Select-Object Name, Enabled, Created

# Get secret value (be careful with output)
$secret = Get-AzKeyVaultSecret -VaultName "VAULT-NAME" -Name "SECRET-NAME" -AsPlainText

# Check access policies
Get-AzKeyVault -VaultName "VAULT-NAME" | Select-Object -ExpandProperty AccessPolicies
```

### SQL Database Operations
```powershell
# List SQL servers
Get-AzSqlServer -ResourceGroupName "RG-NAME"

# Check database status
Get-AzSqlDatabase -ServerName "SERVER" -ResourceGroupName "RG-NAME" |
    Select-Object DatabaseName, Status, CurrentServiceObjectiveName

# Check elastic pool
Get-AzSqlElasticPool -ServerName "SERVER" -ResourceGroupName "RG-NAME"
```

## Safety Rules
1. Always use -WhatIf first for destructive operations
2. Generate rollback commands before applying changes
3. Check resource locks before modifications
4. Verify you're in the correct subscription

## Error Handling
```powershell
try {
    # Azure operation
} catch {
    Write-Error "Operation failed: $($_.Exception.Message)"
    # Log to incident channel if production
}
```

## Changelog
<!-- One-line entries: YYYY-MM-DD description -->
