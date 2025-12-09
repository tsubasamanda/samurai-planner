use std::collections::LinkedList;

use egui::{Id, RichText};
use egui_dnd::dnd;
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
                    });

                    if ui.button("Gear Options").clicked() {
                        self.subwindows.gear_options = true;
                    }
                });

                ui.separator();
                dnd(ui, "actions").show(self.sheet.actions.iter_mut(), |ui, item, handle, state| {
                    handle.ui(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(item.timestamp()).monospace());
                            item.meter_state.render(ui);
                            if ui.button("\u{274e}").clicked() {
                                item.flag_delete = true;
                            }
                        });
                    });
                });

                self.sheet.actions = self.sheet.actions.extract_if(|a| !a.flag_delete).collect::<LinkedList<_>>();

                if ui.button("+").clicked() {
                    self.sheet.actions.push_back(Action::default());
                }
            });
    }

    fn alive(&self) -> bool {
        self.alive
    }
}