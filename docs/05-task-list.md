# Master Task List

The unit files are the working instructions. This index exists to check scope,
stable IDs, sequencing, and curriculum coverage.

| Unit | Rust Core | Async & Backend | Database & Backend | Engineering & Delivery |
|---|---|---|---|---|
| 1 | `R1.1–R1.3` workspace and event basics | — | — | `E1.1–E1.2` brief and context |
| 2 | `R2.1–R2.3` ownership, borrowing, payload bytes | — | — | `E2.1` compiler-led review |
| 3 | `R3.1–R3.4` domain types, errors, traits, tests | — | — | — |
| 4 | — | `A1.1–A1.4` HTTP and first vertical slice | — | `E3.1` observed-flow diagram |
| 5 | — | `A2.1–A2.4` Axum, state, HMAC, HTTP errors | — | `E4.1` trust-boundary tests |
| 6 | — | — | `D1.1–D1.5` schema, SQL, migrations, SQLx | — |
| 7 | — | — | `D2.1–D2.3` transactions and deduplication | `E5.1` design sign-off |
| 8 | — | `A3.1–A3.5` tasks, channels, delivery, shutdown | — | `E6.1` async failure tests |
| 9 | `R4.1` priority-queue lab | `A4.1–A4.2` retry policy and restart | `D3.1–D3.3` attempts and work claiming | — |
| 10 | — | `A5.1–A5.2` management API and exposure | `D4.1–D4.2` dead letters and replay | `E7.1` reliability demonstration |
| 11 | — | `A6.1` request limits | `D5.1–D5.3` auth and optional Redis | `E8.1–E8.2` threat and secret policy |
| 12 | — | — | — | `E9.1–E9.6` quality, tracing, containers, CI, incident |
| 13 | — | — | — | `E10.1–E10.6` architecture, runbook, clean start, handover |

## Milestone tasks

- Unit 1: `E1.1`, `E1.2`
- Unit 4: `A1.2`, `A1.3`, `A1.4`, `E3.1`
- Unit 7: `D2.1`, `D2.2`, `D2.3`, `E5.1`
- Unit 10: `D4.1`, `D4.2`, `E7.1`
- Unit 13: `E10.2`, `E10.3`, `E10.5`

## Coverage that stays outside the shared product when appropriate

- Advanced lifetime exercises
- Basic DSA through the priority-queue comparison
- Redis implementation lab
- Redis Pub/Sub as a possible extension exercise
- Additional smart-pointer exercises beyond `Arc`-based application state

These are learning requirements, not reasons to complicate PayHook.

