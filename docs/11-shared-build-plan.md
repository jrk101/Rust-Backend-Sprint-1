# Shared Build Plan

Every student first attempts the unit's essential practice. The assigned rotation then applies that learning in the shared workspace; a student outside the rotation reviews or witnesses the result. Record names and pull requests in [the rotation log](../shared/delivery/rotation-log.md). The exact implementation is a cohort decision, not a hidden solution in this curriculum.

| Unit | Shared contribution | Evidence before moving on |
|---:|---|---|
| 1 | Agree scope and context map in `shared/delivery/` | Milestone: reviewed brief |
| 2 | Define raw-body contract | Byte-retention and malformed-input tests |
| 3 | Model event, delivery, attempts and errors | Domain tests and trade-off note |
| 4 | Build mock provider and sample merchant | Reproducible request/response capture |
| 5 | Connect provider, Actix Web PayHook, merchant | Milestone: outside witness; begin format, Clippy and fast-test CI |
| 6 | Separate Actix Web handlers and service; verify HMAC | Valid/altered/stale request tests |
| 7 | Add PostgreSQL migrations and SQLx repositories | Fresh migration, relational and restart tests; add DB checks to CI |
| 8 | Commit event and initial delivery atomically | Milestone: concurrent duplicate proof |
| 9 | Add bounded Tokio delivery worker | Slow/failing merchant and shutdown proof |
| 10 | Persist attempts and calculate due retries | Fixed-clock policy tests and indexed query |
| 11 | Claim work safely and resume after restart | Two-worker and restart proof |
| 12 | Add dead-letter state and local replay command | Milestone: observed failure → recovery loop |
| 13 | Add authenticated Actix Web management API | Unauthorized and scope tests |
| 14 | Enforce traffic and destination policy | Adversarial URL, redirect, limit and redaction tests |
| 15 | Expand tests, tracing and CI | Invariant matrix, correlated trace, PR checks |
| 16 | Package local stack and run incident drill | Clean Compose start and postmortem |
| 17 | Finalize runbook, diagram, README and demo | Milestone: non-author clean start and handover |

The shared service remains a modular monolith. PostgreSQL is authoritative for event identity and delivery state. CI starts with first shared code in Unit 5 and expands as the database and system checks arrive; a final-unit-only CI setup is too late to protect the build.
