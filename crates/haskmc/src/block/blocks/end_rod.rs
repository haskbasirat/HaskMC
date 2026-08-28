use crate::block::{BlockBehaviour, OnPlaceArgs};
use haskmc_data::Block;
use haskmc_data::BlockStateId;
use haskmc_data::block_properties::BlockProperties;
use haskmc_data::block_properties::EndRodLikeProperties;
use haskmc_macros::haskmc_block;

#[haskmc_block("minecraft:end_rod")]
pub struct EndRodBlock;

impl BlockBehaviour for EndRodBlock {
    fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props = EndRodLikeProperties::default(args.block);

        let blockstate = args
            .world
            .get_block_state_id(&args.position.offset(args.direction.to_offset()));

        if Block::from_state_id(blockstate).eq(args.block)
            && EndRodLikeProperties::from_state_id(blockstate, args.block).facing
                == args.direction.to_facing().opposite()
        {
            props.facing = args.direction.to_facing();
        } else {
            props.facing = args.direction.to_facing().opposite();
        }

        props.to_state_id(args.block)
    }
}
