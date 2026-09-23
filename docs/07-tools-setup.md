# Tools Setup

Complete the Git and Rust checks before Unit 1. Add the database and container
tools before the units that use them. Ask a mentor for help with any failed
setup step.

## 1. Git and GitHub

Install [Git](https://git-scm.com/downloads), create a GitHub account, and
configure an email verified on that account:

```bash
git --version
git config --global user.name "Your Name"
git config --global user.email "your-verified-email@example.com"
```

Clone the cohort repository using its actual URL, then check that `git status`
works. On Windows, use PowerShell; on macOS/Linux, use your terminal. Students
may use Git Bash on Windows, but should use one shell consistently while
following commands.

## 2. Rust

Install Rust through [rustup](https://rustup.rs/). On Windows, install the
compiler prerequisites rustup requests. Restart the terminal after installation.

```bash
rustc --version
cargo --version
rustup component add rustfmt clippy
cargo fmt --version
cargo clippy --version
```

The starter workspace already commits `rust-toolchain.toml` and `Cargo.lock`.
Use its selected stable toolchain for cohort work. Individual practice crates
may use the same toolchain.

## 3. Editor

Use an editor with [rust-analyzer](https://rust-analyzer.github.io/) support.
VS Code is one option. The check is practical: open a new Cargo crate, see
diagnostics, run a test from the terminal, and jump to a function definition.

## 4. Containers and PostgreSQL

Install a container engine with Compose support from the [Docker](https://docs.docker.com/get-docker/)
or [Podman](https://podman.io/docs/installation) documentation. Verify:

```bash
docker --version
docker compose version
```

If using Podman, replace those commands with the equivalent `podman` commands.
Before Unit 7, run a disposable PostgreSQL container and connect with `psql`
or an approved database client. The cohort's shared `compose.yaml` will define
the exact image, port, and database when the cohort builds its deployment stack.
Do not reuse an existing personal database for exercises.

## 5. HTTP client

Check `curl --version`, or install an equivalent client that can send raw
request bodies and custom headers. PowerShell users should call `curl.exe` if
`curl` resolves to a PowerShell alias on their machine.

## 6. Secrets and local configuration

Use a local `.env` file ignored by Git. The committed `.env.example` lists
required variable names with harmless examples. Use generated secrets only for
the mock provider. Never use real payment accounts, credentials, or payloads.

## 7. Readiness exercise

Create a throwaway crate outside the cohort repository:

```bash
cargo new rust-readiness
cd rust-readiness
cargo run
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

The readiness gate passes when all commands succeed and the student can say
which command compiles, runs, formats, lints, and tests. Do not commit this
throwaway crate to the cohort repository.

If setup fails, post the operating system, command, complete error, and what
you tried. See `13-troubleshooting.md` for common causes.
