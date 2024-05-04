use std::collections::HashMap;

use ratatui::style::Color;
use serde::{
  de::{Deserializer, Visitor},
  Deserialize, Serialize,
};
use strum::Display;

use crate::game::celestial_bodies::solar_system::SolarSystem;
use crate::game::colony::colony::Colony;
use crate::game::research::{Research, ResearchField};
use crate::tabs::Tabs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Action {
  Tick,
  IngameTick,
  Render,
  Resize(u16, u16),
  Suspend,
  Resume,
  Quit,
  Refresh,
  Error(String),
  Help,
  UpdateObjectView,

  // Loading data
  LoadTabs(Vec<Tabs>),
  LoadResearchFields(Vec<ResearchField>),
  LoadResearches(Vec<Research>),
  LoadResearchesForField(Vec<Research>),
  LoadResearchInfo(HashMap<String, String>),
  LoadDependencyInfo(Vec<Vec<(String, bool)>>),
  LoadResearchColors(Vec<Color>),
  LoadSystemView(SolarSystem),
  LoadColonies(Vec<Colony>),

  // Scheduling
  ScheduleLoadResearchesForField(ResearchField),
  ScheduleLoadResearchInfo(Research),

  // Navigation
  NavigateNextTab,
  NavigatePrevTab,

  // Form actions
  StartSelecting,
  ContinueSelecting,
  SelectNext,
  SelectPrevious,
  Select,

  // Tab actions
  MainAction,
  StartResearch(Research)
}
