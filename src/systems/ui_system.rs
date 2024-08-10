use bevy::prelude::{Res, ResMut, State};
use bevy_ratatui::terminal::RatatuiContext;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::{ui::{system::body_list::{render_body_list, BodyList}, tab_menu::{render_tab_menu, TabMenu}}, Tab};

pub fn ui_system(
    mut context: ResMut<RatatuiContext>,
    state: Res<State<Tab>>,
    tab_menu: Res<TabMenu>,
    body_list: Res<BodyList>
) -> color_eyre::Result<()> {
    context.draw(|frame| {
        let area = frame.size();
        let primary_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(2),
                Constraint::Fill(1),
            ]
        ).split(area);
        render_tab_menu(frame, primary_chunks[0], &tab_menu);
        match state.get() {
            Tab::System => {
                let secondary_chunks = Layout::new(
                    Direction::Horizontal,
                    vec![
                        Constraint::Percentage(30),
                        Constraint::Percentage(70),
                    ]
                ).split(primary_chunks[1]);
                render_body_list(frame, secondary_chunks[0], &body_list);
            }
            _ => {}
        }
    })?;

    Ok(())
}
