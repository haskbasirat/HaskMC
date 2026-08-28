use crate::block::registry::BlockActionResult;
use crate::block::{BlockBehaviour, NormalUseArgs};

use haskmc_data::translation;
use haskmc_inventory::player::player_inventory::PlayerInventory;
use haskmc_inventory::screen_handler::{
    InventoryPlayer, ScreenHandlerFactory, SharedScreenHandler,
};
use haskmc_inventory::smithing_table_screen_handler::SmithingTableScreenHandler;
use haskmc_macros::haskmc_block;
use haskmc_util::text::TextComponent;
use std::sync::Arc;
use std::sync::Mutex;

#[haskmc_block("minecraft:smithing_table")]
pub struct SmithingTableBlock;

impl BlockBehaviour for SmithingTableBlock {
    fn normal_use(&self, args: NormalUseArgs<'_>) -> BlockActionResult {
        args.player.increment_stat(
            haskmc_data::statistic::StatisticCategory::Custom,
            haskmc_data::statistic::CustomStatistic::InteractWithSmithingTable as i32,
            1,
        );
        args.player
            .open_handled_screen(&SmithingTableScreenFactory, Some(*args.position));

        BlockActionResult::Success
    }
}

struct SmithingTableScreenFactory;

impl ScreenHandlerFactory for SmithingTableScreenFactory {
    fn create_screen_handler(
        &self,
        sync_id: u8,
        player_inventory: &Arc<PlayerInventory>,
        _player: &dyn InventoryPlayer,
    ) -> Option<SharedScreenHandler> {
        let handler: SharedScreenHandler = Arc::new(Mutex::new(SmithingTableScreenHandler::new(
            sync_id,
            player_inventory,
        )));
        Some(handler)
    }

    fn get_display_name(&self) -> TextComponent {
        TextComponent::translate(translation::java::CONTAINER_UPGRADE, [])
    }
}
