use std::collections::LinkedList;

use crate::project::action::Action;

pub struct Sheet {
    pub title: String,
    pub actions: LinkedList<Action>
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            title: "New Sheet".to_owned(),
            actions: LinkedList::new()
        }
    }
}