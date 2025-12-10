use egui::{Context, Id};
use egui_extras::{Column, TableBuilder};

use crate::project::sheet::Sheet;

pub fn gear_options_window(ctx: &Context, sheet: &mut Sheet, open: &mut bool) {
    egui::Window::new(format!("{} - Gear Options", sheet.title))
                .id(Id::new(format!("{:?}gear", sheet.id)))
                .open(open)
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
                                    ui.add(egui::DragValue::new(&mut sheet.gear.wd));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("HP");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut sheet.gear.hp));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Strength");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut sheet.gear.str));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Critical Hit");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut sheet.gear.crt));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Direct Hit Rate");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut sheet.gear.dh));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Determination");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut sheet.gear.det));
                                });
                            });
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label("Skill Speed");
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut sheet.gear.sks));
                                });
                            });
                        })
                });
}