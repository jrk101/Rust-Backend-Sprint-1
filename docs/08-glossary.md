# Glossary

**Webhook** — An HTTP request one system sends to another when an event occurs.

**Source** — A configured origin of webhook events and the secret/policy used to verify it.

**Destination** — The configured application endpoint to which PayHook delivers events.

**HMAC** — A keyed message-authentication code. PayHook computes it over the
agreed message bytes and compares it in constant time with the sender's signature.

**Deduplication** — Recognizing the same provider event identity so it is not
accepted as a new event repeatedly.

**Idempotency** — Designing an operation so repeating the same request does not
create unintended additional effects.

**At-least-once delivery** — A delivery may occur more than once, but durable
work is not silently abandoned. Receivers should handle duplicates safely.

**Exactly-once delivery** — A stronger claim that is generally not achievable
across an ordinary HTTP boundary without cooperation and additional constraints.

**Delivery attempt** — One outbound HTTP request and its recorded outcome.

**Exponential backoff** — Increasing the wait after repeated failures, usually
with a maximum delay.

**Jitter** — Controlled randomness added to retry delays to prevent many jobs
from retrying at exactly the same instant.

**Dead letter** — A delivery that exhausted its automatic policy and requires
inspection or deliberate replay.

**Replay** — An operator-requested new attempt or delivery based on a stored event.

**Transaction** — A set of database changes that commit or roll back together.

**Unique constraint** — A database rule that rejects duplicate values or value
combinations. It is PayHook's durable event-identity boundary.

**Connection pool** — A bounded reusable set of database connections shared by requests and workers.

**Future** — A value representing asynchronous work that may later produce a result.

**Tokio task** — A cooperatively scheduled asynchronous task running on Tokio's runtime.

**Actix Web** — The HTTP framework used for PayHook's routes, extractors, shared application data, and responses. It runs on Tokio; using Actix Web does not require using the separate Actix actor framework.

**Unit** — A focused learning-and-build block, planned for roughly one week. It ends with reviewable evidence; difficult units may take longer.

**Backpressure** — A mechanism that slows producers when consumers cannot keep up.

**Bounded concurrency** — A fixed maximum number of operations allowed in flight.

**Graceful shutdown** — Stopping new work and resolving in-flight work into a
known recoverable state before exiting.

**SSRF** — Server-side request forgery: an attacker causes the server to make
requests to unintended internal or sensitive locations through a configurable URL.

**Modular monolith** — One deployable application with explicit internal module
boundaries, without splitting prematurely into networked services.
