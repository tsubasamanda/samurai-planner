use crate::window::Window;
use crate::project::sheet::Sheet;

pub struct SheetWindow {
    alive: bool,
    sheet: Sheet
}

impl SheetWindow {
    pub fn new(s: Sheet) -> SheetWindow {
        SheetWindow {
            alive: true,
            sheet: s
        }
    }
}

impl Window for SheetWindow {
    fn window(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.sheet.title.clone())
            .open(&mut self.alive)
            .show(ctx, |ui| {

            });
    }

    fn alive(&self) -> bool {
        true
    }
}