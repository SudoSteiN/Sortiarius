---
name: powershell-automation
description: Generate production-quality PowerShell scripts
triggers:
  - powershell script
  - automate
  - automation
  - scheduled task
pipeline: []
---

# PowerShell Automation Skill

## Script Template
```powershell
#Requires -Version 7.0
#Requires -Modules @{ ModuleName="Az.Accounts"; ModuleVersion="2.0.0" }

<#
.SYNOPSIS
    [Brief description]
.DESCRIPTION
    [Detailed description]
.PARAMETER ParameterName
    [Parameter description]
.EXAMPLE
    .\Script-Name.ps1 -ParameterName "Value"
.NOTES
    Author: Justin (via AI Assistant)
    Date: [Date]
    Version: 1.0
#>

[CmdletBinding(SupportsShouldProcess)]
param (
    [Parameter(Mandatory)]
    [string]$RequiredParam,

    [Parameter()]
    [string]$OptionalParam = "DefaultValue"
)

#region Configuration
$ErrorActionPreference = 'Stop'
$VerbosePreference = 'Continue'
#endregion

#region Functions
function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    Write-Verbose "[$timestamp] [$Level] $Message"
}
#endregion

#region Main
try {
    Write-Log "Starting script execution"

    # Main logic here

    Write-Log "Script completed successfully"
} catch {
    Write-Log "Script failed: $($_.Exception.Message)" -Level "ERROR"
    throw
} finally {
    # Cleanup
}
#endregion
```

## Standards
- Use approved verbs only (Get-, Set-, New-, Remove-, etc.)
- Include comment-based help
- Support -WhatIf for destructive operations
- Use Write-Verbose, not Write-Host
- Include proper error handling
- Log to a standard location

## Microsoft Graph API Pattern
```powershell
# Connect with app registration
$tenantId = "TENANT-ID"
$clientId = "CLIENT-ID"
$clientSecret = Get-AzKeyVaultSecret -VaultName "VAULT" -Name "GraphSecret" -AsPlainText

$body = @{
    grant_type    = "client_credentials"
    client_id     = $clientId
    client_secret = $clientSecret
    scope         = "https://graph.microsoft.com/.default"
}

$tokenResponse = Invoke-RestMethod -Uri "https://login.microsoftonline.com/$tenantId/oauth2/v2.0/token" -Method Post -Body $body
$headers = @{ Authorization = "Bearer $($tokenResponse.access_token)" }

# Handle pagination
$results = @()
$uri = "https://graph.microsoft.com/v1.0/users"
do {
    $response = Invoke-RestMethod -Uri $uri -Headers $headers
    $results += $response.value
    $uri = $response.'@odata.nextLink'
} while ($uri)
```

## Changelog
<!-- One-line entries: YYYY-MM-DD description -->
