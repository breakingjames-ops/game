# Astralis Rust Physics Prototype

A minimal re‑implementation of the core gravitational integrator from the Astralis JavaScript game (`index.html` at commit `dfc3fb66cbe427791bd2d19bea2b963e7c7d9ce6`).

## What is reproduced

* Softened pairwise gravity: `F = G·m₁·m₂ / (r² + SOFTENING²)`
* Sub‑stepped Velocity Verlet integrator (`DT = 0.016`, `SUB_STEPS = 4`)
* Constants `G = 1.0`, `SOFTENING = 5.0`

Collision handling, trails, rendering, audio, and UI are **not** included.

## Running

```bash
cargo test      # run the physics tests
cargo run       # run a symmetric two‑body demo
```

## Comparing with the JavaScript game

1. Open the original `index.html` in a browser and load the “Binary Star Dance” preset.
2. Observe the trajectories (or log positions/velocities inside `stepPhysics`).
3. Adjust the initial conditions in `src/main.rs` to match the preset and compare the qualitative behaviour.

Because the integrator and softening are identical, stable symmetric orbits should appear in both versions.

## Limitations

* Collision response (`merge`/`shatter`) is omitted.
* No visual trails, audio, or rendering.
* The README’s claims about “mechanical energy conservation” and “stable planetary orbits” are not independently verified beyond the short‑term tests included here.
