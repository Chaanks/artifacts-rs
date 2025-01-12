use serde::{Deserialize, Serialize};

use super::character::Skill;

/// A type alias for an item code, which is represented as a `String` with match pattern `^[a-zA-Z0-9_-]+$`.
pub type ItemCode = String;

/// Represents an item.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub code: ItemCode,
    pub level: u32,
    #[serde(rename = "type")]
    pub r#type: ItemType,
    #[serde(rename = "subtype")] // Corrected from sub_type to subtype
    pub sub_type: Option<ItemSubType>,
    pub description: String,
    pub effects: Vec<ItemEffect>, // Need to handle this better
    pub craft: Option<Craft>,
}

/// Represents a resource.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub code: ItemCode,
    pub skill: Skill,
    pub level: i32,
    pub drops: Vec<ResourceDrop>,
}

/// Represents a resource drop.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceDrop {
    pub code: ItemCode,
    pub rate: u32,
    pub min_quantity: u32,
    pub max_quantity: u32,
}

/// Represents craft information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Craft {
    pub skill: Skill,
    pub level: u32,
    pub items: Vec<ItemComponent>,
    pub quantity: u32,
}

/// Represents recycle information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recycle {
    pub items: Vec<ItemComponent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemEffect {
    pub name: String,
    pub value: i32,
}

impl PartialEq for ItemEffect {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ItemEffect {}

impl std::hash::Hash for ItemEffect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// Represents an item drop.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ItemComponent {
    /// The code of the item.
    pub code: ItemCode,
    /// The quantity of the item.
    pub quantity: u32,
}

/// Represents the specific subtypes of items that can exist within the game.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemSubType {
    /// A mask item subtype.
    Mask,
    /// A helm item subtype.
    Helm,
    /// A dagger weapon subtype.
    Dagger,
    /// A staff weapon subtype.
    Staff,
    /// A sword weapon subtype.
    Sword,
    /// A bow weapon subtype.
    Bow,
    /// A tool subtype.
    Tool,
    /// A whip weapon subtype.
    Whip,
    /// An axe weapon subtype.
    Axe,
    /// A wand weapon subtype.
    Wand,
    /// A subtype representing mining resources.
    Mining,
    /// A subtype related to mobs, potentially for mob-specific items or drops.
    Mob,
    /// A subtype related to woodcutting resources.
    WoodCutting,
    /// A subtype related to fishing resources.
    Fishing,
    /// A food item subtype, typically consumable by the character.
    Food,
    /// A bar item subtype, representing a refined metal or resource.
    Bar,
    /// A plank item subtype, representing a transformed wood or ressource.
    Plank,
    /// An alloy item subtype.
    Alloy,
    /// A coat item subtype.
    Coat,
    #[serde(other)]
    Unknown,
}

/// Represents the various types of items that can exist within the game.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    /// An item that can be equipped as a helmet.
    Helmet,
    /// An item that can be used as a weapon.
    Weapon,
    /// A resource item, typically used for crafting or other activities.
    Resource,
    /// A consumable item that can be used or consumed by the character.
    Consumable,
    /// An item that can be equipped as boots.
    Boots,
    /// An item representing currency, used for transactions within the game.
    Currency,
    /// An item that can be equipped as a shield.
    Shield,
    /// An item that can be equipped as a ring.
    Ring,
    /// An item that can be equipped as body armor.
    #[serde(rename = "body_armor")]
    BodyArmor,
    /// An item that can be equipped as leg armor.
    #[serde(rename = "leg_armor")]
    LegArmor,
    /// An item that can be equipped as an amulet.
    Amulet,
    /// An item that can be equipped as artifact.
    Artifact,
    /// An item that can be consumed by the character.
    Food,
    /// An item that can be used by the character.
    Utility,
}

impl ItemType {
    /// Get possible slots for this item type
    pub fn possible_slots(&self) -> Vec<ItemSlot> {
        match self {
            ItemType::Helmet => vec![ItemSlot::Helmet],
            ItemType::Weapon => vec![ItemSlot::Weapon],
            ItemType::Shield => vec![ItemSlot::Shield],
            ItemType::Ring => vec![ItemSlot::Ring1, ItemSlot::Ring2],
            ItemType::BodyArmor => vec![ItemSlot::BodyArmor],
            ItemType::LegArmor => vec![ItemSlot::LegArmor],
            ItemType::Boots => vec![ItemSlot::Boots],
            ItemType::Amulet => vec![ItemSlot::Amulet],
            ItemType::Artifact => vec![
                ItemSlot::Artifact1,
                ItemSlot::Artifact2,
                ItemSlot::Artifact3,
            ],
            ItemType::Consumable | ItemType::Food | ItemType::Utility => {
                vec![ItemSlot::Utility1, ItemSlot::Utility2]
            }
            _ => vec![],
        }
    }
}

/// Represents the various item slots available for a character.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemSlot {
    /// The weapon slot.
    Weapon,
    /// The shield slot.
    Shield,
    /// The helmet slot.
    Helmet,
    /// The body armor slot.
    BodyArmor,
    /// The leg armor slot.
    LegArmor,
    /// The boots slot.
    Boots,
    /// The first ring slot.
    Ring1,
    /// The second ring slot.
    Ring2,
    /// The amulet slot.
    Amulet,
    /// The first artifact slot.
    Artifact1,
    /// The second artifact slot.
    Artifact2,
    /// The third artifact slot.
    Artifact3,
    /// The first consumable slot.
    Utility1,
    /// The second consumable slot.
    Utility2,
    /// Undefined slot.
    Undefined,
}

impl ItemSlot {
    pub fn to_item_type(&self) -> ItemType {
        match self {
            ItemSlot::Weapon => ItemType::Weapon,
            ItemSlot::Shield => ItemType::Shield,
            ItemSlot::Helmet => ItemType::Helmet,
            ItemSlot::BodyArmor => ItemType::BodyArmor,
            ItemSlot::LegArmor => ItemType::LegArmor,
            ItemSlot::Boots => ItemType::Boots,
            ItemSlot::Ring1 | ItemSlot::Ring2 => ItemType::Ring,
            ItemSlot::Amulet => ItemType::Amulet,
            ItemSlot::Artifact1 | ItemSlot::Artifact2 | ItemSlot::Artifact3 => ItemType::Artifact,
            _ => panic!("Unhandled item slot!"),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ItemDetails {
    /// The amount of xp gained.
    pub xp: i32,
    /// Objects received.
    pub items: Vec<ItemComponent>,
}
