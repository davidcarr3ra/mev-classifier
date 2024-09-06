use actions::{Action, AtomicArbitrage, Block, DexSwap};
use classifier_core::ClassifiableTransaction;
use mongodb::bson;
use solana_sdk::pubkey::Pubkey;

fn pubkey_to_bson(pubkey: &Pubkey) -> bson::Binary {
    bson::Binary {
        subtype: bson::spec::BinarySubtype::Generic,
        bytes: pubkey.to_bytes().to_vec(),
    }
}

pub trait MongoSerialize {
    fn metadata_bson(&self) -> Option<bson::Document> {
        None
    }
}

impl MongoSerialize for Block {
    fn metadata_bson(&self) -> Option<bson::Document> {
        let mut document = bson::doc! {
            "_id": self.slot as i64,
            "block_time": self.block_time,
        };

        if let Some(total_base_fees) = self.total_base_fees {
            document.insert(
                "total_base_fees",
                bson::Binary {
                    subtype: bson::spec::BinarySubtype::Generic,
                    bytes: total_base_fees.to_be_bytes().to_vec(),
                },
            );
        }

        if let Some(total_priority_fees) = self.total_priority_fees {
            document.insert(
                "total_priority_fees",
                bson::Binary {
                    subtype: bson::spec::BinarySubtype::Generic,
                    bytes: total_priority_fees.to_be_bytes().to_vec(),
                },
            );
        }

        if let Some(total_tips) = self.total_tips {
            document.insert(
                "total_tips",
                bson::Binary {
                    subtype: bson::spec::BinarySubtype::Generic,
                    bytes: total_tips.to_be_bytes().to_vec(),
                },
            );
        }

        Some(document)
    }
}

impl MongoSerialize for Action {
    fn metadata_bson(&self) -> Option<bson::Document> {
        match self {
            Action::Block(block) => block.metadata_bson(),
            Action::ClassifiableTransaction(tx) => tx.metadata_bson(),
            Action::AtomicArbitrage(arbitrage) => arbitrage.metadata_bson(),
            Action::DexSwap(swap) => swap.metadata_bson(),
            _ => None,
        }
    }
}

impl MongoSerialize for ClassifiableTransaction {
    fn metadata_bson(&self) -> Option<bson::Document> {
        let bytes: [u8; 64] = self.signature.into();

        Some(bson::doc! {
            "signature": bson::Binary {
                subtype: bson::spec::BinarySubtype::Generic,
                bytes: bytes.to_vec(),
            },
        })
    }
}

impl MongoSerialize for AtomicArbitrage {
    fn metadata_bson(&self) -> Option<bson::Document> {
        Some(bson::doc! {
            "type": "AtomicArbitrage",
            "mint": pubkey_to_bson(&self.mint),
        })
    }
}

impl MongoSerialize for DexSwap {
    fn metadata_bson(&self) -> Option<bson::Document> {
        Some(bson::doc! {
            "type": "DexSwap",
            "input_mint": pubkey_to_bson(&self.input_mint),
            "output_mint": pubkey_to_bson(&self.output_mint),
        })
    }
}
