# Contributing

## Core rules

1. One task, one focused commit. Begin the message with the stable task ID.
2. Use a branch and pull request for every contribution to the cohort repository.
3. Work in your own `practice/<student-id>/` folder unless assigned to the
   shared-build rotation.
4. Do not squash shared student commits; authorship is part of the learning record.
5. Never commit secrets, `.env` files, database volumes, tokens, or real payment data.
6. Generated webhook secrets are test credentials only and must not be reused elsewhere.
7. A design answer must name the reason, the trade-off, and at least one rejected option.
8. Shared code must pass formatting, linting, and tests before merge.

## Local quality checks

The exact commands will be finalized when the starter workspace is created.
The intended baseline is:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Review expectations

A useful review checks correctness, failure behavior, readability, tests,
security boundaries, and whether the implementation matches the agreed design.
At least one comment must explain reasoning; “looks good” alone is not a review.

