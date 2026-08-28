use haskmc_data::BlockStateId;
use haskmc_data::block_properties::BlockProperties;
use haskmc_macros::haskmc_block_from_tag;

use crate::block::BlockBehaviour;
use crate::block::OnPlaceArgs;

type LogProperties = haskmc_data::block_properties::PaleOakWoodLikeProperties;

#[haskmc_block_from_tag("minecraft:logs")]
pub struct LogBlock;

impl BlockBehaviour for LogBlock {
    fn on_place(&self, args: OnPlaceArgs<'_>) -> BlockStateId {
        let mut log_props = LogProperties::default(args.block);
        log_props.axis = args.direction.to_axis();

        log_props.to_state_id(args.block)
    }
}
