# Unit 5: The first PayHook request path

## Before you start

Bring the mock provider and sample merchant from Unit 4. Read the Actix Web
getting-started and server sections in `docs/09-resources.md`. Decide which
local port each application uses.

## At a glance

| Learn | Use immediately |
|---|---|
| Actix Web `App`, route, handler, and response | `A1.4` relay |
| Async outbound HTTP and timeout | `A1.4` relay |
| Request/response and failure boundary | `E3.1` diagram |

## By the end of this unit you can

- run the mock provider, PayHook, and merchant together;
- describe one request across all three applications;
- explain what is lost when the in-memory PayHook process restarts.

## 1 · First vertical slice

- [ ] **Task 1: Build an in-memory relay** (`A1.4`): Receive one event in
  Actix Web and forward it to the merchant with a timeout. Record the response.
  Keep this version unsigned and in memory so its limitations are visible.
  **Primary path:** `practice/<student-id>/unit-05/payhook-slice/`.
- [ ] **Task 2: Draw the observed flow** (`E3.1`): Update the context diagram
  with actual ports, requests, responses, timeout, and process-restart loss.
  **Primary path:** `practice/<student-id>/unit-05/design/first-flow.md`.

## First end-to-end event [MILESTONE]

An outside witness starts the three programs and reproduces a mock-provider →
PayHook → merchant event. The event is unsigned; verification arrives in Unit
6. The witness records one success and one forced merchant failure.

## End-of-unit checklist

- [ ] Outside witness runs the full HTTP path
- [ ] Diagram matches actual requests and failure points
- [ ] Shared rotation merges the first PayHook slice
- [ ] Shared pull request passes format, Clippy, and fast tests

**If short on time, cut:** diagram polish. Never cut the witnessed request.

Next: `unit-06.md`.
