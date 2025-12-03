use std::time::Duration;

use crate::game::{action::{Action, ComboState}, meter_state::{MeterState, SenState}};
use rand::prelude::*;

pub fn random_states() -> Vec<Action> {
    let mut actions = Vec::new();
    let mut rng = rand::rng();
    for _i in 1..100 {
        actions.push(Action {
            action: rng.random::<u16>(),
            meter_state: MeterState {
                sen: SenState {
                    getsu: rng.random_bool(0.5f64),
                    ka: rng.random_bool(0.5f64),
                    setsu:rng.random_bool(0.5f64)
                },
                kenki: rng.random_range(0..100)
            },
            combo_state: ComboState {
                prev_action: rng.random::<u16>()
            },
            encounter_time: Duration::from_millis(rng.random_range(0..100000))
        });
    }

    return actions;
}