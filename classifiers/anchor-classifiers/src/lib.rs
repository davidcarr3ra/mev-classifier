use anchor_lang::{prelude::*, Discriminator};
use macros::declare_anchor_classifier;

declare_anchor_classifier!(whirlpools, Swap, SwapV2);
declare_anchor_classifier!(jupiter_v6, Route);
