# Cohort Operations Notes

This is the cohort's starting repository. The curriculum, individual practice
path, and compiling shared starter are ready to use from Unit 1. The items
below are coordination notes for the cohort lead, not approval gates or missing
student work.

## Record as the cohort is organized

- Dates, student IDs, practice folders, and rotation assignments.
- Review contacts and the supported development environments.
- The final demonstration environment and repository access policy.
- Actual shared pull requests and outside witnesses in
  `../shared/delivery/rotation-log.md`.

## Build and check at the relevant unit

- Unit 5: students implement the first unsigned provider-to-merchant path;
  the existing starter CI checks formatting, Clippy, and tests.
- Unit 7: students add PostgreSQL migrations and test them against a
  disposable database.
- Unit 12: students demonstrate retries and replay using a fresh disposable
  database so earlier tests do not consume the merchant's failure sequence.
- Unit 16: students add and test the final Compose deployment.
- Unit 17: someone other than the author follows the handover runbook.

The mock provider and merchant use no real payment account or money. Their
behavior, the shared service, and later deployment artifacts are built during
the course. Do not describe those future implementations as already working.
