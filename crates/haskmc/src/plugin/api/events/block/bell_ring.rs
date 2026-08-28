use haskmc_data::BlockDirection;
use haskmc_macros::{Event, cancellable};
use haskmc_util::math::position::BlockPos;
use std::sync::Arc;

use crate::{entity::EntityBase, world::World};

/// An event that occurs when a bell is rung.
#[cancellable]
#[derive(Event, Clone)]
pub struct BellRingEvent {
    pub block_pos: BlockPos,
    pub world: Arc<World>,
    pub entity: Option<Arc<dyn EntityBase>>,
    pub direction: Option<BlockDirection>,
}

impl BellRingEvent {
    #[must_use]
    pub const fn new(
        block_pos: BlockPos,
        world: Arc<World>,
        entity: Option<Arc<dyn EntityBase>>,
        direction: Option<BlockDirection>,
    ) -> Self {
        Self {
            block_pos,
            world,
            entity,
            direction,
            cancelled: false,
        }
    }
}
