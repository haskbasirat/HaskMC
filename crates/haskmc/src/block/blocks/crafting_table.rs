use crate::block::registry::BlockActionResult;
use crate::block::{BlockBehaviour, NormalUseArgs};

use haskmc_data::translation;
use haskmc_inventory::crafting::crafting_screen_handler::CraftingTableScreenHandler;
use haskmc_inventory::player::player_inventory::PlayerInventory;
use haskmc_inventory::screen_handler::{
    InventoryPlayer, ScreenHandlerFactory, SharedScreenHandler,
};
use haskmc_macros::haskmc_block;
use haskmc_util::text::TextComponent;
use std::sync::Arc;
use std::sync::Mutex;

#[haskmc_block("minecraft:crafting_table")]
pub struct CraftingTableBlock;

impl BlockBehaviour for CraftingTableBlock {
    fn normal_use(&self, args: NormalUseArgs<'_>) -> BlockActionResult {
        args.player.increment_stat(
            haskmc_data::statistic::StatisticCategory::Custom,
            haskmc_data::statistic::CustomStatistic::InteractWithCraftingTable as i32,
            1,
        );
        let recipe_manager = args.server.recipe_manager.clone();
        args.player.open_handled_screen(
            &CraftingTableScreenFactory(recipe_manager),
            Some(*args.position),
        );

        BlockActionResult::Success
    }
}

struct CraftingTableScreenFactory(Arc<crate::server::RecipeManager>);

impl ScreenHandlerFactory for CraftingTableScreenFactory {
    fn create_screen_handler(
        &self,
        sync_id: u8,
        player_inventory: &Arc<PlayerInventory>,
        _player: &dyn InventoryPlayer,
    ) -> Option<SharedScreenHandler> {
        let handler =
            CraftingTableScreenHandler::new(sync_id, player_inventory, Some(self.0.clone()));
        let concrete_arc = Arc::new(Mutex::new(handler));

        Some(concrete_arc as SharedScreenHandler)
    }

    fn get_display_name(&self) -> TextComponent {
        haskmc_macros::translate_cross!(
            translation::java::CONTAINER_CRAFTING,
            translation::bedrock::CONTAINER_CRAFTING
        )
    }
}
