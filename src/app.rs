#![warn(clippy::all, rust_2018_idioms)]

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {

}

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }
}