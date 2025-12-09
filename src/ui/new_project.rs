pub struct NewProjectDialog {
    alive: bool,
    input: String
}

impl Default for NewProjectDialog {
    fn default() -> Self {
        NewProjectDialog {
            alive: true,
            input: "".to_string()
        }
    }
}

impl crate::window::Window for NewProjectDialog {
    fn window(&mut self, ctx: &egui::Context) {
        egui::Window::new("New Project")
            .open(&mut self.alive)
            .auto_sized()
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.add(egui::TextEdit::singleline(&mut self.input));
                });

                if ui.button("Create").clicked() {
                    ui.close();
                }
            });
    }

    fn alive(&self) -> bool {
        self.alive
    }
} 