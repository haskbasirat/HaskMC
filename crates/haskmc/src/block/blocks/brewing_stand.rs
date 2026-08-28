use std::sync::Arc;
use std::sync::Mutex;

use crate::block::{GetComparatorOutputArgs, PlacedArgs};
use crate::block::{
    registry::BlockActionResult,
    {BlockBehaviour, NormalUseArgs},
};

use crate::block::entities::brewing_stand::BrewingStandBlockEntity;
use haskmc_data::translation;
use haskmc_inventory::player::player_inventory::PlayerInventory;
use haskmc_inventory::screen_handler::{
    InventoryPlayer, ScreenHandlerFactory, SharedScreenHandler,
};
use haskmc_macros::haskmc_block;
use haskmc_util::text::TextComponent;
use haskmc_world::inventory::Inventory;

struct BrewingScreenFactory(
    Arc<dyn Inventory>,
    Arc<dyn crate::block::entities::PropertyDelegate>,
);

impl ScreenHandlerFactory for BrewingScreenFactory {
    fn create_screen_handler(
        &self,
        sync_id: u8,
        player_inventory: &Arc<PlayerInventory>,
        _player: &dyn InventoryPlayer,
    ) -> Option<SharedScreenHandler> {
        let inventory = self.0.clone();
        haskmc_inventory::brewing::create_brewing(sync_id, player_inventory, inventory, &self.1)
            .map(|handler| Arc::new(Mutex::new(handler)) as SharedScreenHandler)
    }

    fn get_display_name(&self) -> TextComponent {
        haskmc_macros::translate_cross!(
            translation::java::CONTAINER_BREWING,
            translation::bedrock::CONTAINER_BREWING
        )
    }
}

#[haskmc_block("minecraft:brewing_stand")]
pub struct BrewingStandBlock;

impl BlockBehaviour for BrewingStandBlock {
    fn normal_use(&self, args: NormalUseArgs<'_>) -> BlockActionResult {
        if let Some(block_entity) = args.world.get_block_entity(args.position)
            && let Some(inventory) = block_entity.clone().get_inventory()
            && let Some(pd) = block_entity.clone().to_property_delegate()
        {
            args.player.increment_stat(
                haskmc_data::statistic::StatisticCategory::Custom,
                haskmc_data::statistic::CustomStatistic::InteractWithBrewingstand as i32,
                1,
            );
            args.player
                .open_handled_screen(&BrewingScreenFactory(inventory, pd), Some(*args.position));
        }

        BlockActionResult::Success
    }

    fn placed(&self, args: PlacedArgs<'_>) {
        let be = BrewingStandBlockEntity::new(*args.position);
        args.world.add_block_entity(Arc::new(be));
    }

    fn get_comparator_output(&self, args: GetComparatorOutputArgs<'_>) -> Option<u8> {
        if let Some(block_entity) = args.world.get_block_entity(args.position)
            && let Some(inventory) = block_entity.get_inventory()
        {
            let mut bottles = 0u8;
            // Bottle slots are 0, 1, 2 in brewing stands
            for slot in 0..3 {
                let stack = inventory.get_stack(slot);
                if !stack.is_empty() {
                    bottles += 1;
                }
            }
            Some(bottles)
        } else {
            None
        }
    }
}
