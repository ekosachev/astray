use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::game::resource::resource::{ResourceTransaction, ResourceType};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]

pub struct ResourceManager {
    resources: HashMap<ResourceType, u32>,
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self {
            resources: HashMap::from([
                // Primary resources
                (ResourceType::PRLightMetals, 0),
                (ResourceType::PRHeavyMetals, 0),
                (ResourceType::PRPreciousMetals, 0),
                (ResourceType::PRWater, 0),
                (ResourceType::PRCrudeOil, 0),
                (ResourceType::PRSilicon, 0),

                // Secondary resources
                (ResourceType::SRKerosene, 0),
                (ResourceType::SRElectronics, 0),
                (ResourceType::SRPlastic, 0),
                (ResourceType::SRSuperconductors, 0),
                (ResourceType::SRHeatResistantAlloys, 0),
                (ResourceType::SRComposites, 0),
                (ResourceType::SRRadioactivePellets, 0),

                // Components
                (ResourceType::CEngineNozzles, 0),
                (ResourceType::CMicroprocessors, 0),
                (ResourceType::CSensors, 0),
                (ResourceType::CFuelRods, 0),
            ])
        }
    }
}

impl ResourceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_applicable(&self, transaction: &ResourceTransaction) -> bool {
        let current_amount = *self.resources.get(transaction.resource_type())
            .unwrap_or(&0) as i32;

        (transaction.amount() >= &0) || (current_amount >= transaction.amount().abs())
    }

    pub fn apply(&mut self, transaction: ResourceTransaction) {
        if self.is_applicable(&transaction) {
            let current_amount = *self.resources.get(transaction.resource_type())
                .unwrap_or(&0) as i32;

            self.resources.insert(
                transaction.resource_type().clone(),
                (current_amount + transaction.amount()) as u32,
            );
        }
    }

    pub fn apply_many(&mut self, transactions: Vec<ResourceTransaction>) {
        if !transactions.iter().all(|rt| self.is_applicable(rt)) {
            return
        }

        transactions.iter().for_each(
            |rt| {
                self.apply(rt.clone())
            }
        )
    }
}