use egui::Context;

pub fn palette_window(ctx: &Context) {
    egui::Window::new("Palette")
        .collapsible(false)
        .auto_sized()
        .show(ctx, |ui| {
            ui.label("I'm going to be an action palette some day!")
        });
}