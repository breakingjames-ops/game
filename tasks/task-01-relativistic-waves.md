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



