#[derive(Debug, Clone, Copy)]
pub struct ItemStats {
    pub level: u32,
    pub initial_durability: u32,
    pub target_quality: u32,
    pub required_progress: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub name: &'static str,
    pub stats: ItemStats,
}
