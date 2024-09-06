use classifier_core::ClassifiableTransaction;

use super::ActionTrait;

impl ActionTrait for ClassifiableTransaction {
    fn recurse_during_classify(&self) -> bool {
        true
    }

    fn is_document_root(&self) -> bool {
        true
    }
}
