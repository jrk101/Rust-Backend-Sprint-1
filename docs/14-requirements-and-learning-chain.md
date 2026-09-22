# Requirements and Learning Chain

This document connects each project limitation to the concept and proof it
creates. It prevents a tool from appearing only because it is on a syllabus.

| Existing limitation or new requirement | Concept introduced | Practice | Shared proof |
|---|---|---|---|
| The client ask is vague | Scope, non-goals, diagrams | Unit 1 brief | Agreed brief and context map |
| Event payloads contain owned and borrowed data | Ownership, borrowing, byte slices | Unit 2 parser | Raw-body contract |
| String states permit contradictions | Structs, enums, `Result`, traits | Unit 3 model | Tested domain model |
| An event needs to cross a network | HTTP, Actix Web, async client | Units 4–5 slice | First provider → merchant request |
| Any sender could call the endpoint | HMAC over raw bytes | Unit 6 signature lab | Rejected altered and stale payloads |
| Process memory disappears | SQL, migrations, pooling | Unit 7 store | Event survives restart |
| Providers repeat and race requests | Transactions, uniqueness | Unit 8 race test | One event and one initial delivery |
| Destination may be slow or down | Tokio tasks, timeouts, bounds | Unit 9 worker | Controlled failed merchant test |
| Failed work must survive restarts | Attempt log, due-work query, locking | Units 10–11 retry | Restart and two-worker proof |
| Retries may exhaust | Dead letter, operator replay | Unit 12 failure lab | Full reliability demonstration |
| Remote operators need protected access | Authentication and authorization | Unit 13 API | Rejected unauthenticated request |
| Configurable URLs and traffic can be abused | Destination policy, limits | Unit 14 threat lab | Policy and adversarial tests |
| A working demo can still regress | CI, tracing, containers, incident analysis | Units 15–16 | Repeatable failure and fix |
| Original builders will leave | Runbook, architecture, clean start | Unit 17 | New operator succeeds alone |

## Product behavior to settle before coding

The Unit 1 cohort brief records product semantics in
`shared/delivery/brief.md`. The following questions are decisions, not silent
assumptions in a handler:

1. **Event identity:** Which source identifier plus provider event ID form the
   unique key? What response is returned for the same ID with changed bytes?
2. **Signature envelope:** Which bytes are signed? Is a timestamp included?
   What timestamp tolerance and secret rotation rule apply?
3. **Acceptance:** At what point may ingress return success: after verification
   alone, or after event and initial delivery commit together?
4. **Delivery success:** Which `2xx` responses count? Which failures are
   retryable? How are redirects handled?
5. **Attempts:** What is the maximum count, delay schedule, cap, and jitter?
   What attempt details can safely be stored?
6. **Replay:** Does replay create a new delivery or a new attempt? How is a
   repeated operator action recognized?
7. **Destination policy:** Which schemes, hosts, address ranges, ports, and
   redirects are allowed in local V1? How is DNS re-resolution checked?
8. **Operator access:** Is V1 one local operator or multi-user? What can one
   operator see or mutate?

The reviewer checks that each answer has a reason, an alternative, and a test.

## Minimum product demonstration

1. Start PostgreSQL, PayHook, mock provider, and sample merchant with only
   documented local commands.
2. Send an unsigned or altered event and observe rejection without persistence.
3. Send a valid signed event and inspect one committed event and delivery.
4. Send the same event again, including concurrently, and inspect the database
   constraint's result.
5. Configure the merchant to fail and time out, then recover. Observe each
   attempt, bounded retry time, and final success.
6. Exhaust the attempt limit, inspect dead-letter state, replay deliberately,
   and show original history remains available.
7. Restart PayHook before due work runs and show it resumes from PostgreSQL.
8. Attempt unauthenticated management access and an unsafe destination URL;
   observe the agreed rejection behavior.
9. Run fast tests, database tests, and the final clean-start test.

This is an acceptance scenario list, not an instruction to build every behavior
in the first unit. Each row becomes relevant at the unit named in the table.
