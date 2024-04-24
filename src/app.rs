use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
  action::Action,
  components::{Component, fps::FpsCounter},
  config::Config,
  mode::Mode,
  tui,
};
use crate::components::research_menu::ResearchMenu;
use crate::components::system_menu::SystemMenu;
use crate::components::top_menu::TopMenu;
use crate::game::game_state::GameState;
use crate::tabs::Tabs;

pub struct App {
  pub config: Config,
  pub tick_rate: f64,
  pub frame_rate: f64,
  pub components: Vec<Box<dyn Component>>,
  pub should_quit: bool,
  pub should_suspend: bool,
  pub mode: Mode,
  pub last_tick_key_events: Vec<KeyEvent>,
  pub state: GameState,
  tabs: Vec<Tabs>,
  cur_tab: usize,
}

impl App {
  pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
    let fps = FpsCounter::default();
    let system_tree = SystemMenu::default();
    let research_menu = ResearchMenu::default();
    let top_menu = TopMenu::default();
    let config = Config::new()?;
    let mode = Mode::Main;
    Ok(Self {
      tick_rate,
      frame_rate,
      components: vec![
        Box::new(top_menu),
        Box::new(system_tree),
        Box::new(research_menu),
        Box::new(fps),
      ],
      should_quit: false,
      should_suspend: false,
      config,
      mode,
      last_tick_key_events: Vec::new(),
      state: GameState::new(),
      tabs: vec![
        Tabs::SystemView,
        Tabs::Research,
      ],
      cur_tab: 0,
    })
  }

  pub async fn run(&mut self) -> Result<()> {
    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    // Preload tasks
    action_tx.send(Action::LoadSystemView(self.state.get_starting_system()))?;
    action_tx.send(Action::LoadTabs(self.tabs.clone()))?;

    let mut tui = tui::Tui::new()?.tick_rate(self.tick_rate).frame_rate(self.frame_rate);
    // tui.mouse(true);
    tui.enter()?;

    for component in self.components.iter_mut() {
      component.register_action_handler(action_tx.clone())?;
    }

    for component in self.components.iter_mut() {
      component.register_config_handler(self.config.clone())?;
    }

    for component in self.components.iter_mut() {
      component.init(tui.size()?)?;
    }

    loop {
      if let Some(e) = tui.next().await {
        match e {
          tui::Event::Quit => action_tx.send(Action::Quit)?,
          tui::Event::Tick => action_tx.send(Action::Tick)?,
          tui::Event::Render => action_tx.send(Action::Render)?,
          tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
          tui::Event::Key(key) => {
            if let Some(keymap) = self.config.keybindings.get(&self.mode) {
              if let Some(action) = keymap.get(&vec![key]) {
                log::info!("Got action: {action:?}");
                action_tx.send(action.clone())?;
              } else {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                  log::info!("Got action: {action:?}");
                  action_tx.send(action.clone())?;
                }
              }
            };
          },
          _ => {},
        }
        for component in self.components.iter_mut() {
          if let Some(action) = component.handle_events(Some(e.clone()))? {
            action_tx.send(action)?;
          }
        }
      }

      while let Ok(action) = action_rx.try_recv() {
        if action != Action::Tick && action != Action::Render {
          log::debug!("{action:?}");
        }
        match action {
          Action::Tick => {
            self.last_tick_key_events.drain(..);
          },
          Action::Quit => self.should_quit = true,
          Action::Suspend => self.should_suspend = true,
          Action::Resume => self.should_suspend = false,
          Action::NavigateNextTab => {
            self.cur_tab += 1;
            self.cur_tab %= self.tabs.len();
          }
          Action::NavigatePrevTab => {
            if self.cur_tab != 0 {
              self.cur_tab -= 1;
            } else {
              self.cur_tab = self.tabs.len() - 1;
            }
          }
          Action::Resize(w, h) => {
            tui.resize(Rect::new(0, 0, w, h))?;
            tui.draw(|f| {
              for component in self.components.iter_mut() {
                if component.is_drawn_in_tab(&self.tabs[self.cur_tab]) {
                  let r = component.draw(f, f.size());
                  if let Err(e) = r {
                    action_tx.send(Action::Error(format!("Failed to draw: {:?}", e))).unwrap();
                  }
                }
              }
            })?;
          },
          Action::Render => {
            tui.draw(|f| {
              for component in self.components.iter_mut() {
                if component.is_drawn_in_tab(&self.tabs[self.cur_tab]) {
                  let r = component.draw(f, f.size());
                  if let Err(e) = r {
                    action_tx.send(Action::Error(format!("Failed to draw: {:?}", e))).unwrap();
                  }
                }
              }
            })?;
          },
          Action::SelectBodyInSystemTree => {
            self.mode = Mode::SelectingBodyInSystemTree;
          }
          Action::Select => {
            self.mode = Mode::Main;
          }
          _ => {},
        }
        for component in self.components.iter_mut() {
          if let Some(action) = component.update(action.clone())? {
            action_tx.send(action)?
          };
        }
      }
      if self.should_suspend {
        tui.suspend()?;
        action_tx.send(Action::Resume)?;
        tui = tui::Tui::new()?.tick_rate(self.tick_rate).frame_rate(self.frame_rate);
        // tui.mouse(true);
        tui.enter()?;
      } else if self.should_quit {
        tui.stop()?;
        break;
      }
    }
    tui.exit()?;
    Ok(())
  }
}
