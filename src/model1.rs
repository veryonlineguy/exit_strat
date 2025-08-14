//! 2‑state Kalman weight model (weight + maintenance offset)
//! ---------------------------------------------------------
//! Enhancements in this version
//!   • Keeps 7‑day *and* 14‑day Kalman‑smoothed weight deltas
//!   • Computes 14‑day **percent‑loss‑per‑week**
//!   • Emits a simple calorie‑tuning recommendation:
//!        < 0.5 % / wk   ⇒ suggest –100 kcal (or +1 k steps)
//!        0.5–1.0 % / wk ⇒ keep calories
//!        > 1.0 % / wk   ⇒ suggest +100 kcal
//!
//! The final stdout block looks like:
//!     7‑day Kalman change : –1.8 lb (loss)
//!     14‑day %‑loss / wk : 0.8 %
//!     Recommendation     : keep calories steady
//! --------------------------------------------------------------

use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate};
use csv::Reader;
use nalgebra::{Matrix3, RowVector3, Vector3};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

use nalgebra::{Matrix2, RowVector2, Vector2};

// ---------------- constants ----------------------------------
const KCAL_PER_KG: f64 = 7_700.0; // ≈ 1 kg fat ≈ 7 700 kcal
const INITIAL_TDEE: f64 = 3_100.0; // baseline guess
const INITIAL_K_PER_RE: f64 = 70.0; // kcal per RE starting point

// ---------------- input row ----------------------------------

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Row {
    pub date: String,
    pub weight: f64,          // kg *or* lb – see flag below
    pub intake_kcal: f64,     // +ve = intake below 3 100 kcal
    pub activity: f64,        // kcal of exercise (optional, can be 0)
    pub protien: Option<f64>, //
}

#[derive(Deserialize, Serialize)]
struct Target {
    target: u32,
    year: i32,
    month: u32,
    day: u32,
}

const PCT_LOSS_LOW: f64 = 0.005; // 0.5 % / wk
const PCT_LOSS_HIGH: f64 = 0.010; // 1.0 % / wk
const CAL_ADJUST_STEP: u32 = 200;
const LB_PER_KG: f64 = 2.205;

// convert incoming weight to kg if the CSV is in pounds
const WEIGHT_IN_POUNDS: bool = true; // flip to false if CSV uses kg
fn to_kg(w: f64) -> f64 {
    if WEIGHT_IN_POUNDS { w / 2.205 } else { w }
}

// ---------------- Kalman 2‑D filter --------------------------

#[derive(Clone, Debug)]
pub struct Kalman3D {
    x: Vector3<f64>, // [ w_kg , tdee , k_per_re ]
    p: Matrix3<f64>, // covariance
    q: Matrix3<f64>, // process noise matrix
}

impl Kalman3D {
    pub fn new(initial_w_kg: f64) -> Self {
        let q_weight_var: f64 = 0.0_f64.powi(2); // process noise on weight
        let q_tdee_var: f64 = 12.0_f64.powi(2); // TDEE drift ±40 kcal
        let q_re_var: f64 = 2.0_f64.powi(2); // k/RE drift ±5 kcal

        let p0 = Matrix3::from_diagonal(&Vector3::new(
            2.0_f64.powi(2),
            200.0_f64.powi(2),
            30.0_f64.powi(2),
        ));
        let q = Matrix3::from_diagonal(&Vector3::new(q_weight_var, q_tdee_var, q_re_var));
        Kalman3D {
            x: Vector3::new(initial_w_kg, INITIAL_TDEE, INITIAL_K_PER_RE),
            p: p0,
            q,
        }
    }

    /// One‑day predict / update
    pub fn step(&mut self, row: &Row) {
        let r_scale_var: f64 = 0.6_f64.powi(2); // measurement variance on scale (kg^2)
        let c = KCAL_PER_KG;
        let a = row.activity;

        // ---------- PREDICT ----------
        // F_t and control b_t
        let f = Matrix3::new(1.0, -1.0 / c, a / c, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        let b = Vector3::new(row.intake_kcal / c, 0.0, 0.0);

        // state & covariance prediction
        self.x = f * self.x + b;
        self.p = f * self.p * f.transpose() + self.q;

        // ---------- UPDATE ----------
        let z = to_kg(row.weight); // measurement
        let h = RowVector3::new(1.0, 0.0, 0.0); // H
        let y = z - (h * self.x)[0]; // residual
        let s = (h * self.p * h.transpose())[0] + r_scale_var; // scalar
        let k = (self.p * h.transpose()) / s; // 3x1 Kalman gain

        self.x = self.x + k * y;

        // Joseph form keeps P symmetric/PSD
        let i = Matrix3::identity();
        self.p = (i.clone() - &k * &h) * self.p * (i - &k * &h).transpose()
            + k * r_scale_var * k.transpose();
    }

    // Accessors
    pub fn weight_kg(&self) -> f64 {
        self.x[0]
    }
    pub fn tdee(&self) -> f64 {
        self.x[1]
    }
    pub fn kcal_per_re(&self) -> f64 {
        self.x[2]
    }
}

#[derive(Clone, Debug)]
pub struct Kalman2D {
    x: Vector2<f64>, // [ w_kg , tdee ]
    p: Matrix2<f64>, // covariance
    q: Matrix2<f64>, // process noise
}

impl Kalman2D {
    pub fn new(initial_w_kg: f64) -> Self {
        // Tune these:
        let q_weight_var: f64 = 0.0_f64.powi(2); // weight has no intrinsic drift in the process model
        let q_tdee_var: f64 = 80.0_f64.powi(2); // allow TDEE to drift day-to-day (e.g., ~±12 kcal)

        // Initial uncertainty (also tune):
        let p0 = Matrix2::from_diagonal(&Vector2::new(0.5_f64.powi(2), 600.0_f64.powi(2)));
        let q = Matrix2::from_diagonal(&Vector2::new(q_weight_var, q_tdee_var));

        Kalman2D {
            x: Vector2::new(initial_w_kg, INITIAL_TDEE),
            p: p0,
            q,
        }
    }

    /// One-day predict / update
    pub fn step(&mut self, row: &Row) {
        let r_scale_var: f64 = 0.5_f64.powi(2); // variance of scale noise (kg^2) — tune to your scale/variance
        let c = KCAL_PER_KG; // e.g., 7700.0

        // ---------- PREDICT ----------
        // State transition F and control B (uses only total intake_kcal)
        let f = Matrix2::new(1.0, -1.0 / c, 0.0, 1.0);
        let b = Vector2::new(row.intake_kcal / c, 0.0);

        // Predict state and covariance
        self.x = f * self.x + b;
        self.p = f * self.p * f.transpose() + self.q;

        // ---------- UPDATE ----------
        let z = to_kg(row.weight); // measured morning weight in kg
        let h = RowVector2::new(1.0, 0.0); // we observe only weight
        let y = z - (h * self.x)[0]; // residual
        let s = (h * self.p * h.transpose())[0] + r_scale_var;
        let k = (self.p * h.transpose()) / s; // 2x1 Kalman gain

        self.x = self.x + k * y;

        // Joseph form for numerical stability (keeps P symmetric/PSD)
        let i = Matrix2::identity();
        self.p = (i.clone() - &k * &h) * self.p * (i - &k * &h).transpose()
            + k * r_scale_var * k.transpose();
    }

    #[inline]
    pub fn w_kg(&self) -> f64 {
        self.x[0]
    }
    #[inline]
    pub fn tdee(&self) -> f64 {
        self.x[1]
    }
}

fn compute_kalman_deltas(weights: &Vec<f64>) -> (f64, f64, f64) {
    let len = weights.len();
    let delta7 = if len >= 7 {
        weights[len - 1] - weights[len - 7]
    } else {
        0.0
    };
    let delta14 = if len >= 15 {
        weights[len - 1] - weights[len - 14]
    } else {
        0.0
    };
    let pct_loss_per_week = if delta14 != 0.0 {
        -delta14 / (14.0 / 7.0) / weights[len - 1]
    } else {
        0.0
    };
    (delta7, delta14, pct_loss_per_week)
}

// ---------------- main runner --------------------------------
pub fn run<P: AsRef<std::path::Path>>(csv_path: P) -> Result<()> {
    // ---------- ingest CSV ----------

    let today = Local::now();

    let target_file = File::open("data/target.json").ok();
    let mut target = Target {
        target: 2700,
        year: today.year(),
        month: today.month(),
        day: today.day(),
    };
    if let Some(mut file) = target_file {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        target = serde_json::from_str(&contents)?;
        println!(
            "Loaded Target date: {:04}-{:02}-{:02}, target: {}",
            target.year, target.month, target.day, target.target
        );
    }
    // ---------- estimate R (scale variance) ----------

    let file = File::open(csv_path)?;
    let mut rdr = Reader::from_reader(file);
    let mut kf = Kalman3D::new(127.0);

    let mut kalman_weights: Vec<f64> = Vec::new();
    let mut cals :Vec<f64> = Vec::new();

    for result in rdr.deserialize() {
        let row: Row = result?;
        kf.step(&row);
        println!(
            "final est: {:.2} lb  TDEE {:.0} kcal  k/RE {:.1}",
            kf.weight_kg() * LB_PER_KG,
            kf.tdee(),
            kf.kcal_per_re()
        );
        cals.push(row.intake_kcal);
        kalman_weights.push(kf.weight_kg() * LB_PER_KG);
    }

    let (delta7, delta14, pct_loss_per_week) = compute_kalman_deltas(&kalman_weights);

    let dir = if delta7 < 0.0 { "loss" } else { "gain" };

    let dir14 = if delta14 < 0.0 { "loss" } else { "gain" };

    println!(
        "7-day Kalman {} {:.2}lb, 14-day {dir14} {:.2}lb, %{dir14}/week: {:.3}%",
        dir,
        delta7.abs(),
        delta14.abs(),
        pct_loss_per_week.abs()
    );

    // 14‑day %‑loss per week & guidance
    // guidance

    let sum: f64 = cals.into_iter().rev().take(7).sum();
    let avg = sum /7.0;


    println!("Guidelines: -{} < {:.3 } < -{} ", PCT_LOSS_LOW, -pct_loss_per_week ,  PCT_LOSS_HIGH);


    let suggestion = if pct_loss_per_week < PCT_LOSS_LOW {
        let target =  avg - CAL_ADJUST_STEP as f64;
        format!("Suggest −{} kcal/day T: {:.0}", CAL_ADJUST_STEP as i32,target)
    } else if pct_loss_per_week > PCT_LOSS_HIGH {
        let target = avg + CAL_ADJUST_STEP as f64;
        format!(
            "Suggest +{} kcal/day T: {:.0}",
            CAL_ADJUST_STEP as i32,
            target
        )
    } else {
        "Keep calories steady".to_string()
    };

    println!("{}", suggestion);

    let today = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();

    let update = NaiveDate::from_ymd_opt(target.year, target.month, target.day).unwrap();

   

    Ok(())
}

// ---------------- main runner --------------------------------
pub fn run_one_state<P: AsRef<std::path::Path>>(csv_path: P) -> Result<()> {
    // ---------- ingest CSV ----------

    // ---------- estimate R (scale variance) ----------

    let file = File::open(csv_path)?;
    let mut rdr = Reader::from_reader(file);
    let mut kf = Kalman2D::new(127.0);

    let mut kalman_weights: Vec<f64> = Vec::new();
    for result in rdr.deserialize() {
        let row: Row = result?;
        kf.step(&row);
        println!(
            "final est: {:.2} lb  TDEE {:.0} kcal",
            kf.w_kg() * LB_PER_KG,
            kf.tdee(),
        );
        kalman_weights.push(kf.w_kg() * LB_PER_KG);
    }

    Ok(())
}
