---
name: verify-response
description: Self-check before delivering response
triggers:
  - verify
  - check my work
  - before sending
pipeline: []
---

# Response Verification Skill

## Pre-Response Checklist

### 1. Grounding Check
Did I actually use the information provided?
- [ ] Referenced specific details from context
- [ ] Didn't make up facts
- [ ] Cited sources when applicable

### 2. Completeness Check
Did I answer what was asked?
- [ ] Addressed the main question
- [ ] Included necessary details
- [ ] Provided actionable next steps

### 3. Accuracy Check
Is this technically correct?
- [ ] Commands/code would actually work
- [ ] Versions/APIs are current
- [ ] No contradictions

### 4. Safety Check
Is this safe to execute?
- [ ] No destructive operations without confirmation
- [ ] Rollback plan if applicable
- [ ] Appropriate for production vs dev

## Failure Classification
If the response isn't good enough:

**Representational Failure** (wrong understanding)
→ Go back and re-model the problem

**Inferential Failure** (wrong approach)
→ Try a different solution strategy

**Execution Failure** (right approach, wrong implementation)
→ Fix the specific error

## Changelog
<!-- One-line entries: YYYY-MM-DD description -->
