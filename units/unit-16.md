# Unit 16: Package, break, and fix

## Before you start

Bring Unit 15's test matrix, tracing fields, and CI checks. Read the Docker
Compose reference in `docs/09-resources.md`. The mentor prepares one reversible
injected failure in a disposable environment and records expected behavior.

## At a glance

| Learn | Use immediately |
|---|---|
| Reproducible local operation | `E9.3` |
| Evidence-led investigation | `E9.5` |
| Blameless incident writing | `E9.6` |

## By the end of this unit you can

- start the three applications and PostgreSQL from a clean local environment;
- trace a failing event through logs and database state;
- add a regression test and explain the fix without blaming a person.

## 1 · Reproducible operation

- [ ] **Task 1: Package local operation** (`E9.3`): Create Dockerfiles and
  Compose configuration for PostgreSQL and the three applications. Use health
  checks and environment-based configuration; do not bake secrets into images.
  **Primary path:** `practice/<student-id>/unit-16/containers/`.

## 2 · Break and fix

- [ ] **Task 2: Investigate the injected failure** (`E9.5`): Use logs,
  database state, tests, and controlled reproduction to build an evidence
  trail before changing code. **Primary path:**
  `practice/<student-id>/unit-16/incident/evidence.md`.
- [ ] **Task 3: Write the postmortem** (`E9.6`): Record impact, timeline,
  contributing conditions, detection, resolution, and one prevention action.
  Avoid blame and hindsight certainty. **Primary path:**
  `practice/<student-id>/unit-16/incident/postmortem.md`.

## End-of-unit checklist

- [ ] Fresh Compose environment starts and reports readiness honestly
- [ ] A regression test fails for the injected bug and passes after the fix
- [ ] Evidence trail identifies the cause before the proposed change
- [ ] Shared rotation merges container setup and reviewed incident fix

**If short on time, cut:** container polish. Never cut the failure evidence,
regression test, or postmortem.

Next: `unit-17.md`.
