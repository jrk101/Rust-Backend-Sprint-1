# Curriculum Map

## Phases

| Units | Phase | Outcome |
|---|---|---|
| 1–3 | Understand | Agree on the problem and learn enough Rust to model it. |
| 4–7 | Build foundations | Complete the first HTTP flow, persistence, signatures, and durable ingestion. |
| 8–10 | Build reliability | Add concurrent delivery, persisted retries, dead-lettering, and replay. |
| 11–13 | Operate and deliver | Secure, observe, test, package, demonstrate, and hand over the system. |

## Units at a glance

| Unit | Theme | Central outcome | Milestone |
|---|---|---|---|
| 1 | Understand the system and Rust workspace | Agreed brief, context map, compiling workspace | Product brief |
| 2 | Ownership and trustworthy input | Parse and validate event data without unsafe shortcuts | — |
| 3 | Domain modelling and errors | Domain types and explicit failure model | — |
| 4 | HTTP foundations and first vertical slice | Signed mock event reaches the sample merchant in memory | First end-to-end event |
| 5 | Axum service design and verification | Structured routes, state, middleware, HMAC verification | — |
| 6 | PostgreSQL, SQL, SQLx | Migrations and durable event/delivery records | — |
| 7 | Transactions and duplicate safety | Concurrent duplicate submissions create one event | Durable ingestion |
| 8 | Tokio delivery workers | Bounded concurrent outbound deliveries with timeouts | — |
| 9 | Retry scheduling and failure policy | Persisted attempts and recoverable retry scheduling | — |
| 10 | Dead letters, replay, and management API | Full reliability loop demonstrated | Reliable delivery core |
| 11 | Authentication, limits, and security | Protected management API and abuse boundaries | — |
| 12 | Testing, observability, containers, CI | Reproducible build plus break/fix evidence | — |
| 13 | Deployment thinking and handover | Clean-start demonstration using the runbook | Project handover |

## Curriculum coverage

### Rust Core

Rust syntax begins in Unit 1. Ownership and borrowing are learned through raw
payload handling in Unit 2. Structs, enums, pattern matching, `Option`, `Result`,
and error design shape the domain in Unit 3. Traits and generics appear when
HTTP-signature and delivery boundaries need substitutable behavior. Iterators
and closures support validation and transformations. `Arc` is introduced for
shared application state; locking is used only where shared mutable memory is
actually justified. Lifetimes and basic DSA receive focused practice rather
than being forced into production architecture.

### Async & Backend

Students first see the HTTP exchange manually, then implement it in Axum.
Tokio tasks, cancellation, channels, timers, concurrency limits, timeouts, and
graceful shutdown appear as the delivery system becomes asynchronous.

### Database & Backend Systems

SQL and relational modelling begin before SQLx abstractions. PostgreSQL owns
durable event identity and delivery state. Migrations, constraints,
transactions, pooling, indexes, and safe work claiming arise from reliability
requirements. Authentication protects management APIs. Redis is a bounded
optional lab for rate limiting or caching, never the source of truth.

### Engineering & Delivery

Every unit uses Git, focused commits, pull requests, and review. Testing grows
from pure unit tests to HTTP, database, concurrency, and end-to-end tests.
Structured tracing, configuration, Docker Compose, CI, failure injection,
security review, and handover arrive before the final demonstration.

## Scope discipline

The curriculum may add a concept exercise without adding that concept to
PayHook. This is especially important for advanced lifetimes, data structures,
Redis Pub/Sub, and patterns that would make the shared system harder without
making it more reliable.

