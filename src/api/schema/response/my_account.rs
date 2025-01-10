use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountBankDetailsData {
    /// Maximum slots in your bank.
    pub slots: u32,
    /// Bank expansions.
    pub expansions: u32,
    /// Next expansion cost.
    pub next_expansion_cost: u32,
    /// Quantity of gold in your bank.
    pub gold: u32,
}
