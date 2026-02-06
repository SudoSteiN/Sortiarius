#!/usr/bin/env python3
"""Sortiarius Local Dashboard — Lightweight web UI for monitoring and management.

Serves a single-page dashboard on localhost. No external dependencies (stdlib only).
Reads directly from the filesystem: agent registry, session logs, skills, memory.

Usage:
    sortiarius ui              # Start on default port 8420
    sortiarius ui --port 9000  # Custom port
"""

import json
import os
import subprocess
import sys
from datetime import datetime
from http.server import HTTPServer, SimpleHTTPRequestHandler
from pathlib import Path
from urllib.parse import parse_qs, urlparse

SORTIARIUS_HOME = os.environ.get("SORTIARIUS_HOME", os.path.expanduser("~/Sortiarius"))
PORT = 8420


class SortiariusHandler(SimpleHTTPRequestHandler):
    """Handle API requests and serve the SPA."""

    def do_GET(self):
        parsed = urlparse(self.path)
        path = parsed.path

        if path.startswith("/api/"):
            self.handle_api(path, parsed)
        elif path == "/" or path == "/index.html":
            self.serve_dashboard()
        else:
            self.send_error(404)

    def handle_api(self, path, parsed):
        """Route API requests to handlers."""
        routes = {
            "/api/agents": self.api_agents,
            "/api/skills": self.api_skills,
            "/api/memory": self.api_memory,
            "/api/logs": self.api_logs,
            "/api/health": self.api_health,
            "/api/stats": self.api_stats,
        }
        handler = routes.get(path)
        if handler:
            try:
                data = handler(parse_qs(parsed.query))
                self.send_json(data)
            except Exception as e:
                self.send_json({"error": str(e)}, status=500)
        else:
            self.send_json({"error": "Unknown endpoint"}, status=404)

    def send_json(self, data, status=200):
        """Send a JSON response."""
        body = json.dumps(data, default=str).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", len(body))
        self.send_header("Access-Control-Allow-Origin", "http://localhost:" + str(PORT))
        self.end_headers()
        self.wfile.write(body)

    def api_agents(self, params):
        """Return agent registry data."""
        registry_path = Path(SORTIARIUS_HOME) / "workspace/scratch/agent-registry.json"
        if not registry_path.exists():
            return {"agents": [], "total": 0}
        try:
            data = json.loads(registry_path.read_text())
            agents = data.get("agents", [])
            # Check PID liveness
            for a in agents:
                if a.get("status") == "running" and a.get("pid"):
                    try:
                        os.kill(int(a["pid"]), 0)
                    except (OSError, ValueError):
                        a["status"] = "failed (orphaned)"
            return {"agents": agents, "total": len(agents),
                    "running": sum(1 for a in agents if a["status"] == "running"),
                    "completed": sum(1 for a in agents if "completed" in a.get("status", "")),
                    "failed": sum(1 for a in agents if "failed" in a.get("status", ""))}
        except (json.JSONDecodeError, KeyError):
            return {"agents": [], "total": 0, "error": "Registry corrupted"}

    def api_skills(self, params):
        """Return all skills with metadata."""
        skills_dir = Path(SORTIARIUS_HOME) / "workspace/skills"
        skills = []
        if skills_dir.exists():
            for skill_dir in sorted(skills_dir.iterdir()):
                if not skill_dir.is_dir():
                    continue
                skill_file = skill_dir / "SKILL.md"
                if not skill_file.exists():
                    continue
                meta = {"name": skill_dir.name, "description": "", "triggers": [], "pipeline": "[]", "has_scripts": False}
                content = skill_file.read_text()
                for line in content.split("\n"):
                    if line.startswith("name:"):
                        meta["name"] = line.split(":", 1)[1].strip()
                    elif line.startswith("description:"):
                        meta["description"] = line.split(":", 1)[1].strip()
                    elif line.startswith("pipeline:"):
                        meta["pipeline"] = line.split(":", 1)[1].strip()
                meta["has_scripts"] = (skill_dir / "scripts").is_dir()
                # Extract triggers
                in_triggers = False
                for line in content.split("\n"):
                    if line.startswith("triggers:"):
                        in_triggers = True
                        continue
                    if in_triggers and line.startswith("  - "):
                        meta["triggers"].append(line.strip("- ").strip())
                    elif in_triggers and not line.startswith("  "):
                        in_triggers = False
                skills.append(meta)
        return {"skills": skills, "total": len(skills)}

    def api_memory(self, params):
        """Return memory file contents."""
        memory_dir = Path(SORTIARIUS_HOME) / "workspace/memory"
        files = {}
        if memory_dir.exists():
            for f in sorted(memory_dir.glob("*.md")):
                files[f.stem] = {
                    "content": f.read_text(),
                    "size": f.stat().st_size,
                    "modified": datetime.fromtimestamp(f.stat().st_mtime).isoformat(),
                }
        return {"files": files, "total": len(files)}

    def api_logs(self, params):
        """Return recent session log entries."""
        log_path = Path(SORTIARIUS_HOME) / "workspace/scratch/session-log.jsonl"
        limit = int(params.get("limit", [50])[0])
        domain_filter = params.get("domain", [None])[0]
        entries = []
        if log_path.exists():
            lines = log_path.read_text().strip().split("\n")
            for line in reversed(lines[-500:]):  # Read last 500 max
                if not line.strip():
                    continue
                try:
                    entry = json.loads(line)
                    if domain_filter and entry.get("domain") != domain_filter:
                        continue
                    entries.append(entry)
                    if len(entries) >= limit:
                        break
                except json.JSONDecodeError:
                    continue
        # Domain summary
        all_entries = []
        if log_path.exists():
            for line in log_path.read_text().strip().split("\n"):
                try:
                    all_entries.append(json.loads(line))
                except (json.JSONDecodeError, ValueError):
                    pass
        domains = {}
        for e in all_entries:
            d = e.get("domain", "general")
            domains[d] = domains.get(d, 0) + 1
        return {"entries": entries, "total_logged": len(all_entries), "domains": domains}

    def api_health(self, params):
        """Run health checks and return results."""
        checks = []
        # Symlink check
        symlink = Path.home() / ".claude" / "CLAUDE.md"
        if symlink.is_symlink():
            target = os.readlink(str(symlink))
            checks.append({"name": "Global symlink", "status": "ok", "detail": f"→ {target}"})
        else:
            checks.append({"name": "Global symlink", "status": "fail", "detail": "Not a symlink"})

        # Skills
        skills_dir = Path(SORTIARIUS_HOME) / "workspace/skills"
        skill_count = sum(1 for d in skills_dir.iterdir() if d.is_dir() and (d / "SKILL.md").exists()) if skills_dir.exists() else 0
        checks.append({"name": "Skills", "status": "ok" if skill_count > 0 else "warn", "detail": f"{skill_count} skills"})

        # Memory
        mem_dir = Path(SORTIARIUS_HOME) / "workspace/memory"
        mem_count = sum(1 for f in mem_dir.glob("*.md")) if mem_dir.exists() else 0
        checks.append({"name": "Memory", "status": "ok" if mem_count > 0 else "warn", "detail": f"{mem_count} files"})

        # Hooks
        hooks_dir = Path(SORTIARIUS_HOME) / ".claude/hooks"
        hook_count = sum(1 for f in hooks_dir.glob("*.sh")) if hooks_dir.exists() else 0
        non_exec = sum(1 for f in hooks_dir.glob("*.sh") if not os.access(str(f), os.X_OK)) if hooks_dir.exists() else 0
        if non_exec > 0:
            checks.append({"name": "Hooks", "status": "fail", "detail": f"{non_exec} non-executable"})
        else:
            checks.append({"name": "Hooks", "status": "ok", "detail": f"{hook_count} hooks"})

        # Git
        try:
            result = subprocess.run(["git", "-C", SORTIARIUS_HOME, "status", "--porcelain"],
                                    capture_output=True, text=True, timeout=5)
            dirty = len([l for l in result.stdout.strip().split("\n") if l.strip()])
            checks.append({"name": "Git", "status": "ok" if dirty == 0 else "warn",
                          "detail": f"{dirty} uncommitted changes" if dirty else "Clean"})
        except Exception:
            checks.append({"name": "Git", "status": "warn", "detail": "Could not check"})

        # Claude CLI
        claude_found = subprocess.run(["which", "claude"], capture_output=True).returncode == 0
        checks.append({"name": "Claude CLI", "status": "ok" if claude_found else "warn",
                       "detail": "Found" if claude_found else "Not in PATH"})

        # Agent launcher
        agent_bin = Path(SORTIARIUS_HOME) / "bin/sortiarius-agent"
        checks.append({"name": "Agent launcher", "status": "ok" if agent_bin.exists() else "fail",
                       "detail": "Found" if agent_bin.exists() else "Missing"})

        ok = sum(1 for c in checks if c["status"] == "ok")
        return {"checks": checks, "ok": ok, "total": len(checks)}

    def api_stats(self, params):
        """Return overall system stats."""
        stats = {}
        # Skill count
        skills_dir = Path(SORTIARIUS_HOME) / "workspace/skills"
        stats["skills"] = sum(1 for d in skills_dir.iterdir() if d.is_dir()) if skills_dir.exists() else 0

        # Memory file count + total size
        mem_dir = Path(SORTIARIUS_HOME) / "workspace/memory"
        if mem_dir.exists():
            mem_files = list(mem_dir.glob("*.md"))
            stats["memory_files"] = len(mem_files)
            stats["memory_size_kb"] = round(sum(f.stat().st_size for f in mem_files) / 1024, 1)
        else:
            stats["memory_files"] = 0
            stats["memory_size_kb"] = 0

        # Hook count
        hooks_dir = Path(SORTIARIUS_HOME) / ".claude/hooks"
        stats["hooks"] = sum(1 for f in hooks_dir.glob("*.sh")) if hooks_dir.exists() else 0

        # Session log stats
        log_path = Path(SORTIARIUS_HOME) / "workspace/scratch/session-log.jsonl"
        if log_path.exists():
            stats["log_entries"] = sum(1 for _ in open(str(log_path)))
            stats["log_size_kb"] = round(log_path.stat().st_size / 1024, 1)
        else:
            stats["log_entries"] = 0
            stats["log_size_kb"] = 0

        # Agent stats from registry
        registry_path = Path(SORTIARIUS_HOME) / "workspace/scratch/agent-registry.json"
        if registry_path.exists():
            try:
                data = json.loads(registry_path.read_text())
                agents = data.get("agents", [])
                stats["agents_total"] = len(agents)
                stats["agents_running"] = sum(1 for a in agents if a["status"] == "running")
            except (json.JSONDecodeError, KeyError):
                stats["agents_total"] = 0
                stats["agents_running"] = 0
        else:
            stats["agents_total"] = 0
            stats["agents_running"] = 0

        return stats

    def serve_dashboard(self):
        """Serve the main dashboard HTML."""
        html_path = Path(__file__).parent / "index.html"
        if html_path.exists():
            body = html_path.read_bytes()
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.send_header("Content-Length", len(body))
            self.end_headers()
            self.wfile.write(body)
        else:
            self.send_error(500, "index.html not found")

    def log_message(self, format, *args):
        """Suppress request logs unless verbose."""
        if os.environ.get("SORTIARIUS_VERBOSE"):
            super().log_message(format, *args)


def main():
    port = PORT
    for i, arg in enumerate(sys.argv[1:]):
        if arg in ("--port", "-p") and i + 2 <= len(sys.argv):
            port = int(sys.argv[i + 2])

    server = HTTPServer(("127.0.0.1", port), SortiariusHandler)
    print(f"Sortiarius Dashboard running at http://127.0.0.1:{port}")
    print("Press Ctrl+C to stop")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nStopped.")
        server.server_close()


if __name__ == "__main__":
    main()
