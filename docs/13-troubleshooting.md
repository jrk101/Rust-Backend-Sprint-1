# Troubleshooting

Post the exact command, full error, operating system, and what you tried when
the guide below does not resolve the problem.

## Rust and Cargo

**`cargo` is not found.** Restart the terminal after rustup installation. Run
`rustup show` and check that the shell can see the rustup binary directory.

**The borrow checker rejects a value.** Identify who owns the value, who needs
to read it, and how long the read lasts. Check whether the function can take
`&T` or `&str`. Do not add `clone()` until you can explain why another owner is
needed.

**Clippy and `cargo test` disagree.** Run both from the repository root and
check that the same toolchain and workspace packages are selected. Read the
first error and its file/line, then rerun the smallest affected test.

## HTTP and signatures

**Signature verification fails on an unchanged event.** Compare the exact raw
request bytes, signed message format, timestamp, and secret. Re-serializing
JSON changes bytes even when the object has the same meaning. Never print the
secret to debug it; use a disposable test vector.

**The merchant receives nothing.** Check the destination URL, PayHook's
delivery state, worker logs, and whether the merchant is reachable from the
container or process running PayHook. `localhost` inside a container points to
that container, not the host.

## PostgreSQL

**Connection refused.** Confirm PostgreSQL is healthy, the port is mapped,
and the connection URL points to the correct host for the process location.
Do not substitute a personal database to make a test pass.

**Migration fails.** Read the first database error, check the migration order,
and test on a fresh disposable database. Do not edit a migration already used
by teammates without agreeing on a migration recovery plan.

**Duplicate event rows appear.** Check the exact provider identity key and
database unique constraint. An application `SELECT` before `INSERT` is not a
concurrency guarantee.

**A delivery stays due.** Inspect attempt history, `next_attempt_at`, worker
claim state, transaction outcome, and process logs. Replaying manually before
understanding the state can hide the original failure.

## Git and CI

**Push rejected.** Fetch the current remote branch, inspect the difference,
then merge or rebase only your own branch. Do not force push the shared `main`.

**A secret was committed.** Stop using it, rotate it, tell a maintainer, and
remove it from the branch. Deleting the latest file does not remove the secret
from Git history.

**CI is red.** Open the failed job, read the first meaningful error, run the
same command locally, and push the fix to the same pull request.
