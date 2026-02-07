---
name: data-analytics
description: Data analysis and querying using CLI tools, database CLIs, and MCPs — no manual SQL
triggers:
  - data analysis
  - query database
  - analytics
  - pull metrics
  - bq
  - psql
  - sqlcmd
  - database query
  - dashboard data
pipeline: []
---

# Data & Analytics Skill

## When to Use
- Querying databases (SQL Server, PostgreSQL, SQLite, BigQuery)
- Pulling and analyzing metrics
- Building data pipelines or dbt models
- Boris tip #9: "Use Claude with the bq CLI (or any database CLI/MCP/API) to pull and analyze metrics."

## Process

### Step 1: Identify the Data Source
Determine which CLI or MCP to use:

| Source | CLI Tool | Notes |
|--------|----------|-------|
| SQL Server | `sqlcmd` | Azure SQL or on-prem |
| PostgreSQL | `psql` | Standard postgres CLI |
| SQLite | `sqlite3` | Local/embedded databases |
| BigQuery | `bq` | Google Cloud |
| MySQL | `mysql` | Standard MySQL CLI |
| Azure Data | `az sql`, `az cosmosdb` | Azure-specific |
| MCP Server | Tool-specific | Use available MCP tools |

### Step 2: Safety First
- **Read-only by default.** Never run INSERT/UPDATE/DELETE without explicit confirmation.
- Check if this is a production database — if so, use READ REPLICA if available.
- For SQL Server: always use `SET NOCOUNT ON; SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED;` for analytics queries.
- For large datasets: add `TOP 100` or `LIMIT 100` to preview before running full query.

### Step 3: Write and Execute the Query
Don't ask Justin to write SQL. Write it yourself based on the question.

Example flow:
1. Justin: "How many users signed up last month?"
2. Explore schema: `SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES`
3. Find relevant table: `SELECT TOP 5 * FROM Users`
4. Write query: `SELECT COUNT(*) FROM Users WHERE CreatedDate >= DATEADD(month, -1, GETDATE())`
5. Format result: present as markdown table or summary

### Step 4: Format Output
Choose format based on data:
- **Single number:** State it directly
- **Small table (< 20 rows):** Markdown table
- **Large dataset:** Summarize with key statistics (count, min, max, avg, distribution)
- **Time series:** ASCII chart or suggest HTML visualization
- **Complex analysis:** Generate a self-contained HTML report

### ASCII Charts for Quick Visualization
```
Users per month (2026):
Jan  ████████████████████ 2,450
Feb  ██████████████████████ 2,890
Mar  ████████████████████████████ 3,210
Apr  ███████████████████████████████ 3,650
```

### Database Schema Discovery
When first connecting to an unknown database:
```sql
-- SQL Server
SELECT s.name AS [schema], t.name AS [table],
       SUM(p.rows) AS row_count
FROM sys.tables t
JOIN sys.schemas s ON t.schema_id = s.schema_id
JOIN sys.partitions p ON t.object_id = p.object_id AND p.index_id IN (0,1)
GROUP BY s.name, t.name
ORDER BY row_count DESC;

-- PostgreSQL
SELECT schemaname, tablename, n_tup_ins AS approx_rows
FROM pg_stat_user_tables
ORDER BY n_tup_ins DESC;
```

## Integration with Memory
After significant data analyses:
- Log common queries to `memory/solutions.md`
- Record database schemas and relationships discovered
- Note performance patterns (slow queries, missing indexes)

## Changelog
- 2026-02-07: Initial creation — data analysis using CLI tools
