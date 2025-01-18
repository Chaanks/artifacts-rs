use serde::{Deserialize, Serialize};

use crate::api::ItemSlot;

use super::item::ItemType;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Character {
    /// Name of the character.
    pub name: String,
    /// Account name.
    pub account: String,
    /// Character skin code.
    pub skin: Skin,
    /// Character movement speed.
    pub speed: i32,
    /// Coordinates.
    #[serde(flatten)]
    pub position: Position,
    /// Combat stats.
    #[serde(flatten)]
    pub combat_stats: CombatStats,
    /// Skills.
    #[serde(flatten)]
    pub skills: Skills,
    /// Elemental attributes.
    #[serde(flatten)]
    pub elemental_attributes: ElementalAttributes,
    /// Equipment slots.
    #[serde(flatten)]
    pub equipment: Equipment,
    /// Utility slots.
    #[serde(flatten)]
    pub utilities: Utilities,
    /// Task details.
    #[serde(flatten)]
    pub task: Task,
    /// Inventory details.
    #[serde(flatten)]
    pub inventory_info: InventoryInfo,
    /// Cooldown details.
    #[serde(flatten)]
    pub cooldown_info: CooldownInfo,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CombatStats {
    /// Combat level.
    pub level: i32,
    /// The current XP of the combat level.
    pub xp: i32,
    /// XP required to level up the character.
    pub max_xp: i32,
    /// The number of gold on this character.
    pub gold: i32,
    /// Character HP.
    pub hp: i32,
    /// Character max HP.
    pub max_hp: i32,
    /// Character Haste.
    pub haste: i32,
    /// Character Critical Strike.
    pub critical_strike: i32,
    /// Character Stamina.
    pub stamina: i32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Skills {
    // Mining
    #[serde(rename = "mining_level")]
    pub mining_level: Option<u32>,
    #[serde(rename = "mining_xp")]
    pub mining_xp: Option<u32>,
    #[serde(rename = "mining_max_xp")]
    pub mining_max_xp: Option<u32>,

    // Woodcutting
    #[serde(rename = "woodcutting_level")]
    pub woodcutting_level: Option<u32>,
    #[serde(rename = "woodcutting_xp")]
    pub woodcutting_xp: Option<u32>,
    #[serde(rename = "woodcutting_max_xp")]
    pub woodcutting_max_xp: Option<u32>,

    // Fishing
    #[serde(rename = "fishing_level")]
    pub fishing_level: Option<u32>,
    #[serde(rename = "fishing_xp")]
    pub fishing_xp: Option<u32>,
    #[serde(rename = "fishing_max_xp")]
    pub fishing_max_xp: Option<u32>,

    // Weaponcrafting
    #[serde(rename = "weaponcrafting_level")]
    pub weaponcrafting_level: Option<u32>,
    #[serde(rename = "weaponcrafting_xp")]
    pub weaponcrafting_xp: Option<u32>,
    #[serde(rename = "weaponcrafting_max_xp")]
    pub weaponcrafting_max_xp: Option<u32>,

    // Gearcrafting
    #[serde(rename = "gearcrafting_level")]
    pub gearcrafting_level: Option<u32>,
    #[serde(rename = "gearcrafting_xp")]
    pub gearcrafting_xp: Option<u32>,
    #[serde(rename = "gearcrafting_max_xp")]
    pub gearcrafting_max_xp: Option<u32>,

    // Jewelrycrafting
    #[serde(rename = "jewelrycrafting_level")]
    pub jewelrycrafting_level: Option<u32>,
    #[serde(rename = "jewelrycrafting_xp")]
    pub jewelrycrafting_xp: Option<u32>,
    #[serde(rename = "jewelrycrafting_max_xp")]
    pub jewelrycrafting_max_xp: Option<u32>,

    // Cooking
    #[serde(rename = "cooking_level")]
    pub cooking_level: Option<u32>,
    #[serde(rename = "cooking_xp")]
    pub cooking_xp: Option<u32>,
    #[serde(rename = "cooking_max_xp")]
    pub cooking_max_xp: Option<u32>,

    // Alchemy
    #[serde(rename = "alchemy_level")]
    pub alchemy_level: Option<u32>,
    #[serde(rename = "alchemy_xp")]
    pub alchemy_xp: Option<u32>,
    #[serde(rename = "alchemy_max_xp")]
    pub alchemy_max_xp: Option<u32>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ElementalAttributes {
    /// Elemental attacks.
    pub attack_fire: i32,
    pub attack_earth: i32,
    pub attack_water: i32,
    pub attack_air: i32,
    /// Elemental damage bonuses.
    pub dmg_fire: i32,
    pub dmg_earth: i32,
    pub dmg_water: i32,
    pub dmg_air: i32,
    /// Elemental resistances.
    pub res_fire: i32,
    pub res_earth: i32,
    pub res_water: i32,
    pub res_air: i32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Equipment {
    /// Equipment slots.
    pub weapon_slot: String,
    pub shield_slot: String,
    pub helmet_slot: String,
    pub body_armor_slot: String,
    pub leg_armor_slot: String,
    pub boots_slot: String,
    pub ring1_slot: String,
    pub ring2_slot: String,
    pub amulet_slot: String,
    pub artifact1_slot: String,
    pub artifact2_slot: String,
    pub artifact3_slot: String,
}

impl Equipment {
    /// Check if a slot is empty
    pub fn is_slot_empty(&self, slot: &ItemSlot) -> bool {
        match slot {
            ItemSlot::Weapon => self.weapon_slot.is_empty(),
            ItemSlot::Shield => self.shield_slot.is_empty(),
            ItemSlot::Helmet => self.helmet_slot.is_empty(),
            ItemSlot::BodyArmor => self.body_armor_slot.is_empty(),
            ItemSlot::LegArmor => self.leg_armor_slot.is_empty(),
            ItemSlot::Boots => self.boots_slot.is_empty(),
            ItemSlot::Ring1 => self.ring1_slot.is_empty(),
            ItemSlot::Ring2 => self.ring2_slot.is_empty(),
            ItemSlot::Amulet => self.amulet_slot.is_empty(),
            ItemSlot::Artifact1 => self.artifact1_slot.is_empty(),
            ItemSlot::Artifact2 => self.artifact2_slot.is_empty(),
            ItemSlot::Artifact3 => self.artifact3_slot.is_empty(),
            _ => false,
        }
    }

    /// Get the item code in a specific slot
    pub fn item_in_slot(&self, slot: &ItemSlot) -> Option<&str> {
        let item = match slot {
            ItemSlot::Weapon => &self.weapon_slot,
            ItemSlot::Shield => &self.shield_slot,
            ItemSlot::Helmet => &self.helmet_slot,
            ItemSlot::BodyArmor => &self.body_armor_slot,
            ItemSlot::LegArmor => &self.leg_armor_slot,
            ItemSlot::Boots => &self.boots_slot,
            ItemSlot::Ring1 => &self.ring1_slot,
            ItemSlot::Ring2 => &self.ring2_slot,
            ItemSlot::Amulet => &self.amulet_slot,
            ItemSlot::Artifact1 => &self.artifact1_slot,
            ItemSlot::Artifact2 => &self.artifact2_slot,
            ItemSlot::Artifact3 => &self.artifact3_slot,
            _ => return None,
        };

        if item.is_empty() {
            None
        } else {
            Some(item)
        }
    }

    /// Find first available slot for an item type
    pub fn find_available_slot(&self, item_type: &ItemType) -> Option<ItemSlot> {
        let possible_slots = item_type.possible_slots();
        possible_slots
            .into_iter()
            .find(|slot| self.is_slot_empty(slot))
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Utilities {
    /// Utility slots.
    pub utility1_slot: String,
    pub utility1_slot_quantity: i32,
    pub utility2_slot: String,
    pub utility2_slot_quantity: i32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Task {
    /// Task details.
    pub task: String,
    pub task_type: String,
    pub task_progress: i32,
    pub task_total: i32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InventoryInfo {
    /// Inventory details.
    pub inventory_max_items: u32,
    pub inventory: Option<Vec<InventorySlot>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CooldownInfo {
    /// Cooldown details.
    pub cooldown: i32,
    pub cooldown_expiration: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Skin {
    #[default]
    Men1,
    Men2,
    Men3,
    Women1,
    Women2,
    Women3,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InventorySlot {
    /// Inventory slot identifier.
    pub slot: i32,
    /// Item code.
    pub code: String,
    /// Quantity in the slot.
    pub quantity: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cooldown {
    /// The total seconds of the cooldown.
    pub total_seconds: i32,
    /// The remaining seconds of the cooldown.
    pub remaining_seconds: i32,
    /// The start of the cooldown.
    pub started_at: String,
    /// The expiration of the cooldown.
    pub expiration: String,
    /// The reason of the cooldown.
    pub reason: CooldownReason,
}

/// The reason for the cooldown.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CooldownReason {
    Movement,
    Fight,
    Crafting,
    Gathering,
    #[serde(rename = "buy_ge")]
    BuyGe,
    #[serde(rename = "sell_ge")]
    SellGe,
    #[serde(rename = "cancel_ge")]
    CancelGe,
    #[serde(rename = "delete_item")]
    DeleteItem,
    Deposit,
    Withdraw,
    #[serde(rename = "deposit_gold")]
    DepositGold,
    #[serde(rename = "withdraw_gold")]
    WithdrawGold,
    Equip,
    Unequip,
    Task,
    #[serde(rename = "christmas_exchange")]
    ChristmasExchange,
    Recycling,
    Rest,
    Use,
    #[serde(rename = "buy_bank_expansion")]
    BuyBankExpansion,
}

/// Represents the different skills that a character has within the game.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Skill {
    /// Fishing skill, allowing for gathering fish.
    Fishing,
    /// Cooking skill, allowing the preparation of food.
    Cooking,
    /// Gear crafting skill, for creating and enhancing gear.
    Gearcrafting,
    /// Mining skill, for gathering ores and minerals.
    Mining,
    /// Jewelry crafting skill, for making rings and amulets.
    Jewelrycrafting,
    /// Weapon crafting skill, for creating various weapons.
    Weaponcrafting,
    /// Woodcutting skill, for gathering wood and related resources.
    Woodcutting,
    /// Alchemy skill, for creating potion and related resources.
    Alchemy,
    /// Fighting skill
    Fighting,
}

impl Skill {
    pub fn to_string(&self) -> String {
        serde_plain::to_string(self).unwrap()
    }

    pub fn is_gathering(&self) -> bool {
        matches!(
            self,
            Skill::Mining | Skill::Woodcutting | Skill::Fishing | Skill::Alchemy
        )
    }
}

impl std::str::FromStr for Skill {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fishing" => Ok(Skill::Fishing),
            "cooking" => Ok(Skill::Cooking),
            "gearcrafting" => Ok(Skill::Gearcrafting),
            "mining" => Ok(Skill::Mining),
            "jewelrycrafting" => Ok(Skill::Jewelrycrafting),
            "weaponcrafting" => Ok(Skill::Weaponcrafting),
            "woodcutting" => Ok(Skill::Woodcutting),
            "alchemy" => Ok(Skill::Alchemy),
            "fighting" => Ok(Skill::Fighting),
            _ => Err(format!("Unknown skill: {}", s)),
        }
    }
}
