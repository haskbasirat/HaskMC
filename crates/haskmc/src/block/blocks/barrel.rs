use std::sync::Arc;
use std::sync::Mutex;

use crate::block::{GetComparatorOutputArgs, OnPlaceArgs, PlacedArgs};
use crate::block::{
    registry::BlockActionResult,
    {BlockBehaviour, NormalUseArgs},
};

use crate::block::entities::barrel::BarrelBlockEntity;
use crate::entity::EntityBase;
use haskmc_data::BlockStateId;
use haskmc_data::block_properties::{BarrelLikeProperties, BlockProperties};
use haskmc_data::translation;
use haskmc_inventory::generic_container_screen_handler::create_generic_9x3;
use haskmc_inventory::player::player_inventory::PlayerInventory;
use haskmc_inventory::screen_handler::{
    InventoryPlayer, ScreenHandlerFactory, SharedScreenHandler,
};
use haskmc_macros::haskmc_block;
use haskmc_util::text::TextComponent;
use haskmc_world::inventory::Inventory;

struct BarrelScreenFactory(Arc<dyn Inventory>);

impl ScreenHandlerFactory for BarrelScreenFactory {
    fn create_screen_handler(
        &self,
        sync_id: u8,
        player_inventory: &Arc<PlayerInventory>,
        _player: &dyn InventoryPlayer,
    ) -> Option<SharedScreenHandler> {
        let handler = create_generic_9x3(sync_id, player_inventory, self.0.clone());
        let concrete_arc = Arc::new(Mutex::new(handler));

        Some(concrete_arc as SharedScreenHandler)
    }

    fn get_display_name(&self) -> TextComponent {
        haskmc_macros::translate_cross!(
            translation::java::CONTAINER_BARREL,
            translation::bedrock::CONTAINER_BARREL
        )
    }
}

#[haskmc_block("minecraft:barrel")]
pub struct BarrelBlock;

impl BlockBehaviour for BarrelBlock {
    fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut props = BarrelLikeProperties::default(args.block);
        props.facing = args.player.get_entity().get_facing().opposite();
        props.to_state_id(args.block)
    }

    fn normal_use(&self, args: NormalUseArgs<'_>) -> BlockActionResult {
        if let Some(block_entity) = args.world.get_block_entity(args.position)
            && let Some(inventory) = block_entity.get_inventory()
        {
            args.player.increment_stat(
                haskmc_data::statistic::StatisticCategory::Custom,
                haskmc_data::statistic::CustomStatistic::OpenBarrel as i32,
                1,
            );
            args.player
                .open_handled_screen(&BarrelScreenFactory(inventory), Some(*args.position));
        }

        BlockActionResult::Success
    }

    fn placed(&self, args: PlacedArgs<'_>) {
        let barrel_block_entity = BarrelBlockEntity::new(*args.position);
        args.world.add_block_entity(Arc::new(barrel_block_entity));
    }

    fn get_comparator_output(&self, args: GetComparatorOutputArgs<'_>) -> Option<u8> {
        if let Some(block_entity) = args.world.get_block_entity(args.position)
            && let Some(inventory) = block_entity.get_inventory()
        {
            Some(crate::block::calculate_comparator_output(
                inventory.as_ref(),
            ))
        } else {
            None
        }
    }
}
