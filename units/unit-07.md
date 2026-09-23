# Unit 7: PostgreSQL, SQL, and SQLx

## Before you start

Run the disposable PostgreSQL check from `docs/07-tools-setup.md`. Read the
PostgreSQL and SQLx sections in `docs/09-resources.md`. The Unit 6 service is
still allowed to run in memory while the schema is designed.

## At a glance

| Learn | Use immediately |
|---|---|
| Table grain, primary and foreign keys | `D1.1` |
| Constraints and migrations | `D1.2` |
| CRUD and joins in SQL | `D1.3` |
| SQLx pool and queries | `D1.4–D1.5` |

## By the end of this unit you can

- design relational tables from domain requirements;
- write and explain a migration before using an ORM-like abstraction;
- use SQLx with a connection pool and configuration from the environment;
- test database behavior against a disposable database.

## 1 · Design durable state

- [ ] **Task 1: Design the first schema** (`D1.1`): Define sources,
  destinations, events, deliveries, and delivery attempts. State each table's
  grain, keys, required fields, and retention-sensitive fields. Add a users
  table only if the Unit 1 V1 choice includes local user accounts; otherwise
  explain the configured-operator alternative.
  **Primary path:** `practice/<student-id>/unit-07/design/data-model.md`.
- [ ] **Task 2: Write forward migrations** (`D1.2`): Create migrations with
  primary keys, foreign keys, timestamps, status checks, and the event identity
  constraint. **Primary path:**
  `practice/<student-id>/unit-07/database/migrations/`.
- [ ] **Task 3: Exercise the SQL directly** (`D1.3`): Insert, query, update,
  and join representative rows. Explain the query that shows an event's full
  attempt history. **Primary path:**
  `practice/<student-id>/unit-07/database/queries.sql`.
- [ ] **Task 4: Add SQLx and pooling** (`D1.4`): Connect through a bounded
  pool, apply configuration through environment variables, and persist an event.
  **Primary path:** `practice/<student-id>/unit-07/sqlx-store/`.
- [ ] **Task 5: Prove persistence** (`D1.5`): Restart the process and show the
  accepted event still exists. Add an integration test with isolated state.
  **Primary path:** `practice/<student-id>/unit-07/sqlx-store/tests/`.

**If short on time, cut:** optional query helpers. Never cut schema grain,
constraints, or the restart proof.

## End-of-unit checklist

- [ ] Migrations run from an empty disposable database
- [ ] Event survives an application restart
- [ ] Query shows source, destination, delivery, and attempts without hidden state
- [ ] Shared rotation merges schema and repository with database test evidence

Next: `unit-08.md`.
