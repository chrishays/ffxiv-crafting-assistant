use crate::profile::{Ability, AbilityAction, AbilityEffect, AbilityType, RequiredState};

use super::random::SimulationRandomnessGenerator;
use super::state::{
    Condition, SimulationConstants, SimulationStep, SimulationVariables, StepVariables,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbilityExecutionStatus {
    NotAvailable,
    Available,
    ComboAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionExecutionStatus {
    ItemBroke,
    ItemCompleted,
    ActionCompleted,
}

fn can_afford_action(actions: &[AbilityAction], variables: &SimulationVariables) -> bool {
    for action in actions {
        match action {
            AbilityAction::CostCP(a) => {
                if *a > variables.remaining_cp {
                    return false;
                }
            }
            AbilityAction::GrantDurability(_) => {}
            AbilityAction::CostDurability(_) => {} // You can always spend more durability, it just breaks
            AbilityAction::GrantProgress(_) => {}
            AbilityAction::GrantQuality(_) => {}
            AbilityAction::GrantFullQuality => {}
            AbilityAction::GrantCP(_) => {}
            AbilityAction::GrantEffect(_) => {}
            AbilityAction::GrantInnerQuiet => {}
            AbilityAction::ConsumeInnerQuiet => {}
            AbilityAction::CostDurabilityWithDiscount(_) => {}
        }
    }

    true
}

fn can_execute_ability(
    ability: &Ability,
    constants: &SimulationConstants,
    variables: &SimulationVariables,
    step_variables: &StepVariables,
) -> AbilityExecutionStatus {
    // Check for a combo first because it can have decreased costs
    if let Some(combo) = &ability.combo_action {
        let mut can_combo = false;
        for curr_effect in &variables.current_state_turns {
            if AbilityEffect::State(combo.required_state) == curr_effect.effect {
                can_combo = true;
                break;
            }
        }

        if can_combo {
            if !can_afford_action(&combo.action, variables) {
                return AbilityExecutionStatus::NotAvailable;
            }
            return AbilityExecutionStatus::ComboAvailable;
        }
    };

    match ability.required_state {
        RequiredState::None => {}
        RequiredState::FirstStep => {
            if step_variables.step_number > 1 {
                return AbilityExecutionStatus::NotAvailable;
            }
        }
        RequiredState::FirstStepLevelAdvantage(l) => {
            if step_variables.step_number != 1 {
                return AbilityExecutionStatus::NotAvailable;
            }
            if constants.class_level < constants.item.level + l {
                return AbilityExecutionStatus::NotAvailable;
            }
        }
        RequiredState::ConditionGood => {
            if step_variables.condition != Condition::Good
                && step_variables.condition != Condition::Excellent
            {
                return AbilityExecutionStatus::NotAvailable;
            }
        }
        RequiredState::NotAbility(a) => {
            let mut has_ability = false;
            for curr_effect in &variables.current_state_turns {
                if AbilityEffect::State(a) == curr_effect.effect {
                    has_ability = true;
                    break;
                }
            }
            if has_ability {
                return AbilityExecutionStatus::NotAvailable;
            }
        }
        RequiredState::InnerQuiet(a) => {
            if variables.inner_quiet_stack < a as u32 {
                return AbilityExecutionStatus::NotAvailable;
            }
        }
    };

    if !can_afford_action(&ability.action.action, variables) {
        return AbilityExecutionStatus::NotAvailable;
    }

    AbilityExecutionStatus::Available
}

fn execute_action(
    action: &AbilityAction,
    constants: &SimulationConstants,
    variables: &mut SimulationVariables,
    step_variables: &StepVariables,
) -> ActionExecutionStatus {
    match action {
        AbilityAction::GrantDurability(a) => {
            variables.remaining_durability += *a as i32;
            if variables.remaining_durability > constants.item.initial_durability as i32 {
                variables.remaining_durability = constants.item.initial_durability as i32;
            }
        }
        AbilityAction::CostDurability(oa) => {
            let a = ((*oa as f32) * step_variables.durability_scale) as u32;
            variables.remaining_durability -= a as i32;
            if variables.remaining_durability <= 0 {
                return ActionExecutionStatus::ItemBroke;
            }
        }
        AbilityAction::GrantProgress(a) => {
            variables.current_progress += (a * (step_variables.base_progress as f32)) as u32;
            if variables.current_progress >= constants.item.required_progress {
                if step_variables.progress_floor {
                    variables.current_progress = constants.item.required_progress - 1;
                } else {
                    return ActionExecutionStatus::ItemCompleted;
                }
            }
        }
        AbilityAction::GrantQuality(a) => {
            variables.current_quality += (a * (step_variables.base_quality as f32)) as u32;
            //log::debug!("Granted quality {} of base {}", a, base_quality);
        }
        AbilityAction::GrantFullQuality => {
            variables.current_quality = constants.item.target_quality;
        }
        AbilityAction::GrantCP(a) => {
            variables.remaining_cp += a;
            if variables.remaining_cp > constants.cp {
                variables.remaining_cp = constants.cp;
            }
        }
        AbilityAction::CostCP(a) => {
            assert!(
                *a <= variables.remaining_cp,
                "Executing an action that can't be afforded"
            );
            variables.remaining_cp -= a;
        }
        AbilityAction::GrantEffect(a) => {
            // Look for duplicates and increase the number rather than duplicate it
            let mut updated_effect = false;
            for curr_effects in &mut variables.current_state_turns {
                if a.effect == curr_effects.effect {
                    // TODO: Figure out if this should be a set or add
                    curr_effects.turns += a.turns;
                    updated_effect = true;
                    break;
                }
            }
            if !updated_effect {
                variables.current_state_turns.push(*a);
            }
        }
        AbilityAction::GrantInnerQuiet => {
            variables.inner_quiet_stack += 1;
        }
        AbilityAction::ConsumeInnerQuiet => {
            assert!(
                variables.inner_quiet_stack > 0,
                "Must have Inner Quiet to consume Inner Quiet"
            );
            let mut q = 0.2 * variables.inner_quiet_stack as f32;
            if q > 3.0 {
                q = 3.0;
            }
            variables.current_quality += (q * step_variables.base_quality as f32) as u32;
            variables.inner_quiet_stack = 0;
        }
        AbilityAction::CostDurabilityWithDiscount(oa) => {
            // Is this actually impacted by the durability scale?
            let a = ((oa.cost as f32) * step_variables.durability_scale) as i32;
            let ad = ((oa.cost_if_too_expensive as f32) * step_variables.durability_scale) as i32;

            if variables.remaining_durability >= a {
                variables.remaining_durability -= a;
                if variables.remaining_durability == 0 {
                    return ActionExecutionStatus::ItemBroke;
                }
            } else {
                variables.remaining_durability -= ad;
                if variables.remaining_durability <= 0 {
                    return ActionExecutionStatus::ItemBroke;
                }
            }
        }
    };

    ActionExecutionStatus::ActionCompleted
}

pub fn use_ability(
    ability: &Ability,
    constants: &SimulationConstants,
    variables: &SimulationVariables,
    step_variables: &StepVariables,
    random_generator: &SimulationRandomnessGenerator,
) -> Result<(SimulationVariables, SimulationStep), ()> {
    let can_execute = can_execute_ability(ability, constants, variables, step_variables);

    // Early out so we don't have to clone for abilities that can't be used
    if can_execute == AbilityExecutionStatus::NotAvailable {
        return Err(());
    }

    let mut build_succeeded = false;
    let mut item_broke = false;
    let mut action_success = None;

    //log::debug!("Trying {:?} with {}", next_variables.current_options, ability.name);

    let next_variables = match can_execute {
        AbilityExecutionStatus::NotAvailable => {
            return Err(());
        }
        AbilityExecutionStatus::Available => {
            let mut next_variables = variables.clone();
            for curr_effect in &mut next_variables.current_state_turns {
                if let Some(l) = &mut curr_effect.limit {
                    if (l.apply_to & ability.ability_type) != AbilityType::NONE {
                        l.uses -= 1;
                    }
                }
            }
            for action in &ability.action.action {
                match execute_action(action, constants, &mut next_variables, step_variables) {
                    ActionExecutionStatus::ItemCompleted => {
                        build_succeeded = true;
                    }
                    ActionExecutionStatus::ItemBroke => {
                        item_broke = true;
                    }
                    ActionExecutionStatus::ActionCompleted => {}
                };
            }
            if let Some(success) = &ability.action.action_success {
                if random_generator.random_action_succeess(success.rate) {
                    action_success = Some(true);
                    for action in &success.action {
                        match execute_action(action, constants, &mut next_variables, step_variables)
                        {
                            ActionExecutionStatus::ItemCompleted => {
                                build_succeeded = true;
                            }
                            ActionExecutionStatus::ItemBroke => {
                                item_broke = true;
                            }
                            ActionExecutionStatus::ActionCompleted => {}
                        };
                    }
                } else {
                    action_success = Some(false);
                }
            }
            next_variables
        }
        AbilityExecutionStatus::ComboAvailable => {
            let mut next_variables = variables.clone();
            for curr_effect in &mut next_variables.current_state_turns {
                if let Some(l) = &mut curr_effect.limit {
                    if (l.apply_to & ability.ability_type) != AbilityType::NONE {
                        l.uses -= 1;
                    }
                }
            }
            if let Some(combo) = &ability.combo_action {
                for action in &combo.action {
                    match execute_action(action, constants, &mut next_variables, step_variables) {
                        ActionExecutionStatus::ItemCompleted => {
                            build_succeeded = true;
                        }
                        ActionExecutionStatus::ItemBroke => {
                            item_broke = true;
                        }
                        ActionExecutionStatus::ActionCompleted => {}
                    };
                }
                if let Some(success) = &combo.action_success {
                    if random_generator.random_action_succeess(success.rate) {
                        action_success = Some(true);
                        for action in &success.action {
                            match execute_action(
                                action,
                                constants,
                                &mut next_variables,
                                step_variables,
                            ) {
                                ActionExecutionStatus::ItemCompleted => {
                                    build_succeeded = true;
                                }
                                ActionExecutionStatus::ItemBroke => {
                                    item_broke = true;
                                }
                                ActionExecutionStatus::ActionCompleted => {}
                            };
                        }
                    } else {
                        action_success = Some(false);
                    }
                }
            }
            next_variables
        }
    };

    if build_succeeded {
        Ok((
            next_variables,
            SimulationStep {
                ability: ability.name,
                condition: step_variables.condition,
                action_success,
            },
        ))
    } else if item_broke {
        Err(())
    } else {
        Ok((
            next_variables,
            SimulationStep {
                ability: ability.name,
                condition: step_variables.condition,
                action_success,
            },
        ))
    }
}
