use uuid::Uuid;

use crate::project::action::Action;

pub struct Sheet {
    pub title: String,
    pub actions: Vec<Action>
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            title: "New Sheet".to_owned(),
            actions: Vec::new()
        }
    }
}