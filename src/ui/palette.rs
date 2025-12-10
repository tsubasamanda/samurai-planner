use egui::Context;

use crate::game::action;

pub fn palette_window(ctx: &Context) {
    egui::Window::new("Palette")
        .collapsible(false)
        .auto_sized()
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                for a in action::GAME_ACTIONS {
                    ui.add(
                        egui::Image::new(a.icon)
                            .fit_to_exact_size(egui::Vec2::new(30.0, 30.0))
                    );
                }
            });
        });
}