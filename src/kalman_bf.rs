// 3‑state Extended Kalman Filter for Body‑Composition
// ---------------------------------------------------
// State vector x = [ BF% , weight_kg , TDEE_kcal ]ᵀ
// Measurements  z = [ BF%_scale , weight_kg_scale ]ᵀ (daily)
// Controls      u = { intake_kcal, activity_kcal }
//
// Weight update:   ΔW = (intake_kcal − (TDEE_kcal + activity_kcal)) / 7700
// TDEE evolves as a random walk with tiny process noise, letting the
// filter "learn" your maintenance calories as you diet or gain.
//
// Crate deps (Cargo.toml)
// [dependencies]
// nalgebra = "0.34"
// csv       = "1.3"
// serde     = { version = "1", features = ["derive"] }

use nalgebra::{Matrix2, Matrix3, Matrix3x2, Matrix2x3, Vector2, Vector3};
use serde::Deserialize;
use std::{error::Error, path::Path};

/// CSV row structure.
#[derive(Debug, Deserialize)]
pub struct Row {
    pub date: String,
    pub weight: f64,        // kg *or* lb – we'll convert
    pub intake_kcal: f64,   // raw calories eaten
    pub activity: f64,      // exercise calories (can be 0)
    pub body_fat: f64,      // scale BF% measurement
}

/// Extended Kalman filter struct
pub struct KalmanBF {
    x: Vector3<f64>,   // state estimate [bf%, w_kg, tdee]
    p: Matrix3<f64>,   // covariance
    q: Matrix3<f64>,   // process noise
    r: Matrix2<f64>,   // measurement noise
}

impl KalmanBF {
    pub fn new(initial_bf: f64, initial_w_kg: f64, tdee_guess: f64) -> Self {
        let x = Vector3::new(initial_bf, initial_w_kg, tdee_guess);

        // Wide priors: 4 % BF var, 1 kg var, 400 kcal var
        let mut p = Matrix3::zeros();
        p[(0, 0)] = 4.0;
        p[(1, 1)] = 1.0;
        p[(2, 2)] = 400.0;

        // Process noise: BF changes very little; weight per formula; TDEE slow drift
        let mut q = Matrix3::zeros();
        q[(0, 0)] = 0.0004;    // (~0.02 %/day σ)
        q[(1, 1)] = 0.0025;    // (±50 g)
        q[(2, 2)] = 25.0;      // (±5 kcal/day drift)

        // Measurement noise: scale BF% σ=2 → var=4 ; weight σ=0.2 kg → var≈0.04
        let r = Matrix2::new(4.0, 0.0, 0.0, 0.04);

        Self { x, p, q, r }
    }

    /// Predict step using control inputs (intake, activity).
    pub fn predict(&mut self, intake: f64, activity: f64) {
        let bf = self.x[0];
        let w = self.x[1];
        let tdee = self.x[2];

        let energy_balance = intake - (tdee + activity); // +ve = surplus
        let delta_w = energy_balance / 7700.0;           // kg change
        let w_pred = w + delta_w;

        // Assume all mass change is fat (can refine later)
        let fat_mass = bf / 100.0 * w;
        let fat_mass_pred = (fat_mass + delta_w).max(0.0);
        let bf_pred = 100.0 * fat_mass_pred / w_pred.max(1e-3);

        // State prediction
        let x_pred = Vector3::new(bf_pred, w_pred, tdee);

        // Jacobian F = ∂f/∂x  (linearised around current state)
        // We'll approximate:
        let mut f = Matrix3::identity();
        f[(1, 2)] =  -1.0 / 7700.0;          // ∂weight/∂tdee
        // bf% sensitivities tiny – ignore for simplicity

        // Covariance prediction: P = FPFᵀ + Q
        self.p = f * &self.p * f.transpose() + &self.q;
        self.x = x_pred;
    }

    /// Measurement update with BF% and weight readings.
    pub fn update(&mut self, bf_meas: f64, w_meas: f64) {
        let z = Vector2::new(bf_meas, w_meas);

        // H maps state → measurement: picks BF and weight.
        let mut h = Matrix2x3::zeros();
        h[(0, 0)] = 1.0; // BF%
        h[(1, 1)] = 1.0; // weight

        // Innovation
        let y = z - &h * &self.x;
        let s = &h * &self.p * h.transpose() + &self.r;
        let k = &self.p * h.transpose() * s.try_inverse().expect("S invertible");

        // Update state & covariance
        self.x += &k * y;
        let i = Matrix3::identity();
        self.p = (i - &k * h) * &self.p;
    }

    pub fn state(&self) -> (f64, f64, f64) {
        (self.x[0], self.x[1], self.x[2])
    }
}

/// Run EKF on CSV; emit: date,bf_est,wt_est_kg,tdee_est_kcal
pub fn run_from_csv<P: AsRef<Path>>(path: P, tdee_guess: f64) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(&path)?;
    let mut rows = rdr.deserialize::<Row>();

    let first = rows.next().ok_or("CSV empty")??;
    let mut kf = KalmanBF::new(first.body_fat, to_kg(first.weight), tdee_guess);
    kf.update(first.body_fat, to_kg(first.weight));

    println!("date,bf_est,wt_est_kg,tdee_est_kcal");
    let (bf, wt, tdee) = kf.state();
    println!("{},{:.2},{:.2},{:.0}", first.date, bf, wt, tdee);

    for row in rows {
        let r = row?;
        kf.predict(r.intake_kcal, r.activity);
        kf.update(r.body_fat, to_kg(r.weight));
        let (bf, wt, tdee) = kf.state();
        println!("{},{:.2},{:.2},{:.0}", r.date, bf, wt, tdee);
    }
    Ok(())
}

fn to_kg(w: f64) -> f64 {
    if w > 200.0 { w * 0.453_592 } else { w }
}

// -------------------------------------------------------------
// Example `main.rs`
// -------------------------------------------------------------
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     kalman_bf::run_from_csv("bodycomp.csv", 3100.0)?;
//     Ok(())
// }
// -------------------------------------------------------------
