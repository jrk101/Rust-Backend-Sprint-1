# Team Roles

The exact number of students and reviewers is pending. These roles scale without
assuming a particular cohort size.

Before each unit opens, write the assigned names in
`shared/delivery/rotation-log.md`. No student should discover during a
milestone that a reviewer or witness was never assigned.

## Program maintainers

Maintain the curriculum, protect the main branch, review shared-zone changes,
hold private answer guidance, and own injected-failure administration.

## Unit review lead

One or more students perform the first review pass on individual-practice pull
requests. The role rotates. Review leads receive criteria, not finished code.

## Shared-build rotation

A small rotating group applies the unit's proven learning to `shared/`. Every
student should serve at least once. The rotation never makes someone the only
person allowed to learn a subsystem.

## Reliability witness

For milestone demonstrations, a student outside the implementation group runs
the scenario and verifies the evidence. The builder does not grade their own demo.

## Security and documentation reviewer

Beginning with signatures and persistence, each shared pull request receives a
review specifically for secret exposure, unsafe logging, failure behavior, and
documentation drift. This may be a rotating responsibility rather than a fixed specialist.

## Merge policy

- Individual practice follows the cohort's chosen fork/branch model.
- Shared changes require passing checks and a maintainer approval.
- Do not squash student commits when preserving individual authorship is a program goal.
- Resolve design conflicts before merging incompatible implementations.
