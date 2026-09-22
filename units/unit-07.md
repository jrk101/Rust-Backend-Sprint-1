# Unit 7 — Transactions and duplicate safety

## By the end of this unit you can

- explain why check-then-insert is unsafe under concurrency;
- use a transaction and unique constraint to protect event identity;
- distinguish an application pre-check from the final database guarantee;
- test simultaneous duplicate requests.

## 1 · Durable ingestion

- [ ] **Task 1 — Define the ingestion transaction** (`D2.1`): Write the steps
  that verify, insert an event, and create its delivery. Name what must commit
  together and what must stay outside the transaction.
  **Primary path:** `practice/<student-id>/unit-07/design/ingestion-transaction.md`.
- [ ] **Task 2 — Enforce provider identity** (`D2.2`): Add the correct composite
  unique constraint and handle the conflict deliberately. State what response a
  duplicate receives and what information it may safely reveal.
  **Primary path:** `practice/<student-id>/unit-07/deduplication/`.
- [ ] **Task 3 — Attack it concurrently** (`D2.3`): Send the same signed event
  concurrently and prove one event row and one initial delivery exist.
  **Primary path:** `practice/<student-id>/unit-07/tests/concurrent-dedup.rs`.
- [ ] **Task 4 — Review the durable-ingestion design** (`E5.1`): Update the
  architecture, data model, sequence diagram, and decision record based on real
  implementation evidence. **Primary path:**
  `practice/<student-id>/unit-07/design/durable-ingestion-review.md`.

## Durable ingestion [MILESTONE]

The shared build may proceed only after signature verification, transaction
boundaries, uniqueness, and concurrency evidence are agreed.

**If short on time, cut:** diagram polish. Never cut the concurrent duplicate test.

Next: `unit-08.md`.

