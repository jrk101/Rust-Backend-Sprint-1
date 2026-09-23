# Curriculum Map

The four tracks are Rust Core (`R`), Async & Backend (`A`), Database & Backend Systems (`D`), and Engineering & Delivery (`E`). Task IDs remain stable even when a task moves to a different unit. See [the task index](05-task-list.md) and [acceptance guide](16-task-acceptance.md).

One unit is a focused learning-and-build block, planned at roughly one week. It is not a promise that all learners must finish in seven days. Advance on evidence at each gate; allow more time when the core work needs it. Seventeen units give the harder reliability and security topics room to breathe.

| Phase | Unit | Focus | Shared outcome |
|---|---:|---|---|
| Discover and model | 1 | Brief, context, Rust setup | Agreed product brief and context map: milestone 1 |
| | 2 | Ownership, borrowing, raw bytes | Safe byte-handling contract |
| | 3 | Domain types, errors, test seams | Tested domain model |
| First request | 4 | HTTP and two test applications | Mock provider and sample merchant |
| | 5 | Actix Web first vertical slice | Provider → PayHook → merchant: milestone 2 |
| | 6 | Actix Web boundaries and HMAC | Verified ingress |
| Durable core | 7 | PostgreSQL, SQL, SQLx | Migrated schema and restart proof |
| | 8 | Transactions and deduplication | Durable, once-recorded ingestion: milestone 3 |
| Delivery | 9 | Tokio workers and bounded concurrency | Recorded delivery attempts |
| | 10 | Retry policy and due work | Deterministic schedule |
| | 11 | Claiming and restart recovery | Two-worker safety and resumed work |
| | 12 | Dead letters and replay | Full reliability loop: milestone 4 |
| Operate and secure | 13 | Protected management | Authenticated operator API |
| | 14 | Limits and destination security | Abuse-resistant boundary |
| | 15 | Tests, tracing, CI | Regression and diagnosis evidence |
| | 16 | Containers and incident analysis | Clean deployment and failure drill |
| Handover | 17 | Runbook, independent start, demo | Operable handover: milestone 5 |

Actix Web is the HTTP framework for PayHook. Tokio remains the async runtime and basis for the background-delivery lessons. The curriculum does not require the separate Actix actor framework.

The five milestone gates are Units 1, 5, 8, 12, and 17. They are not cut to meet a calendar date. See [pacing and triage](15-pacing-and-triage.md) for scope adjustments.
