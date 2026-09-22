# Unit 4 — HTTP foundations and the first vertical slice

## Before you start

Bring the Unit 3 domain model. Read the HTTP and Axum links for Unit 4 in
`docs/09-resources.md`. Start three applications on distinct local ports and
record the values in a local, ignored configuration file.

## At a glance

| Learn | Use immediately |
|---|---|
| Request method, URL, headers, body, status | `A1.1` capture |
| Axum route and JSON handler | `A1.3` merchant and `A1.4` relay |
| Async HTTP client and timeout | `A1.2` provider and `A1.4` relay |
| Failure boundaries | `E3.1` observed-flow diagram |

## By the end of this unit you can

- describe an HTTP request and response without framework vocabulary;
- create small Axum servers and an async HTTP client;
- run three local applications together;
- demonstrate one event moving end to end.

## 1 · See the protocol

- [ ] **Task 1 — Capture an HTTP exchange** (`A1.1`): Send a request manually
  and annotate method, path, headers, body, status, and timeout behavior.
  **Primary path:** `practice/<student-id>/unit-04/http/http-notes.md`.
- [ ] **Task 2 — Build the mock provider** (`A1.2`): Create a minimal program
  that sends one deterministic JSON event. Signing arrives next unit.
  **Primary path:** `practice/<student-id>/unit-04/mock-provider/`.
- [ ] **Task 3 — Build the sample merchant** (`A1.3`): Create an Axum endpoint
  that records a safe summary and returns a configurable status sequence.
  **Primary path:** `practice/<student-id>/unit-04/sample-merchant/`.
- [ ] **Task 4 — Build an in-memory relay** (`A1.4`): Receive an event and
  forward it to the merchant with a timeout. Do not add persistence or retries.
  **Primary path:** `practice/<student-id>/unit-04/payhook-slice/`.
- [ ] **Task 5 — Draw the observed flow** (`E3.1`): Update the context diagram
  with actual ports, requests, responses, and failure points.
  **Primary path:** `practice/<student-id>/unit-04/design/first-flow.md`.

## First end-to-end event [MILESTONE]

The gate passes when another student can run all three programs and show an
unsigned, in-memory test event reaching the merchant. Signing arrives in Unit
5. This version is intentionally unreliable; its limitations become the next
requirements.

## End-of-unit checklist

- [ ] Outside witness runs provider → PayHook → merchant
- [ ] Request and response evidence is saved without secrets
- [ ] Shared rotation merges the three-application slice
- [ ] Shared pull request passes format, Clippy, and fast tests

**If short on time, cut:** merchant status configurability, then diagram polish.
Never cut the demonstrated end-to-end request.

Next: `unit-05.md`.
