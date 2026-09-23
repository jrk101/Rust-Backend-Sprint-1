# Simulated Cohort Findings

An isolated three-perspective simulation walked through Units 1–17 using a
local audit prototype. It is **not** a real learner trial: one agent played
all perspectives, no outside witness signed off, and the prototype contains
later-unit solutions. It remains on the separate `simulated-cohort-audit`
branch and should not be merged into the student starter. This document
records the non-spoiler curriculum fixes promoted from that exercise.

## What was reproduced

- A fresh Actix Web, Tokio, SQLx, and PostgreSQL audit prototype compiled and
  passed format, Clippy, unit tests, and local database tests.
- Twenty simultaneous copies of one signed event produced one event and one
  initial delivery in the disposable database.
- A local merchant sequence produced `500`, timeout, `500`, dead-letter,
  replay, and `200`. The original attempts remained visible. The same replay
  key did not create a second replay.
- A retry scheduled after a `500` survived a PayHook process restart.
- The promised Unit 2 broken ownership program was missing. It is now in
  `practice/fixtures/unit-02-ownership/`.
- Reusing a database from an integration test consumed the merchant's
  programmed failure sequence. Unit 12 now explicitly requires a fresh
  disposable database for that demonstration.

These were local audit observations, not milestone passes for students.

## Improvements in this branch

1. A **minimal, answer-free Cargo workspace** now compiles. The three shared
   crates contain only placeholders. This closes the missing-starter-file
   blocker without exposing a finished service.
2. The Unit 2 compiler-error fixture is supplied at the referenced path.
3. Unit 7 no longer mandates a `users` table when the Unit 1 decision is a
   single configured operator. Students must justify that choice.
4. The merchant task now calls for ordered failure and slow-response control,
   and Unit 12 calls for an isolated database.
5. The publication checklist distinguishes files and local checks from CI on
   a sample PR, Compose on a clean machine, staged teaching checkpoints, and
   a real Unit 1 learner trial.

## Still open before students arrive

- Validate the GitHub Actions workflow on an actual pull request.
- Test PostgreSQL migration commands and Compose on supported systems.
- Trial Unit 1 with a learner at the intended level and measure task time.
- Decide cohort dates, rotation assignments, mentor capacity, and witnesses.
- Agree the Unit 1 product semantics before learners make incompatible choices.
- Prepare an unsigned Unit 5 checkpoint and review Units 8, 10–11, and 14
  with mentors. The audit prototype showed why concurrency, claim recovery,
  and outbound destination policy need unusually careful tests.
- Have an independent operator run the Unit 17 clean-start handover.

The starter workspace passing `cargo test` proves only that the scaffold
compiles. It does not mean PayHook works or the cohort is publication-ready.
