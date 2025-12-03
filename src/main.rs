use std::path::PathBuf;

use gpui::{AppContext, Bounds, SharedString, TitlebarOptions, WindowBounds, WindowDecorations, WindowOptions, px, size};
use gpui::Application as GPUIApplication;
use gpui_component::{Root, Theme, ThemeRegistry};

mod application;
mod ui;
mod project;
mod game;
mod debug;
use crate::application::*;
use crate::project::action_list::ActionList;
use crate::project::project::Project;

fn main() {
	GPUIApplication::new().run(|cx: &mut gpui::App| {
		gpui_component::init(cx);

		// Window size
		let bounds = Bounds::centered(None, size(px(1920.), px(1080.)), cx);

		// Theme management, theme is currently hardcoded
		let theme_name = SharedString::from("Catppuccin Mocha");
		if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
			if let Some(theme) = ThemeRegistry::global(cx)
				.themes()
				.get(&theme_name)
				.cloned()
			{
				Theme::global_mut(cx).apply_config(&theme);
			}
		}) {
			print!("Failed to watch themes directory: {}", err);
		}


		// Set up global app state management
		let project: Project = Project {
			actions: ActionList {
				actions: debug::action_list::random_states()
			}
		};
	
		cx.set_global(AppState {
			active_project: project,
		});

		cx.spawn(async move |cx| {
			cx.open_window(
				WindowOptions {
					titlebar: Some(TitlebarOptions {
						..Default::default()
					}),
					window_bounds: Some(WindowBounds::Windowed(bounds)),
					..Default::default()
				},
				|window, cx| {
					let view = cx.new(|_| Application);
					cx.new(|cx| Root::new(
						view,
						window,
						cx
					))
				}
			)
		}).detach();
	});
}