#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CraftingClass {
    Alchemist,
    // Armorer,
    // Blacksmith,
    // Carpenter,
    // Culinarian,
    // Goldsmith,
    // Leatherworker,
    // Weaver,
}

#[derive(Debug, Clone, Copy)]
pub struct CraftingAttributes {
    pub cp: u32,
    pub control: u32,
    pub craftmanship: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltCraftingAttributes {
    pub gear_attributes: CraftingAttributes,
    pub buffs: CraftingAttributes,
}

#[derive(Debug, Clone, Copy)]
pub enum BuffedCraftingAttributes {
    Built(BuiltCraftingAttributes),
    Final(CraftingAttributes),
}

impl BuffedCraftingAttributes {
    pub fn final_attributes(&self) -> CraftingAttributes {
        match self {
            BuffedCraftingAttributes::Built(ref built) => {
                let mut add_stats: CraftingAttributes = built.gear_attributes;
                add_stats.control += built.buffs.control;
                add_stats.cp += built.buffs.cp;
                add_stats.craftmanship += built.buffs.craftmanship;
                add_stats
            }
            BuffedCraftingAttributes::Final(ref ca) => *ca,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CraftingClassStats {
    pub level: u32,
    pub class_quest_level: u32,

    pub attributres: BuffedCraftingAttributes,
}
