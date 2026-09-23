# Unit 14: Limits and security boundaries

## Before you start

Bring the protected management API from Unit 13. Read the OWASP SSRF guidance
and Actix Web middleware material in `docs/09-resources.md`. Use only local
test targets when exploring blocked destination URLs.

## At a glance

| Learn | Use immediately |
|---|---|
| Size, timeout, and rate limits | `A6.1` |
| SSRF and configurable destinations | `E8.1` |
| Secret rotation and redaction | `E8.2` |
| Redis versus in-process limiter | `D5.3` extension |

## By the end of this unit you can

- bound untrusted HTTP requests;
- state which destination URLs V1 allows and why;
- rotate and redact a source signing secret;
- explain when Redis would improve rate limiting and when it would add cost.

## 1 · Bound the API and its destinations

- [ ] **Task 1: Apply request limits** (`A6.1`): Add payload-size,
  request-timeout, and rate limits at the appropriate Actix Web boundaries.
  Explain the identity each limiter keys on and its false-positive trade-off.
  **Primary path:** `practice/<student-id>/unit-14/limits/`.
- [ ] **Task 2: Threat-model destination URLs** (`E8.1`): Identify SSRF,
  redirect, DNS, private-network, and credential-leak risks. Propose a V1
  allow/block policy the cohort can test. Include local and link-local targets,
  every resolved address, redirect behavior, and whether the HTTP client can
  connect to a different address after validation. **Primary path:**
  `practice/<student-id>/unit-14/security/threat-model.md`.
- [ ] **Task 3: Rotate and redact secrets** (`E8.2`): Define storage,
  display-once behavior, logging redaction, and a simple source-secret
  rotation path. **Primary path:**
  `practice/<student-id>/unit-14/security/secret-policy.md`.

## Optional Redis lab

- [ ] **Task 4: Compare rate-limit stores** (`D5.3`, extension): Implement
  the same small limiter in process and with Redis. Compare restart behavior,
  multi-instance consistency, failure modes, and operational cost. Do not move
  durable deduplication into Redis. **Primary path:**
  `practice/<student-id>/unit-14/redis-rate-limit-lab/`.

## End-of-unit checklist

- [ ] Oversized and excessive requests receive deliberate responses
- [ ] Private-network and redirect targets follow the agreed destination policy
- [ ] Secrets are absent from logs and management responses
- [ ] Shared rotation merges policy with adversarial tests

**If short on time, cut:** Task 4, then secondary limiter tuning. Never cut
the destination policy or secret redaction.

**Mentor checkpoint:** Budget an additional review session for `E8.1` and
provide a mentor-owned policy and adversarial test set for comparison. Review
the actual outbound client configuration, not only URL parsing. If the cohort
cannot demonstrate a safe general-purpose policy, restrict V1 to explicitly
approved destinations and keep that limitation visible in the runbook.

Next: `unit-15.md`.
