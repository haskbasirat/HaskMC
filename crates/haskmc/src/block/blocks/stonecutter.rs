use crate::block::registry::BlockActionResult;
use crate::block::{BlockBehaviour, NormalUseArgs};

use haskmc_data::translation;
use haskmc_inventory::player::player_inventory::PlayerInventory;
use haskmc_inventory::screen_handler::{
    InventoryPlayer, ScreenHandlerFactory, SharedScreenHandler,
};
use haskmc_macros::haskmc_block;
use haskmc_util::text::TextComponent;
use std::sync::Arc;
use std::sync::Mutex;

use haskmc_inventory::stonecutter_screen_handler::StonecutterScreenHandler;

#[haskmc_block("minecraft:stonecutter")]
pub struct StonecutterBlock;

impl BlockBehaviour for StonecutterBlock {
    fn normal_use(&self, args: NormalUseArgs<'_>) -> BlockActionResult {
        args.player.increment_stat(
            haskmc_data::statistic::StatisticCategory::Custom,
            haskmc_data::statistic::CustomStatistic::InteractWithStonecutter as i32,
            1,
        );
        args.player
            .open_handled_screen(&StonecutterScreenFactory, Some(*args.position));

        BlockActionResult::Success
    }
}

struct StonecutterScreenFactory;

impl ScreenHandlerFactory for StonecutterScreenFactory {
    fn create_screen_handler(
        &self,
        sync_id: u8,
        player_inventory: &Arc<PlayerInventory>,
        _player: &dyn InventoryPlayer,
    ) -> Option<SharedScreenHandler> {
        let handler: SharedScreenHandler = Arc::new(Mutex::new(StonecutterScreenHandler::new(
            sync_id,
            player_inventory,
        )));
        Some(handler)
    }

    fn get_display_name(&self) -> TextComponent {
        haskmc_macros::translate_cross!(
            translation::java::CONTAINER_STONECUTTER,
            translation::bedrock::CONTAINER_STONECUTTER
        )
    }
}
