---
name: healthcheck
description: Multi-phase infrastructure and host health audit with risk-rated report output.
triggers:
  - healthcheck
  - health check
  - system audit
  - security audit
  - host audit
  - server check
  - system status
  - machine health
pipeline: []
---

# Healthcheck Skill

Run a structured multi-phase audit. Each phase is independent -- if one fails, continue. Produce a risk-rated report at the end.

## Phase 1: System Context
```bash
hostname -f 2>/dev/null || hostname; uname -a
cat /etc/os-release 2>/dev/null | head -5
uptime; nproc; free -h; df -h --total 2>/dev/null || df -h
cat /proc/loadavg 2>/dev/null
```
**Thresholds:** Disk > 85% = WARNING, > 95% = CRITICAL. Memory > 90% = WARNING. Load > nproc = WARNING.

## Phase 2: Network
```bash
ip addr show 2>/dev/null || ifconfig
ip route show default 2>/dev/null
dig +short google.com 2>/dev/null || nslookup google.com 2>/dev/null
curl -so /dev/null -w "%{http_code}" --max-time 5 https://google.com
ss -tlnp 2>/dev/null || netstat -tlnp 2>/dev/null
```
**Thresholds:** No default route = CRITICAL. DNS failure = CRITICAL. Unexpected public ports = WARNING.

## Phase 3: Services
```bash
systemctl list-units --state=failed 2>/dev/null
systemctl list-units --state=running --type=service --no-pager 2>/dev/null | head -30
ps aux --sort=-%mem | head -15
```
**Thresholds:** Failed systemd units = WARNING. Zombie processes = WARNING.

## Phase 4: Security
```bash
getent group sudo wheel 2>/dev/null
grep -E "^(PermitRootLogin|PasswordAuthentication|Port)" /etc/ssh/sshd_config 2>/dev/null
ufw status 2>/dev/null || firewall-cmd --state 2>/dev/null || iptables -L -n 2>/dev/null | head -20
last -5 2>/dev/null
```
**Thresholds:** PermitRootLogin yes = WARNING. No firewall = WARNING.

## Phase 5: Package Updates
```bash
apt list --upgradable 2>/dev/null | tail -n +2 | wc -l
apt list --upgradable 2>/dev/null | grep -i security | wc -l
yum check-update 2>/dev/null | grep -c "\.x86_64\|\.noarch" || true
```
**Thresholds:** > 20 pending = WARNING. Any security updates pending = WARNING.

## Phase 6: Docker (if present)
```bash
if command -v docker &>/dev/null; then
  docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" 2>/dev/null
  docker ps -a --filter "status=exited" --format "{{.Names}}: {{.Status}}" 2>/dev/null
  docker system df 2>/dev/null
else echo "Docker not installed -- skipping"; fi
```
**Thresholds:** Exited containers = INFO. Docker disk > 10GB = WARNING.

## Report Format

```markdown
# Host Health Report
**Host:** [hostname]  |  **OS:** [os]  |  **Time:** [timestamp]
## Risk Summary
| Rating   | Count | Items |
|----------|-------|-------|
| CRITICAL | 0     |       |
| WARNING  | 2     | Disk 87%, 3 failed units |
| INFO     | 1     | 2 exited containers |
| OK       | 4     | Network, Security, Packages, System |
## [Phase Name] [OK/WARNING/CRITICAL]
[findings per phase]
## Recommended Actions
1. [Highest priority first]
```

## Notes
- All commands use `2>/dev/null` to suppress permission errors.
- If running without sudo, note limited checks and offer to re-run elevated.
- Do not auto-store in memory -- let Justin decide.

## Changelog
- 2026-02-06: Initial creation
