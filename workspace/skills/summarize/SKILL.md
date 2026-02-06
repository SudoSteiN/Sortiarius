---
name: summarize
description: Summarize URLs, documents, PDFs, git diffs, PRs, issues, and long files into structured output.
triggers:
  - summarize
  - summary
  - TL;DR
  - digest
  - recap
  - what does this say
  - explain this article
  - summarize this PR
  - summarize this diff
pipeline: []
---

# Summarize Skill

Produce structured summaries from any content source. Output is always in three tiers:
1. **TL;DR** -- 1-2 sentence executive summary
2. **Key Points** -- 3-7 bullet points
3. **Details** -- Expanded analysis (only when explicitly requested)

## Content Source Routing

### URLs / Articles
```bash
# Fetch the page content, strip HTML
curl -sL "<URL>" | sed 's/<[^>]*>//g' | head -c 50000
```
If `curl` output is too noisy, try `lynx -dump "<URL>"` or `w3m -dump "<URL>"` as fallbacks.
Feed the cleaned text to yourself for summarization.

### PDFs
Attempt in this order:
1. Use the Read tool directly on the PDF path (Claude Code supports PDF reading).
2. Fallback: `pdftotext <file> -` to pipe text to stdout.
3. For large PDFs, read page ranges: `pdftotext -f <start> -l <end> <file> -`.
Summarize each chunk, then produce a combined summary.

### Git Diffs
```bash
# Unstaged changes
git diff

# Staged changes
git diff --cached

# Between branches or commits
git diff <base>..<head> --stat   # overview first
git diff <base>..<head>          # full diff
```
Summarize: what changed, why it likely changed, and what to watch for in review.

### Pull Requests / Issues
```bash
# PR summary with diff
gh pr view <number> --json title,body,additions,deletions,files
gh pr diff <number>

# Issue details
gh issue view <number> --json title,body,comments
```
Summarize: the goal, the approach, open questions, and review risk areas.

### Long Files
For files exceeding ~2000 lines:
1. Read the first 200 lines for structure/headers.
2. Chunk the file into ~500-line segments using `offset` and `limit` on Read.
3. Summarize each chunk independently.
4. Produce a combined summary with section references (line numbers).

## Output Format

```markdown
## TL;DR
[1-2 sentences capturing the essence]

## Key Points
- [Point 1]
- [Point 2]
- [Point 3]

## Details (if requested)
[Expanded analysis organized by topic or section]
```

## Options

The user may specify:
- **Audience**: technical, executive, non-technical -- adjust jargon level accordingly.
- **Focus**: security, performance, architecture, cost -- emphasize that lens.
- **Length**: brief (TL;DR only), standard (TL;DR + Key Points), full (all three tiers).

Default is standard length, technical audience, no specific focus.

## Edge Cases
- If a URL returns 403/404, report it and suggest alternatives (cached version, archive.org).
- If content is empty or too short, say so rather than hallucinating a summary.
- For binary files or unsupported formats, report the limitation clearly.
- Rate-limit awareness: if fetching multiple URLs, add a 1-second delay between requests.

## Changelog
- 2026-02-06: Initial creation
