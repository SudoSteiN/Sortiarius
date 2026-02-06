---
name: incident-response
description: Handle production incidents systematically
triggers:
  - incident
  - outage
  - production issue
  - alert
  - pages
  - emergency
pipeline: []
---

# Incident Response Skill

## Immediate Actions (First 5 minutes)
1. **Assess scope:** What's affected? How many users?
2. **Check dashboards:** Azure Monitor, Application Insights
3. **Recent changes:** Any deployments in last 24 hours?
4. **Communicate:** Post to incident channel

## Diagnostic Commands

### SQL Server Health
```powershell
# Check AG status
Invoke-Sqlcmd -Query "SELECT * FROM sys.dm_hadr_availability_replica_states" -ServerInstance "SERVER"

# Check blocking
Invoke-Sqlcmd -Query "EXEC sp_who2" -ServerInstance "SERVER" | Where-Object { $_.BlkBy -ne '  .' }

# Check long-running queries
Invoke-Sqlcmd -Query @"
SELECT TOP 10
    r.session_id, r.start_time, r.status, r.command,
    t.text as query_text,
    r.wait_type, r.wait_time
FROM sys.dm_exec_requests r
CROSS APPLY sys.dm_exec_sql_text(r.sql_handle) t
WHERE r.session_id > 50
ORDER BY r.start_time
"@ -ServerInstance "SERVER"
```

### Azure Resource Health
```powershell
# Check resource health
Get-AzResourceHealth -ResourceGroupName "RG-NAME"

# Check App Service status
Get-AzWebApp -ResourceGroupName "RG-NAME" | Select-Object Name, State, DefaultHostName

# Check recent activity log
Get-AzActivityLog -ResourceGroupName "RG-NAME" -StartTime (Get-Date).AddHours(-2) |
    Where-Object { $_.Status.Value -ne "Succeeded" }
```

## Escalation Path
1. Try standard remediation (restart, failover)
2. If no improvement in 15 min → Escalate to team lead
3. If customer-facing impact → Notify Satya
4. Document everything in incident ticket

## Post-Incident
- Write incident summary
- Identify root cause
- Create action items
- Update runbooks if needed

## Changelog
<!-- One-line entries: YYYY-MM-DD description -->
