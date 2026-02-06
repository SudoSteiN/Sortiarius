---
name: problem-modeling
description: Model complex problems before solving them. Use when tasks are ambiguous, multi-step, or high-stakes.
triggers:
  - complex problem
  - need to think through
  - multiple steps
  - not sure how to approach
pipeline: []
---

# Problem Modeling Skill

Use this methodology when facing complex or ambiguous problems.

## When to Use
- Task has 3+ steps
- Requirements are ambiguous
- Multiple valid approaches exist
- Failure would be costly
- You've failed at this type of problem before

## Problem Model Template

Before solving, fill out this model:

### 1. Entities
What actors, objects, resources, or services are involved?
```yaml
entities:
  - name: [name]
    type: actor | resource | service
    properties: [relevant attributes]
```

### 2. Current State
What is true right now?
```yaml
state:
  - variable: [name]
    value: [current value]
    valid_range: [constraints]
```

### 3. Goal State
What does success look like?
```yaml
goal:
  - condition: [what must be true]
    required: true | false
```

### 4. Actions Available
What can we do?
```yaml
actions:
  - name: [action]
    preconditions: [what must be true first]
    effects: [what changes]
    risks: [what could go wrong]
```

### 5. Constraints
What must remain true throughout?
```yaml
constraints:
  - type: invariant | temporal | resource
    rule: [the constraint]
    severity: warning | error | fatal
```

### 6. Assumptions
What are we assuming? (Flag if confidence < 0.7)
```yaml
assumptions:
  - assumption: [what we're assuming]
    confidence: [0.0-1.0]
    validation: [how to verify]
```

## Validation Checklist
Before proceeding to solution:
- [ ] All entities identified
- [ ] Current state is accurate
- [ ] Goal is measurable
- [ ] Constraints won't be violated
- [ ] Low-confidence assumptions flagged

## After Solving
If the solution fails, check:
1. Was the model wrong? (Go back to modeling)
2. Was the execution wrong? (Fix the approach)

## Changelog
<!-- One-line entries: YYYY-MM-DD description -->
