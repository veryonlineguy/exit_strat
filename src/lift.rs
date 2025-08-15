use chrono::{DateTime, Datelike, Days, Local, NaiveDate, Weekday};

#[allow(dead_code)]
#[derive(Clone)]
struct Lift<'a> {
    name: String,
    target: &'a str,
    weight: u8,
    sets: Vec<u8>,
}

#[allow(dead_code)]
#[derive(Clone)]
struct Workout<'a> {
    warmup: Option<String>,
    strength: Option<Vec<Lift<'a>>>,
    cardio: Option<String>,
}

struct Week<'a> {
    monday: Option<Workout<'a>>,
    tuesday: Option<Workout<'a>>,
    wednesday: Option<Workout<'a>>,
    thursday: Option<Workout<'a>>,
    friday: Option<Workout<'a>>,
    saturday: Option<Workout<'a>>,
    sunday: Option<Workout<'a>>,
}

fn format_workout<'a>(workout:  Workout<'a>) -> String {
    

    let mut result = String::new();
 {
        result.push_str("# Heben\n");

        
     
        if workout.warmup.is_some() {
            result.push_str("## LIFT\n");
            let workout_str = "- [ ] ".to_owned() + &workout.warmup.clone().unwrap() + "\n";
            result.push_str(&workout_str);
        }
        if workout.strength.is_some() {
        for lift in workout.strength.as_ref().unwrap() {
            for set in lift.sets.clone() {
                let set_str = "- [ ] ".to_owned()
                    + &lift.name.clone()
                    + " "
                    + &set.to_string()
                    + " sets @ "
                    + &lift.weight.to_string()
                    + "lbs\n";
                result.push_str(&set_str);
            }
        }
        result.push_str("\n");
    }
    }

    if workout.cardio.is_some() {
        result.push_str("# Cardio\n");
        result.push_str(&workout.cardio.clone().unwrap());
    }

    result
}

pub fn get_lifts() -> String {
    let week_no = crate::schedule::utils::current_week_number();
    let _monday = if week_no % 5 == 5 {
        Workout {
            warmup: Some("- [ ] 5 min erg @ 22spm w df 1".to_string()),
            cardio: Some("- [ ] 2000k @28spm".to_string()),
            strength: None,
        }
    } else {
        Workout {
            warmup: Some("5 min Elliptical".to_string()),
            strength: vec![
                Lift {
                    name: "Leg Extensions".to_string(),
                    target: "Quads",
                    weight: 20,
                    sets: vec![4],
                },
                Lift {
                    name: "Slant board situp (weighted)".to_string(),
                    target: "Abs",
                    weight: 5,
                    sets: vec![2, 2],
                },
                Lift {
                    name: "Dumbell Skull Crushers".to_string(),
                    target: "Triceps",
                    weight: 10,
                    sets: vec![4, 5],
                },
                Lift {
                    name: "Dumbell Curl (2-arm)".to_string(),
                    target: "Biceps",
                    weight: 15,
                    sets: vec![5, 5, 6],
                },
                Lift {
                    name: "Dumbell Facepull".to_string(),
                    target: "Shoulders",
                    weight: 10,
                    sets: vec![6, 6, 5],
                },
            ]
            .into(),
            cardio: None,
        }
    };

    let _tuesday = Workout {
        warmup: Some("15 min erg @ 18spm".to_string()),
        strength: vec![
            Lift {
                name: "Barbell Hip Thrusts".to_string(),
                target: "Glutes",
                weight: 20,
                sets: vec![6, 6, 6],
            },
            Lift {
                name: "Dumbell Shrugs".to_string(),
                target: "Shoulders",
                weight: 15,
                sets: vec![6, 6],
            },
        ]
        .into(),
        cardio: None,
    };

    let _wednesday = Workout {
        warmup: Some("5 min Elliptical".to_string()),
        strength: vec![
            Lift {
                name: "Pulldown (Normal Grip)".to_string(),
                target: "Back",
                weight: 40,
                sets: vec![7],
            },
            Lift {
                name: "Seated Cable Row".to_string(),
                target: "Back",
                weight: 30,
                sets: vec![6],
            },
            Lift {
                name: "Machine Chest Press".to_string(),
                target: "Chest",
                weight: 60,
                sets: vec![6, 6],
            },
            Lift {
                name: "Dumbell Flye (Flat)".to_string(),
                target: "Chest",
                weight: 15,
                sets: vec![4, 5],
            },
            Lift {
                name: "Dumbell Skull Crushers".to_string(),
                target: "Triceps",
                weight: 10,
                sets: vec![4, 3],
            },
            Lift {
                name: "Dumbell Curl (2-arm)".to_string(),
                target: "Biceps",
                weight: 15,
                sets: vec![5, 5],
            },
            Lift {
                name: "Slant board situp (weighted)".to_string(),
                target: "Abs",
                weight: 5,
                sets: vec![2, 2, 2],
            },
        ]
        .into(),
        cardio: None,
    };

    let _thursday = Workout {
        warmup: Some("5 min Elliptical".to_string()),
        strength: vec![
            Lift {
                name: "Barbell Hip Thrusts".to_string(),
                target: "Glutes",
                weight: 20,
                sets: vec![6, 6],
            },
            Lift {
                name: "Dumbell Stiff Legeged Deadlift".to_string(),
                target: "Quads",
                weight: 10,
                sets: vec![5],
            },
            Lift {
                name: "Dumbell Facepull".to_string(),
                target: "Shoulders",
                weight: 10,
                sets: vec![6, 6],
            },
            Lift {
                name: "Leg Extensions".to_string(),
                target: "Quads",
                weight: 20,
                sets: vec![4],
            },
        ]
        .into(),
        cardio: None,
    };

    let _friday = Workout {
        warmup: Some("5 min Elliptical".to_string()),
        strength: vec![
            Lift {
                name: "Slant board situp (weighted)".to_string(),
                target: "Abs",
                weight: 5,
                sets: vec![2, 2],
            },
            Lift {
                name: "Pulldown (Normal Grip)".to_string(),
                target: "Back",
                weight: 40,
                sets: vec![8,],
            },
            Lift {
                name: "Seated Cable Row".to_string(),
                target: "Back",
                weight: 30,
                sets: vec![7],
            },
            Lift {
                name: "Machine Chest Press".to_string(),
                target: "Chest",
                weight: 60,
                sets: vec![6],
            },
            Lift {
                name: "Dumbell Flye (Flat)".to_string(),
                target: "Chest",
                weight: 15,
                sets: vec![4, 3],
            },
            Lift {
                name: "Dumbell Skull Crushers".to_string(),
                target: "Triceps",
                weight: 10,
                sets: vec![4, 3],
            },
            Lift {
                name: "Dumbell Curl (2-arm)".to_string(),
                target: "Biceps",
                weight: 15,
                sets: vec![5, 5],
            },
        ]
        .into(),
        cardio: None,
    };

    let saturday = Workout {
        warmup: None,
        strength: None,
        cardio: Some("- [ ] 5 min df 1\n- [ ] 5000m @ 22spm".to_string()),
    };
    
    let grindset = Workout {
        warmup: None,
        strength: None,
        cardio: Some("- [ ] 5 min df 1\n- [ ] 20m @ 22spm".to_string()),
    };


    let none = Workout{warmup: None, strength: None, cardio: None};
 

    let deload = Workout {
        cardio: Some("4k @ 18spm Df 1".to_string()),
        strength: None,
        warmup: None,
    };

    let local: DateTime<Local> = Local::now();
    let tomorrow: NaiveDate =
        NaiveDate::from_ymd_opt(local.year(), local.month(), local.day()).unwrap();
    let tomorrow = tomorrow + Days::new(1);
    let day = tomorrow.weekday();

    let week_no = crate::schedule::utils::current_week_number();

    let monday = if week_no % 5 == 0 {
           Workout {
            warmup: Some("- [ ] 5 min erg @ 22spm w df 1".to_string()),
            cardio: Some("- [ ] 2000k @28spm".to_string()),
            strength: None,
        }
    } else {
        grindset.clone()
    };
    
    if week_no % 5 != 4 {
        match day {
            Weekday::Mon => format_workout(monday),
            Weekday::Tue => format_workout(grindset.clone()),
            Weekday::Wed => format_workout(grindset.clone()),
            Weekday::Thu => format_workout(grindset.clone()),
            Weekday::Fri => format_workout(grindset.clone()),
            Weekday::Sat => format_workout(saturday),
            Weekday::Sun => format_workout(none),
        }
    } else {
        match day {
            Weekday::Mon => format_workout(deload),
            Weekday::Tue => format_workout(none),
            Weekday::Wed => format_workout(deload),
            Weekday::Thu => format_workout(none),
            Weekday::Fri => format_workout(deload),
            Weekday::Sat => format_workout(none),
            Weekday::Sun => format_workout(none),
        }
    }
}
