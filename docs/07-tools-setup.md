# Tools Setup — Draft

This guide will receive OS-specific commands after the cohort's supported
operating systems are confirmed. Do not install paid services for this sprint.

## Required

| Tool | Purpose | Check |
|---|---|---|
| Git | Version control and review workflow | `git --version` |
| Rust via rustup | Compiler, Cargo, formatting, Clippy | `rustup show` |
| Editor with rust-analyzer | Feedback and navigation | Open a Rust crate without errors |
| Docker with Compose | Local PostgreSQL and final reproducible environment | `docker compose version` |
| PostgreSQL client | Inspect and debug database state | `psql --version` or approved equivalent |
| API client | Send and inspect HTTP requests | `curl --version` or approved equivalent |

## Rust toolchain baseline

Use the repository's pinned stable toolchain once `rust-toolchain.toml` is
created. The starter workspace must also commit a lockfile.

```bash
rustup component add rustfmt clippy
cargo --version
rustc --version
```

## Configuration rules

- Local secrets go in ignored environment files or the operating system's environment.
- Provide a committed `.env.example` containing names, never values.
- Use only generated local secrets in mock-provider exercises.
- Database data directories and container volumes never enter Git.
- No real payment-provider keys or customer payloads are used.

## Readiness check

Before Unit 1, every student must be able to clone, create a branch, compile a
hello-world crate, run a test, format it, run Clippy, and start then stop a
minimal PostgreSQL container.

