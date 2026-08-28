use crate::block::entities::conduit::ConduitBlockEntity;
use crate::block::{BlockBehaviour, OnPlaceArgs, PlacedArgs};
use haskmc_data::BlockStateId;
use haskmc_data::block_properties::BlockProperties;
use haskmc_macros::haskmc_block;
use std::sync::Arc;

#[haskmc_block("minecraft:conduit")]
pub struct ConduitBlock;

impl BlockBehaviour for ConduitBlock {
    fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props =
            haskmc_data::block_properties::MangroveRootsLikeProperties::default(args.block);
        props.r#waterlogged = args.replacing.water_source();

        props.to_state_id(args.block)
    }

    fn placed(&self, args: PlacedArgs<'_>) {
        {
            let entity = ConduitBlockEntity::new(*args.position);
            args.world.add_block_entity(Arc::new(entity));
        }
    }
}
