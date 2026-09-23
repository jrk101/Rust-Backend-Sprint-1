# Unit 13: Protected management

## Before you start

Bring the Unit 12 local operator flow. Read the password storage guidance in
`docs/09-resources.md`. Decide whether the shared V1 uses one
configured operator or local user accounts; document the decision and
authorization ownership before exposing management routes.

## At a glance

| Learn | Use immediately |
|---|---|
| Authentication and authorization | `D5.1–D5.2`, `A5.1` |
| Safe response shaping | `A5.2` |

## By the end of this unit you can

- hash passwords and authenticate management users safely;
- distinguish management authentication from webhook signatures;
- reject unauthenticated access to inspection and replay endpoints.

## 1 · Management authentication

- [ ] **Task 1: Define the auth model** (`D5.1`): Specify registration,
  login, password reset non-goals, authorization ownership, token expiry, and
  revocation limitations. **Primary path:**
  `practice/<student-id>/unit-13/design/auth-model.md`.
- [ ] **Task 2: Implement password and token handling** (`D5.2`): Use Argon2
  for password hashing if V1 has local user accounts. If V1 uses one configured
  operator, implement a bounded credential check and complete the Argon2
  practice lab separately. Define token/session expiry and revocation limits.
  Do not use management JWTs to authenticate incoming provider webhooks.
  **Primary path:** `practice/<student-id>/unit-13/auth/`.

## 2 · Protected management

- [ ] **Task 3: Build inspection endpoints** (`A5.1`): After authentication
  is enforced, list and fetch events, deliveries, and attempts with stable
  pagination, filtering, and bounded page sizes. **Primary path:**
  `practice/<student-id>/unit-13/management-api/`.
- [ ] **Task 4: Protect response data** (`A5.2`): Define which payload,
  headers, URLs, and error details are returned or redacted. Test an unauthenticated
  request and a user requesting another user's data. **Primary path:**
  `practice/<student-id>/unit-13/design/data-exposure.md`.

**If short on time, cut:** account-management extras. Never cut authentication
or the authorization tests.

## End-of-unit checklist

- [ ] Unauthenticated management requests fail
- [ ] Authorized operator can inspect and replay through HTTP
- [ ] Cross-owner requests fail if the cohort chose local user accounts
- [ ] Shared rotation merges protected routes and an auth decision record

Next: `unit-14.md`.
