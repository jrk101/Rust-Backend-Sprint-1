# Unit 11 — Protected management and security boundaries

## Before you start

Bring the Unit 10 local operator flow. Read the password storage and SSRF
guidance in `docs/09-resources.md`. Decide whether the shared V1 uses one
configured operator or local user accounts; document the decision and
authorization ownership before exposing management routes.

## At a glance

| Learn | Use immediately |
|---|---|
| Authentication and authorization | `D5.1–D5.2`, `A5.1` |
| Safe response shaping | `A5.2` |
| Request limits and SSRF | `A6.1`, `E8.1` |
| Secret handling | `E8.2` |
| Redis comparison | `D5.3` extension |

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
  for password hashing if V1 has local user accounts. If V1 uses one configured
  operator, implement a bounded credential check and complete the Argon2
  practice lab separately. Define token/session expiry and revocation limits.
  Do not use management JWTs to authenticate incoming provider webhooks.
  **Primary path:** `practice/<student-id>/unit-11/auth/`.

## 2 · Protected management

- [ ] **Task 3 — Build inspection endpoints** (`A5.1`): After authentication
  is enforced, list and fetch events, deliveries, and attempts with stable
  pagination, filtering, and bounded page sizes. **Primary path:**
  `practice/<student-id>/unit-11/management-api/`.
- [ ] **Task 4 — Protect response data** (`A5.2`): Define which payload,
  headers, URLs, and error details are returned or redacted. Test an unauthenticated
  request and a user requesting another user's data. **Primary path:**
  `practice/<student-id>/unit-11/design/data-exposure.md`.

## 3 · Abuse boundaries

- [ ] **Task 5 — Apply request limits** (`A6.1`): Add payload-size, request-
  timeout, and rate limits to the appropriate endpoints. Explain the identity
  each limiter keys on and its false-positive trade-off.
  **Primary path:** `practice/<student-id>/unit-11/limits/`.
- [ ] **Task 6 — Threat-model destination URLs** (`E8.1`): Identify SSRF,
  redirect, DNS, private-network, and credential-leak risks. Propose a V1 policy
  the cohort can realistically enforce and test.
  **Primary path:** `practice/<student-id>/unit-11/security/threat-model.md`.
- [ ] **Task 7 — Rotate and redact secrets** (`E8.2`): Define storage,
  display-once behavior, logging redaction, and a simple rotation path for source
  secrets. **Primary path:**
  `practice/<student-id>/unit-11/security/secret-policy.md`.

## Optional Redis lab

- [ ] **Task 8 — Compare rate-limit stores** (`D5.3`, optional): Implement the
  same small limiter in process and with Redis. Compare restart behavior,
  multi-instance consistency, failure modes, and operational cost. Do not move
  durable deduplication into Redis. **Primary path:**
  `practice/<student-id>/unit-11/redis-rate-limit-lab/`.

**If short on time, cut:** Task 8, then account-management extras. Never cut
authentication, the authorization tests, or the destination threat model.

## End-of-unit checklist

- [ ] Unauthenticated management requests fail
- [ ] Authorized operator can inspect and replay through HTTP
- [ ] Cross-owner requests fail if the cohort chose local user accounts
- [ ] Private-network and redirect destination cases follow the agreed policy
- [ ] Shared rotation merges protected routes and a security decision record

Next: `unit-12.md`.
