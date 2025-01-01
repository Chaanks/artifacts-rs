use serde::{Deserialize, Serialize};

use crate::api::schema::r#type::{
    bank::BankGold,
    character::{Character, Cooldown},
    fight::Fight,
    item::{Item, ItemComponent, ItemDetails, ItemSlot, Recycle},
    map::Map,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterMovementData {
    /// Cooldown details.
    pub cooldown: Cooldown,
    /// Destination details.
    pub destination: Map,
    /// Character details.
    pub character: Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterRestData {
    /// Cooldown details.
    pub cooldown: Cooldown,
    /// The amount of HP restored.
    pub hp_restored: u32,
    /// Character details.
    pub character: Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterEquipData {
    /// Cooldown details.
    pub cooldown: Cooldown,
    /// Item slot.
    pub slot: ItemSlot,
    /// Item details.
    pub item: Item,
    /// Character details.
    pub character: Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterUseItemData {
    /// Cooldown details.
    pub cooldown: Cooldown,
    /// Item details.
    pub item: Item,
    /// Character details.
    pub character: Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterFightData {
    /// Cooldown details
    pub cooldown: Cooldown,
    /// fight details.
    pub fight: Fight,
    /// Character details.
    pub character: Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterGatherData {
    /// Cooldown details.
    pub cooldown: Cooldown,
    /// item details.
    pub details: ItemDetails,
    /// Character details.
    pub character: Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterCraftData {
    /// Cooldown details.
    pub cooldown: Cooldown,
    /// item details.
    pub details: ItemDetails,
    /// Character details.
    pub character: Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterGoldTransactionData {
    /// Cooldown details.
    pub cooldown: Cooldown,
    /// Bank details.
    pub bank: BankGold,
    /// Character details.
    pub character: Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterItemTransactionData {
    /// Cooldown details.
    pub cooldown: Cooldown,
    /// Item details.
    pub item: Item,
    /// Bank details.
    pub bank: Vec<ItemComponent>,
    /// Character details.
    pub character: Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterRecycleData {
    /// Cooldown details.
    pub cooldown: Cooldown,
    /// reclycling details.
    pub details: Recycle,
    /// Character details.
    pub character: Character,
}
