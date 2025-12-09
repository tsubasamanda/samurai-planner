use egui::{Id, RichText};
use uuid::Uuid;

use crate::project::action::Action;
use crate::window::Window;
use crate::project::sheet::Sheet;

pub struct SheetWindow {
    alive: bool,
    sheet: Sheet,
    id: Id,
    subwindows: SubWindowState
}

#[derive(Default)]
struct SubWindowState {
    gear_options: bool
}

impl SheetWindow {
    pub fn new(s: Sheet) -> SheetWindow {
        SheetWindow {
            alive: true,
            sheet: s,
            id: Id::new(Uuid::new_v4()),
            subwindows: SubWindowState::default()
        }
    }
}

impl Window for SheetWindow {
    fn window(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.sheet.title.clone())
            .id(self.id)
            .open(&mut self.alive)
            .scroll([false, true])
            .show(ctx, |ui| {
                ui.collapsing("Options", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.text_edit_singleline(&mut self.sheet.title);
                    })
                });

                ui.separator();
                for a in self.sheet.actions.iter() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(a.timestamp()).monospace());
                    });
                }

                if ui.button("+").clicked() {
                    self.sheet.actions.push(Action::default());
                }
            });
    }

    fn alive(&self) -> bool {
        self.alive
    }
}