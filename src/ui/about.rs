use crate::game_version;

pub struct AboutWindow {
    alive: bool
}

impl Default for AboutWindow {
    fn default() -> Self {
        Self { alive: true }
    }
}

impl crate::window::Window for AboutWindow {
    fn window(&mut self, ctx: &egui::Context) {
        egui::Window::new("About")
            .open(&mut self.alive)
            .auto_sized()
            .show(ctx, |ui| {
                ui.heading("Amanda's Samurai Planner");
                ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));

                ui.add_space(12.0);

                ui.label(format!("Built for FFXIV {}", game_version!()));

                ui.add_space(12.0);

                {
                    ui.heading("Built with:");

                    ui.hyperlink_to(
                        "Rust",
                        "https://rust-lang.org/",
                    );

                    ui.hyperlink_to(
                        "egui and eframe",
                        "https://github.com/emilk/egui",
                    );
                }
            });
    }

    fn alive(&self) -> bool {
        self.alive
    }
}