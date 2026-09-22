# Learning Resources

This is the reading map for the units. Use the chapter or section shown in the
current unit. Read enough to complete the task, then return when a later task
needs a deeper section. Links are to primary project documentation; maintainers
check them before each cohort starts.

| Units | Need created by the project | Read |
|---|---|---|
| 1 | Compile and model a small event | [Rust Book chapters 1–3](https://doc.rust-lang.org/book/), then chapters 5–6 for structs and enums; [Cargo Book: workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) |
| 2 | Preserve raw bytes and avoid unnecessary copies | Rust Book chapter 4 (ownership) and chapter 8 (strings, collections) |
| 3 | Represent states and failures | Rust Book chapters 6, 9, 10 and 11 (enums, errors, traits, tests) |
| 4 | Send an event over HTTP | [HTTP Semantics](https://httpwg.org/specs/rfc9110.html) sections on methods and status codes; [Axum crate documentation](https://docs.rs/axum/latest/axum/) routing examples |
| 5 | Authenticate a signed request | Axum extractors/state documentation; [RustCrypto HMAC documentation](https://docs.rs/hmac/latest/hmac/); [OWASP Cheat Sheet Series](https://cheatsheetseries.owasp.org/) for safe boundary questions |
| 6 | Store state durably | [PostgreSQL tutorial](https://www.postgresql.org/docs/current/tutorial.html), then constraints and transactions; [SQLx documentation](https://docs.rs/sqlx/latest/sqlx/) for pools, queries, and migrations |
| 7 | Prevent races during ingestion | PostgreSQL unique constraints and [transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html); SQLx transaction API |
| 8 | Deliver without blocking or overload | [Tokio tutorial](https://tokio.rs/tokio/tutorial), especially spawning, channels, and shared state; Tokio timeout and semaphore API docs |
| 9 | Retry due work safely | PostgreSQL [explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html) and `SELECT` locking clause; Tokio time docs; Rust standard `BinaryHeap` docs for the separate lab |
| 10 | Inspect, dead-letter, and replay | PostgreSQL transaction docs; revisit the domain state diagram and the cohort's retry decision record |
| 11 | Protect management and configured URLs | [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html), [OWASP SSRF Prevention Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html), Axum middleware docs |
| 12 | Diagnose and reproduce failures | [tracing documentation](https://docs.rs/tracing/latest/tracing/), [Docker Compose documentation](https://docs.docker.com/compose/), [GitHub Actions Rust guide](https://docs.github.com/en/actions/tutorials/build-and-test-code/rust) |
| 13 | Hand over work another developer can run | [Pro Git](https://git-scm.com/book/en/v2) collaboration chapters and the cohort's final runbook |

## Reading rule

The table is a route, not a reading quota. Each unit's `At a glance` section
maps a concept to the task that uses it. Students should explain the relevant
part in their own words and demonstrate it through code or a design decision.

## Outside the shared product

Advanced lifetimes, the priority queue lab, and Redis Pub/Sub may be taught
as small exercises. They do not become PayHook dependencies by default.

