use std::collections::LinkedList;

use egui::{Id, RichText};
use egui_dnd::dnd;
use egui_extras::{Column, TableBuilder};
use uuid::Uuid;

use crate::project::action::Action;
use crate::window::Window;
use crate::project::sheet::Sheet;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SheetWindow {
    alive: bool,
    sheet: Sheet,
    id: Id,
    subwindows: SubWindowState
}

#[derive(Default)]
#[derive(serde::Deserialize, serde::Serialize)]
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

#[typetag::serde]
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
                dnd(ui, "actions").show(self.sheet.actions.iter_mut(), |ui, item, handle, _state| {
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
        

        if self.subwindows.gear_options {
            egui::Window::new(format!("{} - Gear Options", self.sheet.title))
                .id(Id::new(format!("{:?}gear", self.id)))
                .open(&mut self.subwindows.gear_options)
                .auto_sized()
                .show(ctx, |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(false)
                        .cell_layout(egui::Layout::right_to_left(egui::Align::Center))
                        .column(Column::auto())
                        .column(Column::auto())
                        .body(|mut body| {
                            // TODO: Clean this up
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Weapon Damage");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut self.sheet.gear.wd));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("HP");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut self.sheet.gear.hp));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Strength");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut self.sheet.gear.str));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Critical Hit");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut self.sheet.gear.crt));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Direct Hit Rate");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut self.sheet.gear.dh));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Determination");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut self.sheet.gear.det));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Skill Speed");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut self.sheet.gear.sks));
                                });
                            });
                        })
                });
        }
    }

    fn alive(&self) -> bool {
        self.alive
    }
}