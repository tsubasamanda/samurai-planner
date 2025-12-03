use crate::project::action_list::ActionList;

pub struct Project {
    pub(crate) actions: ActionList,
}

impl Project {
    pub fn default() -> Project {
        Self {
            actions: ActionList {
                actions: Vec::new()
            }
        }
    }
}