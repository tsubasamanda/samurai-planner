use gpui::{App, ClickEvent, Window};
use gpui_component::WindowExt;

pub fn about_dialog(click_event: &ClickEvent, window: &mut Window, app: &mut App) {
    window.open_dialog(app, |dialog, _, _| {
        dialog.title("Test")
    });
}