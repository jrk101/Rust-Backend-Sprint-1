# Unit 12: Dead letters and operator replay

## Before you start

Bring persisted attempts and restart-safe due work from Unit 11. Configure the
sample merchant to fail enough times to exhaust the agreed retry policy, then
to succeed after replay. Use a fresh disposable database so prior test work
cannot consume the merchant's programmed responses. Keep the operator command local to the trusted
development machine.

## At a glance

| Learn | Use immediately |
|---|---|
| Terminal delivery state | `D4.1` |
| Idempotent operator intent | `D4.2` |
| Traceable inspection and replay | `E7.2` |
| Failure-sequence evidence | `E7.1` milestone |

## By the end of this unit you can

- move exhausted work into an explicit dead-letter state;
- replay without erasing history or creating uncontrolled duplicates;
- inspect event and attempt history from a local operator command;
- demonstrate the complete reliability loop.

## 1 · Complete the reliability state machine

- [ ] **Task 1: Enter dead-letter state** (`D4.1`): Apply the attempt limit
  atomically, store the terminal reason, and ensure ordinary workers no longer
  select the delivery. **Primary path:**
  `practice/<student-id>/unit-12/dead-letter/`.
- [ ] **Task 2: Design safe replay** (`D4.2`): Decide whether replay creates a
  new delivery or changes an existing one, record operator intent, and make
  repeated replay requests idempotent. **Primary path:**
  `practice/<student-id>/unit-12/design/replay-decision.md` and implementation.
- [ ] **Task 3: Inspect from a local operator command** (`E7.2`): List one
  event's deliveries and attempts and replay a dead letter from a command run
  on the same trusted machine. Document the command and result in
  `practice/<student-id>/unit-12/operator-cli.md`. No remote operator API is
  required for this milestone.
- [ ] **Task 4: Demonstrate the reliability loop** (`E7.1`): Run
  `500 → timeout → 500 → dead letter → replay → 200`, preserving every attempt.
  **Primary path:** `practice/<student-id>/unit-12/reliability-demo.md`.

## Reliable delivery core [MILESTONE]

The gate passes when the flow works after process restarts and the recorded
history explains every transition.

## End-of-unit checklist

- [ ] Automatic worker stops selecting dead letters
- [ ] Replay creates one deliberate new attempt path and retains old history
- [ ] A repeated replay command cannot create uncontrolled duplicate work
- [ ] Outside witness runs the failure sequence after a restart

**If short on time, cut:** operator-command presentation polish. Never cut
attempt history, dead-letter state, replay safety, or the demonstration.

Next: `unit-13.md`.
