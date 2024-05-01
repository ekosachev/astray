use std::cmp::min;

use derive_getters::Getters;

use crate::game::colony::building::BuildingType;

#[derive(Getters)]
pub struct ConstructionProcess {
    building_type: BuildingType,
    progress: u32,
    construction_time: u32,
}

impl From<BuildingType> for ConstructionProcess {
    fn from(value: BuildingType) -> Self {
        Self {
            building_type: value.clone(),
            progress: 0,
            construction_time: value.get_construction_time(),
        }
    }
}

impl ConstructionProcess {
    pub fn update(&mut self, construction_speed: u32) -> bool {
        self.progress = min(
            self.progress + construction_speed,
            self.construction_time,
        );

        self.progress == self.construction_time
    }
}