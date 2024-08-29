use solana_sdk::{instruction::CompiledInstruction, message::VersionedMessage, pubkey::Pubkey};

pub const ID: Pubkey = solana_sdk::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

pub fn process_instruction(msg: &VersionedMessage, ix: &CompiledInstruction) {
    println!("Processing instruction for whirlpools");
}
