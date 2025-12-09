#[derive(Clone)]
pub struct Project {
    pub title: String,
    pub sheets: Vec<Box<crate::project::sheet::Sheet>>
}