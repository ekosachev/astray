use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets;
use ratatui::widgets::{Block, Borders, BorderType, ListDirection, ListState};

use crate::components::Component;
use crate::game::research::Research;
use crate::tabs::Tabs;
use crate::tui::Frame;

pub struct ResearchMenu {
    field_list_state: ListState,
    research_list_state: ListState,
    field_list: Vec<String>,
    research_list: Vec<Research>,
    field_list_focused: bool,
    research_list_focused: bool,
}

impl Default for ResearchMenu {
    fn default() -> Self {
        let mut field_list_state = ListState::default();
        field_list_state.select(Some(0));
        let mut research_list_state = ListState::default();
        research_list_state.select(Some(0));

        Self {
            field_list_state,
            research_list_state,
            field_list: Vec::new(),
            research_list: Vec::new(),
            field_list_focused: false,
            research_list_focused: false,
        }
    }
}

impl Component for ResearchMenu {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::Result<()> {
        let v_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Constraint::Length(3),
                Constraint::Min(0),
            ],
        ).split(area);

        let chunks = Layout::new(
            Direction::Horizontal,
            vec![
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ],
        ).split(v_chunks[1]);

        let fields_list = widgets::List::new(self.field_list.clone())
            .block(
                Block::default()
                    .title("Fields")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        if self.field_list_focused {
                            Style::default().fg(Color::LightBlue)
                        } else {
                            Style::default()
                        }
                    )
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(false)
            .direction(ListDirection::TopToBottom);

        f.render_stateful_widget(fields_list, chunks[0], &mut self.field_list_state);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::Research
    }
}