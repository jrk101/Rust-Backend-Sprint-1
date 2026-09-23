# Curriculum Design Decisions

This file records decisions that affect the whole curriculum. Changes should
state what changed and why.

## Accepted

### D-001: PayHook is the flagship project

PayHook naturally requires HTTP, async work, durable state, database constraints,
security boundaries, failure handling, testing, and operations. It is substantial
without requiring distributed infrastructure.

### D-002: Use four tracks

The tracks are Rust Core (`R`), Async & Backend (`A`), Database & Backend Systems
(`D`), and Engineering & Delivery (`E`). Diagram and design tasks belong to the
track whose engineering decision they explain, normally `E`.

### D-003: Begin with a modular monolith

One deployable main service keeps local development and reasoning manageable.
Internal modules may be separated by responsibility, but service boundaries are
not introduced for appearance.

### D-004: PostgreSQL is authoritative

Event identity, delivery state, attempt history, and retry eligibility survive
process restarts. PostgreSQL constraints and transactions protect correctness.

### D-005: Redis is optional in V1

Redis may support a rate-limiting or caching exercise after the core system is
measured. Deduplication and delivery truth remain in PostgreSQL.

### D-006: Use a mock provider and sample merchant

They make signature verification, duplicates, failures, delays, and retries
demonstrable without real money or external accounts.

### D-007: Use 17 units as the draft cadence

Seventeen units separate first HTTP contact from the first vertical slice,
retry policy from work claiming, management from security hardening, and CI
from incident operations. Each unit is planned for roughly one week; gates
depend on evidence, not an inflexible date. Stable task IDs are unchanged.

### D-008: Separate the reliability gate from HTTP management

The Unit 12 milestone uses a local operator command to inspect and replay
deliveries. Unit 13 adds authentication before exposing those operations as
management HTTP endpoints. This keeps the core failure loop testable without
teaching an unprotected management API as a deployable pattern.

### D-009: Start CI with the first shared Rust code

Formatting, Clippy, and fast tests run on shared pull requests from Unit 5.
Database checks begin with Unit 7; Unit 15 strengthens the full test matrix.

### D-010: Use Actix Web for HTTP and Tokio for background work

Actix Web is the selected HTTP framework for the cohort's preference. Tokio
remains the runtime and explicit learning target for delivery workers. Actix
Web's `web::Data` already uses shared ownership internally, so lessons should
not require an extra `Arc` around it without a specific reason. The separate
Actix actor framework is not part of V1.

## Pending

- Cohort size and the shared-build rotation size
- Supported development operating systems
- Exact deployment target for the final demonstration
- Whether the repository is public from day one
- Assessment rubric and minimum/core/stretch completion policy
