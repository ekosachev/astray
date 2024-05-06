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
  LoadColonies(Vec<String>),
  LoadColonyInfo(Vec<(String, Color)>),
  LoadColonyBuildings(Vec<(String, u32, Color)>),
  LoadConstructionInfo(Vec<(String, u32)>),

  // Scheduling
  ScheduleLoadResearchesForField(ResearchField),
  ScheduleLoadResearchInfo(Research),
  ScheduleLoadColonyInfo(String),
  ScheduleLoadConstructionInfo(String),

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
  StartResearch(Research),
  StartSelectingBuilding,
  StartConstruction((String /* Colony name */, String /* Building type name */))
}
