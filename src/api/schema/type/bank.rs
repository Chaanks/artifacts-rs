use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BankGold {
    /// The quantity of the item.
    pub quantity: u32,
}
