# Unit 10 — Dead letters, replay, and management API

## By the end of this unit you can

- move exhausted work into an explicit dead-letter state;
- replay without erasing history or creating uncontrolled duplicates;
- expose paginated management endpoints for events and attempts;
- demonstrate the complete reliability loop.

## 1 · Complete the reliability state machine

- [ ] **Task 1 — Enter dead-letter state** (`D4.1`): Apply the attempt limit
  atomically, store the terminal reason, and ensure ordinary workers no longer
  select the delivery. **Primary path:**
  `practice/<student-id>/unit-10/dead-letter/`.
- [ ] **Task 2 — Design safe replay** (`D4.2`): Decide whether replay creates a
  new delivery or changes an existing one, record operator intent, and make
  repeated replay requests idempotent. **Primary path:**
  `practice/<student-id>/unit-10/design/replay-decision.md` and implementation.
- [ ] **Task 3 — Build inspection endpoints** (`A5.1`): List and fetch events,
  deliveries, and attempts with stable pagination, filtering, and bounded page
  sizes. **Primary path:** `practice/<student-id>/unit-10/management-api/`.
- [ ] **Task 4 — Protect response data** (`A5.2`): Define which payload,
  headers, URLs, and error details are returned or redacted.
  **Primary path:** `practice/<student-id>/unit-10/design/data-exposure.md`.
- [ ] **Task 5 — Demonstrate the reliability loop** (`E7.1`): Run
  `500 → timeout → 500 → dead letter → replay → 200`, preserving every attempt.
  **Primary path:** `practice/<student-id>/unit-10/reliability-demo.md`.

## Reliable delivery core [MILESTONE]

The gate passes when the flow works after process restarts and the recorded
history explains every transition.

**If short on time, cut:** secondary filters and response presentation. Never
cut attempt history, dead-letter state, replay safety, or the demonstration.

Next: `unit-11.md`.

