# TASK: Repository Scaffolding & Orchestration for Astralis

You are acting as the project orchestrator. We are setting up a standalone repository named `game` for **Astralis**—a single-file interactive N-Body Orbital Gravity & Spacetime Sandbox built with HTML5 Canvas and Vanilla JavaScript.

Set up the project files as specified below, verify the architecture boundaries, and prepare the repository for autonomous feature development.

---

### Project File Structure

```text
game/
├── index.html
├── README.md
├── ARCHITECTURE.md
├── .github/
│   └── workflows/
│       └── deploy.yml
└── tasks/
    └── task-01-relativistic-waves.md


And

# Astralis Architecture Specification

## 1. Engine Core
- **Integration**: Sub-stepped Velocity Verlet ($dt = 0.016$, $N_{\text{substeps}} = 4$).
- **Force Formula**: $F = G \frac{m_1 m_2}{r^2 + \epsilon^2}$ with softening parameter $\epsilon = 5.0$.
- **World Coordinates**: Centered at `(0, 0)`. Screen projection transformed via `camera = { x, y, zoom }`.

## 2. Global State Schema
\`\`\`javascript
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
\`\`\`

## 3. Strict Development Rules for AI Agents
- All physics calculations must live inside \`stepPhysics(dt)\`.
- Rendering is strictly decoupled inside \`draw()\`.
- Zero memory allocations inside \`stepPhysics()\` and \`draw()\` loops (reuse typed arrays or scalar scratch variables to maintain 60 FPS).
- The entire game must remain self-contained in a single file (\`index.html\`) without external CDN libraries.


And
# Astralis

An interactive N-Body Orbital Gravity & Spacetime Sandbox running in a single self-contained HTML file.

## Features
- **Velocity Verlet Engine**: Stable planetary orbits with mechanical energy conservation.
- **Warped Spacetime Grid**: Dynamic 2D Einsteinian gravity well mesh.
- **Slingshot Trajectory Prediction**: Real-time forward numerical integration with orbital classification (`BOUND`, `ESCAPE`, `IMPACT`).
- **Collision Dynamics**: Momentum-conserving mergers or explosive debris fragmentation.
- **Presets**: Binary Star Dance, Solar System, Figure-8 Choreography, Black Hole Accretion, and Chaos Cluster.

## Running Locally
Open `index.html` in any modern web browser. No installation or external dependencies required.


name: Deploy Astralis to GitHub Pages

on:
  push:
    branches: [ main ]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
      - name: Setup Pages
        uses: actions/configure-pages@v4
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: '.'
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4


# Task 01: Relativistic Precession & Gravitational Wave Radiation

## Mathematical Specifications
1. **Post-Newtonian Gravitational Correction:**
   Modify Newtonian gravity $a_N = G \frac{M}{r^2}$ with the leading-order General Relativistic correction:
   $$\vec{a}_{12} = - \frac{G M_2}{r^2} \left( 1 + \frac{3 G (M_1 + M_2)}{c^2 r} \right) \hat{r}$$
   Set tunable constant `c_light = 1500` in simulation units to produce visible perihelion advance on close orbits.

2. **Gravitational Radiation & Orbital Decay:**
   When massive bodies (Stars, Black Holes) orbit in close proximity:
   $$\frac{dE}{dt} = -\frac{32}{5} \frac{G^4}{c^5} \frac{M_1^2 M_2^2 (M_1 + M_2)}{r^5}$$
   Apply an opposing tangential damping acceleration to simulate orbital inspiral and accelerating periods.

3. **Spacetime Ripples:**
   When body centripetal acceleration exceeds $a_{\text{threshold}}$, emit concentric wave rings through the grid mesh propagating outward at speed $c_{\text{light}}$.

4. **New Preset:**
   Add `"LIGO Binary Black Hole Merger"`: Two black holes in close orbit that naturally spiral inward, radiate visible waves, and coalesce into a single black hole with an audio chirp.


<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Astralis | N-Body Orbital Gravity & Spacetime Sandbox</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; user-select: none; }
    body, html { width: 100%; height: 100%; overflow: hidden; background: #050608; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, monospace; color: #d1d5db; }
    #canvas { display: block; width: 100vw; height: 100vh; cursor: crosshair; }
    
    .panel {
      position: absolute; background: rgba(13, 17, 23, 0.85); backdrop-filter: blur(10px);
      border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 8px; padding: 12px 16px;
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5); z-index: 10;
    }
    #top-bar { top: 16px; left: 16px; display: flex; align-items: center; gap: 12px; }
    #stats-bar { top: 16px; right: 16px; font-size: 12px; font-family: monospace; display: flex; gap: 14px; }
    #controls { bottom: 16px; left: 16px; display: flex; flex-direction: column; gap: 10px; width: 280px; }
    #archetypes { bottom: 16px; right: 16px; display: flex; gap: 8px; }

    h1 { font-size: 15px; font-weight: 700; color: #f9fafb; letter-spacing: 0.05em; text-transform: uppercase; margin: 0; }
    .badge { font-size: 10px; background: #2563eb; color: #fff; padding: 2px 6px; border-radius: 4px; text-transform: uppercase; }

    button, select {
      background: rgba(255, 255, 255, 0.08); border: 1px solid rgba(255, 255, 255, 0.15);
      color: #e5e7eb; padding: 6px 10px; border-radius: 6px; font-size: 12px; cursor: pointer;
      transition: all 0.2s ease; outline: none;
    }
    button:hover, select:hover { background: rgba(255, 255, 255, 0.18); border-color: rgba(255, 255, 255, 0.3); }
    button.active { background: #3b82f6; border-color: #60a5fa; color: #fff; font-weight: 600; }
    
    .row { display: flex; align-items: center; justify-content: space-between; gap: 8px; font-size: 12px; }
    .btn-group { display: flex; gap: 6px; width: 100%; }
    .btn-group button { flex: 1; text-align: center; }

    .archetype-btn {
      display: flex; flex-direction: column; align-items: center; gap: 4px; padding: 8px 10px; min-width: 65px;
    }
    .archetype-btn .dot { width: 12px; height: 12px; border-radius: 50%; display: inline-block; }

    #hud {
      position: absolute; pointer-events: none; top: 0; left: 0; width: 100%; height: 100%; z-index: 5;
    }
  </style>
</head>
<body>
  <div id="top-bar" class="panel">
    <h1>Astralis</h1>
    <span class="badge">N-Body Sandbox</span>
    <select id="preset-select">
      <option value="binary">Preset: Binary Star Dance</option>
      <option value="solar">Preset: Inner Solar System</option>
      <option value="figure8">Preset: Figure-8 Three-Body</option>
      <option value="accretion">Preset: Black Hole Accretion</option>
      <option value="chaos">Preset: Chaos Cluster</option>
    </select>
  </div>

  <div id="stats-bar" class="panel">
    <div>BODIES: <span id="stat-bodies">0</span></div>
    <div>FPS: <span id="stat-fps">60</span></div>
    <div>TIME: <span id="stat-time">1.0x</span></div>
  </div>

  <div id="controls" class="panel">
    <div class="row">
      <span>Simulation</span>
      <div class="btn-group">
        <button id="btn-pause">Pause</button>
        <button id="btn-clear">Clear</button>
      </div>
    </div>
    <div class="row">
      <span>Collisions</span>
      <div class="btn-group">
        <button id="btn-merge" class="active">Merge</button>
        <button id="btn-shatter">Shatter</button>
      </div>
    </div>
    <div class="row">
      <span>Visuals</span>
      <div class="btn-group">
        <button id="btn-grid" class="active">Grid</button>
        <button id="btn-trails" class="active">Trails</button>
        <button id="btn-audio">Audio</button>
      </div>
    </div>
  </div>

  <div id="archetypes" class="panel">
    <button class="archetype-btn active" data-type="asteroid">
      <span class="dot" style="background:#9ca3af;"></span>
      <span>Asteroid</span>
    </button>
    <button class="archetype-btn" data-type="rocky">
      <span class="dot" style="background:#38bdf8;"></span>
      <span>Rocky</span>
    </button>
    <button class="archetype-btn" data-type="gas">
      <span class="dot" style="background:#fbbf24;"></span>
      <span>Gas Giant</span>
    </button>
    <button class="archetype-btn" data-type="star">
      <span class="dot" style="background:#f97316; box-shadow: 0 0 8px #f97316;"></span>
      <span>Star</span>
    </button>
    <button class="archetype-btn" data-type="black_hole">
      <span class="dot" style="background:#000; border: 1px solid #a855f7; box-shadow: 0 0 8px #a855f7;"></span>
      <span>Singularity</span>
    </button>
  </div>

  <canvas id="canvas"></canvas>
  <canvas id="hud"></canvas>

  <script>
    /* =========================================================================
       MODULE 1: CONSTANTS & CONFIGURATION
       ========================================================================= */
    const G = 1.0;
    const SOFTENING = 5.0;
    const SUB_STEPS = 4;
    const DT = 0.016;

    const ARCHETYPES = {
      asteroid:   { mass: 0.5,   radius: 3,  color: "#9ca3af", glow: null },
      rocky:      { mass: 10,    radius: 5,  color: "#38bdf8", glow: "rgba(56,189,248,0.4)" },
      gas:        { mass: 150,   radius: 9,  color: "#fbbf24", glow: "rgba(251,191,36,0.5)" },
      star:       { mass: 3000,  radius: 16, color: "#f97316", glow: "rgba(249,115,22,0.8)" },
      black_hole: { mass: 12000, radius: 10, color: "#000000", glow: "rgba(168,85,247,0.9)", ring: true }
    };

    let state = {
      paused: false,
      collisionMode: "merge",
      showGrid: true,
      showTrails: true,
      audioEnabled: false,
      selectedArchetype: "asteroid",
      camera: { x: 0, y: 0, zoom: 1.0 },
      isDragging: false,
      dragStart: { x: 0, y: 0 },
      dragCurrent: { x: 0, y: 0 },
      isPanning: false,
      panStart: { x: 0, y: 0 }
    };

    let bodies = [];
    let particles = [];
    let audioCtx = null;

    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");
    const hudCanvas = document.getElementById("hud");
    const hudCtx = hudCanvas.getContext("2d");

    function resize() {
      canvas.width = hudCanvas.width = window.innerWidth;
      canvas.height = hudCanvas.height = window.innerHeight;
      state.camera.x = canvas.width / 2;
      state.camera.y = canvas.height / 2;
    }
    window.addEventListener("resize", resize);
    resize();

    /* =========================================================================
       MODULE 2: AUDIO SYNTHESIZER
       ========================================================================= */
    function playTone(freq, duration = 0.15, type = "sine") {
      if (!state.audioEnabled) return;
      if (!audioCtx) audioCtx = new (window.AudioContext || window.webkitAudioContext)();
      if (audioCtx.state === "suspended") audioCtx.resume();

      const osc = audioCtx.createOscillator();
      const gain = audioCtx.createGain();
      osc.type = type;
      osc.frequency.setValueAtTime(freq, audioCtx.currentTime);
      gain.gain.setValueAtTime(0.08, audioCtx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.001, audioCtx.currentTime + duration);
      osc.connect(gain);
      gain.connect(audioCtx.destination);
      osc.start();
      osc.stop(audioCtx.currentTime + duration);
    }

    /* =========================================================================
       MODULE 3: PHYSICS ENGINE (VELOCITY VERLET)
       ========================================================================= */
    function computeAccelerations(bodyList) {
      const n = bodyList.length;
      const ax = new Float64Array(n);
      const ay = new Float64Array(n);

      for (let i = 0; i < n; i++) {
        const bi = bodyList[i];
        for (let j = i + 1; j < n; j++) {
          const bj = bodyList[j];
          const dx = bj.x - bi.x;
          const dy = bj.y - bi.y;
          const distSq = dx * dx + dy * dy + SOFTENING * SOFTENING;
          const dist = Math.sqrt(distSq);
          const force = (G * bi.mass * bj.mass) / distSq;
          
          const fx = (force * dx) / dist;
          const fy = (force * dy) / dist;

          ax[i] += fx / bi.mass;
          ay[i] += fy / bi.mass;
          ax[j] -= fx / bj.mass;
          ay[j] -= fy / bj.mass;
        }
      }
      return { ax, ay };
    }

    function stepPhysics(subDt) {
      if (bodies.length === 0) return;

      const { ax: ax1, ay: ay1 } = computeAccelerations(bodies);

      for (let i = 0; i < bodies.length; i++) {
        const b = bodies[i];
        b.vx += 0.5 * ax1[i] * subDt;
        b.vy += 0.5 * ay1[i] * subDt;
        b.x += b.vx * subDt;
        b.y += b.vy * subDt;

        if (state.showTrails && Math.random() < 0.2) {
          b.trail.push({ x: b.x, y: b.y });
          if (b.trail.length > 40) b.trail.shift();
        }
      }

      const { ax: ax2, ay: ay2 } = computeAccelerations(bodies);

      for (let i = 0; i < bodies.length; i++) {
        const b = bodies[i];
        b.vx += 0.5 * ax2[i] * subDt;
        b.vy += 0.5 * ay2[i] * subDt;
      }

      resolveCollisions();
    }

    function resolveCollisions() {
      for (let i = 0; i < bodies.length; i++) {
        for (let j = i + 1; j < bodies.length; j++) {
          const b1 = bodies[i];
          const b2 = bodies[j];
          const dx = b2.x - b1.x;
          const dy = b2.y - b1.y;
          const dist = Math.hypot(dx, dy);

          if (dist < (b1.radius + b2.radius) * 0.8) {
            handleCollision(i, j);
            return;
          }
        }
      }
    }

    function handleCollision(i, j) {
      const b1 = bodies[i];
      const b2 = bodies[j];
      const primary = b1.mass >= b2.mass ? b1 : b2;
      const secondary = b1.mass >= b2.mass ? b2 : b1;

      if (state.collisionMode === "merge") {
        const totalMass = primary.mass + secondary.mass;
        primary.vx = (primary.vx * primary.mass + secondary.vx * secondary.mass) / totalMass;
        primary.vy = (primary.vy * primary.mass + secondary.vy * secondary.mass) / totalMass;
        primary.mass = totalMass;
        primary.radius = Math.max(primary.radius, Math.cbrt(Math.pow(primary.radius, 3) + Math.pow(secondary.radius, 3)));
        
        spawnExplosion(secondary.x, secondary.y, secondary.color, 12);
        playTone(180, 0.2, "triangle");
        bodies = bodies.filter(b => b !== secondary);
      } else {
        spawnExplosion((b1.x + b2.x) / 2, (b1.y + b2.y) / 2, "#f87171", 30);
        playTone(320, 0.3, "sawtooth");
        bodies = bodies.filter((_, idx) => idx !== i && idx !== j);
      }
    }

    function spawnExplosion(x, y, color, count) {
      for (let k = 0; k < count; k++) {
        const angle = Math.random() * Math.PI * 2;
        const speed = Math.random() * 5 + 1;
        particles.push({
          x, y,
          vx: Math.cos(angle) * speed,
          vy: Math.sin(angle) * speed,
          color,
          life: 1.0,
          decay: Math.random() * 0.03 + 0.015
        });
      }
    }

    /* =========================================================================
       MODULE 4: PRESETS
       ========================================================================= */
    function loadPreset(name) {
      bodies = [];
      particles = [];

      if (name === "binary") {
        bodies.push(
          { id: 1, x: -90, y: 0, vx: 0, vy: -3.8, mass: 2200, radius: 14, color: "#f97316", type: "star", trail: [] },
          { id: 2, x: 90, y: 0, vx: 0, vy: 3.8, mass: 2200, radius: 14, color: "#38bdf8", type: "star", trail: [] },
          { id: 3, x: 260, y: 0, vx: 0, vy: 4.8, mass: 12, radius: 6, color: "#34d399", type: "rocky", trail: [] }
        );
      } else if (name === "solar") {
        bodies.push(
          { id: 1, x: 0, y: 0, vx: 0, vy: 0, mass: 4500, radius: 18, color: "#fbbf24", type: "star", trail: [] },
          { id: 2, x: 80, y: 0, vx: 0, vy: 7.5, mass: 2, radius: 4, color: "#9ca3af", type: "rocky", trail: [] },
          { id: 3, x: 150, y: 0, vx: 0, vy: 5.5, mass: 8, radius: 6, color: "#38bdf8", type: "rocky", trail: [] },
          { id: 4, x: 230, y: 0, vx: 0, vy: 4.4, mass: 6, radius: 5, color: "#f87171", type: "rocky", trail: [] },
          { id: 5, x: 340, y: 0, vx: 0, vy: 3.6, mass: 80, radius: 10, color: "#fcd34d", type: "gas", trail: [] }
        );
      } else if (name === "figure8") {
        const m = 1500;
        const x1 = -180, y1 = 0, vx1 = 1.95, vy1 = 1.82;
        bodies.push(
          { id: 1, x: x1, y: y1, vx: vx1, vy: vy1, mass: m, radius: 12, color: "#ec4899", type: "star", trail: [] },
          { id: 2, x: -x1, y: -y1, vx: vx1, vy: vy1, mass: m, radius: 12, color: "#3b82f6", type: "star", trail: [] },
          { id: 3, x: 0, y: 0, vx: -2 * vx1, vy: -2 * vy1, mass: m, radius: 12, color: "#10b981", type: "star", trail: [] }
        );
      } else if (name === "accretion") {
        bodies.push({ id: 1, x: 0, y: 0, vx: 0, vy: 0, mass: 12000, radius: 12, color: "#000000", type: "black_hole", trail: [] });
        for (let i = 0; i < 90; i++) {
          const r = Math.random() * 240 + 70;
          const theta = Math.random() * Math.PI * 2;
          const v = Math.sqrt((G * 12000) / r);
          bodies.push({
            id: 10 + i,
            x: Math.cos(theta) * r,
            y: Math.sin(theta) * r,
            vx: -Math.sin(theta) * v,
            vy: Math.cos(theta) * v,
            mass: 0.2,
            radius: 2,
            color: "#e2e8f0",
            type: "asteroid",
            trail: []
          });
        }
      } else if (name === "chaos") {
        for (let i = 0; i < 18; i++) {
          const angle = Math.random() * Math.PI * 2;
          const dist = Math.random() * 250;
          bodies.push({
            id: i,
            x: Math.cos(angle) * dist,
            y: Math.sin(angle) * dist,
            vx: (Math.random() - 0.5) * 4,
            vy: (Math.random() - 0.5) * 4,
            mass: Math.random() * 400 + 40,
            radius: Math.random() * 5 + 4,
            color: `hsl(${Math.random() * 360}, 75%, 60%)`,
            type: "rocky",
            trail: []
          });
        }
      }
    }

    /* =========================================================================
       MODULE 5: TRAJECTORY PREDICTION (SLINGSHOT)
       ========================================================================= */
    function predictTrajectory(startX, startY, vx, vy, mass, radius) {
      const simSteps = 220;
      const stepDt = 0.12;
      const path = [];
      let x = startX, y = startY, curVx = vx, curVy = vy;
      let status = "BOUND ORBIT";

      for (let s = 0; s < simSteps; s++) {
        let ax = 0, ay = 0;
        for (let b of bodies) {
          const dx = b.x - x;
          const dy = b.y - y;
          const dSq = dx * dx + dy * dy + SOFTENING * SOFTENING;
          const d = Math.sqrt(dSq);
          if (d < b.radius + radius) {
            status = "IMPACT";
            path.push({ x, y });
            return { path, status };
          }
          const f = (G * b.mass) / dSq;
          ax += (f * dx) / d;
          ay += (f * dy) / d;
        }

        curVx += ax * stepDt;
        curVy += ay * stepDt;
        x += curVx * stepDt;
        y += curVy * stepDt;
        path.push({ x, y });

        if (Math.hypot(x, y) > 2800) {
          status = "ESCAPE";
          break;
        }
      }
      return { path, status };
    }

    /* =========================================================================
       MODULE 6: RENDERING & VISUAL PIPELINE
       ========================================================================= */
    function draw() {
      ctx.fillStyle = "#050608";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      ctx.save();
      ctx.translate(state.camera.x, state.camera.y);
      ctx.scale(state.camera.zoom, state.camera.zoom);

      if (state.showGrid) {
        ctx.strokeStyle = "rgba(75, 85, 99, 0.22)";
        ctx.lineWidth = 1 / state.camera.zoom;
        const gridSize = 45;
        const range = 1400;

        for (let x = -range; x <= range; x += gridSize) {
          ctx.beginPath();
          for (let y = -range; y <= range; y += 20) {
            let offset = computeGridWarp(x, y);
            if (y === -range) ctx.moveTo(x + offset.dx, y + offset.dy);
            else ctx.lineTo(x + offset.dx, y + offset.dy);
          }
          ctx.stroke();
        }

        for (let y = -range; y <= range; y += gridSize) {
          ctx.beginPath();
          for (let x = -range; x <= range; x += 20) {
            let offset = computeGridWarp(x, y);
            if (x === -range) ctx.moveTo(x + offset.dx, y + offset.dy);
            else ctx.lineTo(x + offset.dx, y + offset.dy);
          }
          ctx.stroke();
        }
      }

      if (state.showTrails) {
        for (let b of bodies) {
          if (b.trail.length < 2) continue;
          ctx.beginPath();
          ctx.strokeStyle = b.color;
          ctx.lineWidth = 1.2 / state.camera.zoom;
          ctx.globalAlpha = 0.35;
          for (let i = 0; i < b.trail.length; i++) {
            const p = b.trail[i];
            if (i === 0) ctx.moveTo(p.x, p.y);
            else ctx.lineTo(p.x, p.y);
          }
          ctx.stroke();
          ctx.globalAlpha = 1.0;
        }
      }

      for (let b of bodies) {
        if (b.type === "black_hole") {
          const grad = ctx.createRadialGradient(b.x, b.y, b.radius * 0.8, b.x, b.y, b.radius * 2.8);
          grad.addColorStop(0, "rgba(0,0,0,1)");
          grad.addColorStop(0.4, "rgba(168,85,247,0.8)");
          grad.addColorStop(1, "rgba(168,85,247,0)");
          ctx.fillStyle = grad;
          ctx.beginPath();
          ctx.arc(b.x, b.y, b.radius * 2.8, 0, Math.PI * 2);
          ctx.fill();

          ctx.fillStyle = "#000000";
          ctx.beginPath();
          ctx.arc(b.x, b.y, b.radius, 0, Math.PI * 2);
          ctx.fill();
        } else {
          if (ARCHETYPES[b.type]?.glow) {
            ctx.shadowColor = ARCHETYPES[b.type].color;
            ctx.shadowBlur = b.radius * 2.2;
          }
          ctx.fillStyle = b.color;
          ctx.beginPath();
          ctx.arc(b.x, b.y, b.radius, 0, Math.PI * 2);
          ctx.fill();
          ctx.shadowBlur = 0;
        }
      }

      for (let i = particles.length - 1; i >= 0; i--) {
        const p = particles[i];
        p.x += p.vx;
        p.y += p.vy;
        p.life -= p.decay;
        if (p.life <= 0) {
          particles.splice(i, 1);
          continue;
        }
        ctx.fillStyle = p.color;
        ctx.globalAlpha = p.life;
        ctx.beginPath();
        ctx.arc(p.x, p.y, 2, 0, Math.PI * 2);
        ctx.fill();
      }
      ctx.globalAlpha = 1.0;
      ctx.restore();

      renderSlingshotHUD();
    }

    function computeGridWarp(x, y) {
      let dx = 0, dy = 0;
      for (let b of bodies) {
        const distSq = (b.x - x) ** 2 + (b.y - y) ** 2 + 100;
        const pull = Math.min(b.mass * 0.08 / distSq, 14);
        const dist = Math.sqrt(distSq);
        dx += ((b.x - x) / dist) * pull;
        dy += ((b.y - y) / dist) * pull;
      }
      return { dx, dy };
    }

    function renderSlingshotHUD() {
      hudCtx.clearRect(0, 0, hudCanvas.width, hudCanvas.height);
      if (!state.isDragging) return;

      const sx = state.dragStart.x;
      const sy = state.dragStart.y;
      const cx = state.dragCurrent.x;
      const cy = state.dragCurrent.y;

      const vx = (sx - cx) * 0.06;
      const vy = (sy - cy) * 0.06;

      const worldStart = screenToWorld(sx, sy);
      const spec = ARCHETYPES[state.selectedArchetype];
      const { path, status } = predictTrajectory(worldStart.x, worldStart.y, vx, vy, spec.mass, spec.radius);

      hudCtx.beginPath();
      hudCtx.setLineDash([4, 4]);
      hudCtx.strokeStyle = status === "IMPACT" ? "#f87171" : status === "ESCAPE" ? "#fbbf24" : "#38bdf8";
      hudCtx.lineWidth = 1.5;

      for (let i = 0; i < path.length; i++) {
        const scr = worldToScreen(path[i].x, path[i].y);
        if (i === 0) hudCtx.moveTo(scr.x, scr.y);
        else hudCtx.lineTo(scr.x, scr.y);
      }
      hudCtx.stroke();
      hudCtx.setLineDash([]);

      hudCtx.beginPath();
      hudCtx.moveTo(sx, sy);
      hudCtx.lineTo(cx, cy);
      hudCtx.strokeStyle = "#e5e7eb";
      hudCtx.lineWidth = 2;
      hudCtx.stroke();

      hudCtx.fillStyle = status === "IMPACT" ? "#f87171" : status === "ESCAPE" ? "#fbbf24" : "#38bdf8";
      hudCtx.font = "11px monospace";
      hudCtx.fillText(`${status} | SPEED: ${Math.hypot(vx, vy).toFixed(1)}`, cx + 12, cy - 8);
    }

    function screenToWorld(sx, sy) {
      return {
        x: (sx - state.camera.x) / state.camera.zoom,
        y: (sy - state.camera.y) / state.camera.zoom
      };
    }

    function worldToScreen(wx, wy) {
      return {
        x: wx * state.camera.zoom + state.camera.x,
        y: wy * state.camera.zoom + state.camera.y
      };
    }

    /* =========================================================================
       MODULE 7: INTERACTION & INPUT CONTROLLER
       ========================================================================= */
    window.addEventListener("mousedown", (e) => {
      if (e.target.closest(".panel")) return;
      if (e.button === 0) {
        state.isDragging = true;
        state.dragStart = { x: e.clientX, y: e.clientY };
        state.dragCurrent = { x: e.clientX, y: e.clientY };
      } else if (e.button === 2) {
        state.isPanning = true;
        state.panStart = { x: e.clientX - state.camera.x, y: e.clientY - state.camera.y };
      }
    });

    window.addEventListener("mousemove", (e) => {
      if (state.isDragging) {
        state.dragCurrent = { x: e.clientX, y: e.clientY };
      }
      if (state.isPanning) {
        state.camera.x = e.clientX - state.panStart.x;
        state.camera.y = e.clientY - state.panStart.y;
      }
    });

    window.addEventListener("mouseup", (e) => {
      if (state.isDragging && e.button === 0) {
        state.isDragging = false;
        const wPos = screenToWorld(state.dragStart.x, state.dragStart.y);
        const vx = (state.dragStart.x - e.clientX) * 0.06;
        const vy = (state.dragStart.y - e.clientY) * 0.06;
        const spec = ARCHETYPES[state.selectedArchetype];

        bodies.push({
          id: Date.now(),
          x: wPos.x,
          y: wPos.y,
          vx, vy,
          mass: spec.mass,
          radius: spec.radius,
          color: spec.color,
          type: state.selectedArchetype,
          trail: []
        });

        playTone(480, 0.1, "sine");
      }
      if (e.button === 2) state.isPanning = false;
    });

    window.addEventListener("contextmenu", (e) => e.preventDefault());

    window.addEventListener("wheel", (e) => {
      const zoomFactor = e.deltaY < 0 ? 1.1 : 0.9;
      state.camera.zoom = Math.max(0.2, Math.min(state.camera.zoom * zoomFactor, 4.0));
    });

    document.getElementById("btn-pause").onclick = (e) => {
      state.paused = !state.paused;
      e.target.innerText = state.paused ? "Play" : "Pause";
      e.target.classList.toggle("active", state.paused);
    };

    document.getElementById("btn-clear").onclick = () => { bodies = []; particles = []; };

    document.getElementById("btn-merge").onclick = (e) => {
      state.collisionMode = "merge";
      e.target.classList.add("active");
      document.getElementById("btn-shatter").classList.remove("active");
    };

    document.getElementById("btn-shatter").onclick = (e) => {
      state.collisionMode = "shatter";
      e.target.classList.add("active");
      document.getElementById("btn-merge").classList.remove("active");
    };

    document.getElementById("btn-grid").onclick = (e) => {
      state.showGrid = !state.showGrid;
      e.target.classList.toggle("active", state.showGrid);
    };

    document.getElementById("btn-trails").onclick = (e) => {
      state.showTrails = !state.showTrails;
      e.target.classList.toggle("active", state.showTrails);
    };

    document.getElementById("btn-audio").onclick = (e) => {
      state.audioEnabled = !state.audioEnabled;
      e.target.classList.toggle("active", state.audioEnabled);
      if (state.audioEnabled && !audioCtx) audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    };

    document.querySelectorAll(".archetype-btn").forEach((btn) => {
      btn.onclick = () => {
        document.querySelectorAll(".archetype-btn").forEach((b) => b.classList.remove("active"));
        btn.classList.add("active");
        state.selectedArchetype = btn.dataset.type;
      };
    });

    document.getElementById("preset-select").onchange = (e) => loadPreset(e.target.value);

    /* =========================================================================
       MAIN LOOP
       ========================================================================= */
    let lastTime = performance.now();
    let frameCount = 0;
    let fpsTime = lastTime;

    function loop(now) {
      if (!state.paused) {
        const subDt = DT / SUB_STEPS;
        for (let i = 0; i < SUB_STEPS; i++) stepPhysics(subDt);
      }

      draw();

      frameCount++;
      if (now - fpsTime > 500) {
        document.getElementById("stat-fps").innerText = Math.round((frameCount * 1000) / (now - fpsTime));
        document.getElementById("stat-bodies").innerText = bodies.length;
        frameCount = 0;
        fpsTime = now;
      }

      requestAnimationFrame(loop);
    }

    loadPreset("binary");
    requestAnimationFrame(loop);
  </script>
</body>
</html>



