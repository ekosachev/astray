use bevy::prelude::{EventReader, ResMut};
use crate::InputEvent;
use crate::ui::tab_menu::TabMenu;

pub fn tab_system(
    mut tab_menu: ResMut<TabMenu>,
    mut events: EventReader<InputEvent>,
) {
    for event in events.read() {
        match event {
            InputEvent::NextTab => {
                tab_menu.selected_tab += 1;
                tab_menu.selected_tab %= tab_menu.tab_titles.len();
            }
            InputEvent::PrevTab => {
                tab_menu.selected_tab += tab_menu.tab_titles.len() - 1;
                tab_menu.selected_tab %= tab_menu.tab_titles.len();
            }
        }
    }
}