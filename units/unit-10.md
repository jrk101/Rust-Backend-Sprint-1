# Unit 10: Retry policy and due work

## Before you start

Bring the Unit 9 attempt result and a database with failed deliveries. Read
PostgreSQL index and Tokio time sections in `docs/09-resources.md`. Decide
the retryable response classes and the maximum attempt count in a cohort
decision record before running the worker.

## At a glance

| Learn | Use immediately |
|---|---|
| Attempt state and retry classes | `D3.1` |
| Backoff, jitter, deterministic time | `A4.1` |
| Due-work query and index | `D3.2` |

## By the end of this unit you can

- classify delivery outcomes into success, retry, and terminal failure;
- calculate bounded exponential backoff with jitter;
- query due work efficiently from persistent state.

## 1 · Failure policy

- [ ] **Task 1: Model attempts and next action** (`D3.1`): Define the stored
  attempt record, sanitized response fields, next-attempt time, and terminal
  reason. **Primary path:**
  `practice/<student-id>/unit-10/design/attempt-model.md`.
- [ ] **Task 2: Implement backoff and jitter** (`A4.1`): Write a pure function
  with a maximum delay and injectable randomness/time for deterministic tests.
  Explain why jitter exists. **Primary path:**
  `practice/<student-id>/unit-10/retry-policy/`.
- [ ] **Task 3: Index due work** (`D3.2`): Write the due-delivery query and
  justify the supporting index from its filter and ordering.
  **Primary path:** `practice/<student-id>/unit-10/database/due-work.sql`.

**If short on time, cut:** benchmark polish. Never cut the persisted attempt
record, capped policy, or due-work query.

## End-of-unit checklist

- [ ] Retry times are reproducible under a fixed test clock and randomness
- [ ] Due-work query returns only eligible deliveries in deadline order
- [ ] Shared rotation records the retry policy and due-work query

Next: `unit-11.md`.
