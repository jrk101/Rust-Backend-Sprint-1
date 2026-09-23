# Shared Build Zone

This folder will hold the cohort's single reviewed implementation.

- `payhook/`: main modular-monolith service
- `mock-provider/`: signed test-event generator
- `sample-merchant/`: controllably successful/failing destination
- `delivery/`: shared brief, diagrams, demonstrations, and handover material

The three application crates compile but contain only placeholder entry
points. Students add the Actix Web, Tokio, and SQLx behavior as the units
introduce it. The placeholders are not evidence that a milestone is complete.
