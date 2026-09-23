# Unit 11: Claim due work and survive restart

## Before you start

Bring Unit 10's due-work query, attempt model, and failure policy. Read the
PostgreSQL locking and Tokio task references in `docs/09-resources.md`. Use a
disposable database so two-worker tests can run repeatedly.

## At a glance

| Learn | Use immediately |
|---|---|
| Row locking and ownership of work | `D3.3` |
| Durable scheduling across process restarts | `A4.2` |
| In-memory priority queue trade-off | `R4.1` extension |

## By the end of this unit you can

- explain why two workers might select the same due row;
- claim work safely without a long transaction around the HTTP request;
- restart PayHook and process due work from PostgreSQL.

## 1 · Safe worker coordination

- [ ] **Task 1: Claim work safely** (`D3.3`): Compare optimistic claims with
  a PostgreSQL row-locking approach such as `FOR UPDATE SKIP LOCKED`. Agree a
  short transaction that marks a claim and commits before outbound HTTP;
  define ownership, expiry or recovery of abandoned claims, and the state
  update after the response. Prove two live workers cannot claim the same
  eligible attempt at the same time. **Primary path:**
  `practice/<student-id>/unit-11/work-claiming/`.
- [ ] **Task 2: Survive a restart** (`A4.2`): Stop PayHook after scheduling
  a retry, restart it, and show the persisted delivery becomes eligible and
  runs. **Primary path:** `practice/<student-id>/unit-11/restart-proof.md`.

## Focused DSA lab

- [ ] **Task 3: Compare scheduling structures** (`R4.1`, extension): Build
  a small `BinaryHeap` scheduler and explain why it cannot replace PostgreSQL
  as the durable due-work source. **Primary path:**
  `practice/<student-id>/unit-11/priority-queue-lab/`.

## End-of-unit checklist

- [ ] Two workers claim one eligible delivery once per attempt
- [ ] HTTP is performed outside a long-held database lock
- [ ] Crashed or expired claims have a documented recovery path
- [ ] Restarted worker finds persisted due work
- [ ] Shared rotation merges claim strategy and restart test

**If short on time, cut:** Task 3. Never cut the multiple-worker or restart
proof.

**Mentor checkpoint:** Approve one shared claim protocol before rotation
implementation. Walk through two simultaneous workers, a crash after claim,
and a crash after HTTP but before recording the result. The last case may
produce another delivery attempt; describe the system as at-least-once, not
exactly-once.

Next: `unit-12.md`.
