# Unit 17: Demonstrate and hand over

## Before you start

Freeze the core V1 scope after the Unit 16 fix. Assemble the agreed diagrams,
configuration example, migration instructions, and complete test matrix. Pick
a clean-start witness who has not authored the runbook.

## At a glance

| Learn | Use immediately |
|---|---|
| Actual architecture versus original plan | `E10.1`, `E10.6` |
| Operator instructions | `E10.2` |
| Clean-room reproduction | `E10.3` |
| Technical and non-technical explanation | `E10.4–E10.5` |

## By the end of this unit you can

- explain PayHook to technical and non-technical audiences;
- document the architecture as it actually runs;
- operate and recover the system using a runbook;
- prove the repository works from a clean checkout.

## 1 · Final product story

- [ ] **Task 1: Draw the final architecture** (`E10.1`): Show trust
  boundaries, applications, modules, PostgreSQL, optional components, request
  paths, worker flow, security controls, and observability. Remove planned
  components that were not built. **Primary path:**
  `practice/<student-id>/unit-17/delivery/final-architecture.md`.
- [ ] **Task 2: Write the runbook** (`E10.2`): Document startup, migrations,
  configuration, health checks, common failures, stuck-delivery diagnosis,
  replay, secret rotation, backup assumptions, and shutdown.
  **Primary path:** `practice/<student-id>/unit-17/delivery/runbook.md`.
- [ ] **Task 3: Run the clean-start test** (`E10.3`): Give the repository and
  runbook to someone who did not write them. Record every ambiguity and fix the
  documentation or automation. **Primary path:**
  `practice/<student-id>/unit-17/delivery/clean-start-log.md`.
- [ ] **Task 4: Present the outcome** (`E10.4`): Demonstrate success,
  duplicates, retries, dead letters, replay, and recovery. State limitations
  honestly, including at-least-once delivery. **Primary path:**
  `practice/<student-id>/unit-17/delivery/demo-script.md`.
- [ ] **Task 5: Finish the project README** (`E10.5`): Make the root README
  useful to a new developer and an interviewer: problem, architecture, local
  run, test, demonstration, trade-offs, and limitations.
  **Primary path:** proposed update to `/README.md`.
- [ ] **Task 6: Close the learning loop** (`E10.6`): Record what changed from
  the first design, the hardest failure, one rejected technology, and the next
  responsible improvement. **Primary path:**
  `practice/<student-id>/unit-17/delivery/retrospective.md`.

## Project handover [MILESTONE]

The project is complete only when the clean-start operator succeeds using the
written runbook, all required checks pass, and unfinished work is documented
rather than hidden.

## End-of-unit checklist

- [ ] Fresh operator runs migrations, starts services, and generates a signed event
- [ ] Duplicate, failed delivery, retry, dead letter, and replay are demonstrated
- [ ] Tests and CI pass at the final commit
- [ ] Limitations and deferred work are named in the handover
- [ ] Shared rotation merges the final runbook and architecture

**If short on time, cut:** presentation polish and optional feature discussion.
Never cut the runbook, clean-start test, or honest limitations.

Next: the project belongs to its maintainers, not only its original builders.
