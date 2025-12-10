use crate::ui::sheet::SheetWindow;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ActiveWindows {
    windows: Vec<SheetWindow>
}

impl ActiveWindows {
    pub fn new() -> Self {
        Self {
            windows: Vec::new()
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        for window in self.windows.iter_mut() {
            if window.alive() {
                window.window(ctx);
            }
        }
    }

    pub fn add(&mut self, window: SheetWindow) {
        self.windows.push(window);
    }

    pub fn prune(&mut self) {
        self.windows.retain(|window| window.alive());
    }
}