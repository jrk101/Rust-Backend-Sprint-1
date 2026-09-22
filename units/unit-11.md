# Unit 11 — Authentication, limits, and security boundaries

## By the end of this unit you can

- hash passwords and authenticate management users safely;
- distinguish management authentication from webhook signatures;
- apply rate and size limits at deliberate boundaries;
- identify SSRF and secret-handling risks in configurable destinations.

## 1 · Management authentication

- [ ] **Task 1 — Define the auth model** (`D5.1`): Specify registration,
  login, password reset non-goals, authorization ownership, token expiry, and
  revocation limitations. **Primary path:**
  `practice/<student-id>/unit-11/design/auth-model.md`.
- [ ] **Task 2 — Implement password and token handling** (`D5.2`): Use Argon2
  for password hashing and a bounded token/session design for management APIs.
  Do not use management JWTs to authenticate incoming provider webhooks.
  **Primary path:** `practice/<student-id>/unit-11/auth/`.

## 2 · Abuse boundaries

- [ ] **Task 3 — Apply request limits** (`A6.1`): Add payload-size, request-
  timeout, and rate limits to the appropriate endpoints. Explain the identity
  each limiter keys on and its false-positive trade-off.
  **Primary path:** `practice/<student-id>/unit-11/limits/`.
- [ ] **Task 4 — Threat-model destination URLs** (`E8.1`): Identify SSRF,
  redirect, DNS, private-network, and credential-leak risks. Propose a V1 policy
  the cohort can realistically enforce and test.
  **Primary path:** `practice/<student-id>/unit-11/security/threat-model.md`.
- [ ] **Task 5 — Rotate and redact secrets** (`E8.2`): Define storage,
  display-once behavior, logging redaction, and a simple rotation path for source
  secrets. **Primary path:**
  `practice/<student-id>/unit-11/security/secret-policy.md`.

## Optional Redis lab

- [ ] **Task 6 — Compare rate-limit stores** (`D5.3`, optional): Implement the
  same small limiter in process and with Redis. Compare restart behavior,
  multi-instance consistency, failure modes, and operational cost. Do not move
  durable deduplication into Redis. **Primary path:**
  `practice/<student-id>/unit-11/redis-rate-limit-lab/`.

**If short on time, cut:** Task 6, then account-management extras. Never cut the
destination threat model or password hashing.

Next: `unit-12.md`.

