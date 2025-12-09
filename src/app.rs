#![warn(clippy::all, rust_2018_idioms)]

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
	#[serde(skip)]
    active_windows: crate::window::ActiveWindows,
    #[serde(skip)]
    open_projects: Vec<crate::project::project::Project>
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_windows: crate::window::ActiveWindows::new(),
			open_projects: Vec::new()
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
                    if ui.button("New Project").clicked() {
                        self.active_windows.add(Box::new(crate::ui::new_project::NewProjectDialog::default()));
                    }

                    if ui.button("Open Project").clicked() {

                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.active_windows.add(Box::new(crate::ui::about::AboutWindow::default()));
                    }
                });
            })
        });

        egui::SidePanel::left("project_panel")
            .min_width(300.0)
            .show(ctx, |ui| {
                for p in self.open_projects.iter() {
                    ui.collapsing(p.title.clone(), |ui| {
                        for s in p.sheets.iter() {
                            ui.label(&*s.title);
                        }
                    });
                }
            });

        self.active_windows.render(ctx);
		self.active_windows.prune();
    }
}