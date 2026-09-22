# Unit 9 — Persisted retries and work claiming

## By the end of this unit you can

- classify delivery outcomes into success, retry, and terminal failure;
- calculate bounded exponential backoff with jitter;
- query and claim due work safely with multiple workers;
- restart the service without losing scheduled work.

## 1 · Failure policy

- [ ] **Task 1 — Model attempts and next action** (`D3.1`): Define the stored
  attempt record, sanitized response fields, next-attempt time, and terminal
  reason. **Primary path:**
  `practice/<student-id>/unit-09/design/attempt-model.md`.
- [ ] **Task 2 — Implement backoff and jitter** (`A4.1`): Write a pure function
  with a maximum delay and injectable randomness/time for deterministic tests.
  Explain why jitter exists. **Primary path:**
  `practice/<student-id>/unit-09/retry-policy/`.
- [ ] **Task 3 — Index due work** (`D3.2`): Write the due-delivery query and
  justify the supporting index from its filter and ordering.
  **Primary path:** `practice/<student-id>/unit-09/database/due-work.sql`.
- [ ] **Task 4 — Claim work safely** (`D3.3`): Compare optimistic claims with a
  PostgreSQL row-locking approach such as `FOR UPDATE SKIP LOCKED`. Prove that
  two workers do not own the same attempt concurrently.
  **Primary path:** `practice/<student-id>/unit-09/work-claiming/`.
- [ ] **Task 5 — Survive a restart** (`A4.2`): Stop PayHook after scheduling a
  retry, restart it, and show the persisted delivery becomes eligible and runs.
  **Primary path:** `practice/<student-id>/unit-09/restart-proof.md`.

## Focused DSA lab

- [ ] **Task 6 — Compare scheduling structures** (`R4.1`): Implement a small
  priority-queue scheduler using `BinaryHeap`, then explain why the production
  system still queries PostgreSQL for durable due work.
  **Primary path:** `practice/<student-id>/unit-09/priority-queue-lab/`.

**If short on time, cut:** Task 6, then benchmark polish. Never cut the restart
proof or multiple-worker claim test.

Next: `unit-10.md`.

