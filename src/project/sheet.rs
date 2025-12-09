
pub struct Sheet {
    pub title: String
}

impl Default for Sheet {
    fn default() -> Self {
        Self {
            title: "New Sheet".to_owned()
        }
    }
}