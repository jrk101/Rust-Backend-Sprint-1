# Unit 2: Ownership and trustworthy input

## Before you start

Bring the Unit 1 event inspector and an agreed example webhook body. Read Rust
Book chapter 4 and the string sections of chapter 8. The raw bytes must remain
available for the signature task in Unit 6.

## By the end of this unit you can

- explain ownership, moves, borrowing, and slices using event payloads;
- choose deliberately between owned and borrowed data;
- parse untrusted input without panics;
- make small Git changes that are easy to review.

## At a glance

| Learn | Do |
|---|---|
| Rust Book chapters 4 and 8 | Parse and inspect borrowed payload data |
| UTF-8, byte slices, JSON boundaries | Preserve raw webhook bytes safely |
| Branches and pull-request review | Submit focused changes |

## What proves the work

Run the ownership lab and payload-parser tests. Show that an altered or
malformed byte sequence returns an error without a panic. In review, explain
one necessary allocation and one avoided allocation. The shared rotation
publishes `shared/delivery/payload-contract.md` for the next units.

## 1 · Ownership through payloads

- [ ] **Task 1: Trace moves and borrows** (`R2.1`): Repair a provided event
  program containing move errors from
  `practice/fixtures/unit-02-ownership/`, then explain each compiler message.
  **Primary path:** `practice/<student-id>/unit-02/rust/ownership-lab/`.
- [ ] **Task 2: Borrow before allocating** (`R2.2`): Write functions that
  accept `&str` and `&[u8]` where ownership is unnecessary; compare them with
  owned alternatives. **Primary path:**
  `practice/<student-id>/unit-02/rust/borrowing-lab/`.
- [ ] **Task 3: Preserve raw bytes** (`R2.3`): Parse an event envelope while
  retaining the original bytes that a later signature check will authenticate.
  Reject malformed input through `Result`, never `unwrap` on request data.
  **Primary path:** `practice/<student-id>/unit-02/rust/payload-parser/`.

## 2 · Reviewable engineering

- [ ] **Task 4: Review a compiler-led fix** (`E2.1`): Review a teammate's
  ownership task. Identify whether their fix removes an allocation or merely
  hides the ownership problem by cloning. **Evidence:** pull-request link in
  `practice/<student-id>/unit-02/review-log.md`.

## Notes

Cloning is sometimes correct. “Never clone” is as unhelpful as cloning until
the compiler becomes quiet. The question is who needs to own the value and for
how long.

**If short on time, cut:** performance comparison polish. Never cut Task 3.

## End-of-unit checklist

- [ ] Raw bytes and parsed values remain distinguishable
- [ ] Tests cover invalid input
- [ ] A teammate reviewed the ownership explanation
- [ ] Shared payload contract reviewed

Next: `unit-03.md`.
