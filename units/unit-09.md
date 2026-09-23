# Unit 9: Tokio delivery workers

## Before you start

Bring the durable delivery rows from Unit 8 and a sample merchant that can
delay or fail. Read the Tokio tutorial sections on tasks and channels. A
channel may wake a worker, while PostgreSQL continues to hold due work.

## At a glance

| Learn | Use immediately |
|---|---|
| Futures and `.await` | `A3.1` |
| Bounded channels and backpressure | `A3.2` |
| Timeouts and HTTP outcomes | `A3.3` |
| Semaphore or worker bound, shutdown | `A3.4–A3.5`, `E6.1` |

## By the end of this unit you can

- explain futures, async functions, tasks, and cooperative scheduling;
- use channels without treating process memory as durable storage;
- make outbound HTTP calls with timeouts and bounded concurrency;
- shut a worker down without abandoning claimed work silently.

## 1 · Async mechanics

- [ ] **Task 1: Observe a future** (`A3.1`): Build a small timing lab that
  compares sequential work, concurrent async work, and accidentally blocking
  work. Explain what `.await` does and does not do.
  **Primary path:** `practice/<student-id>/unit-09/async-lab/`.
- [ ] **Task 2: Coordinate tasks with a channel** (`A3.2`): Send delivery IDs
  through a bounded Tokio channel. Demonstrate backpressure and explain why the
  database, not the channel, remains the source of truth.
  **Primary path:** `practice/<student-id>/unit-09/channel-lab/`.

## 2 · Outbound delivery

- [ ] **Task 3: Implement one delivery attempt** (`A3.3`): POST the stored raw
  payload with safe headers, connect/request timeouts, response-size limits, and
  an explicit success policy. **Primary path:**
  `practice/<student-id>/unit-09/delivery-client/`.
- [ ] **Task 4: Bound concurrency** (`A3.4`): Process several independent
  deliveries concurrently while enforcing a configured limit. Prove the limit
  with a controllably slow merchant. **Primary path:**
  `practice/<student-id>/unit-09/delivery-worker/`.
- [ ] **Task 5: Handle shutdown** (`A3.5`): Stop accepting new work, allow or
  cancel in-flight attempts deliberately, and leave recoverable database state.
  **Primary path:** `practice/<student-id>/unit-09/shutdown-notes.md` and tests.
- [ ] **Task 6: Test async failure modes** (`E6.1`): Cover timeout, connection
  refusal, `2xx`, `4xx`, `5xx`, and shutdown behavior.
  **Primary path:** `practice/<student-id>/unit-09/tests/`.

**If short on time, cut:** throughput comparison. Never cut timeouts or the
concurrency bound.

## End-of-unit checklist

- [ ] Slow merchant cannot create unbounded concurrent requests
- [ ] Timeout and connection failure are recorded as distinct outcomes
- [ ] Shutdown leaves work in a state the restarted worker can recover
- [ ] Shared rotation demonstrates the worker with the failing merchant

Next: `unit-10.md`.
