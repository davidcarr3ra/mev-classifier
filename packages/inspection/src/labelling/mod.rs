use action_tree::ActionTree;

mod dex_swap;

pub fn label_tree(tree: &mut ActionTree) {
    dex_swap::classify_dex_swaps(tree);
}