#![warn(clippy::all, rust_2018_idioms)]

use crate::project::sheet::Sheet;
use crate::ui::sheet::SheetWindow;
use crate::window::ActiveWindows;
use crate::ui::about::AboutWindow;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
	#[serde(skip)]
    active_windows: ActiveWindows
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_windows: ActiveWindows::new()
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Sheet").clicked() {
                        self.active_windows.add(
                            Box::new(
                                SheetWindow::new(Sheet::default())
                            )
                        );
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.active_windows.add(Box::new(AboutWindow::default()));
                    }
                });
            })
        });

        self.active_windows.render(ctx);
		self.active_windows.prune();
    }
}