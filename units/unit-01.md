# Unit 1 — Understand the problem and start Rust

Read this entire file before beginning. This unit establishes the product
boundary and a compiling workspace; neither may be skipped.

## Before you start

Read `docs/00-START-HERE.md`, `docs/01-project-brief.md`, and
`docs/03-student-guide.md`. Complete the readiness exercise in
`docs/07-tools-setup.md`. Find your assigned practice folder and reviewer.

## By the end of this unit you can

- explain PayHook to a non-specialist;
- distinguish the main service from its two test-support applications;
- compile, run, format, and test a Cargo workspace;
- use basic Rust values, functions, structs, and enums in a small event program.

## At a glance

| Learn | Do |
|---|---|
| Project brief and webhook vocabulary | Agree on scope and draw the system |
| Rust Book chapters 1–3 and relevant parts of 5–6 | Build a small event inspector |
| Cargo workspace basics | Create the practice workspace |

## 1 · Product brief [MILESTONE]

- [ ] **Task 1 — Write the cohort brief** (`E1.1`): In your own words, state
  the user, problem, core flow, V1 boundary, non-goals, success evidence, and
  five open questions. **Primary path:**
  `practice/<student-id>/unit-01/discovery/product-brief.md`.
- [ ] **Task 2 — Draw the context map** (`E1.2`): Draw provider, PayHook,
  PostgreSQL, merchant, and operator. Label trust boundaries and which arrows
  are synchronous versus background work. **Primary path:**
  `practice/<student-id>/unit-01/design/context-map.md`.

The brief is a gate. The reviewer checks that it names a user, a failure they
experience, a testable result, and a deliberate V1 boundary. Open questions
remain explicit; they are not silently resolved in code.

## 2 · First Rust program

- [ ] **Task 3 — Create the practice crate** (`R1.1`): Create a binary crate,
  run it, add one unit test, and record the commands and outputs.
  **Primary path:** `practice/<student-id>/unit-01/rust/getting-started/`.
- [ ] **Task 4 — Inspect an event** (`R1.2`): Model three payment event kinds
  with an enum and one event envelope with a struct. Print a safe summary using
  functions and pattern matching. **Primary path:**
  `practice/<student-id>/unit-01/rust/event-inspector/`.
- [ ] **Task 5 — Explain the first choices** (`R1.3`): Explain why event kind
  is an enum, why amount should not be a floating-point value, and what data
  must never be printed. **Primary path:**
  `practice/<student-id>/unit-01/rust/event-inspector/NOTES.md`.

## End-of-unit checklist

- [ ] Product brief and context map reviewed
- [ ] Code passes `cargo fmt`, `cargo clippy`, and `cargo test`
- [ ] Task IDs appear in commits and pull requests
- [ ] At least one peer review completed
- [ ] Shared rotation promotes an agreed brief and context diagram, following
      `docs/11-shared-build-plan.md`

**If short on time, cut:** extra event variants, then visual polish. Never cut
Tasks 1–2 or the first test.

Next: `unit-02.md`.
