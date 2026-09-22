# Shared Build Plan

The `units/` tasks first produce individual evidence. Each unit then promotes
one reviewed result into the shared product. The rotation group implements it;
the rest of the cohort review and explain it. Names and group size are assigned
in `shared/delivery/rotation-log.md` before a unit opens.

| Unit | Shared contribution | Target | Acceptance evidence |
|---|---|---|---|
| 1 | Agree brief, non-goals, context diagram | `shared/delivery/brief.md`, `shared/delivery/context.md` | Mentor and student witness sign-off |
| 2 | Choose raw-payload representation and parsing rule | `shared/delivery/payload-contract.md` | Invalid-input examples and ownership explanation |
| 3 | Merge domain model and error vocabulary | `shared/payhook/` | State and error unit tests |
| 4 | Build mock provider, merchant, and in-memory relay | all three `shared/` applications | Another student reproduces one HTTP delivery; fast CI begins |
| 5 | Verify mock-provider signatures at the ingress boundary | `shared/payhook/`, `shared/mock-provider/` | Altered/stale/missing signatures rejected |
| 6 | Merge migrations and repository queries | `shared/payhook/` | Fresh database migration and restart proof |
| 7 | Merge transactional ingestion and unique event identity | `shared/payhook/` | Concurrent duplicate test against PostgreSQL |
| 8 | Merge bounded delivery worker and merchant failure modes | `shared/payhook/`, `shared/sample-merchant/` | Timeout, concurrency, and shutdown tests |
| 9 | Merge persisted attempts, due-work query, and work claiming | `shared/payhook/` | Two workers, one claim; restart proof |
| 10 | Merge dead-letter and local operator replay | `shared/payhook/`, `shared/delivery/reliability-demo.md` | Outside witness runs the complete failure loop |
| 11 | Merge protected management API and destination policy | `shared/payhook/` | Unauthenticated and cross-owner requests rejected; SSRF tests |
| 12 | Add tracing, Compose, CI integration checks, and incident fix | `.github/workflows/`, `shared/` | Clean CI and evidence-based postmortem |
| 13 | Merge final architecture, runbook, and README | `shared/delivery/`, repository root | New operator succeeds from clean checkout |

## Rotation contract

1. The rotation opens an issue or task card naming the IDs it will promote.
2. It records the exact files it plans to touch and asks for one reviewer from
   outside the rotation.
3. Each person owns a small contribution and commits with their own identity.
4. The rotation opens a shared pull request with tests and a short explanation
   of how the individual practice informed the implementation.
5. The witness runs the unit acceptance scenario. A milestone requires a
   witness who did not implement that milestone.
6. The group updates `shared/delivery/rotation-log.md` with students, component,
   pull request, witness, and outcome.

## Shared code rule

Practice code is evidence, not a source to paste automatically. The rotation
may use a student's design after review, but must integrate it with the agreed
domain model, migrations, error handling, tests, and configuration. A shared
pull request may contain multiple files because real Rust changes often do.

## Continuous integration

From Unit 4, shared Rust pull requests run `cargo fmt --all -- --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, and the fast workspace
tests. Unit 6 adds migration validation in a disposable PostgreSQL instance.
Unit 12 adds the full end-to-end scenario. The mentor configures branch
protection so the checks and one human review are required before merge.
