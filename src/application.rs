use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{ActiveTheme, Root, TitleBar, scroll::ScrollableElement};

use crate::ui;


pub struct AppState {
    pub(crate) active_project: crate::project::project::Project
}

impl gpui::Global for AppState {}

pub struct Application;

impl Render for Application {
	fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = cx.global::<AppState>();

		div()
            .bg(cx.theme().background)
            .size_full()

            //.child(crate::ui::titlebar::titlebar())
            //.child(crate::ui::project_bar::project_bar())
            //.child(crate::ui::inspector::inspector())

            // Internal content
            .child(
                div()
                    .overflow_y_scrollbar()
                    .children(app_state.active_project.actions.actions.iter().map(|a| {
                        a.render()
                    }))
            )

            // Fill dialog layers
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window ,cx))
	}
}