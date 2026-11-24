use approx::assert_relative_eq;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub mass: f64,
}

pub const G: f64 = 1.0;
pub const SOFTENING: f64 = 5.0;
pub const SUB_STEPS: usize = 4;
pub const DT: f64 = 0.016;

pub fn compute_accelerations(bodies: &[Body]) -> (Vec<f64>, Vec<f64>) {
    let n = bodies.len();
    let mut ax = vec![0.0; n];
    let mut ay = vec![0.0; n];
    let soft_sq = SOFTENING * SOFTENING;
    for i in 0..n {
        let bi = &bodies[i];
        for j in (i + 1)..n {
            let bj = &bodies[j];
            let dx = bj.x - bi.x;
            let dy = bj.y - bi.y;
            let dist_sq = dx * dx + dy * dy + soft_sq;
            let dist = dist_sq.sqrt();
            let force = G * bi.mass * bj.mass / dist_sq;
            let fx = force * dx / dist;
            let fy = force * dy / dist;
            ax[i] += fx / bi.mass;
            ay[i] += fy / bi.mass;
            ax[j] -= fx / bj.mass;
            ay[j] -= fy / bj.mass;
        }
    }
    (ax, ay)
}

pub fn step_physics(bodies: &mut [Body], sub_dt: f64) {
    if bodies.is_empty() {
        return;
    }
    let (ax1, ay1) = compute_accelerations(bodies);
    for (i, b) in bodies.iter_mut().enumerate() {
        b.vx += 0.5 * ax1[i] * sub_dt;
        b.vy += 0.5 * ay1[i] * sub_dt;
        b.x += b.vx * sub_dt;
        b.y += b.vy * sub_dt;
    }
    let (ax2, ay2) = compute_accelerations(bodies);
    for (i, b) in bodies.iter_mut().enumerate() {
        b.vx += 0.5 * ax2[i] * sub_dt;
        b.vy += 0.5 * ay2[i] * sub_dt;
    }
}

pub fn total_momentum(bodies: &[Body]) -> (f64, f64) {
    bodies.iter().fold((0.0, 0.0), |(px, py), b| {
        (px + b.mass * b.vx, py + b.mass * b.vy)
    })
}

pub fn total_energy(bodies: &[Body]) -> f64 {
    let kinetic: f64 = bodies.iter().map(|b| 0.5 * b.mass * (b.vx * b.vx + b.vy * b.vy)).sum();
    let mut potential = 0.0;
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let dx = bodies[j].x - bodies[i].x;
            let dy = bodies[j].y - bodies[i].y;
            let r2 = dx * dx + dy * dy + SOFTENING * SOFTENING;
            potential -= G * bodies[i].mass * bodies[j].mass / r2.sqrt();
        }
    }
    kinetic + potential
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symmetric_two_body() {
        let mut bodies = vec![
            Body { x: -1.0, y: 0.0, vx: 0.0, vy: -1.0, mass: 1.0 },
            Body { x: 1.0, y: 0.0, vx: 0.0, vy: 1.0, mass: 1.0 },
        ];
        let sub_dt = DT / SUB_STEPS as f64;
        for _ in 0..1000 {
            for _ in 0..SUB_STEPS {
                step_physics(&mut bodies, sub_dt);
            }
        }
        assert_relative_eq!(bodies[0].x, -bodies[1].x, epsilon = 1e-10);
        assert_relative_eq!(bodies[0].y, -bodies[1].y, epsilon = 1e-10);
        assert_relative_eq!(bodies[0].vx, -bodies[1].vx, epsilon = 1e-10);
        assert_relative_eq!(bodies[0].vy, -bodies[1].vy, epsilon = 1e-10);
    }

    #[test]
    fn momentum_conservation() {
        let mut bodies = vec![
            Body { x: 0.0, y: 0.0, vx: 0.1, vy: 0.2, mass: 1.0 },
            Body { x: 1.0, y: 0.0, vx: -0.1, vy: -0.2, mass: 1.0 },
            Body { x: 0.0, y: 1.0, vx: 0.0, vy: 0.0, mass: 2.0 },
        ];
        let initial = total_momentum(&bodies);
        let sub_dt = DT / SUB_STEPS as f64;
        for _ in 0..1000 {
            for _ in 0..SUB_STEPS {
                step_physics(&mut bodies, sub_dt);
            }
        }
        let final_m = total_momentum(&bodies);
        assert_relative_eq!(final_m.0, initial.0, epsilon = 1e-12);
        assert_relative_eq!(final_m.1, initial.1, epsilon = 1e-12);
    }

    #[test]
    fn energy_conservation() {
        let mut bodies = vec![
            Body { x: -1.0, y: 0.0, vx: 0.0, vy: -0.5, mass: 1.0 },
            Body { x: 1.0, y: 0.0, vx: 0.0, vy: 0.5, mass: 1.0 },
        ];
        let e0 = total_energy(&bodies);
        let sub_dt = DT / SUB_STEPS as f64;
        for _ in 0..1000 {
            for _ in 0..SUB_STEPS {
                step_physics(&mut bodies, sub_dt);
            }
        }
        let e1 = total_energy(&bodies);
        let rel_err = (e1 - e0).abs() / e0.abs();
        assert!(rel_err < 1e-6, "Energy drift: {}", rel_err);
    }

    #[test]
    fn newton_third_law() {
        let bodies = vec![
            Body { x: 0.0, y: 0.0, vx: 0.0, vy: 0.0, mass: 2.0 },
            Body { x: 1.0, y: 1.0, vx: 0.0, vy: 0.0, mass: 3.0 },
        ];
        let (ax, ay) = compute_accelerations(&bodies);
        assert_relative_eq!(bodies[0].mass * ax[0] + bodies[1].mass * ax[1], 0.0, epsilon = 1e-12);
        assert_relative_eq!(bodies[0].mass * ay[0] + bodies[1].mass * ay[1], 0.0, epsilon = 1e-12);
    }
}
