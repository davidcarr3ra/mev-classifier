use classifier_core::ClassifiableTransaction;

use super::ActionTrait;

impl ActionTrait for ClassifiableTransaction {
    fn recurse_during_classify(&self) -> bool {
        true
    }

    fn is_document_root(&self) -> bool {
        true
    }

    fn serializable(&self) -> bool {
        true
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "transaction",
            "signature": self.signature.to_string(),
        })
    }
}
