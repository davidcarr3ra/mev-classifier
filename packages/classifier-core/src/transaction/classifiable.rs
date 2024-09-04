use std::str::FromStr;

use solana_sdk::{
    message::v0::LoadedAddresses, pubkey::Pubkey, signature::Signature,
    transaction::{TransactionError, VersionedTransaction},
};
use solana_transaction_status::{
    UiInnerInstructions, UiLoadedAddresses, UiTransactionStatusMeta, UiTransactionTokenBalance,
};

use super::instruction::ClassifiableInstruction;

#[derive(Debug, Clone)]
pub struct ClassifiableTransaction {
    pub signature: Signature,
    pub status: Result<(), TransactionError>,
    pub instructions: Vec<ClassifiableInstruction>,
    pub pre_token_balances: Option<Vec<UiTransactionTokenBalance>>,
    pub post_token_balances: Option<Vec<UiTransactionTokenBalance>>,

    static_keys: Vec<Pubkey>,
    loaded_addresses: Option<LoadedAddresses>,
}

impl PartialEq for ClassifiableTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature
    }
}

impl Eq for ClassifiableTransaction {}

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

        let signature = txn.signatures.first().unwrap().clone();
        
        Self {
            signature,
            status: meta.status,
            instructions: classifiable_instructions,
            static_keys,
            loaded_addresses,
            pre_token_balances: meta.pre_token_balances.into(),
            post_token_balances: meta.post_token_balances.into(),
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
                    Err(err) => {
                        tracing::trace!("Failed to decode inner instruction: {:?}", err);
                        return None;
                    }
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

    pub fn get_index_for_pubkey(&self, pubkey: &Pubkey) -> Option<u8> {
        if let Some(index) = self.static_keys.iter().position(|k| k == pubkey) {
            return Some(index as u8);
        }

        if let Some(loaded_addresses) = &self.loaded_addresses {
            if let Some(index) = loaded_addresses.writable.iter().position(|k| k == pubkey) {
                return Some((index + self.static_keys.len()) as u8);
            }

            if let Some(index) = loaded_addresses.readonly.iter().position(|k| k == pubkey) {
                return Some(
                    (index + self.static_keys.len() + loaded_addresses.writable.len()) as u8,
                );
            }
        }

        None
    }

    pub fn get_pre_token_balance(
        &self,
        pubkey: &Pubkey,
    ) -> Result<UiTransactionTokenBalance, anyhow::Error> {
        let index = self.get_index_for_pubkey(pubkey).ok_or_else(|| {
            anyhow::anyhow!("Could not find pubkey {:?} in loaded addresses", pubkey)
        })?;

        if let Some(pre_balances) = &self.pre_token_balances {
            for balance in pre_balances {
                if balance.account_index == index {
                    return Ok(balance.clone());
                }
            }
        } else {
            return Err(anyhow::anyhow!("No pre token balances found"));
        }

        Err(anyhow::anyhow!(
            "Could not find pre token balance for pubkey {:?}",
            pubkey
        ))
    }

    pub fn get_post_token_balance(
        &self,
        pubkey: &Pubkey,
    ) -> Result<UiTransactionTokenBalance, anyhow::Error> {
        let index = self.get_index_for_pubkey(pubkey).ok_or_else(|| {
            anyhow::anyhow!("Could not find pubkey {:?} in loaded addresses", pubkey)
        })?;

        if let Some(post_balances) = &self.post_token_balances {
            for balance in post_balances {
                if balance.account_index == index {
                    return Ok(balance.clone());
                }
            }
        } else {
            return Err(anyhow::anyhow!("No post token balances found"));
        }

        Err(anyhow::anyhow!(
            "Could not find post token balance for pubkey {:?}",
            pubkey
        ))
    }

    pub fn get_mint_for_token_account(&self, pubkey: &Pubkey) -> Result<Pubkey, anyhow::Error> {
        let balance = self.get_pre_token_balance(pubkey)?;

        Ok(Pubkey::from_str(&balance.mint).unwrap())
    }
}
