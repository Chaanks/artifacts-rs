use artifacts_rs::api::schema::{
    response::my_characters::{
        CharacterCraftData, CharacterEquipData, CharacterFightData, CharacterGatherData,
        CharacterGoldTransactionData, CharacterItemTransactionData, CharacterMovementData,
        CharacterRecycleData, CharacterRestData, CharacterUseItemData,
    },
    SchemaWrapper,
};
use std::fs;

fn load_test_data(file_name: &str) -> String {
    let path = format!("tests/data/my_characters/{}", file_name);
    fs::read_to_string(path).expect("Failed to read test data file")
}

// println!(
//     "{:?}",
//     serde_json::from_str::<SchemaWrapper<...>>(&data)
// );

#[test]
fn test_character_movement() {
    let data = load_test_data("character_movement.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterMovementData>>(&data).is_ok());
}

#[test]
fn test_character_rest() {
    let data = load_test_data("character_rest.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterRestData>>(&data).is_ok());
}

#[test]
fn test_character_equip() {
    let data = load_test_data("character_equip.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterEquipData>>(&data).is_ok());
}

#[test]
fn test_character_use_item() {
    let data = load_test_data("character_use_item.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterUseItemData>>(&data).is_ok());
}

#[test]
fn test_character_fight() {
    let data = load_test_data("character_fight.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterFightData>>(&data).is_ok());
}

#[test]
fn test_character_gather() {
    let data = load_test_data("character_gather.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterGatherData>>(&data).is_ok());
}

#[test]
fn test_character_craft() {
    let data = load_test_data("character_craft.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterCraftData>>(&data).is_ok());
}

#[test]
fn test_character_deposit_bank_gold() {
    let data = load_test_data("character_deposit_bank_gold.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterGoldTransactionData>>(&data).is_ok());
}

#[test]
fn test_character_withdraw_bank_gold() {
    let data = load_test_data("character_withdraw_bank_gold.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterGoldTransactionData>>(&data).is_ok());
}

#[test]
fn test_character_deposit_bank_item() {
    let data = load_test_data("character_deposit_bank_item.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterItemTransactionData>>(&data).is_ok());
}

#[test]
fn test_character_withdraw_bank_item() {
    let data = load_test_data("character_withdraw_bank_item.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterItemTransactionData>>(&data).is_ok());
}

#[test]
fn test_character_recycle() {
    let data = load_test_data("character_recycle.json");
    assert!(serde_json::from_str::<SchemaWrapper<CharacterRecycleData>>(&data).is_ok());
}
