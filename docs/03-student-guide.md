# Student Guide

## The working loop

1. Read the current unit file completely.
2. Read or watch only the resources linked to the task you are beginning.
3. Create a branch named from the task ID and short purpose.
4. Complete the task in the exact path specified.
5. Run the relevant checks and record the required evidence.
6. Commit with the task ID at the start of the message.
7. Push and open a pull request with the task ID in its title.
8. Explain why the change is correct and name one trade-off.
9. Review at least one teammate's work.
10. Apply review feedback in the same branch and pull request.

## How to read a task path

Find your assigned ID in [`practice/`](../practice/README.md). Open your
current unit's `problem_statement.md`, then follow its link to the full unit
tasks. Keep your answers and code in your own folder.

`<student-id>` means the ID assigned before the cohort begins. A task with a
Markdown or SQL path commits that exact file. A task with a directory path
commits the small crate or module files needed for that task under that
directory. Rust code commonly needs a manifest, source, and tests; the rule is
one focused task per commit, not one source file regardless of language needs.
Do not combine unrelated task IDs just because they touch the same crate.

Every task needs observable evidence. For code, include the relevant passing
test command and output in the pull-request description. For a design, include
the reason, one alternative, and a reviewer question. For a milestone, ask a
student outside the build rotation to reproduce the scenario.
The minimum proof for each ID is in `16-task-acceptance.md`.

## Example

```bash
git switch -c r2-3-payload-parser
cd practice/STUDENT_ID/unit-02/rust/payload-parser
cargo test
cd ../../../../..
git add practice/STUDENT_ID/unit-02/
git commit -m "R2.3 parse webhook event envelope"
git push -u origin r2-3-payload-parser
```

Replace `STUDENT_ID` with your assigned ID. Create the practice crate at the
task path before running `cargo test`. Run Git commands from the repository
root. The example shows the payload parser task `R2.3`.

The [shared build plan](11-shared-build-plan.md) says which practice result is
promoted into the actual product after each unit.

## When you are stuck

Report four things:

1. What you intended to happen.
2. The exact command or request you ran.
3. What happened instead, including the error.
4. What you already tried and what evidence changed.

Ask for the next useful hint, not a finished answer.

## Shared-build rotation

Only the assigned rotation edits `shared/` during a unit. Rotation work must:

- reference the agreed design;
- preserve migration and API compatibility or document the change;
- include tests proportional to the failure risk;
- pass the shared CI checks;
- update the relevant decision, diagram, or runbook entry.
