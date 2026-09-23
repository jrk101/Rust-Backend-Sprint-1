# Publication Checklist

The curriculum content is a working draft until the following cohort-specific
facts are recorded. The program lead owns this checklist; students should not
have to guess its answers.

## Cohort facts

- [ ] Dates and duration of the 17 units recorded in the cohort announcement
- [ ] Student IDs and practice folders created
- [ ] Rotation size and assignments recorded in `shared/delivery/rotation-log.md`
- [ ] Review lead and mentor capacity confirmed for each unit
- [ ] Supported operating systems named and setup guide tested on each
- [ ] Final demo environment named (a local Compose deployment is acceptable)
- [ ] Public/private repository policy and data retention agreed

## Repository readiness

- [ ] A mentor follows `07-tools-setup.md` on a fresh machine
- [x] A minimal shared Cargo workspace exists before the Unit 5 rotation; placeholder crates pass local format, Clippy, and tests
- [x] `rust-toolchain.toml`, `Cargo.lock`, `.gitignore`, and `.env.example` exist
- [ ] Shared CI runs on a sample pull request before Unit 5
- [ ] The Unit 5 unsigned path is demonstrated before Unit 6 signing work, with a recoverable teaching checkpoint for late joiners
- [ ] PostgreSQL/SQLx migration commands are tested before Unit 7
- [ ] Compose starts the final stack cleanly on a supported machine before Unit 16
- [ ] The mock provider and merchant require no payment account or money
- [ ] Every required documentation link resolves
- [ ] Task IDs are unique and all referenced paths are intentional
- [ ] Every task ID in the unit files has one entry in `16-task-acceptance.md`
- [ ] Unit 1 is trialled with at least one learner from the intended entry level
- [ ] Every promised exercise fixture exists; the Unit 2 ownership fixture is present and intentionally fails to compile

## Curriculum readiness

- [ ] Unit 1 prerequisite check is explicit and achievable
- [ ] Each unit's required work fits the agreed time budget
- [ ] Mentor review time is reserved for Units 8, 10–11, and 14
- [ ] Unit 11's claim, expiry, and crash-recovery protocol is reviewed before implementation
- [ ] Unit 14's destination policy and adversarial test set are reviewed before implementation
- [ ] Every milestone has an outside witness and observable pass criteria
- [ ] Core, standard, and extension work are marked in `15-pacing-and-triage.md`
- [ ] The Unit 12 failure sequence is rehearsed on a disposable database
- [ ] The Unit 17 clean-start test is performed by a non-author

No unchecked item should be described as already complete. The project can be
reviewed and improved before these operational choices are settled.
