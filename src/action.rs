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
  Render,
  Resize(u16, u16),
  Suspend,
  Resume,
  Quit,
  Refresh,
  Error(String),
  Help,
  SelectBodyInSystemTree,
  SelectNext,
  SelectPrevious,
  Select,
  UpdateObjectView,
  LoadTabs(Vec<Tabs>),
  NavigateNextTab,
  NavigatePrevTab,
  LoadSystemView(SolarSystem),
}
