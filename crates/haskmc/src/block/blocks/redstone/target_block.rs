use haskmc_macros::haskmc_block;

use crate::block::{BlockBehaviour, EmitsRedstonePowerArgs};

#[haskmc_block("minecraft:target")]
pub struct TargetBlock;

impl BlockBehaviour for TargetBlock {
    fn emits_redstone_power(&self, _args: EmitsRedstonePowerArgs<'_>) -> bool {
        true
    }
}
