# Rust Backend Sprint 1

**PayHook · Project-based Rust backend engineering · Cohort curriculum**

This repository is the learning environment for a cohort that designs, builds,
tests, and hands over PayHook: a self-hosted payment-webhook reliability
service written in Rust.

The project is not a prewritten tutorial. Each capability appears because the
system needs it. Students research the requirement, practise the underlying
skill individually, apply it to the shared build, review one another's work,
and explain the trade-off behind the result.

## Read these first

1. `docs/00-START-HERE.md`
2. `docs/01-project-brief.md`
3. `docs/02-curriculum-map.md`
4. `docs/03-student-guide.md`
5. The current file in `units/`

The [cohort operations notes](docs/12-cohort-operations.md) provide a place
to record schedules, assignments, and checks as the cohort runs.

The [requirements and learning chain](docs/14-requirements-and-learning-chain.md)
explains why each concept appears. The [shared build plan](docs/11-shared-build-plan.md)
connects individual practice to the product, and the
[task acceptance guide](docs/16-task-acceptance.md) gives reviewable proof for
every task.

## The four tracks

| ID | Track | Purpose |
|---|---|---|
| `R` | Rust Core | Learn the language by modelling and implementing the domain safely. |
| `A` | Async & Backend | Build HTTP APIs with Actix Web and background work with Tokio. |
| `D` | Database & Backend Systems | Make events, deliveries, authentication, and operational state durable. |
| `E` | Engineering & Delivery | Work through Git, reviews, tests, observability, containers, CI, and handover. |

## Repository zones

```text
Rust-Backend-Sprint-1/
├── docs/                 curriculum-wide reference documents
├── units/                one working file per unit
├── practice/             individual student work (created for the cohort)
├── shared/
│   ├── payhook/          the real shared product
│   ├── mock-provider/    a small signed-event generator
│   ├── sample-merchant/  a small controllably failing receiver
│   └── delivery/         briefs, diagrams, demonstrations, and handover
└── admin/                public administration notes; no answer keys
```

`practice/` is where every student attempts every important skill. `shared/`
contains the one product the cohort will hand over. The shared applications
currently contain only compiling placeholder crates, not a completed PayHook
implementation for students to copy.

## Course shape

- Seventeen units are the pacing model. A unit is planned for roughly a week, but
  milestone evidence, not elapsed time, determines whether the group advances.
- PayHook is the flagship project.
- PostgreSQL is the durable source of truth.
- Redis is an optional extension unless the cohort proves a real need for it.
- The shared system begins as a modular monolith.
- The mock provider and sample merchant remain deliberately small.
- The cohort lead records dates, assignments, and the demonstration environment
  when they are known. Those logistics do not change the learning path.

## What completion means

The sprint is complete when a new person can start the system from a clean
checkout, generate a signed event, observe safe ingestion and duplicate
protection, watch delivery fail and retry, inspect attempt history, replay a
dead-lettered delivery, and operate the system using only the written runbook.

## Start of cohort

This is the cohort's starting repository: 17 units, 69 stable task IDs, a
compiling Cargo workspace, and a Unit 2 practice fixture. Start with
`docs/00-START-HERE.md` and Unit 1. The shared applications are intentionally
placeholders because implementing PayHook, database migrations, the final
Compose stack, and delivery checks is the cohort's work in later units.
