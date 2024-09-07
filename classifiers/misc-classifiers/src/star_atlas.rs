use actions::{Action, StarAtlasAction};
use classifier_core::{ClassifiableInstruction, ClassifiableTransaction};
use classifier_trait::{ClassifyInstructionResult, InstructionClassifier};
use solana_sdk::pubkey::Pubkey;

pub struct StarAtlasGalacticMarketplaceClassifier;

impl InstructionClassifier for StarAtlasGalacticMarketplaceClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasSAGEClassifier;

impl InstructionClassifier for StarAtlasSAGEClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasCraftingClassifier;

impl InstructionClassifier for StarAtlasCraftingClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("CRAFT2RPXPJWCEix4WpJST3E7NLf79GTqZUL75wngXo5");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasCargoClassifier;

impl InstructionClassifier for StarAtlasCargoClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasPlayerProfileClassifier;

impl InstructionClassifier for StarAtlasPlayerProfileClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasProfileVaultClassifier;

impl InstructionClassifier for StarAtlasProfileVaultClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("pv1ttom8tbyh83C1AVh6QH2naGRdVQUVt3HY1Yst5sv");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasProfileFactionClassifier;

impl InstructionClassifier for StarAtlasProfileFactionClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasPointsClassifier;

impl InstructionClassifier for StarAtlasPointsClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("Point2iBvz7j5TMVef8nEgpmz4pDr7tU7v3RjAfkQbM");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasPointsStoreClassifier;

impl InstructionClassifier for StarAtlasPointsStoreClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("PsToRxhEPScGt1Bxpm7zNDRzaMk31t8Aox7fyewoVse");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasPrimeClassifier;

impl InstructionClassifier for StarAtlasPrimeClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("APR1MEny25pKupwn72oVqMH4qpDouArsX8zX4VwwfoXD");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasClaimStakesClassifier;

impl InstructionClassifier for StarAtlasClaimStakesClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("STAKEr4Bh8sbBMoAVmTDBRqouPzgdocVrvtjmhJhd65");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasSCOREClassifier;

impl InstructionClassifier for StarAtlasSCOREClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("FLEET1qqzpexyaDpqb2DGsSzE2sDCizewCg9WjrA6DBW");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasSAGEEscapeVelocityClassifier;

impl InstructionClassifier for StarAtlasSAGEEscapeVelocityClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("TESTWCwvEv2idx6eZVQrFFdvEJqGHfVA1soApk2NFKQ");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasDAOProxyRewarderClassifier;

impl InstructionClassifier for StarAtlasDAOProxyRewarderClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("gateVwTnKyFrE8nxUUgfzoZTPKgJQZUbLsEidpG4Dp2");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasLockerClassifier;

impl InstructionClassifier for StarAtlasLockerClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("ATLocKpzDbTokxgvnLew3d7drZkEzLzDpzwgrgWKDbmc");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasPolisLockerClassifier;

impl InstructionClassifier for StarAtlasPolisLockerClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("Lock7kBijGCQLEFAmXcengzXKA88iDNQPriQ7TbgeyG");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasPolisLockerSnapshotsClassifier;

impl InstructionClassifier for StarAtlasPolisLockerSnapshotsClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("snapNQkxsiqDWdbNfz8KVB7e3NPzLwtHHA6WV8kKgUc");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}

pub struct StarAtlasFactionEnlistmentClassifier;

impl InstructionClassifier for StarAtlasFactionEnlistmentClassifier {
    const ID: Pubkey = solana_sdk::pubkey!("FACTNmq2FhA2QNTnGM2aWJH3i7zT3cND5CgvjYTjyVYe");

    fn classify_instruction(
        _txn: &ClassifiableTransaction,
        _ix: &ClassifiableInstruction,
    ) -> ClassifyInstructionResult {
        Ok(Action::StarAtlasAction(StarAtlasAction {}).into())
    }
}
