use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
  #[default]
  Main,
  SelectingBodyInSystemTree,
  SelectingResearchField,
  SelectingResearch,
  SelectingColony,
  SelectingBuilding,
  SystemMapNavigation
}
