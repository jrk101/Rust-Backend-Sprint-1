# Task Acceptance Guide

The unit file supplies instructions and the primary path. This guide supplies
the minimum observable proof for review. A passing test log belongs in the
pull-request description, while code and design evidence belongs in the task
path. An extension task is assessed only when attempted.

## Unit 1

| ID | Minimum proof |
|---|---|
| `E1.1` | Brief names user, failure, V1, non-goals, success check, and five open questions. |
| `E1.2` | Diagram labels provider, PayHook, database, merchant, operator, and trust boundaries. |
| `R1.1` | `cargo run`, formatting, Clippy, and one passing test are recorded. |
| `R1.2` | Three event kinds produce distinct safe summaries. |
| `R1.3` | Written reasons cover enum, integer money representation, and redacted data. |

## Unit 2

| ID | Minimum proof |
|---|---|
| `R2.1` | Repaired program compiles and each original move error is explained. |
| `R2.2` | API accepts borrowed text/bytes; note explains one owned alternative. |
| `R2.3` | Tests prove raw bytes are retained and malformed data returns `Err`. |
| `E2.1` | Linked review identifies a justified or unjustified clone. |

## Unit 3

| ID | Minimum proof |
|---|---|
| `R3.1` | Tests show accepted and rejected delivery transitions. |
| `R3.2` | Failure classes map to safe messages without secrets. |
| `R3.3` | Fake verifier can substitute for the real boundary in a test; note justifies the trait. |
| `R3.4` | Tests cover invalid state, missing optional data, and safe formatting. |
| `R3.5` | Extension: borrowed view compiles and ownership alternative is explained. |

## Unit 4

| ID | Minimum proof |
|---|---|
| `A1.1` | Captured request and response correctly label method, headers, body, and status. |
| `A1.2` | Mock provider sends a deterministic local test event. |
| `A1.3` | Merchant returns at least success and deliberate failure. |

## Unit 5

| ID | Minimum proof |
|---|---|
| `A1.4` | Relay forwards once with a timeout; outside witness reproduces flow. |
| `E3.1` | Diagram matches observed ports and failure boundaries. |

## Unit 6

| ID | Minimum proof |
|---|---|
| `A2.1` | Handler delegates domain work; a service test runs without HTTP. |
| `A2.2` | State explanation justifies `Arc` and avoids an unnecessary lock. |
| `A2.3` | Shared test vector verifies raw-body HMAC and rejects altered bytes. |
| `A2.4` | Missing, stale, invalid, malformed, and oversized cases return safe responses. |
| `E4.1` | Black-box tests exercise the signature boundary. |

## Unit 7

| ID | Minimum proof |
|---|---|
| `D1.1` | Every table has a one-row meaning, key, and required relationship. |
| `D1.2` | Migrations run on a fresh disposable database and reject a broken relation. |
| `D1.3` | SQL inserts, updates, joins, and retrieves complete attempt history. |
| `D1.4` | SQLx uses a bounded pool and environment-supplied connection URL. |
| `D1.5` | Event survives process restart; isolated database test passes. |

## Unit 8

| ID | Minimum proof |
|---|---|
| `D2.1` | Diagram states what commits atomically and what happens on crash. |
| `D2.2` | Database unique constraint rejects repeated source/event identity. |
| `D2.3` | Repeated synchronized concurrent test yields one event and one initial delivery; reviewer verifies the database constraint is the final guarantee. |
| `E5.1` | Revised sequence diagram and decision record match tested behavior. |

## Unit 9

| ID | Minimum proof |
|---|---|
| `A3.1` | Timings show the difference between sequential, concurrent, and blocking operations. |
| `A3.2` | Bounded channel demonstrates backpressure; note states database authority. |
| `A3.3` | One attempt records success, timeout, and network failure distinctly. |
| `A3.4` | Slow merchant test proves configured in-flight limit. |
| `A3.5` | Shutdown test leaves claimed work recoverable. |
| `E6.1` | Failure matrix includes `2xx`, `4xx`, `5xx`, timeout, refusal, and shutdown. |

## Unit 10

| ID | Minimum proof |
|---|---|
| `D3.1` | Attempt record and policy classify retry versus terminal outcome. |
| `A4.1` | Fixed clock/random source makes backoff tests deterministic and capped. |
| `D3.2` | Due-work query and index rationale match filter and ordering. |

## Unit 11

| ID | Minimum proof |
|---|---|
| `D3.3` | Two workers cannot claim the same eligible attempt simultaneously; short transaction and abandoned-claim recovery are documented. |
| `A4.2` | Scheduled retry runs after process restart. |
| `R4.1` | Extension: heap scheduler works and persistence trade-off is explained. |

## Unit 12

| ID | Minimum proof |
|---|---|
| `D4.1` | Exhausted delivery enters dead letter and is excluded from ordinary due work. |
| `D4.2` | Repeated replay request has a defined idempotent outcome. |
| `E7.2` | Local command inspects history and initiates one replay. |
| `E7.1` | Outside witness observes failure, dead letter, replay, recovery, and retained history. |

## Unit 13

| ID | Minimum proof |
|---|---|
| `D5.1` | Auth design names operator scope, expiry, revocation limit, and rejected alternative. |
| `D5.2` | Protected credential check works; Argon2 lab covers local accounts if not in V1. |
| `A5.1` | Authenticated operator can list and fetch paginated history. |
| `A5.2` | Unauthenticated and cross-owner cases are rejected where applicable; sensitive fields redacted. |

## Unit 14

| ID | Minimum proof |
|---|---|
| `A6.1` | Oversized or excessive requests receive a deliberate response; key trade-off documented. |
| `E8.1` | Mentor-reviewed destination policy and adversarial tests handle local/private targets, redirects, and DNS or connection changes. |
| `E8.2` | Secrets do not appear in log or management response; rotation path documented. |
| `D5.3` | Extension: in-process and Redis limiters compared under restart and outage. |

## Unit 15

| ID | Minimum proof |
|---|---|
| `E9.1` | Each core invariant maps to a meaningful test with a named gap. |
| `E9.2` | Correlation fields connect ingress to attempt without logging payload secrets. |
| `E9.4` | PR check runs format, Clippy, fast tests, and migration/database check. |

## Unit 16

| ID | Minimum proof |
|---|---|
| `E9.3` | Clean Compose start succeeds with documented environment settings. |
| `E9.5` | Evidence trail identifies failure before the fix. |
| `E9.6` | Postmortem includes impact, timeline, cause, detection, fix, and prevention. |

## Unit 17

| ID | Minimum proof |
|---|---|
| `E10.1` | Final diagram depicts only components actually running. |
| `E10.2` | Runbook explains startup, migration, health, diagnosis, replay, and shutdown. |
| `E10.3` | Non-author succeeds from clean checkout and records corrections. |
| `E10.4` | Demo shows core reliability sequence and honest limitation statement. |
| `E10.5` | README lets a new developer find setup, tests, architecture, and demo. |
| `E10.6` | Retrospective names changed design, hard failure, rejected technology, and next step. |
