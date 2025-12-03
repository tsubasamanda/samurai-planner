use gpui_component::{Side, sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarToggleButton
}};
use gpui::{Context, ParentElement};

use crate::application::Application;

pub fn project_bar() -> Sidebar<SidebarMenu> {
    return Sidebar::new(Side::Left)
        .child(
            SidebarMenu::new()
        )
        .footer(
            SidebarFooter::new()
                .child(
                    SidebarMenuItem::new(format!("About Version {}", env!("CARGO_PKG_VERSION")))
                        .on_click(crate::ui::about::about_dialog)
                )
        )
}