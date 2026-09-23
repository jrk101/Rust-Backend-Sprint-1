# Pacing and Triage

Seventeen units are a proposal for the cohort schedule, not proof that every
student can complete every task in one week. The program lead should estimate
hours using a learner from the target entry level and record the real timings
after each unit. If the core task set regularly exceeds the allotted time,
extend the unit or reduce scope before advancing.

Units 8, 10–11, and 14 need planned mentor attention. Unit 8's concurrent
duplicate test must start requests together and rely on a database uniqueness
guarantee. Units 10–11 require one reviewed claim and recovery protocol before
students implement competing worker designs. Unit 14 needs extra security
review time and adversarial destination-policy tests. These are time-budget
risks, not optional milestone work.

| Units | Core that cannot be cut | Standard work | Extension / first cut |
|---|---|---|---|
| 1–3 | Brief, context, ownership, domain and error tests | Additional event cases and trait comparison | Lifetime depth beyond the focused lab |
| 4–6 | First request and signature boundary | Configurable merchant and polished errors | Extra routes and formatting |
| 7–8 | Schema, restart proof, unique constraint, concurrent duplicate test | Query helpers and diagram refinement | Extra database abstraction |
| 9–12 | Timeouts, bounded worker, persisted retry, restart, dead letter, replay | Broader status matrix and filters | Priority queue lab and throughput benchmarks |
| 13–14 | Protected management and destination policy | Rate limits and rotation ergonomics | Redis lab and Pub/Sub |
| 15–17 | Failure test, CI, runbook, clean start | Full incident and presentation polish | Extra deployment automation |

## Overload response

The student marks a standard or extension task as deferred, links the work
completed, and names the blocker. The mentor checks whether a milestone is
affected. Milestone tasks are never silently shortened. At each unit close,
collect actual time spent, the tasks deferred, and the concept that caused the
most confusion. Revise the next cohort's schedule from that evidence.

## Expected prerequisite

The default entry point assumes students have used a terminal and written a
small program before, but are new to Rust backend work. Students without that
experience complete the readiness exercise and a short mentor-supported
programming bridge before Unit 1. That bridge is not counted as a hidden unit.
