// todo: figure out why the anchor proc macro isn't working for phoenix v1

pub struct PhoenixV1Classifier;
use solana_sdk::pubkey::Pubkey;
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use classifier_trait::{ClassifyInstructionResult, InstructionClassifier};
use actions::{Action, PhoenixV1Action};
use borsh::BorshDeserialize;

#[derive(BorshDeserialize, Debug)]
#[repr(u8)]
pub enum Side {
    Bid = 0,
    Ask = 1,
}

#[derive(BorshDeserialize, Debug)]
#[repr(u8)]
enum SelfTradeBehavior {
    Abort = 0,
    CancelProvide = 1,
    DecrementTake = 2,
}

#[derive(BorshDeserialize, Debug)]
enum OrderPacket {
    PostOnly {
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        client_order_id: u128,
        reject_post_only: bool,
        use_only_deposited_funds: bool,
        last_valid_slot: Option<u64>,
        last_valid_unix_timestamp_in_seconds: Option<u64>,
        fail_silently_on_insufficient_funds: bool,
    },
    Limit {
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
        use_only_deposited_funds: bool,
        last_valid_slot: Option<u64>,
        last_valid_unix_timestamp_in_seconds: Option<u64>,
        fail_silently_on_insufficient_funds: bool,
    },
    ImmediateOrCancel {
        side: Side,
        price_in_ticks: Option<u64>,
        num_base_lots: u64,
        num_quote_lots: u64,
        min_base_lots_to_fill: u64,
        min_quote_lots_to_fill: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
        use_only_deposited_funds: bool,
        // last_valid_slot: Option<u64>,
        // last_valid_unix_timestamp_in_seconds: Option<u64>,
    },
}

impl InstructionClassifier for PhoenixV1Classifier {
    const ID: Pubkey = solana_sdk::pubkey!("PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY");

    fn classify_instruction(_txn: &ClassifiableTransaction, ix: &ClassifiableInstruction) -> ClassifyInstructionResult {
        let (&tag, rest) = ix
            .data
            .split_first()
            .ok_or_else(|| anyhow::anyhow!("Invalid Phoenix V1 instruction data"))?;
        
        match tag {
            0 => classify_swap(ix, rest),
            _ => Ok(None),
        }
    }
}

fn classify_swap(ix: &ClassifiableInstruction, rest: &[u8]) -> ClassifyInstructionResult {
    if ix.accounts.len() < 9 {
        return Err(anyhow::anyhow!(
            "Invalid Phoenix V1 swap instruction: not enough accounts"
        ));
    }

    // Log the entire byte slice length before deserialization.
    println!("Data length for OrderPacket deserialization: {}", rest.len());

    // Deserialize directly from the provided slice.
    let order_packet = OrderPacket::deserialize(&mut &rest[..])
        .map_err(|e| anyhow::anyhow!("Failed to deserialize OrderPacket: {}", e))?;

    println!("Order packet: {:?}", order_packet);

		Ok(Some(Action::PhoenixV1Action(PhoenixV1Action::Swap)))

    // // Match on the order packet variant and extract side and amount traded.
    // match order_packet {
    //     OrderPacket::ImmediateOrCancel {
    //         side,
    //         num_base_lots,
    //         num_quote_lots,
    //         ..
    //     } => {
    //         // For demonstration, we assume:
    //         // If it's a Bid, the traded amount is the quote lots.
    //         // If it's an Ask, the traded amount is the base lots.
    //         let amount_traded = match side {
    //             Side::Bid => num_quote_lots,
    //             Side::Ask => num_base_lots,
    //         };
    //         println!("Extracted side: {:?}, amount traded: {}", side, amount_traded);
    //         Ok(Some(Action::PhoenixV1Action(PhoenixV1Action::Swap { side, amount_traded })))
    //     },
    //     _ => Err(anyhow::anyhow!("Unsupported order packet variant")),
    // }
}