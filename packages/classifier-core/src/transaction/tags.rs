use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub enum TransactionTag {
    AtomicArbitrage(AtomicArbitrageTag),
    SandwichAttack(SandwichAttackTag),
}

impl TransactionTag {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            TransactionTag::AtomicArbitrage(tag) => tag.to_json(),
            TransactionTag::SandwichAttack(tag) => tag.to_json(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AtomicArbitrageTag {
    pub mint: Pubkey,
    pub profit_amount: i128,
}

impl AtomicArbitrageTag {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "atomicArbitrage",
            "mint": self.mint.to_string(),
            "profitAmount": self.profit_amount,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SandwichAttackTag {
    pub mint: Pubkey,
    pub profit_amount: i128,
}

impl SandwichAttackTag {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "sandwichAttack",
            "mint": self.mint.to_string(),
            "profitAmount": self.profit_amount,
        })
    }
}