use std::{fmt, string::ToString};

use serde::{
  de::{self, Deserializer, Visitor},
  Deserialize, Serialize,
};
use strum::Display;
use crate::game::celestial_bodies::CelestialBody;
use crate::game::celestial_bodies::planet::Planet;
use crate::game::celestial_bodies::star::Star;

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
  UpdateObjectView
}
