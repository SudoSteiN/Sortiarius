---
name: testing
description: Testing strategy, patterns, and workflows for unit, integration, and TDD
triggers:
  - write tests
  - test
  - testing
  - unit test
  - integration test
  - TDD
  - test coverage
  - spec
pipeline: []
---

# Testing Skill

Practical testing patterns and workflows. Apply based on the project's stack and testing framework.

## Test File Naming and Organization

- **Unit tests:** Co-locate as `[module].test.ts` next to source files
- **Integration tests:** Separate directory at `tests/integration/[module].integration.test.ts`
- **Fixtures:** `tests/fixtures/` for shared test data
- **Helpers:** `tests/helpers/` for shared setup/teardown

## Unit Test Pattern: Arrange-Act-Assert

```typescript
describe('AuthService', () => {
  describe('validateToken', () => {
    it('should return user when token is valid', () => {
      // Arrange — set up inputs and dependencies
      const token = createTestJWT({ userId: '123', exp: future() });
      const mockUserRepo = { findById: jest.fn().mockResolvedValue(testUser) };
      const service = new AuthService(mockUserRepo);
      // Act — call the function under test
      const result = await service.validateToken(token);
      // Assert — verify the outcome
      expect(result).toEqual(testUser);
      expect(mockUserRepo.findById).toHaveBeenCalledWith('123');
    });
  });
});
```

## What to Test (and What Not To)

**Always test:** Business logic, conditional branches, error handling paths, edge cases (null, empty, boundaries), state transitions.

**Skip:** Framework boilerplate, simple getters/setters, third-party internals, private methods.

## Mocking Strategies

| What to Mock | How |
|---|---|
| External APIs | Mock HTTP client or use MSW for request interception |
| Database | Mock the repository layer, NOT the DB driver |
| Time/dates | `jest.useFakeTimers()` or inject a clock |
| File system | Mock `fs` module or use in-memory filesystem |
| Environment | Set `process.env` in `beforeEach`, restore in `afterEach` |

**Rule:** Mock at boundaries, not internals. If you're mocking within the same module, refactor the design.

## Integration Tests

Test that modules work together. Use real (test) database, seed data in `beforeAll`, tear down in `afterAll`. Test full request-response cycles via supertest or equivalent. Cover happy path AND error responses.

## TDD Workflow

When TDD is requested, follow strictly:
1. **Red:** Write a failing test for the next small behavior
2. **Green:** Write the minimum code to make it pass
3. **Refactor:** Clean up without changing behavior, ensure tests still pass
4. Repeat. Do NOT write production code before the test.

## Coverage Targets

| Type | Target | Notes |
|---|---|---|
| Business logic | 90%+ | Core domain must be thoroughly tested |
| API endpoints | 80%+ | Happy path + error cases |
| Utilities | 80%+ | Edge cases matter here |
| UI components | 60%+ | Test behavior, not rendering details |
| Config/glue code | Skip | No value in testing wiring |

Focus on **branch coverage**, not just line coverage. Run: `npx jest --coverage`.

## Snapshot Tests vs Assertion Tests

**Snapshots:** Only for large, stable serialized output (API response shapes, config objects).
**Assertions:** Everything else. When a snapshot breaks, read the diff -- don't auto-update.

## Changelog
- 2026-02-06: Initial creation
