# Learning Resources

Use primary documentation to answer the unit's questions. The unit file states what to build and test; these links are starting points, not a substitute for investigation.

| Unit(s) | Primary references |
|---|---|
| 1–3 | [The Rust Book](https://doc.rust-lang.org/book/), [Rust by Example](https://doc.rust-lang.org/rust-by-example/), [Cargo Book](https://doc.rust-lang.org/cargo/) |
| 2, 6 | [Rust byte slices](https://doc.rust-lang.org/std/primitive.slice.html), [HMAC crate](https://docs.rs/hmac/latest/hmac/) |
| 4–6 | [MDN HTTP overview](https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview), [Actix Web getting started](https://actix.rs/docs/getting-started/), [Actix Web applications and state](https://actix.rs/docs/application/), [Actix Web testing](https://actix.rs/docs/testing/) |
| 6, 13–14 | [Actix Web middleware](https://actix.rs/docs/middleware/), [HTTP Semantics](https://www.rfc-editor.org/rfc/rfc9110) |
| 7–8 | [PostgreSQL tutorial](https://www.postgresql.org/docs/current/tutorial.html), [PostgreSQL constraints](https://www.postgresql.org/docs/current/ddl-constraints.html), [SQLx docs](https://docs.rs/sqlx/latest/sqlx/) |
| 9–11 | [Tokio tutorial](https://tokio.rs/tokio/tutorial), [Tokio synchronization](https://docs.rs/tokio/latest/tokio/sync/index.html), [Actix Web runtime](https://docs.rs/actix-web/latest/actix_web/rt/) |
| 10–12 | [PostgreSQL explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html), [PostgreSQL SELECT](https://www.postgresql.org/docs/current/sql-select.html) |
| 13 | [OWASP authentication cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html), [OWASP password storage cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html) |
| 14 | [OWASP SSRF prevention cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html), [OWASP secrets management cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html) |
| 15–16 | [Rust testing](https://doc.rust-lang.org/book/ch11-00-testing.html), [tracing docs](https://docs.rs/tracing/latest/tracing/), [GitHub Actions docs](https://docs.github.com/en/actions), [Docker Compose docs](https://docs.docker.com/compose/) |
| 17 | [README guidance](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes), [GitHub pull requests](https://docs.github.com/en/pull-requests) |

Actix Web is our HTTP framework. It uses Tokio for its runtime; the background-worker units teach Tokio directly. No Actix actor-system knowledge is required.
