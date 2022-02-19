use super::state::Condition;

use getrandom::getrandom;

fn get_random_int() -> u32 {
    let mut buf = [0u8; 4];
    getrandom(&mut buf).unwrap();

    ((buf[0] as u32) << 24) + ((buf[1] as u32) << 16) + ((buf[2] as u32) << 8) + (buf[3] as u32)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RandomActionSuccessChance {
    Normal,
    AlwaysSucceed,
    AlwaysFail,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConditionSimulation {
    AlwaysNormal,
    WeightedRandom,
}

#[derive(Debug, Clone)]
enum RandomCondition {
    Value(Condition),
    Random(u32),
}

#[derive(Debug, Clone)]
enum RandomActionSuccess {
    Value(bool),
    Random(u32),
}

#[derive(Debug, Clone)]
struct SimulationRandomness {
    condition: RandomCondition,
    action_success: RandomActionSuccess,
}

impl SimulationRandomness {
    fn new() -> SimulationRandomness {
        SimulationRandomness {
            condition: RandomCondition::Random(get_random_int()),
            action_success: RandomActionSuccess::Random(get_random_int()),
        }
    }

    fn first_step() -> SimulationRandomness {
        SimulationRandomness {
            condition: RandomCondition::Value(Condition::Normal),
            action_success: RandomActionSuccess::Random(get_random_int()),
        }
    }

    fn from_results(condition: Condition, action_success: bool) -> SimulationRandomness {
        SimulationRandomness {
            condition: RandomCondition::Value(condition),
            action_success: RandomActionSuccess::Value(action_success),
        }
    }

    fn get_condition(&self, last: Condition, strategy: ConditionSimulation) -> Condition {
        match strategy {
            ConditionSimulation::AlwaysNormal => Condition::Normal,
            ConditionSimulation::WeightedRandom => {
                match self.condition {
                    RandomCondition::Value(v) => v,
                    RandomCondition::Random(v) => {
                        let condition_percent = v as f64 / u32::MAX as f64;

                        // Poor - (always?) after excellent gives 50% quality
                        // Good - 25% chance to buff gives 200% quality
                        // Excellent - 10% chance to buff a lot gives 400% quality
                        if last == Condition::Excellent {
                            Condition::Poor
                        } else if last == Condition::Normal && condition_percent >= 0.9 {
                            Condition::Excellent
                        } else if last == Condition::Normal && condition_percent > 0.65 {
                            Condition::Good
                        } else {
                            Condition::Normal
                        }
                    }
                }
            }
        }
    }

    fn random_action_succeess(&self, rate: u32, strategy: RandomActionSuccessChance) -> bool {
        match strategy {
            RandomActionSuccessChance::Normal => match self.action_success {
                RandomActionSuccess::Value(v) => v,
                RandomActionSuccess::Random(v) => {
                    let condition_percent = v as f64 / u32::MAX as f64;

                    condition_percent < (rate as f64 / 100.0)
                }
            },
            RandomActionSuccessChance::AlwaysSucceed => true,
            RandomActionSuccessChance::AlwaysFail => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationRandomnessGenerator {
    randomness: SimulationRandomness,
    condition_strategy: ConditionSimulation,
    action_strategy: RandomActionSuccessChance,
}

impl SimulationRandomnessGenerator {
    pub fn new(
        condition_strategy: ConditionSimulation,
        action_strategy: RandomActionSuccessChance,
    ) -> SimulationRandomnessGenerator {
        SimulationRandomnessGenerator {
            randomness: SimulationRandomness::new(),
            condition_strategy,
            action_strategy,
        }
    }

    pub fn first_step(
        condition_strategy: ConditionSimulation,
        action_strategy: RandomActionSuccessChance,
    ) -> SimulationRandomnessGenerator {
        SimulationRandomnessGenerator {
            randomness: SimulationRandomness::first_step(),
            condition_strategy,
            action_strategy,
        }
    }

    pub fn from_results(
        condition: Condition,
        action_success: bool,
    ) -> SimulationRandomnessGenerator {
        // Do not use next with this
        SimulationRandomnessGenerator {
            randomness: SimulationRandomness::from_results(condition, action_success),
            condition_strategy: ConditionSimulation::AlwaysNormal,
            action_strategy: RandomActionSuccessChance::AlwaysFail,
        }
    }

    pub fn next(&self) -> SimulationRandomnessGenerator {
        SimulationRandomnessGenerator {
            randomness: SimulationRandomness::new(),
            condition_strategy: self.condition_strategy,
            action_strategy: self.action_strategy,
        }
    }

    pub fn get_condition(&self, last: Condition) -> Condition {
        self.randomness.get_condition(last, self.condition_strategy)
    }

    pub fn random_action_succeess(&self, rate: u32) -> bool {
        self.randomness
            .random_action_succeess(rate, self.action_strategy)
    }
}
