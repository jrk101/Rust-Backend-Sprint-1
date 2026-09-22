# Unit 3 — Domain modelling and explicit failure

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

Complete a short lifetime exercise involving a borrowed parsed view. Do not
carry a lifetime-heavy abstraction into PayHook unless it makes the application
simpler than owning the data.

**If short on time, cut:** the lifetime extension. Never cut Tasks 1–2.

Next: `unit-04.md`.

