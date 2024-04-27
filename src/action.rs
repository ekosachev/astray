use std::collections::HashMap;

use ratatui::style::Color;
use serde::{
  de::{Deserializer, Visitor},
  Deserialize, Serialize,
};
use strum::Display;

use crate::game::celestial_bodies::solar_system::SolarSystem;
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
  StartSelecting,
  ContinueSelecting,
  SelectNext,
  SelectPrevious,
  Select,
  UpdateObjectView,

  LoadTabs(Vec<Tabs>),

  LoadResearchFields(Vec<ResearchField>),

  LoadResearches(Vec<Research>),
  ScheduleLoadResearchesForField(ResearchField),

  LoadResearchesForField(Vec<Research>),
  ScheduleLoadResearchInfo(Research),

  LoadResearchInfo(HashMap<String, String>),
  LoadDependencyInfo(Vec<Vec<(String, bool)>>),

  LoadResearchColors(Vec<Color>),

  NavigateNextTab,
  NavigatePrevTab,

  LoadSystemView(SolarSystem),

  MainAction,

  StartResearch(Research)
}
