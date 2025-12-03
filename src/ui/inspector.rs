use gpui_component::{Side, sidebar::{Sidebar, SidebarGroup, SidebarMenu}};

pub fn inspector() -> Sidebar<SidebarGroup<SidebarMenu>> {
    return Sidebar::new(Side::Right)
        .child(
            SidebarGroup::new("Gear Options")
            .child(
                SidebarMenu::new()
            )
        )
}