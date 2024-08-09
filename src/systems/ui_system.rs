use bevy::prelude::ResMut;
use bevy_ratatui::terminal::RatatuiContext;
use ratatui::text::Line;

use crate::ui::tab_menu::render_tab_menu;

pub fn ui_system(mut context: ResMut<RatatuiContext>) -> color_eyre::Result<()> {
    context.draw(|frame| {
        let area = frame.size();
        let hw = Line::from("Hello World!").centered();
        frame.render_widget(hw, area);
        render_tab_menu(frame, area);
    })?;

    Ok(())
}
