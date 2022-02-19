use super::spec::{Item, ItemStats};
use crate::profile::{CraftingClass, CraftingClassStats};

pub fn get_craftable_items_for_class_at_level(
    crafting_class: CraftingClass,
    _stats: &CraftingClassStats,
) -> Vec<Item> {
    // TODO: Filter out  things more than 10? level over crafting level

    get_craftable_items_for_class(crafting_class)
}

pub fn get_craftable_items_for_class(crafting_class: CraftingClass) -> Vec<Item> {
    match crafting_class {
        CraftingClass::Alchemist => {
            vec!(
                Item{
                    name: "Distilled Water",
                    stats: ItemStats{
                        level : 1,
                        initial_durability : 40,
                        target_quality : 80,
                        required_progress : 9,
                    }
                },
                Item{
                    name: "Quicksilver",
                    stats: ItemStats{
                        level : 1,
                        initial_durability : 40,
                        target_quality : 80,
                        required_progress : 9,
                    }
                },
                Item{
                    name: "Animal Glue",
                    stats: ItemStats{
                        level : 2,
                        initial_durability : 40,
                        target_quality : 88,
                        required_progress : 10,
                    }
                },
                Item{
                    name: "Growth Formula Alpha",
                    stats: ItemStats{
                        level : 3,
                        initial_durability : 40,
                        target_quality : 96,
                        required_progress : 10,
                    }
                },
                Item{
                    name: "Enchanted Copper Ink",
                    stats: ItemStats{
                        level : 4,
                        initial_durability : 40,
                        target_quality : 104,
                        required_progress : 10,
                    }
                },
                Item{
                    name: "Maple Wand",
                    stats: ItemStats{
                        level : 4,
                        initial_durability : 60,
                        target_quality : 130,
                        required_progress : 21,
                    }
                },
                Item{
                    name: "Wanderer's Campfire",
                    stats: ItemStats{
                        level : 80,
                        initial_durability : 80,
                        target_quality : 2300,
                        required_progress : 1780,
                    }
                },
            )
        }
        // CraftingClass::Carpenter => {
        //     get_basic_skill_list()
        // }
        //_ => panic!("Not implemented"),
    }
}
