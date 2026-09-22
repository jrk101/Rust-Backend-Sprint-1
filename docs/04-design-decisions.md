# Curriculum Design Decisions

This file records decisions that affect the whole curriculum. Changes should
state what changed and why.

## Accepted

### D-001 — PayHook is the flagship project

PayHook naturally requires HTTP, async work, durable state, database constraints,
security boundaries, failure handling, testing, and operations. It is substantial
without requiring distributed infrastructure.

### D-002 — Use four tracks

The tracks are Rust Core (`R`), Async & Backend (`A`), Database & Backend Systems
(`D`), and Engineering & Delivery (`E`). Diagram and design tasks belong to the
track whose engineering decision they explain, normally `E`.

### D-003 — Begin with a modular monolith

One deployable main service keeps local development and reasoning manageable.
Internal modules may be separated by responsibility, but service boundaries are
not introduced for appearance.

### D-004 — PostgreSQL is authoritative

Event identity, delivery state, attempt history, and retry eligibility survive
process restarts. PostgreSQL constraints and transactions protect correctness.

### D-005 — Redis is optional in V1

Redis may support a rate-limiting or caching exercise after the core system is
measured. Deduplication and delivery truth remain in PostgreSQL.

### D-006 — Use a mock provider and sample merchant

They make signature verification, duplicates, failures, delays, and retries
demonstrable without real money or external accounts.

### D-007 — Use 13 units as the draft cadence

Thirteen units align the learning arc with the reference curriculum while
leaving room for project discovery, reliability work, and handover. Units can
later be compressed or expanded without changing stable task IDs.

## Pending

- Cohort size and the shared-build rotation size
- Supported development operating systems
- Exact deployment target for the final demonstration
- Whether the repository is public from day one
- Assessment rubric and minimum/core/stretch completion policy

