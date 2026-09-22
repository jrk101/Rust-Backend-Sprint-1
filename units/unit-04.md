# Unit 4 — HTTP foundations and the two test applications

## Before you start

Bring the Unit 3 domain model. Read the HTTP and Actix Web links for Unit 4 in
`docs/09-resources.md`. Start the mock provider and sample merchant on distinct
local ports and record the values in a local, ignored configuration file.

## At a glance

| Learn | Use immediately |
|---|---|
| Request method, URL, headers, body, status | `A1.1` capture |
| Actix Web route and JSON handler | `A1.3` merchant |
| Async HTTP client | `A1.2` provider |

## By the end of this unit you can

- describe an HTTP request and response without framework vocabulary;
- create a small Actix Web receiver and an async HTTP client;
- explain why a provider can send a request but cannot ensure a merchant accepts it.

## 1 · See the protocol

- [ ] **Task 1 — Capture an HTTP exchange** (`A1.1`): Send a request manually
  and annotate method, path, headers, body, status, and timeout behavior.
  **Primary path:** `practice/<student-id>/unit-04/http/http-notes.md`.
- [ ] **Task 2 — Build the mock provider** (`A1.2`): Create a minimal program
  that sends one deterministic JSON event. Signing arrives in Unit 6.
  **Primary path:** `practice/<student-id>/unit-04/mock-provider/`.
- [ ] **Task 3 — Build the sample merchant** (`A1.3`): Create an Actix Web endpoint
  that records a safe summary and returns a configurable status sequence.
  **Primary path:** `practice/<student-id>/unit-04/sample-merchant/`.

## End-of-unit checklist

- [ ] Mock provider reaches the sample merchant directly
- [ ] Success and deliberate failure responses are captured without secrets
- [ ] Shared rotation merges the two small support applications

**If short on time, cut:** additional response patterns. Never cut the first
manual HTTP exchange or the two working support applications.

Next: `unit-05.md`.
