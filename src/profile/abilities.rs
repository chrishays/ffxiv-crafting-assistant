use bitflags::bitflags;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AbilityState {
    Observing,
    BasicTouch,
    StandardTouch,
    WasteNot,
}

bitflags! {
    pub struct AbilityType : u8 {
        const NONE =            0b00000000;
        const TOUCH =           0b00000001;
        const SYNTHESIS =       0b00000010;
        const TOUCH_BUFF =      0b00000100;
        const SYNTHESIS_BUFF =  0b00001000;
        const BUFF =            Self::TOUCH_BUFF.bits | Self::SYNTHESIS_BUFF.bits;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbilityEffectAction {
    GrantDurability(u32),
    ReduceDurabilityCost(f32),
    IncreaseSynthesisEfficiency(f32),
    IncreaseTouchEfficiency(f32),
    ProgressFloor,
}

#[derive(Debug, Clone, Copy)]
pub struct AbilityEffectLimit {
    pub apply_to: AbilityType,
    pub uses: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbilityEffect {
    Action(AbilityEffectAction),
    State(AbilityState),
}

#[derive(Debug, Clone, Copy)]
pub struct AbilityEffectTurns {
    pub effect: AbilityEffect,
    pub limit: Option<AbilityEffectLimit>,
    pub turns: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct DurabilityWithDiscount {
    pub cost: u32,
    pub cost_if_too_expensive: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum AbilityAction {
    // Base actions
    GrantDurability(u32),
    CostDurability(u32),
    GrantProgress(f32),
    GrantQuality(f32),
    GrantFullQuality,
    GrantCP(u32),
    CostCP(u32),
    GrantEffect(AbilityEffectTurns),
    GrantInnerQuiet,
    ConsumeInnerQuiet,
    CostDurabilityWithDiscount(DurabilityWithDiscount),
}

#[derive(Debug, Clone)]
pub struct SuccessAction {
    pub rate: u32,
    pub action: Vec<AbilityAction>,
}

#[derive(Debug, Clone)]
pub struct ComboAction {
    pub required_state: AbilityState,
    pub action_success: Option<SuccessAction>,
    pub action: Vec<AbilityAction>,
}

#[derive(Debug, Clone)]
pub struct BaseAction {
    pub action_success: Option<SuccessAction>,
    pub action: Vec<AbilityAction>,
}

#[derive(Debug, Clone, Copy)]
pub enum RequiredState {
    None,
    FirstStep,
    FirstStepLevelAdvantage(u32),
    ConditionGood,
    NotAbility(AbilityState),
    InnerQuiet(u8),
}

#[derive(Debug, Clone)]
pub struct Ability {
    pub name: &'static str,
    pub ability_type: AbilityType,
    pub required_level: u32,
    pub required_class_quest: u32,

    pub required_state: RequiredState,

    pub action: BaseAction,
    pub combo_action: Option<ComboAction>,
}
