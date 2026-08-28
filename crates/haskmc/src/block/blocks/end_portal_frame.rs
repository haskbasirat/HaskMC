use haskmc_data::BlockStateId;
use haskmc_data::block_properties::BlockProperties;
use haskmc_macros::haskmc_block;

use crate::{
    block::{BlockBehaviour, OnPlaceArgs},
    entity::EntityBase,
};

type EndPortalFrameProperties = haskmc_data::block_properties::EndPortalFrameLikeProperties;

#[haskmc_block("minecraft:end_portal_frame")]
pub struct EndPortalFrameBlock;

impl BlockBehaviour for EndPortalFrameBlock {
    fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut end_portal_frame_props = EndPortalFrameProperties::default(args.block);
        end_portal_frame_props.facing = args.player.get_entity().get_horizontal_facing().opposite();

        end_portal_frame_props.to_state_id(args.block)
    }
}
