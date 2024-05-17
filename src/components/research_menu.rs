use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets;
use ratatui::widgets::{Block, Borders, BorderType, ListDirection, ListState, Paragraph};

use crate::action::Action;
use crate::components::Component;
use crate::components::utils::widget_utils;
use crate::tabs::Tabs;
use crate::tui::Frame;

pub struct ResearchMenu {
    is_initialised: bool,
    field_list_state: ListState,
    research_list_state: ListState,
    field_list: Vec<(String, String, Color)>,
    research_list: Vec<(String, String, Color)>,
    research_selected: Option<String>,
    field_list_focused: bool,
    research_list_focused: bool,
    info: Vec<Vec<String>>,
    dependency_info: Option<Vec<Vec<(String, bool)>>>,
    research_progress: u32,
    gauge_text: String,
}

impl Default for ResearchMenu {
    fn default() -> Self {
        let mut field_list_state = ListState::default();
        field_list_state.select(Some(0));
        let mut research_list_state = ListState::default();
        research_list_state.select(Some(0));

        Self {
            is_initialised: false,
            field_list_state,
            research_list_state,
            field_list: Vec::new(),
            research_list: Vec::new(),
            research_selected: None,
            field_list_focused: false,
            research_list_focused: false,
            info: Vec::new(),
            dependency_info: None,
            research_progress: 0,
            gauge_text: String::from("")
        }
    }
}

impl Component for ResearchMenu {
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        if !self.is_initialised {
            self.is_initialised = true;
            return Ok(Some(Action::InitResearch))
        }

        match action {
            Action::Tick => {
                if let Some(r) = self.research_selected.clone() {
                    return Ok(
                        Some(
                            Action::ScheduleLoadResearchInfo(r)
                        )
                    )
                }
            },

            Action::LoadResearchFields(fields) => {
                self.field_list = fields;
            }

            Action::LoadDependencyInfo(info) => {
                self.dependency_info = Some(info)
            }
            
            Action::StartSelecting => {
                self.field_list_focused = true;
            }
            
            Action::SelectPrevious => {
                if self.field_list_focused {
                    self.field_list_state.select(Some(
                        widget_utils::select_prev_in_list(
                            self.field_list_state.selected().unwrap(),
                            self.field_list.len()
                        )
                    ))
                } else if self.research_list_focused {
                    self.research_list_state.select(Some(
                        widget_utils::select_prev_in_list(
                            self.research_list_state.selected().unwrap(),
                            self.research_list.len()
                        )
                    ))
                }
            }
            
            Action::SelectNext => {
                if self.field_list_focused {
                    self.field_list_state.select(Some(
                        widget_utils::select_next_in_list(
                            self.field_list_state.selected().unwrap(),
                            self.field_list.len()
                        )
                    ))
                } else if self.research_list_focused {
                    self.research_list_state.select(Some(
                        widget_utils::select_next_in_list(
                            self.research_list_state.selected().unwrap(),
                            self.research_list.len()
                        )
                    ))
                }
            }
            
            Action::ContinueSelecting => {
                self.research_list_focused = true;
                self.field_list_focused = false;
                self.research_list_state.select(Some(0));
                return Ok(Some(
                    Action::ScheduleLoadResearchesForField(
                        self.field_list[self.field_list_state.selected().unwrap()].clone().0
                    )
                ))
            }
            
            Action::Select => {
                self.research_list_focused = false;

                if !self.research_list.is_empty() {
                    self.research_selected = Some(
                        self.research_list[self.research_list_state.selected().unwrap()].clone().0
                    );

                    return Ok(Some(
                        Action::ScheduleLoadResearchInfo(
                            self.research_selected.clone().unwrap()
                        )
                    ))
                }
            }
            
            Action::LoadResearchesForField(researches) => {
                self.research_list = researches;
            }

            Action::LoadResearchProgress(progress) => {
                self.research_progress = progress
            }
            
            Action::LoadResearchInfo(info) => {
                self.info = info;
            }

            Action::LoadResearchProgressText(text) => {
                self.gauge_text = text;
            }

            Action::MainAction => {
                if let Some(r) = self.research_selected.clone() {
                    return Ok(
                        Some(
                            Action::StartResearch(r)
                        )
                    )
                }
            }
            
            _ => {}
        }
        
        Ok(None)
    }
    
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

        let fields_list = widgets::List::new(self.field_list.iter().map(
            |(id, name, color)| {
                Line::styled(
                    name,
                    Style::default().fg(*color),
                )
            }
        ))
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

        let research_list = widgets::List::new(
            self.research_list.iter().map(
                |(i, r, c)| {
                    Line::styled(
                        r,
                        Style::default().fg(*c),
                    )
                }
            )
        )
            .block(
                Block::default()
                    .title("Researches")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(
                        if self.research_list_focused {
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
        f.render_stateful_widget(research_list, chunks[1], &mut self.research_list_state);

        let default_text = "No tech selected".to_string();

        let info_chunks = Layout::new(
            Direction::Vertical,
            vec![
                Fill(1),
                Length(5),
                Length(3),
            ],
        ).split(chunks[2]);


        let mut info_text: Vec<Line> = self.info.iter().map(
            |x| {
                Line::from(x[0].clone())
            }
        ).collect();

        if let Some(d) = self.dependency_info.clone() {
            info_text.push(Line::from(""));
            info_text.push(
                Line::from("All of these technologies must be researched:")
            );
            // All of
            d[0].iter().for_each(
                |(name, is_finished)| {
                    info_text.push(
                        Line::from(
                            if *is_finished {
                                Span::styled(
                                    format!("    ✓ {name}"),
                                    Style::default().fg(Color::LightGreen),
                                )
                            } else {
                                Span::styled(
                                    format!("    ✘ {name}"),
                                    Style::default().fg(Color::LightRed),
                                )
                            }
                        )
                    )
                }
            );

            info_text.push(Line::from(""));

            info_text.push(
                Line::from("At least one of these technologies must be researched:")
            );

            // All of
            d[1].iter().for_each(
                |(name, is_finished)| {
                    info_text.push(
                        Line::from(
                            if *is_finished {
                                Span::styled(
                                    format!("    ✓ {name}"),
                                    Style::default().fg(Color::LightGreen),
                                )
                            } else {
                                Span::styled(
                                    format!("    ✘ {name}"),
                                    Style::default().fg(Color::LightRed),
                                )
                            }
                        )
                    )
                }
            );
        }

        let info = widgets::Paragraph::new(info_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            );

        let research_progress = widgets::Gauge::default()
            .block(
                Block::default()
                    .title("Research Progress")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            )
            .gauge_style(
                match self.research_progress {
                    0..=33 => { Color::Red },
                    34..=67 => { Color::Yellow },
                    68..=99 => { Color::Green },
                    100 => { Color::Cyan },
                    _ => { Color::Red },
                }
            )
            .percent(
                self.research_progress as u16
            )
            .label(self.gauge_text.clone());


        f.render_widget(info, info_chunks[0]);
        f.render_widget(research_progress, info_chunks[1]);

        let help_key_style = Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD);

        let help = Paragraph::new(
            match (self.research_list_focused, self.field_list_focused) {
                (false, false) => "Press <Alt+S> to select a research, <Alt+R> to start \
                researching selected tech",
                (true, false) => "Use arrows to highlight a research, then press <Enter> to select \
                it",
                (false, true) => "Use arrows to highlight a research field and <Enter> to select \
                it",
                (true, true) => "This is a bug! Thanks for catching it!",
            }
        ).block(
            Block::default()
                .title("Controls help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        );

        f.render_widget(help, info_chunks[2]);

        Ok(())
    }

    fn is_drawn_in_tab(&self, tab: &Tabs) -> bool {
        *tab == Tabs::Research
    }
}