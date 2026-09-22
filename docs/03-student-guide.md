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

## Example

```bash
git switch -c r2-1-event-parser
cargo test -p practice_unit_02
git add practice/STUDENT_ID/unit-02/
git commit -m "R2.1 parse webhook event envelope"
git push -u origin r2-1-event-parser
```

Commands and paths will be adjusted when the starter workspace and student IDs
are generated. Do not create a new architecture because an example path has not
yet been populated; ask for the current shared decision.

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

