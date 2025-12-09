use std::collections::LinkedList;

use crate::project::{action::Action, gear::GearStats};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Sheet {
    pub title: String,
    pub actions: LinkedList<Action>,
    pub gear: GearStats,
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            title: "New Sheet".to_owned(),
            actions: LinkedList::new(),
            gear: GearStats::default()
        }
    }
}