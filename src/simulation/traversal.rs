use crate::profile::{AbilityEffect, AbilityEffectAction, AbilityType};

use super::ability::use_ability;
use super::random::{
    ConditionSimulation, RandomActionSuccessChance, SimulationRandomnessGenerator,
};
use super::state::{
    Condition, SimulationConstants, SimulationRun, SimulationStep, SimulationVariables,
    StepVariables,
};

fn next_quality(item_level: u32, class_level: u32, control: u32, condition: Condition) -> u32 {
    // Quality = round ( (1 - 0.05 * recipe level difference) * (0.36 * Control + 34) )

    let quality_condition = match condition {
        Condition::Poor => 0.5,
        Condition::Normal => 1.0,
        Condition::Good => 2.0,
        Condition::Excellent => 4.0,
    };
    let quality = (1.0
        + 0.05
            * ((class_level as i32 - item_level as i32) as f32)
            * (0.36 * (control as f32) + 34.0))
        * quality_condition;
    //log::debug!("Actual quality {}", quality);
    quality as u32
}

fn _next_step_variables(
    constants: &SimulationConstants,
    start_variables: &SimulationVariables,
    random_generator: &SimulationRandomnessGenerator,
    options: &[SimulationStep],
) -> (StepVariables, SimulationVariables) {
    // Evaluate the new state
    let last_condition = match options.last() {
        Some(s) => s.condition,
        None => Condition::Normal,
    };
    let condition = random_generator.get_condition(last_condition);
    let start_base_quality = next_quality(
        constants.item.level,
        constants.class_level,
        constants.control,
        condition,
    );
    let mut step_variables = StepVariables {
        condition,
        step_number: (options.len() + 1) as u8,
        base_progress: constants.base_progress(),
        base_quality: start_base_quality,
        durability_scale: 1.0,
        progress_floor: false,
    };
    let mut variables = start_variables.clone();

    // Execute any turn based actions
    variables.current_state_turns.retain(|v| {
        if v.turns == 0 {
            return false;
        }

        if let Some(l) = &v.limit {
            if l.uses == 0 {
                return false;
            }
        }
        true
    });

    for curr_effect in &mut variables.current_state_turns {
        curr_effect.turns -= 1;

        // Don't evaluate actions that have a limit as they apply to the ability itself
        if curr_effect.limit.is_some() {
            continue;
        }

        match &curr_effect.effect {
            AbilityEffect::Action(a) => match a {
                AbilityEffectAction::GrantDurability(a) => {
                    variables.remaining_durability += *a as i32;
                    if variables.remaining_durability > constants.item.initial_durability as i32 {
                        variables.remaining_durability = constants.item.initial_durability as i32;
                    }
                }
                AbilityEffectAction::ReduceDurabilityCost(a) => {
                    step_variables.durability_scale = *a;
                }
                AbilityEffectAction::IncreaseSynthesisEfficiency(a) => {
                    step_variables.base_progress = (step_variables.base_progress as f32 * a) as u32
                }
                AbilityEffectAction::IncreaseTouchEfficiency(a) => {
                    step_variables.base_quality = (step_variables.base_quality as f32 * a) as u32
                }
                AbilityEffectAction::ProgressFloor => {
                    step_variables.progress_floor = true;
                }
            },
            AbilityEffect::State(_) => {
                // States will be checked in abilities for requirements
            }
        }
    }

    (step_variables, variables)
}

fn _traverse_depth_first(
    constants: &SimulationConstants,
    start_variables: &SimulationVariables,
    random_generator: &SimulationRandomnessGenerator,
    current_options: &[SimulationStep],
) -> Result<SimulationRun, ()> {
    // Evaluate the new state
    let (step_variables, variables) = _next_step_variables(
        constants,
        start_variables,
        random_generator,
        current_options,
    );

    let mut all_options = vec![];
    let mut lowest_target_quality_steps = variables.lowest_target_quality_steps;
    for ability in &constants.available_abilities {
        // Allow any actions when the condition is poor
        if step_variables.condition != Condition::Poor {
            if variables.current_quality >= constants.item.target_quality {
                // Only focus on progress
                if ability.ability_type & (AbilityType::SYNTHESIS | AbilityType::SYNTHESIS_BUFF)
                    == AbilityType::NONE
                {
                    continue;
                }
            } else {
                // Only focus on quality
                if ability.ability_type & (AbilityType::TOUCH | AbilityType::TOUCH_BUFF)
                    == AbilityType::NONE
                {
                    continue;
                }
            }
        }
        let (next_variables, step) = match use_ability(
            ability,
            constants,
            &variables,
            &step_variables,
            random_generator,
        ) {
            Ok((mut v, s)) => {
                if v.current_progress >= constants.item.required_progress
                    && v.current_quality >= constants.item.target_quality
                    && step_variables.step_number < lowest_target_quality_steps
                {
                    lowest_target_quality_steps = step_variables.step_number;
                }
                v.lowest_target_quality_steps = lowest_target_quality_steps;
                (v, s)
            }
            Err(()) => {
                continue;
            }
        };

        //log::debug!("Tried ability {} at step {} with resulting quality {} and progress {}", ability.name, step_variables.step_number, next_variables.current_quality, next_variables.current_progress);

        if next_variables.current_progress >= constants.item.required_progress {
            let mut next_options = current_options.to_vec();
            next_options.push(step);
            //log::debug!("Fake succeeded {:?} with {}", variables.current_options, ability.name);
            let run = SimulationRun {
                steps: next_options,
                quality: next_variables.current_quality,
                progress: next_variables.current_progress,
            };
            if run.quality >= constants.item.target_quality {
                // No need to try more at the same or further depth
                log::debug!(
                    "Found acceptable quality option in {} steps: {:?}",
                    run.steps.len(),
                    run
                );
                return Ok(run);
            }
            all_options.push(run);
            continue;
        }

        if step_variables.step_number >= constants.max_steps {
            continue;
        }

        // Don't continue the simulation if we already have a match on the next step as further ones could only be more steps
        if step_variables.step_number + 1 >= lowest_target_quality_steps {
            continue;
        }

        let next_randomness = random_generator.next();
        let mut next_options = current_options.to_vec();
        next_options.push(step);

        // Continue the simulation
        match _traverse_depth_first(constants, &next_variables, &next_randomness, &next_options) {
            Ok(run) => {
                if run.quality >= constants.item.target_quality {
                    if run.steps.len() as u8 <= constants.accept_first_of_length {
                        return Ok(run);
                    }
                    if run.steps.len() < lowest_target_quality_steps as usize {
                        lowest_target_quality_steps = run.steps.len() as u8;
                    }
                }
                all_options.push(run);
            }
            Err(()) => {
                continue;
            }
        };
    }

    //log::debug!("Sorting {:?}", all_options);
    all_options.sort_by(|a, b| a.compare(b, constants.item.target_quality));
    //log::debug!("Sorted {:?}", all_options);
    if let Some(a) = all_options.last() {
        Ok(a.clone())
    } else {
        Err(())
    }
}

fn _try_predetermined_starters(
    constants: &SimulationConstants,
    random_generator: &SimulationRandomnessGenerator,
) -> Result<SimulationRun, ()> {
    let mut ability_starts = vec![];

    if constants.item.target_quality > 0 {
        if constants.class_level >= constants.item.level + 10 {
            ability_starts.push(vec!["Trained Eye", "Waste Not II", "Groundwork"]);
            ability_starts.push(vec!["Trained Eye", "Groundwork"]);
        } else {
            // This can get real lengthy at higher levels
            ability_starts.push(vec![
                "Reflect",
                "Manipulation",
                "Waste Not II",
                "Innovation",
                "Preparatory Touch",
                "Preparatory Touch",
                "Preparatory Touch",
                "Preparatory Touch",
                "Preparatory Touch",
                "Great Strides",
                "Byregot's Blessing",
            ]);
            ability_starts.push(vec![
                "Reflect",
                "Waste Not II",
                "Innovation",
                "Preparatory Touch",
                "Preparatory Touch",
                "Preparatory Touch",
            ]);
        }
        ability_starts.push(vec!["Reflect", "Waste Not II", "Innovation"]);
        ability_starts.push(vec!["Waste Not II", "Innovation"]);
        ability_starts.push(vec!["Reflect", "Innovation"]);
        ability_starts.push(vec!["Waste Not", "Basic Touch", "Standard Touch"]);
    } else {
        ability_starts.push(vec![
            "Muscle Memory",
            "Manipulation",
            "Waste Not II",
            "Groundwork",
        ]);
        ability_starts.push(vec![
            "Muscle Memory",
            "Veneration",
            "Waste Not II",
            "Groundwork",
        ]);
    }

    ability_starts.push(vec![]);

    let mut best_run: Result<SimulationRun, ()> = Err(());
    let mut lowest_target_quality_steps = constants.max_steps;
    for ability_names in &ability_starts {
        // Try again with a cap set at the best trained option to see if we can find a shorter solution
        match _simulate_from_abilities(
            ability_names,
            constants,
            random_generator,
            lowest_target_quality_steps,
        ) {
            Ok(r) => {
                log::debug!(
                    "Got run length of {} for starter {:?}",
                    r.steps.len(),
                    ability_names
                );
                if (r.steps.len() as u8) < lowest_target_quality_steps {
                    lowest_target_quality_steps = r.steps.len() as u8;
                }
                if let Ok(old_run) = &best_run {
                    if old_run.compare(&r, constants.item.target_quality)
                        == std::cmp::Ordering::Less
                    {
                        best_run = Ok(r);
                    }
                } else {
                    best_run = Ok(r);
                }
            }
            Err(()) => {
                log::debug!("Failed run for starter {:?}", ability_names);
            }
        }
    }

    best_run
}

// TODO: Change result
pub fn find_and_use_ability(
    ability_name: &str,
    constants: &SimulationConstants,
    variables: &SimulationVariables,
    random_generator: &SimulationRandomnessGenerator,
    options: &[SimulationStep],
) -> Result<(SimulationVariables, SimulationStep), ()> {
    for ability in &constants.available_abilities {
        if ability.name != ability_name {
            continue;
        }

        let (step_variables, next_variables) =
            _next_step_variables(constants, variables, random_generator, options);
        return use_ability(
            ability,
            constants,
            &next_variables,
            &step_variables,
            random_generator,
        );
    }

    log::error!("No ability named '{}'", ability_name);
    Err(())
}

fn _simulate_from_abilities(
    ability_names: &[&str],
    constants: &SimulationConstants,
    random_generator: &SimulationRandomnessGenerator,
    lowest_target_quality_steps: u8,
) -> Result<SimulationRun, ()> {
    let mut options = vec![];

    let mut variables = SimulationVariables::new(constants);
    let mut random_generator = random_generator.clone();

    variables.lowest_target_quality_steps = lowest_target_quality_steps;

    for ability_name in ability_names {
        let p = find_and_use_ability(
            ability_name,
            constants,
            &variables,
            &random_generator,
            &options,
        )
        .map_err(|e| {
            log::error!(
                "Failed to execute ability {} in simulation of {:?}",
                ability_name,
                ability_names
            );
            e
        })?;
        variables = p.0;

        options.push(p.1);

        if variables.current_progress >= constants.item.required_progress {
            if variables.current_quality < constants.item.target_quality {
                log::error!("Failed to reach target quality when executing ability {} in simulation of {:?}", ability_name, ability_names);
                return Err(());
            }
            return Ok(SimulationRun {
                steps: options,
                quality: variables.current_quality,
                progress: variables.current_progress,
            });
        }

        random_generator = random_generator.next();
    }

    log::debug!("Running further simulation for {:?}", ability_names);
    _traverse_depth_first(constants, &variables, &random_generator, &options)
}

pub fn simulate_from_abilities(
    ability_names: &[&str],
    constants: &SimulationConstants,
    random_generator: &SimulationRandomnessGenerator,
) -> Result<SimulationRun, ()> {
    _simulate_from_abilities(
        ability_names,
        constants,
        random_generator,
        constants.max_steps,
    )
}

pub fn simulate_from_run(
    run: &SimulationRun,
    constants: &SimulationConstants,
    condition_strategy: ConditionSimulation,
    action_strategy: RandomActionSuccessChance,
) -> Result<SimulationRun, ()> {
    let mut options = vec![];

    let mut variables = SimulationVariables::new(constants);

    for step in &run.steps {
        let random_generator = SimulationRandomnessGenerator::from_results(
            step.condition,
            step.action_success.unwrap_or(false),
        );
        let p = find_and_use_ability(
            step.ability,
            constants,
            &variables,
            &random_generator,
            &options,
        )?;
        variables = p.0;
        //log::debug!("Used ability {} with state {:?}", ability_name, variables);

        options.push(p.1);

        if variables.current_progress >= constants.item.required_progress {
            if variables.current_quality < constants.item.target_quality {
                return Err(());
            }
            //log::debug!("Fake succeeded {:?} with {}", variables.current_options, ability.name);
            return Ok(SimulationRun {
                steps: options,
                quality: variables.current_quality,
                progress: variables.current_progress,
            });
        }
    }

    let random_generator = SimulationRandomnessGenerator::new(condition_strategy, action_strategy);
    _traverse_depth_first(constants, &variables, &random_generator, &options)
}

pub fn simulate(
    constants: &SimulationConstants,
    condition_strategy: ConditionSimulation,
    action_strategy: RandomActionSuccessChance,
) -> Result<SimulationRun, ()> {
    log::debug!("Starting {:?}", constants);

    let generator = SimulationRandomnessGenerator::first_step(condition_strategy, action_strategy);

    // In a basic test this went from 16 seconds without to 2 seconds with
    _try_predetermined_starters(constants, &generator)
}
