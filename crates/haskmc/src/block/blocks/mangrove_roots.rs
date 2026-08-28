use crate::block::{BlockBehaviour, GetStateForNeighborUpdateArgs, OnPlaceArgs};
use haskmc_data::BlockStateId;
use haskmc_data::block_properties::{BlockProperties, MangroveRootsLikeProperties};
use haskmc_data::fluid::Fluid;
use haskmc_macros::haskmc_block;
use haskmc_world::tick::TickPriority;

#[haskmc_block("minecraft:mangrove_roots")]
pub struct MangroveRootsBlock;

impl BlockBehaviour for MangroveRootsBlock {
    fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props = MangroveRootsLikeProperties::default(args.block);
        props.waterlogged = args.replacing.water_source();
        props.to_state_id(args.block)
    }

    fn get_state_for_neighbor_update(
        &self,
        args: GetStateForNeighborUpdateArgs<'_>,
    ) -> BlockStateId {
        let props = MangroveRootsLikeProperties::from_state_id(args.state_id, args.block);
        if props.waterlogged {
            args.world.schedule_fluid_tick(
                &Fluid::WATER,
                *args.position,
                Fluid::WATER.flow_speed as u8,
                TickPriority::Normal,
            );
        }
        props.to_state_id(args.block)
    }
}
