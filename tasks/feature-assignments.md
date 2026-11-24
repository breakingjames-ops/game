# Independent feature experiments

Use separate branches from the same baseline. Do not merge or deploy. Each assignment is deliberately small; resolve unrelated defects only if strictly necessary and explain them.

## Kimi — Explore the controls
Add a compact, accessible in-app help panel explaining existing controls and gestures. Include a close button and Escape key support. Verify mouse and keyboard use and narrow-screen readability. Do not change physics.

## DeepSeek — Physics diagnostics
Add an optional readout of total linear momentum in x and y. Document units and any assumptions. Calculate outside the inner force loop, do not change motion or collisions, and check a symmetric two-body case and behavior through pause/resume.

## Qwen — Simulation convenience
Add a single-step button that advances exactly one normal simulation frame while paused and never advances while running. Verify repeated stepping, pause/resume, and all current presets. Do not change the integration algorithm.

## Common constraints
Keep the application in index.html with no external dependencies. Inspect ARCHITECTURE.md and existing code: the supplied baseline already allocates inside hot loops, so do not claim it meets the allocation-free target. Do not increase hot-loop allocations. Give exact commit and validation evidence, including what could not be tested. Open a draft pull request if able; otherwise supply a patch. Do not push to main, merge, or deploy. Do not modify the review rules or tests merely to make checks pass.
