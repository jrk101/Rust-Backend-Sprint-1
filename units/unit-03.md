# Unit 3 — Domain modelling and explicit failure

## Before you start

Bring the payload contract from Unit 2. Read Rust Book chapters 6, 9, 10, and
11 as routed by `docs/09-resources.md`. Draft the legal delivery states before
writing transition code.

## By the end of this unit you can

- model valid and invalid states with structs and enums;
- use `Option`, `Result`, pattern matching, and the `?` operator;
- design domain errors that do not leak secrets;
- use traits and generics where a real boundary needs substitution.

## At a glance

| Learn | Do |
|---|---|
| Rust Book chapters 5–7, 9–10 | Model events, deliveries, and errors |
| Traits and generic bounds | Define a signature-verifier boundary |
| Unit testing | Prove domain invariants |

## Project need

Strings such as `"failed"` allow invalid and inconsistent states. Typed states
and errors give the future HTTP and database layers one shared vocabulary.

## 1 · Domain types

- [ ] **Task 1 — Model the delivery lifecycle** (`R3.1`): Create types for
  source, destination, event identity, delivery status, and attempt outcome.
  Make impossible transitions difficult to express. **Primary path:**
  `practice/<student-id>/unit-03/domain-model/`.
- [ ] **Task 2 — Design the error model** (`R3.2`): Separate malformed input,
  failed authentication, duplicate events, temporary delivery failures, and
  internal failures. Map them to safe messages. **Primary path:**
  `practice/<student-id>/unit-03/error-model/`.
- [ ] **Task 3 — Introduce one useful trait** (`R3.3`): Define a verifier
  trait and implement a deterministic fake. Explain why a trait is useful at
  this boundary and why every struct does not need one. **Primary path:**
  `practice/<student-id>/unit-03/verifier-trait/`.
- [ ] **Task 4 — Test the invariants** (`R3.4`): Test valid transitions,
  rejected transitions, optional fields, and safe error formatting.
  **Primary path:** tests beside the code plus
  `practice/<student-id>/unit-03/test-notes.md`.

## Focused concept lab

- [ ] **Task 5 — Explain one borrowed view** (`R3.5`, extension): Complete a
  small lifetime exercise involving a parsed view of the Unit 2 payload.
  Explain when ownership is simpler. **Primary path:**
  `practice/<student-id>/unit-03/lifetime-lab/`.

## End-of-unit checklist

- [ ] Invalid transitions have failing tests before they are fixed
- [ ] Safe error formatting omits secrets and full payloads
- [ ] Shared rotation merges a domain vocabulary with test evidence

**If short on time, cut:** Task 5. Never cut Tasks 1–2.

Next: `unit-04.md`.
