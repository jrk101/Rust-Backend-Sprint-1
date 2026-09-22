# Rust Backend Sprint 1

**PayHook · Project-based Rust backend engineering · Curriculum draft v0.2**

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

The [publication checklist](docs/12-publication-checklist.md) records the few
cohort-specific choices that must be filled in before Unit 1 opens.

The [requirements and learning chain](docs/14-requirements-and-learning-chain.md)
explains why each concept appears. The [shared build plan](docs/11-shared-build-plan.md)
connects individual practice to the product, and the
[task acceptance guide](docs/16-task-acceptance.md) gives reviewable proof for
every task.

## The four tracks

| ID | Track | Purpose |
|---|---|---|
| `R` | Rust Core | Learn the language by modelling and implementing the domain safely. |
| `A` | Async & Backend | Build HTTP APIs and concurrent background work with Axum and Tokio. |
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
contains the one product the cohort will hand over. The shared applications are
empty curriculum targets at this stage; this repository does not ship a hidden
implementation for students to copy.

## Working assumptions requiring review

- Thirteen units are used as the pacing model. A unit may be a week, but
  milestone evidence, not elapsed time, determines whether the group advances.
- PayHook is the final flagship project; the product name remains provisional.
- PostgreSQL is the durable source of truth.
- Redis is an optional extension unless the cohort proves a real need for it.
- The shared system begins as a modular monolith.
- The mock provider and sample merchant remain deliberately small.
- Cohort size, calendar dates, reviewer count, and deployment target are still
  operational decisions recorded in `docs/12-publication-checklist.md`.

## What completion means

The sprint is complete when a new person can start the system from a clean
checkout, generate a signed event, observe safe ingestion and duplicate
protection, watch delivery fail and retry, inspect attempt history, replay a
dead-lettered delivery, and operate the system using only the written runbook.

## Draft status

This is a curriculum draft for review. It has 13 units and 69 stable task IDs.
The program lead must complete the publication checklist, including cohort
assignments and a tested shared starter workspace, before students begin.
