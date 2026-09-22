# Assessment and Review

## Evidence categories

Each task is evaluated on the smallest relevant set:

1. **Behavior** — Does the required scenario work, including failure behavior?
2. **Reasoning** — Can the student explain the choice and rejected alternative?
3. **Safety** — Are secrets, untrusted input, concurrency, and data handled deliberately?
4. **Testing** — Does evidence target the important risk rather than only the happy path?
5. **Reviewability** — Is the change focused, named, documented, and understandable?

## Completion levels

- **Core:** milestone and never-cut work; required for sprint completion.
- **Standard:** the normal unit task set.
- **Extension:** optional depth such as Redis, additional provider adapters, or
  performance experiments.

Extension work never compensates for missing core reliability behavior.

## Review prompts

- What invariant does this code or constraint protect?
- What happens after a crash at each state-changing line?
- What input is controlled by an attacker or external system?
- What is persisted, and what exists only in process memory?
- Which failure is retryable, and why?
- How does the test fail when the implementation is wrong?
- What simpler design was considered?

## Milestone rule

Milestones are demonstrated to someone outside the implementation group. The
witness records the commands, requests, database evidence, and unresolved gaps.

