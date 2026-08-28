use crate::block::registry::BlockActionResult;
use crate::block::{BlockBehaviour, NormalUseArgs};

use haskmc_macros::haskmc_block;

#[haskmc_block("minecraft:fletching_table")]
pub struct FletchingTableBlock;

impl BlockBehaviour for FletchingTableBlock {
    fn normal_use(&self, _args: NormalUseArgs<'_>) -> BlockActionResult {
        BlockActionResult::Pass
    }
}
