use bevy::prelude::{EventReader, NextState, Res, ResMut, State};
use crate::{InputEvent, Tab};
use crate::ui::tab_menu::TabMenu;

pub fn tab_system(
    mut tab_menu: ResMut<TabMenu>,
    mut events: EventReader<InputEvent>,
    mut cur_tab: Res<State<Tab>>,
    mut tab: ResMut<NextState<Tab>>,
) {
    for event in events.read() {
        match event {
            InputEvent::NextTab => {
                tab_menu.selected_tab += 1;
                tab_menu.selected_tab %= tab_menu.tab_titles.len();
                tab.set(cur_tab.get().next())
            }
            InputEvent::PrevTab => {
                tab_menu.selected_tab += tab_menu.tab_titles.len() - 1;
                tab_menu.selected_tab %= tab_menu.tab_titles.len();
                tab.set(cur_tab.get().prev())
            }
        }
    }
}