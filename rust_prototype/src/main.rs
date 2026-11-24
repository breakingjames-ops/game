use astralis_rust_physics::{step_physics, total_momentum, Body, DT, SUB_STEPS};

fn main() {
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
    println!("Final state: {:?}", bodies);
    println!("Total momentum: {:?}", total_momentum(&bodies));
}
