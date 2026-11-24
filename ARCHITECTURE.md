# Astralis Architecture Specification

## 1. Engine Core
- **Integration**: Sub-stepped Velocity Verlet ($dt = 0.016$, $N_{\text{substeps}} = 4$).
- **Force Formula**: $F = G \frac{m_1 m_2}{r^2 + \epsilon^2}$ with softening parameter $\epsilon = 5.0$.
- **World Coordinates**: Centered at `(0, 0)`. Screen projection transformed via `camera = { x, y, zoom }`.

## 2. Global State Schema
```javascript
let bodies = [
  {
    id: Number,
    x: Number,
    y: Number,
    vx: Number,
    vy: Number,
    mass: Number,
    radius: Number,
    color: String,
    type: 'asteroid' | 'rocky' | 'gas_giant' | 'star' | 'black_hole',
    trail: Array<{x, y}>
  }
];
```

## 3. Strict Development Rules for AI Agents
- All physics calculations must live inside `stepPhysics(dt)`.
- Rendering is strictly decoupled inside `draw()`.
- Zero memory allocations inside `stepPhysics()` and `draw()` loops (reuse typed arrays or scalar scratch variables to maintain 60 FPS).
- The entire game must remain self-contained in a single file (`index.html`) without external CDN libraries.



