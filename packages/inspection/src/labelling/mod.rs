use actions::ActionTree;

mod atomic_arbitrage;
mod dex_swap;

pub fn label_tree(tree: &mut ActionTree) {
    let root = tree.root();

    dex_swap::classify_dex_swaps(root, tree);
    atomic_arbitrage::classify_atomic_arbitrage(root, tree);
}
