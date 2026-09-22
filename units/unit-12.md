# Unit 12 — Test, observe, package, break, and fix

## Before you start

Bring the protected management API and shared CI baseline from Units 4 and 11.
Read the tracing, Compose, and GitHub Actions references in
`docs/09-resources.md`. The mentor prepares one reversible injected failure on
a disposable environment and records the unmodified expected behavior.

## At a glance

| Learn | Use immediately |
|---|---|
| Risk-based test selection | `E9.1` |
| Correlated tracing and health | `E9.2` |
| Reproducible containers | `E9.3` |
| Extend existing CI | `E9.4` |
| Evidence-led debugging | `E9.5–E9.6` |

## By the end of this unit you can

- choose unit, integration, concurrency, and end-to-end tests by risk;
- diagnose a request across structured tracing fields;
- run the system reproducibly with containers;
- use CI as a merge gate;
- investigate an injected failure and write a blameless postmortem.

## 1 · Production confidence

- [ ] **Task 1 — Write the test matrix** (`E9.1`): Map each core requirement
  and failure mode to its cheapest trustworthy test. Identify gaps rather than
  chasing a coverage percentage. **Primary path:**
  `practice/<student-id>/unit-12/quality/test-matrix.md`.
- [ ] **Task 2 — Add structured tracing** (`E9.2`): Correlate event, delivery,
  and attempt identifiers without logging secrets or full sensitive payloads.
  Add health and readiness behavior that reflects dependencies honestly.
  **Primary path:** `practice/<student-id>/unit-12/observability/`.
- [ ] **Task 3 — Package local operation** (`E9.3`): Create Dockerfiles and
  Compose configuration for PostgreSQL and the three applications. Use health
  checks and environment-based configuration; do not bake secrets into images.
  **Primary path:** `practice/<student-id>/unit-12/containers/`.
- [ ] **Task 4 — Extend the CI gate** (`E9.4`): Keep the Unit 4 format, Clippy,
  and fast-test gate; add migration validation and a targeted database or
  end-to-end check on pull requests. Record the required checks and their run
  time. Avoid tests that depend on a paid service.
  **Primary path:** `practice/<student-id>/unit-12/ci/`.

## 2 · Break and fix

- [ ] **Task 5 — Investigate the injected failure** (`E9.5`): Use logs,
  database state, tests, and controlled reproduction to build an evidence trail
  before changing code. **Primary path:**
  `practice/<student-id>/unit-12/incident/evidence.md`.
- [ ] **Task 6 — Write the postmortem** (`E9.6`): Record impact, timeline,
  contributing conditions, detection, resolution, and one prevention action.
  Avoid blame and hindsight certainty. **Primary path:**
  `practice/<student-id>/unit-12/incident/postmortem.md`.

**If short on time, cut:** CI optimization and container polish. Never cut the
evidence trail, postmortem, or tests around the discovered failure.

## End-of-unit checklist

- [ ] Fresh Compose environment starts and reports readiness honestly
- [ ] CI catches a deliberately broken migration or integration test
- [ ] Trace IDs connect ingress, delivery, and attempt without leaking secrets
- [ ] A test now fails for the injected bug and passes for the reviewed fix

Next: `unit-13.md`.
