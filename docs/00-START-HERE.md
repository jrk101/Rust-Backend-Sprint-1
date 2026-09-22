# Start Here

PayHook is a self-hosted reliability layer between a payment provider and an
application backend. It receives signed webhook events, stores them safely,
prevents duplicate processing, delivers them to a configured destination, and
records every attempt. Failed deliveries retry and eventually enter a dead-letter
state from which an operator can replay them.

You are not expected to understand all of that on day one. The project exists
to create reasons to learn each part.

## How learning works

```text
project requirement
        ↓
question and research
        ↓
small individual exercise
        ↓
shared implementation
        ↓
review, test, and explanation
```

Every unit ends in evidence: a brief, diagram, tested behavior, pull request,
failure investigation, or handover artifact. “Covered the topic” is not an outcome.

## Two zones

### Individual practice

Every student practises the important skill in `practice/<student-id>/unit-N/`.
Mistakes are cheap and expected here.

### Shared build

A rotating group applies the cohort's learning to `shared/`. The shared product
has stricter review and automated checks because later work depends on it.

## Stable task IDs

Task IDs never change once published:

- `R` — Rust Core
- `A` — Async & Backend
- `D` — Database & Backend Systems
- `E` — Engineering & Delivery

For example, `A4.2` identifies an Async & Backend task in task group 4. Use the
ID in the commit message and pull-request title.

## Milestones

| Unit | Gate | Evidence |
|---|---|---|
| 1 | Product brief agreed | Scope, non-goals, success criteria, system context |
| 5 | First end-to-end event | Mock provider → PayHook → sample merchant |
| 8 | Durable ingestion agreed | Signed event stored once under concurrency |
| 12 | Reliable delivery core | Persisted attempts, retry schedule, dead letter, replay |
| 17 | Project handover | Clean-start run using only the runbook |

Milestones are never part of the cut list.
One unit is roughly one week of learning and building, with flexible time for
harder topics. Evidence, not the calendar alone, determines progression.

The [requirements and learning chain](14-requirements-and-learning-chain.md)
shows which project limitation creates each new concept. The
[shared build plan](11-shared-build-plan.md) names what the rotation contributes
after individual practice.

## Before Unit 1

- Read `01-project-brief.md`.
- Read `02-curriculum-map.md`.
- Follow `07-tools-setup.md` and pass its readiness exercise.
- Configure Git with the email attached to your GitHub account.
- Find your assigned practice folder and team role.

Next: `01-project-brief.md`.
