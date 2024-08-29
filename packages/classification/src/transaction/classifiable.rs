use std::str::FromStr;

use solana_sdk::{message::v0::LoadedAddresses, pubkey::Pubkey, transaction::VersionedTransaction};
use solana_transaction_status::{UiInnerInstructions, UiLoadedAddresses, UiTransactionStatusMeta};

use super::instruction::ClassifiableInstruction;

#[derive(Debug)]
pub struct ClassifiableTransaction {
    pub instructions: Vec<ClassifiableInstruction>,
    static_keys: Vec<Pubkey>,
    loaded_addresses: Option<LoadedAddresses>,
}

impl ClassifiableTransaction {
    pub fn new(txn: VersionedTransaction, meta: UiTransactionStatusMeta) -> Self {
        let instructions = txn.message.instructions().to_owned();
        let mut inner_instructions =
            Option::<Vec<UiInnerInstructions>>::from(meta.inner_instructions);

        let mut classifiable_instructions = Vec::with_capacity(
            instructions.len() + inner_instructions.as_ref().map_or(0, |v| v.len()),
        );

        for (idx, ix) in instructions.into_iter().enumerate() {
            classifiable_instructions.push(ClassifiableInstruction::from_compiled(ix, 1));

            // Add any inner instructions if they exist
            if let Some(inner_instructions) = inner_instructions.as_mut() {
                match Self::find_and_pop_inners(inner_instructions, idx) {
                    Some(inners) => classifiable_instructions.extend(inners),
                    None => {}
                }
            }
        }

        // Load any ALT addresses needed
        let loaded_addresses = if let Some(meta_loaded_addresses) =
            Option::<UiLoadedAddresses>::from(meta.loaded_addresses)
        {
            Some(LoadedAddresses {
                readonly: meta_loaded_addresses
                    .readonly
                    .iter()
                    .map(|k| Pubkey::from_str(k).unwrap())
                    .collect(),
                writable: meta_loaded_addresses
                    .writable
                    .iter()
                    .map(|k| Pubkey::from_str(k).unwrap())
                    .collect(),
            })
        } else {
            None
        };

        let static_keys = txn.message.static_account_keys().into();

        Self {
            instructions: classifiable_instructions,
            static_keys,
            loaded_addresses,
        }
    }

    /// If inner instructions exist for the given index, remove them from the list and return them
    /// in decoded format.
    ///
    /// If any of the inner instructions cannot be decoded, return None.
    fn find_and_pop_inners(
        inner_instructions: &mut Vec<UiInnerInstructions>,
        idx: usize,
    ) -> Option<Vec<ClassifiableInstruction>> {
        let inners_idx = inner_instructions
            .iter()
            .position(|inner| inner.index == idx as u8);

        if let Some(inners_idx) = inners_idx {
            let mut inners = inner_instructions.remove(inners_idx);
            let mut ui_instructions = Vec::with_capacity(inners.instructions.len());

            while inners.instructions.len() > 0 {
                let inner = inners.instructions.pop().unwrap();

                let decoded = match ClassifiableInstruction::from_ui(inner) {
                    Ok(decoded) => decoded,
                    Err(_) => return None,
                };

                ui_instructions.push(decoded);
            }

            return Some(ui_instructions);
        }

        None
    }

    pub fn get_pubkey(&self, index: u8) -> Option<Pubkey> {
        let mut index = index as usize;

        if index < self.static_keys.len() {
            return Some(self.static_keys[index]);
        }

        if let Some(loaded_addresses) = &self.loaded_addresses {
            index -= self.static_keys.len();

            if index < loaded_addresses.writable.len() {
                return Some(loaded_addresses.writable[index]);
            }

            index -= loaded_addresses.writable.len();

            if index < loaded_addresses.readonly.len() {
                return Some(loaded_addresses.readonly[index]);
            }
        }

        None
    }
}
