use ratatui::style::Color;
use serde::{
  de::{Deserializer, Visitor},
  Deserialize, Serialize,
};
use strum::Display;

use crate::game::celestial_bodies::solar_system::SolarSystem;
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

  // Initialising
  InitResearch,
  InitColonies,

  // Loading data
  LoadTabs(Vec<Tabs>),
  LoadResearchFields(Vec<(String, String, Color)>),
  LoadResearches(Vec<String>),
  LoadResearchesForField(Vec<(String, String, Color)>),
  LoadResearchInfo(Vec<Vec<String>>),
  LoadDependencyInfo(Vec<Vec<(String, bool)>>),
  LoadResearchProgressText(String),
  LoadResearchProgress(u32),
  LoadSystemView(SolarSystem),
  LoadColonies(Vec<String>),
  LoadColonyInfo(Vec<(String, Color)>),
  LoadColonyBuildings(Vec<(String, u32, Color)>),
  LoadConstructionInfo(Vec<(String, u32)>),

  // Scheduling
  ScheduleLoadSystemView,
  ScheduleLoadResearchesForField(String),
  ScheduleLoadResearchInfo(String),
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
  Up,
  Down,
  Left,
  Right,
  ZoomIn,
  ZoomOut,

  // Tab actions
  MainAction,
  SecondaryAction,
  EnterSystemMapNavigation,
  StartResearch(String),
  StartSelectingBuilding,
  StartConstruction((String /* Colony name */, String /* Building type name */))
}
