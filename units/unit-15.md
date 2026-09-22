# Unit 15 — Test and observe the system

## Before you start

Bring the protected management API and shared CI baseline from Units 5 and 13.
Read the tracing and GitHub Actions references in `docs/09-resources.md`.

## At a glance

| Learn | Use immediately |
|---|---|
| Risk-based test selection | `E9.1` |
| Correlated tracing and health | `E9.2` |
| Extend existing CI | `E9.4` |

## By the end of this unit you can

- choose unit, integration, concurrency, and end-to-end tests by risk;
- diagnose a request across structured tracing fields;
- use CI as a merge gate;
- show that the tests catch a deliberately broken behavior.

## 1 · Production confidence

- [ ] **Task 1 — Write the test matrix** (`E9.1`): Map each core requirement
  and failure mode to its cheapest trustworthy test. Identify gaps rather than
  chasing a coverage percentage. **Primary path:**
  `practice/<student-id>/unit-15/quality/test-matrix.md`.
- [ ] **Task 2 — Add structured tracing** (`E9.2`): Correlate event, delivery,
  and attempt identifiers without logging secrets or full sensitive payloads.
  Add health and readiness behavior that reflects dependencies honestly.
  **Primary path:** `practice/<student-id>/unit-15/observability/`.
- [ ] **Task 3 — Extend the CI gate** (`E9.4`): Keep the Unit 5 format, Clippy,
  and fast-test gate; add migration validation and a targeted database or
  end-to-end check on pull requests. Record the required checks and their run
  time. Avoid tests that depend on a paid service.
  **Primary path:** `practice/<student-id>/unit-15/ci/`.

**If short on time, cut:** CI optimization and extra instrumentation. Never cut
tests for signature, deduplication, retries, replay, and protected management.

## End-of-unit checklist

- [ ] CI catches a deliberately broken migration or integration test
- [ ] Trace IDs connect ingress, delivery, and attempt without leaking secrets
- [ ] Shared rotation merges the test matrix, tracing, and CI extension

Next: `unit-16.md`.
