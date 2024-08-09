use bevy::prelude::{Res, ResMut};
use bevy_ratatui::terminal::RatatuiContext;

use crate::ui::tab_menu::{render_tab_menu, TabMenu};

pub fn ui_system(
    mut context: ResMut<RatatuiContext>,
    tab_menu: Res<TabMenu>
) -> color_eyre::Result<()> {
    context.draw(|frame| {
        let area = frame.size();
        render_tab_menu(frame, area, &tab_menu);
    })?;

    Ok(())
}
