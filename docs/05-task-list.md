# Master Task List

Use the unit file for instructions and [the acceptance guide](16-task-acceptance.md) for minimum proof. IDs identify tasks, not weeks; they remain stable if a task changes unit.

| Unit | Focus | Task IDs | Gate |
|---:|---|---|---|
| 1 | Brief and Rust setup | `E1.1`, `E1.2`, `R1.1`, `R1.2`, `R1.3` | Brief agreed |
| 2 | Ownership and raw bytes | `R2.1`, `R2.2`, `R2.3`, `E2.1` | |
| 3 | Domain model | `R3.1`, `R3.2`, `R3.3`, `R3.4`, `R3.5` (extension) | |
| 4 | HTTP and test apps | `A1.1`, `A1.2`, `A1.3` | |
| 5 | First request path | `A1.4`, `E3.1` | First vertical slice |
| 6 | Actix Web and verification | `A2.1`, `A2.2`, `A2.3`, `A2.4`, `E4.1` | |
| 7 | PostgreSQL and SQLx | `D1.1`, `D1.2`, `D1.3`, `D1.4`, `D1.5` | |
| 8 | Transactional ingestion | `D2.1`, `D2.2`, `D2.3`, `E5.1` | Durable ingress |
| 9 | Tokio workers | `A3.1`, `A3.2`, `A3.3`, `A3.4`, `A3.5`, `E6.1` | |
| 10 | Retry policy | `D3.1`, `A4.1`, `D3.2` | |
| 11 | Work claiming and restart | `D3.3`, `A4.2`, `R4.1` (extension) | |
| 12 | Dead letter and replay | `D4.1`, `D4.2`, `E7.2`, `E7.1` | Reliable delivery |
| 13 | Protected management | `D5.1`, `D5.2`, `A5.1`, `A5.2` | |
| 14 | Limits and security | `A6.1`, `E8.1`, `E8.2`, `D5.3` (extension) | |
| 15 | Tests, tracing, CI | `E9.1`, `E9.2`, `E9.4` | |
| 16 | Containers and incident | `E9.3`, `E9.5`, `E9.6` | |
| 17 | Handover | `E10.1`, `E10.2`, `E10.3`, `E10.4`, `E10.5`, `E10.6` | Independent clean start |

Individual practice precedes shared implementation. Every shared change needs a reviewer and evidence; see [the shared build plan](11-shared-build-plan.md).
