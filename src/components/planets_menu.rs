use ratatui::layout::Rect;
use ratatui::widgets::ListState;

use crate::action::Action;
use crate::components::Component;
use crate::game::celestial_bodies::planet::Planet;
use crate::tabs::Tabs;
use crate::tui::Frame;

pub struct PlanetsMenu {
    planets: Vec<Planet>,
    list_state: ListState,
}

impl Default for PlanetsMenu {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            planets: Vec::new(),
            list_state: state,
        }
    }
}

impl Component for PlanetsMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {}
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::Planets
    }
}