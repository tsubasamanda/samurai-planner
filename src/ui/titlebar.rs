use gpui::div;
use gpui::ParentElement;
use gpui_component::TitleBar;

pub fn titlebar() -> TitleBar {
    return TitleBar::new()
        .child(
            div()
                .child("Amanda's Samurai Planner")
            );
}