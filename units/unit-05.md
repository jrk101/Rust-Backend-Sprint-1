# Unit 5 — Axum service boundaries and webhook verification

## By the end of this unit you can

- structure Axum routes, extractors, state, and errors;
- share immutable application dependencies with `Arc`;
- explain and implement HMAC verification over raw bytes;
- test valid, invalid, and malformed requests.

## 1 · Service structure

- [ ] **Task 1 — Separate routes from domain work** (`A2.1`): Refactor the
  vertical slice so handlers translate HTTP while a service layer owns the use
  case. **Primary path:** `practice/<student-id>/unit-05/payhook-api/`.
- [ ] **Task 2 — Share application state** (`A2.2`): Use Axum state and `Arc`
  for dependencies. Explain why `Rc` does not fit a multithreaded server and
  why `Mutex` is not automatically required. **Primary path:**
  `practice/<student-id>/unit-05/payhook-api/STATE-NOTES.md`.

## 2 · Verify the sender

- [ ] **Task 3 — Sign and verify raw payloads** (`A2.3`): Implement HMAC signing
  in the mock provider and constant-time verification in PayHook. Define the
  signed message format, timestamp tolerance, and test vectors.
  **Primary path:** `practice/<student-id>/unit-05/signature-lab/`.
- [ ] **Task 4 — Return safe HTTP errors** (`A2.4`): Map missing signatures,
  stale timestamps, invalid signatures, malformed JSON, and oversized bodies to
  deliberate responses without leaking the secret or signature.
  **Primary path:** `practice/<student-id>/unit-05/http-errors.md` and tests.
- [ ] **Task 5 — Test the trust boundary** (`E4.1`): Add black-box HTTP tests
  for correct, altered, stale, and malformed requests.
  **Primary path:** `practice/<student-id>/unit-05/tests/`.

**If short on time, cut:** custom error presentation. Never cut altered-payload
and stale-timestamp tests.

Next: `unit-06.md`.

