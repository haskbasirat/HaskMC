use std::sync::Arc;

use haskmc_data::entity::EntityType;
use haskmc_macros::haskmc_block_from_tag;
use haskmc_util::GameMode;

use crate::block::BlockBehaviour;
use crate::block::BrokenArgs;
use crate::entity::Entity;

#[haskmc_block_from_tag("c:cobblestones/infested")]
pub struct InfestedBlock;

impl BlockBehaviour for InfestedBlock {
    fn broken(&self, args: BrokenArgs<'_>) {
        {
            // TODO: ugly fix, use onStacksDropped
            if args.player.gamemode.load() == GameMode::Creative {
                return;
            }
            let entity = Entity::new(
                args.world.clone(),
                args.position.0.to_f64(),
                &EntityType::SILVERFISH,
            );

            args.world.spawn_entity(Arc::new(entity));
        }
    }
}
