use std::cmp::Ordering;

use crate::items::ItemStats;
use crate::profile::{Ability, AbilityEffectTurns};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Condition {
    Poor,
    Normal,
    Good,
    Excellent,
}

#[derive(Debug, Clone)]
pub struct SimulationConstants {
    pub class_level: u32,
    pub craftmanship: u32,
    pub control: u32,
    pub cp: u32,
    pub available_abilities: Vec<Ability>,

    pub item: ItemStats,
    pub max_steps: u8,
    pub accept_first_of_length: u8,
}

impl SimulationConstants {
    pub fn base_progress(&self) -> u32 {
        // Progress = round ( (1 + 0.05 * recipe level difference) * (0.21 * Craftsmanship + 1.6) )
        /*
            For every level a recipe is above you, up to 5 above, you take a 10% penalty to progress.
            For every level a recipe is below you, up to 5 below, you gain a 5% bonus to progress.
            For every addition level greater than 5 level below you, you gain another 2-2.5% (needs refinement) bonus to progress, until 15 levels below you.
            After 15 levels, it continues to gradually increase until around 55% bonus
        */
        // https://www.bluegartr.com/threads/117684-The-crafting-thread

        (1.0 + 0.05
            * ((self.class_level as i32 - self.item.level as i32) as f32)
            * (0.21 * (self.craftmanship as f32) + 1.6)) as u32
    }
}

#[derive(Debug, Clone)]
pub struct SimulationStep {
    pub ability: &'static str,

    pub condition: Condition,
    pub action_success: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct SimulationRun {
    pub steps: Vec<SimulationStep>,

    pub quality: u32,
    pub progress: u32,
}

impl SimulationRun {
    pub fn compare(&self, other: &Self, target_quality: u32) -> Ordering {
        // Sort by target quality vs not target quality, then shortest to longest, then quality level
        let a_tq = self.quality >= target_quality;
        let b_tq = other.quality >= target_quality;

        // Check target quality status
        if a_tq != b_tq {
            return self.quality.cmp(&other.quality);
        }
        // If both have the same hq status sort by length with shorter being better
        if self.steps.len() != other.steps.len() {
            if self.steps.len() < other.steps.len() {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        // If same target quality status and length sort by quality
        self.quality.cmp(&other.quality)
    }
}

#[derive(Debug, Clone)]
pub struct SimulationVariables {
    pub remaining_durability: i32,
    pub remaining_cp: u32,
    pub current_progress: u32,
    pub current_quality: u32,

    pub current_state_turns: Vec<AbilityEffectTurns>,
    pub inner_quiet_stack: u32,

    pub lowest_target_quality_steps: u8,
}

impl SimulationVariables {
    pub fn new(constants: &SimulationConstants) -> SimulationVariables {
        SimulationVariables {
            remaining_durability: constants.item.initial_durability as i32,
            remaining_cp: constants.cp,
            current_progress: 0,
            current_quality: 0,

            current_state_turns: vec![],
            inner_quiet_stack: 0,

            lowest_target_quality_steps: constants.max_steps + 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StepVariables {
    pub condition: Condition,

    pub step_number: u8,
    pub base_progress: u32,
    pub base_quality: u32,
    pub durability_scale: f32,
    pub progress_floor: bool,
}
