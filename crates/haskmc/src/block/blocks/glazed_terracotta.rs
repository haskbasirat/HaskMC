use crate::block::{BlockBehaviour, OnPlaceArgs};
use haskmc_data::BlockStateId;
use haskmc_data::block_properties::{BlockProperties, WallTorchLikeProperties};
use haskmc_macros::haskmc_block_from_tag;

#[haskmc_block_from_tag("minecraft:glazed_terracotta")]
pub struct GlazedTerracottaBlock;

impl BlockBehaviour for GlazedTerracottaBlock {
    fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut prop = WallTorchLikeProperties::default(args.block);
        prop.facing = args
            .player
            .living_entity
            .entity
            .get_horizontal_facing()
            .opposite();
        prop.to_state_id(args.block)
    }
}
