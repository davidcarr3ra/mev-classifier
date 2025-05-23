use crate::ActionTrait;
use macros::action;
use solana_sdk::pubkey::Pubkey;

// https://jito-foundation.gitbook.io/mev/mev-payment-and-distribution/on-chain-addresses
pub const JITO_TIP_ADDRESSES: [Pubkey; 8] = [
    solana_sdk::pubkey!("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5"),
    solana_sdk::pubkey!("HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe"),
    solana_sdk::pubkey!("Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY"),
    solana_sdk::pubkey!("ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49"),
    solana_sdk::pubkey!("DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh"),
    solana_sdk::pubkey!("ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt"),
    solana_sdk::pubkey!("DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL"),
    solana_sdk::pubkey!("3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT"),
];

pub fn is_jito_tip_address(address: &Pubkey) -> bool {
    JITO_TIP_ADDRESSES.contains(address)
}

#[action]
pub struct JitoTip {
    pub tipper: Pubkey,
    pub tip_amount: u64,
}

impl JitoTip {
    pub fn new(tipper: Pubkey, tip_amount: u64) -> Self {
        Self { tipper, tip_amount }
    }
}

impl ActionTrait for JitoTip {
    fn recurse_during_classify(&self) -> bool {
        false
    }

    fn serializable(&self) -> bool {
        true
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "jitoTip",
            "tipper": self.tipper.to_string(),
            "tipAmount": self.tip_amount,
        })
    }
}
