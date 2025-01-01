use super::item::ItemCode;

use serde::{Deserialize, Serialize};

/// Represents a Monster in the game.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Monster {
    /// Name of the monster.
    pub name: String,
    /// The code of the monster. This is the monster's unique identifier (ID).
    pub code: String,
    /// The level of the monster.
    pub level: i32,
    /// The hit points (HP) of the monster.
    pub hp: i32,
    /// Monster's fire attack strength.
    pub attack_fire: i32,
    /// Monster's earth attack strength.
    pub attack_earth: i32,
    /// Monster's water attack strength.
    pub attack_water: i32,
    /// Monster's air attack strength.
    pub attack_air: i32,
    /// Percentage of resistance to fire attacks.
    pub res_fire: i32,
    /// Percentage of resistance to earth attacks.
    pub res_earth: i32,
    /// Percentage of resistance to water attacks.
    pub res_water: i32,
    /// Percentage of resistance to air attacks.
    pub res_air: i32,
    /// The minimum amount of gold dropped by the monster upon defeat.
    pub min_gold: i32,
    /// The maximum amount of gold dropped by the monster upon defeat.
    pub max_gold: i32,
    /// Monster drops. This is a list of items that the monster drops after killing the monster.
    pub drops: Vec<MonsterDrop>,
}

/// Represents item that a monster can drop.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonsterDrop {
    /// Item code. This is a unique identifier for the item.
    pub code: ItemCode,
    /// Chance rate for this item to drop, must be >= 1.
    pub rate: i32,
    /// The minimum quantity of the item that can drop, must be >= 1.
    pub min_quantity: i32,
    /// The maximum quantity of the item that can drop, must be >= 1.
    pub max_quantity: i32,
}
