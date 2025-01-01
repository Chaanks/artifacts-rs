use super::item::ItemComponent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fight {
    /// The amount of xp gained by the fight.
    pub xp: i32,
    /// The amount of gold gained by the fight.
    pub gold: i32,
    /// The items dropped by the fight.
    pub drops: Vec<ItemComponent>,
    /// Numbers of the turns of the combat.
    pub turns: i32,
    /// The amount of blocked hits by the monster.
    pub monster_blocked_hits: BlockedHits,
    /// The amount of blocked hits by the player.
    pub player_blocked_hits: BlockedHits,
    /// The fight logs.
    pub logs: Vec<String>,
    /// The result of the fight.
    pub result: FightResult,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BlockedHits {
    /// The amount of fire hits blocked.
    pub fire: i32,
    /// The amount of earth hits blocked.
    pub earth: i32,
    /// The amount of water hits blocked.
    pub water: i32,
    /// The amount of air hits blocked.
    pub air: i32,
    /// The amount of total hits blocked.
    pub total: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FightResult {
    Win,
    Lose,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Element {
    Fire,
    Earth,
    Water,
    Air,
}
