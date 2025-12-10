#![warn(clippy::all, rust_2018_idioms)]

use crate::project::sheet::Sheet;
use crate::ui::palette::palette_window;
use crate::ui::sheet::SheetWindow;
use crate::window::ActiveWindows;
use crate::ui::about::about_window;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    active_windows: ActiveWindows,
    toolbars: Toolbars
}

#[derive(Default)]
#[derive(serde::Deserialize, serde::Serialize)]
struct Toolbars {
    about: bool,
    palette: bool,
    comparison: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_windows: ActiveWindows::new(),
            toolbars: Toolbars::default()
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
    
    fn update_toolbars(&mut self, ctx: &egui::Context) {
        if self.toolbars.about {
			about_window(ctx, &mut self.toolbars.about);
		}

		if self.toolbars.palette {
			palette_window(ctx);
		}
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Sheet").clicked() {
                        self.active_windows.add(SheetWindow::new(Sheet::default())
                        );
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.toolbars.palette, "Palette");
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.toolbars.about = true;
                    }
                });
            })
        });

		self.update_toolbars(ctx);

        self.active_windows.render(ctx);
		self.active_windows.prune();
    }
}