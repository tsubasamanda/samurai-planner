pub struct ActiveWindows {
    windows: Vec<Box<dyn Window>>
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

    pub fn add(&mut self, window: Box<dyn Window>) {
        self.windows.push(window);
    }

    pub fn prune(&mut self) {
        self.windows.retain(|window| window.alive());
    }
}

pub trait Window {
    fn window(&mut self, ctx: &egui::Context);
    fn alive(&self) -> bool;
}