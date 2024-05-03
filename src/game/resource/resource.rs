use derive_getters::Getters;
use rand::{prelude::*, Rng, thread_rng};
use rand::distributions::WeightedIndex;
use serde::{Deserialize, Serialize};

use crate::game::colony::building::{BuildingType, FactoryType};

pub enum ResourceGrade {
    Primary,
    Secondary,
    Component,
}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub enum ResourceType {
    // Primary resources
    PRLightMetals,
    PRHeavyMetals,
    PRPreciousMetals,
    PRWater,
    PRCrudeOil,
    PRSilicon,

    // Secondary resources
    SRKerosene,
    SRElectronics,
    SRPlastic,
    SRSuperconductors,
    SRHeatResistantAlloys,
    SRComposites,
    SRRadioactivePellets,

    // Components
    CEngineNozzles,
    CMicroprocessors,
    CSensors,
    CFuelRods,
}

impl Into<ResourceGrade> for ResourceType {
    fn into(self) -> ResourceGrade {
        match self {
            ResourceType::PRLightMetals => ResourceGrade::Primary,
            ResourceType::PRHeavyMetals => ResourceGrade::Primary,
            ResourceType::PRPreciousMetals => ResourceGrade::Primary,
            ResourceType::PRWater => ResourceGrade::Primary,
            ResourceType::PRCrudeOil => ResourceGrade::Primary,
            ResourceType::PRSilicon => ResourceGrade::Primary,

            ResourceType::SRKerosene => ResourceGrade::Secondary,
            ResourceType::SRElectronics => ResourceGrade::Secondary,
            ResourceType::SRPlastic => ResourceGrade::Secondary,
            ResourceType::SRSuperconductors => ResourceGrade::Secondary,
            ResourceType::SRHeatResistantAlloys => ResourceGrade::Secondary,
            ResourceType::SRComposites => ResourceGrade::Secondary,
            ResourceType::SRRadioactivePellets => ResourceGrade::Secondary,

            ResourceType::CEngineNozzles => ResourceGrade::Component,
            ResourceType::CMicroprocessors => ResourceGrade::Component,
            ResourceType::CSensors => ResourceGrade::Component,
            ResourceType::CFuelRods => ResourceGrade::Component,
        }
    }
}

#[derive(Getters, Clone)]
pub struct ResourceTransaction {
    resource_type: ResourceType,
    amount: i32,
}

impl ResourceTransaction {
    pub fn new(resource_type: ResourceType, amount: i32) -> Self {
        Self { resource_type, amount }
    }
}

impl From<BuildingType> for Option<Vec<ResourceTransaction>> {
    fn from(value: BuildingType) -> Self {
        match value {
            BuildingType::Factory(factory_type) => {
                Some(factory_type.into())
            }
            _ => { None }
        }
    }
}

impl Into<Vec<ResourceTransaction>> for FactoryType {
    fn into(self) -> Vec<ResourceTransaction> {
        match self {

            // Secondary resources
            FactoryType::ElectronicsFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::PRPreciousMetals, -1),
                    ResourceTransaction::new(ResourceType::PRSilicon, -5),
                    ResourceTransaction::new(ResourceType::SRElectronics, 3),
                ]
            }
            FactoryType::KeroseneFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::PRCrudeOil, -10),
                    ResourceTransaction::new(ResourceType::SRKerosene, 5),
                ]
            }
            FactoryType::HeatResistantAlloyFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::PRHeavyMetals, -3),
                    ResourceTransaction::new(ResourceType::PRPreciousMetals, -2),
                    ResourceTransaction::new(ResourceType::SRHeatResistantAlloys, 4),
                ]
            }
            FactoryType::SuperconductorsFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::PRPreciousMetals, -4),
                    ResourceTransaction::new(ResourceType::SRSuperconductors, 1),
                ]
            }
            FactoryType::PlasticsFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::PRCrudeOil, -7),
                    ResourceTransaction::new(ResourceType::SRPlastic, 2),
                ]
            }
            FactoryType::CompositesFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::SRPlastic, -5),
                    ResourceTransaction::new(ResourceType::PRLightMetals, -5),
                    ResourceTransaction::new(ResourceType::SRComposites, 8),
                ]
            }
            FactoryType::RadioactivePelletsFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::PRHeavyMetals, -2),
                    ResourceTransaction::new(ResourceType::PRPreciousMetals, -1),
                    ResourceTransaction::new(ResourceType::SRRadioactivePellets, 1),
                ]
            }

            // Ship components
            FactoryType::EngineNozzlesFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::SRHeatResistantAlloys, -15),
                    ResourceTransaction::new(ResourceType::CEngineNozzles, 1),
                ]
            }
            FactoryType::MicroprocessorsFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::SRSuperconductors, -1),
                    ResourceTransaction::new(ResourceType::SRElectronics, -4),
                    ResourceTransaction::new(ResourceType::CMicroprocessors, 2),
                ]
            }
            FactoryType::SensorsFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::PRPreciousMetals, -2),
                    ResourceTransaction::new(ResourceType::SRElectronics, -3),
                    ResourceTransaction::new(ResourceType::CSensors, 1),
                ]
            }
            FactoryType::FuelRodsFactory => {
                vec![
                    ResourceTransaction::new(ResourceType::PRLightMetals, -3),
                    ResourceTransaction::new(ResourceType::SRRadioactivePellets, -4),
                    ResourceTransaction::new(ResourceType::CFuelRods, 2),
                ]
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ResourceDeposit {
    amounts: Vec<(ResourceType, i32)>,
}

impl ResourceDeposit {
    pub fn generate_for_planet() -> Self {
        let mut values = [
            (ResourceType::PRLightMetals, 1),
            (ResourceType::PRHeavyMetals, 1),
            (ResourceType::PRPreciousMetals, 1),
            (ResourceType::PRWater, 1),
            (ResourceType::PRCrudeOil, 1),
            (ResourceType::PRSilicon, 1),
        ];

        let mut rng = thread_rng();

        for _ in 0..(100 - values.len()) {
            let index = match rng.gen_range(1..=100) {
                1..=20 => 0,
                21..=30 => 1,
                31..=35 => 2,
                36..=60 => 3,
                61..=70 => 4,
                71..=100 => 5,

                _ => unreachable!()
            };

            values[index].1 += 1;
        }

        Self {
            amounts: Vec::from(values)
        }
    }

    pub fn sample(&self) -> ResourceType {
        let mut rng = thread_rng();

        let weights: Vec<i32> = self.amounts.iter().map(
            |(rt, w)| { w.clone() }
        ).collect();

        let choices: Vec<ResourceType> = self.amounts.iter().map(
            |(rt, w)| { rt.clone() }
        ).collect();

        let mut dist = WeightedIndex::new(
            weights
        ).unwrap();

        choices[dist.sample(&mut rng)].clone()
    }
}