---
name: contrastive-scoring
description: Compare multiple approaches before choosing one
triggers:
  - multiple options
  - which approach
  - compare solutions
  - trade-offs
---

# Contrastive Scoring Skill

Use when multiple valid approaches exist.

## Process
1. Generate at least 2 approaches
2. Score each on 4 dimensions
3. Normalize and compare
4. Document the decision

## Scoring Dimensions

| Dimension | Question | Weight |
|-----------|----------|--------|
| Feasibility | Can we actually do this? | 0.3 |
| Simplicity | Is this the simplest solution? | 0.25 |
| Robustness | Does it handle edge cases? | 0.25 |
| Alignment | Does it solve the REAL problem? | 0.2 |

## Scoring Template
```markdown
### Approach A: [Name]
- Description: [What is this approach?]
- Feasibility: [1-10] - [Why?]
- Simplicity: [1-10] - [Why?]
- Robustness: [1-10] - [Why?]
- Alignment: [1-10] - [Why?]
- **Weighted Score:** [Calculate]

### Approach B: [Name]
- Description: [What is this approach?]
- Feasibility: [1-10] - [Why?]
- Simplicity: [1-10] - [Why?]
- Robustness: [1-10] - [Why?]
- Alignment: [1-10] - [Why?]
- **Weighted Score:** [Calculate]

### Decision
Choosing [A/B] because: [Reasoning]
Trade-off accepted: [What we're giving up]
```

## When Scores Are Close
If approaches are within 10% of each other:
- Default to simpler option
- Or ask Justin for preference
