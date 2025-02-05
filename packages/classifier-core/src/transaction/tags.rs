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

// Todo: add signatures, timestamps
#[derive(Debug, Clone)]
pub enum SandwichAttackTag {
    Frontrun {
        token_bought: Pubkey,
        amount: u64,
        attacker_pubkey: Pubkey,
    },
    Victim {
        token_bought: Pubkey,
        amount: u64,
        victim_pubkey: Pubkey,
    },
    Backrun {
        token_sold: Pubkey,
        amount: u64,
        attacker_pubkey: Pubkey,
        profit_amount: i64,
    },
}

impl SandwichAttackTag {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            SandwichAttackTag::Frontrun { token_bought, amount, attacker_pubkey } => {
                serde_json::json!({
                    "type": "sandwich_frontrun",
                    "tokenBought": token_bought.to_string(),
                    "amount": amount,
                    "attackerPubkey": attacker_pubkey.to_string(),
                })
            },
            SandwichAttackTag::Victim { token_bought, amount, victim_pubkey } => {
                serde_json::json!({
                    "type": "sandwich_victim",
                    "tokenBought": token_bought.to_string(),
                    "amount": amount,
                    "victimPubkey": victim_pubkey.to_string(),
                })
            },
            SandwichAttackTag::Backrun { token_sold, amount, attacker_pubkey, profit_amount } => {
                serde_json::json!({
                    "type": "sandwich_backrun",
                    "tokenSold": token_sold.to_string(),
                    "amount": amount,
                    "attackerPubkey": attacker_pubkey.to_string(),
                    "profitAmount": profit_amount,
                })
            },
        }
    }
}
