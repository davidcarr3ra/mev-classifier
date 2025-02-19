pub struct PhoenixV1Classifier;
use solana_sdk::pubkey::Pubkey;
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use classifier_trait::{ClassifyInstructionResult, InstructionClassifier};
use actions::{Action, PhoenixV1Action};

impl InstructionClassifier for PhoenixV1Classifier {
	const ID: Pubkey = solana_sdk::pubkey!("PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY");

	fn classify_instruction(_txn: &ClassifiableTransaction, ix: &ClassifiableInstruction) -> ClassifyInstructionResult {
		let (&tag, _rest) = ix
			.data
			.split_first()
			.ok_or_else(|| anyhow::anyhow!("Invalid Phoenix V1 instruction data"))?;
		
		match tag {
			0 => Ok(Some(Action::PhoenixV1Action(PhoenixV1Action::Swap))),
			_ => Ok(None),
		}
	}
}