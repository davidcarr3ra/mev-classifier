// todo: figure out why the anchor proc macro isn't working for phoenix v1

pub struct PhoenixV1Classifier;
use actions::{Action, PhoenixV1Action, SelfTradeBehavior, Side, SwapAction::ImmediateOrCancel};
use borsh::BorshDeserialize;
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use classifier_trait::{ClassifyInstructionResult, InstructionClassifier};
use solana_sdk::pubkey::Pubkey;

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

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
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
    println!(
        "Data length for OrderPacket deserialization: {}",
        rest.len()
    );

    // Deserialize directly from the provided slice.
    let order_packet = OrderPacket::deserialize(&mut &rest[..])
        .map_err(|e| anyhow::anyhow!("Failed to deserialize OrderPacket: {}", e))?;

    println!("Order packet: {:?}", order_packet);

    match order_packet {
        // todo: add post only and limit
        OrderPacket::ImmediateOrCancel {
            side,
            price_in_ticks,
            num_base_lots,
            num_quote_lots,
            min_base_lots_to_fill,
            min_quote_lots_to_fill,
            self_trade_behavior,
            match_limit,
            client_order_id,
            use_only_deposited_funds,
            ..
        } => Ok(Some(Action::PhoenixV1Action(PhoenixV1Action::Swap(
            ImmediateOrCancel {
                side,
                price_in_ticks,
                num_base_lots,
                num_quote_lots,
                min_base_lots_to_fill,
                min_quote_lots_to_fill,
                self_trade_behavior,
                match_limit,
                client_order_id,
                use_only_deposited_funds,
            },
        )))),
        _ => Ok(None),
    }
}
