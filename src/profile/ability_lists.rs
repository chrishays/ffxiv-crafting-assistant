use super::abilities::{
    Ability, AbilityAction, AbilityEffect, AbilityEffectAction, AbilityEffectLimit,
    AbilityEffectTurns, AbilityState, AbilityType, BaseAction, ComboAction, DurabilityWithDiscount,
    RequiredState, SuccessAction,
};
use super::classes::CraftingClassStats;

pub fn get_all_abilities_at_level(stats: &CraftingClassStats) -> Vec<Ability> {
    let mut abilities = get_all_abilities();

    abilities.retain(|a| {
        if stats.level < a.required_level {
            return false;
        }
        if stats.class_quest_level < a.required_class_quest {
            return false;
        }

        true
    });

    abilities
}

pub fn get_all_abilities() -> Vec<Ability> {
    vec![
        Ability {
            name: "Basic Synthesis",
            ability_type: AbilityType::SYNTHESIS,
            required_level: 1,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantProgress(1.2),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Basic Touch",
            ability_type: AbilityType::TOUCH,
            required_level: 5,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(18),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(1.0),
                    AbilityAction::GrantInnerQuiet,
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::State(AbilityState::BasicTouch),
                        limit: None,
                        turns: 1,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Master's Mend",
            ability_type: AbilityType::BUFF,
            required_level: 7,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(88),
                    AbilityAction::GrantDurability(30),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Hasty Touch",
            ability_type: AbilityType::TOUCH,
            required_level: 9,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![AbilityAction::CostDurability(10)],
                action_success: Some(SuccessAction {
                    rate: 60,
                    action: vec![
                        AbilityAction::GrantQuality(1.0),
                        AbilityAction::GrantInnerQuiet,
                    ],
                }),
            },
            combo_action: None,
        },
        Ability {
            name: "Rapid Synthesis",
            ability_type: AbilityType::SYNTHESIS,
            required_level: 9,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![AbilityAction::CostDurability(10)],
                action_success: Some(SuccessAction {
                    rate: 50,
                    action: vec![AbilityAction::GrantProgress(2.5)],
                }),
            },
            combo_action: None,
        },
        Ability {
            name: "Observe",
            ability_type: AbilityType::BUFF,
            required_level: 13,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(12),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::State(AbilityState::Observing),
                        limit: None,
                        turns: 1,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Tricks of the Trade",
            ability_type: AbilityType::BUFF,
            required_level: 13,
            required_class_quest: 1,
            required_state: RequiredState::ConditionGood,
            action: BaseAction {
                action: vec![AbilityAction::GrantCP(20)],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Waste Not",
            ability_type: AbilityType::BUFF,
            required_level: 15,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(56),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::Action(AbilityEffectAction::ReduceDurabilityCost(
                            0.5,
                        )),
                        limit: None,
                        turns: 4,
                    }),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::State(AbilityState::WasteNot),
                        limit: None,
                        turns: 4,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Veneration",
            ability_type: AbilityType::SYNTHESIS_BUFF,
            required_level: 15,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(18),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::Action(
                            AbilityEffectAction::IncreaseSynthesisEfficiency(0.5),
                        ),
                        limit: None,
                        turns: 4,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Standard Touch",
            ability_type: AbilityType::TOUCH,
            required_level: 18,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(32),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(1.25),
                    AbilityAction::GrantInnerQuiet,
                ],
                action_success: None,
            },
            combo_action: Some(ComboAction {
                required_state: AbilityState::BasicTouch,
                action: vec![
                    AbilityAction::CostCP(18),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(1.25),
                    AbilityAction::GrantInnerQuiet,
                ],
                action_success: None,
            }),
        },
        Ability {
            name: "Great Strides",
            ability_type: AbilityType::TOUCH_BUFF,
            required_level: 21,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(32),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::Action(
                            AbilityEffectAction::IncreaseTouchEfficiency(1.0),
                        ),
                        limit: Some(AbilityEffectLimit {
                            apply_to: AbilityType::TOUCH,
                            uses: 1,
                        }),
                        turns: 3,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Innovation",
            ability_type: AbilityType::TOUCH_BUFF,
            required_level: 26,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(18),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::Action(
                            AbilityEffectAction::IncreaseTouchEfficiency(0.5),
                        ),
                        limit: None,
                        turns: 4,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Final Appraisal",
            ability_type: AbilityType::BUFF, // It is a touch buff but only useful when doing a synthesis step with the 1 extra progress
            required_level: 42,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(1),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::Action(AbilityEffectAction::ProgressFloor),
                        limit: Some(AbilityEffectLimit {
                            apply_to: AbilityType::TOUCH,
                            uses: 1,
                        }),
                        turns: 4,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Waste Not II",
            ability_type: AbilityType::BUFF,
            required_level: 47,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(98),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::Action(AbilityEffectAction::ReduceDurabilityCost(
                            0.5,
                        )),
                        limit: None,
                        turns: 8,
                    }),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::State(AbilityState::WasteNot),
                        limit: None,
                        turns: 8,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Byregot's Blessing",
            ability_type: AbilityType::TOUCH,
            required_level: 50,
            required_class_quest: 1,
            required_state: RequiredState::InnerQuiet(1),
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(24),
                    // Consuming is touch efficiency of 100% + 20% for each inner quiet (max stack 10)
                    AbilityAction::ConsumeInnerQuiet,
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Precise Touch",
            ability_type: AbilityType::TOUCH,
            required_level: 53,
            required_class_quest: 1,
            required_state: RequiredState::ConditionGood,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(18),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(1.5),
                    AbilityAction::GrantInnerQuiet,
                    AbilityAction::GrantInnerQuiet,
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Muscle Memory",
            ability_type: AbilityType::SYNTHESIS,
            required_level: 54,
            required_class_quest: 1,
            required_state: RequiredState::FirstStep,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(6),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantProgress(3.0),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::Action(
                            AbilityEffectAction::IncreaseSynthesisEfficiency(1.0),
                        ),
                        limit: None,
                        turns: 5,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Careful Synthesis",
            ability_type: AbilityType::SYNTHESIS,
            required_level: 62,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(7),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantProgress(1.5),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Manipulation",
            ability_type: AbilityType::BUFF,
            required_level: 65,
            required_class_quest: 65,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(96),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantEffect(AbilityEffectTurns {
                        effect: AbilityEffect::Action(AbilityEffectAction::GrantDurability(5)),
                        limit: None,
                        turns: 5,
                    }),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Prudent Touch",
            ability_type: AbilityType::TOUCH,
            required_level: 66,
            required_class_quest: 1,
            required_state: RequiredState::NotAbility(AbilityState::WasteNot),
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(25),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(1.0),
                    AbilityAction::GrantInnerQuiet,
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Focused Synthesis",
            ability_type: AbilityType::SYNTHESIS,
            required_level: 67,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![AbilityAction::CostCP(5), AbilityAction::CostDurability(10)],
                action_success: Some(SuccessAction {
                    rate: 50,
                    action: vec![AbilityAction::GrantProgress(2.0)],
                }),
            },
            combo_action: Some(ComboAction {
                required_state: AbilityState::Observing,
                action: vec![
                    AbilityAction::CostCP(5),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantProgress(2.0),
                ],
                action_success: None,
            }),
        },
        Ability {
            name: "Focused Touch",
            ability_type: AbilityType::TOUCH,
            required_level: 68,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![AbilityAction::CostCP(18), AbilityAction::CostDurability(10)],
                action_success: Some(SuccessAction {
                    rate: 50,
                    action: vec![
                        AbilityAction::GrantQuality(2.0),
                        AbilityAction::GrantInnerQuiet,
                    ],
                }),
            },
            combo_action: Some(ComboAction {
                required_state: AbilityState::Observing,
                action: vec![
                    AbilityAction::CostCP(18),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(2.0),
                    AbilityAction::GrantInnerQuiet,
                ],
                action_success: None,
            }),
        },
        Ability {
            name: "Reflect",
            ability_type: AbilityType::TOUCH,
            required_level: 69,
            required_class_quest: 1,
            required_state: RequiredState::FirstStep,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(6),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(1.0),
                    AbilityAction::GrantInnerQuiet,
                    AbilityAction::GrantInnerQuiet,
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Preparatory Touch",
            ability_type: AbilityType::TOUCH,
            required_level: 71,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(40),
                    AbilityAction::CostDurability(20),
                    AbilityAction::GrantQuality(2.0),
                    AbilityAction::GrantInnerQuiet,
                    AbilityAction::GrantInnerQuiet,
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Groundwork",
            ability_type: AbilityType::SYNTHESIS,
            required_level: 72,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(18),
                    AbilityAction::CostDurabilityWithDiscount(DurabilityWithDiscount {
                        cost: 20,
                        cost_if_too_expensive: 10,
                    }),
                    AbilityAction::GrantProgress(3.0),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Delicate Synthesis",
            ability_type: AbilityType::SYNTHESIS | AbilityType::TOUCH,
            required_level: 76,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(32),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(1.0),
                    AbilityAction::GrantProgress(1.0),
                    AbilityAction::GrantInnerQuiet,
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Intensive Synthesis",
            ability_type: AbilityType::SYNTHESIS,
            required_level: 78,
            required_class_quest: 1,
            required_state: RequiredState::ConditionGood,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(6),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantProgress(4.0),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Trained Eye",
            ability_type: AbilityType::SYNTHESIS,
            required_level: 80,
            required_class_quest: 1,
            required_state: RequiredState::FirstStepLevelAdvantage(10),
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(250),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantFullQuality,
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Advanced Touch",
            ability_type: AbilityType::TOUCH,
            required_level: 84,
            required_class_quest: 1,
            required_state: RequiredState::None,
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(46),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(1.5),
                ],
                action_success: None,
            },
            combo_action: Some(ComboAction {
                required_state: AbilityState::StandardTouch,
                action: vec![
                    AbilityAction::CostCP(18),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantQuality(1.5),
                    AbilityAction::GrantInnerQuiet,
                ],
                action_success: None,
            }),
        },
        Ability {
            name: "Prudent Synthesis",
            ability_type: AbilityType::SYNTHESIS,
            required_level: 88,
            required_class_quest: 1,
            required_state: RequiredState::NotAbility(AbilityState::WasteNot),
            action: BaseAction {
                action: vec![
                    AbilityAction::CostCP(18),
                    AbilityAction::CostDurability(10),
                    AbilityAction::GrantProgress(1.8),
                ],
                action_success: None,
            },
            combo_action: None,
        },
        Ability {
            name: "Trained Finesse",
            ability_type: AbilityType::TOUCH,
            required_level: 90,
            required_class_quest: 1,
            required_state: RequiredState::InnerQuiet(10),
            action: BaseAction {
                action: vec![AbilityAction::CostCP(32), AbilityAction::GrantQuality(1.0)],
                action_success: None,
            },
            combo_action: None,
        },
    ]
}
