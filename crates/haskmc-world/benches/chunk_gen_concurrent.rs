#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use criterion::{Criterion, criterion_group, criterion_main};
use haskmc_data::BlockStateId;
use haskmc_data::dimension::Dimension;
use haskmc_util::world_seed::Seed;
use haskmc_world::chunk_system::{StagedChunkEnum, generate_single_chunk};
use haskmc_world::generation::get_world_gen;
use haskmc_world::world::WorldPortalExt;
use std::hint::black_box;
use std::sync::Arc;
use std::time::Instant;

const SEED: Seed = Seed(42);
const THREAD_COUNT: usize = 4;
const GRID_SIZE: i32 = 4;

// Stub portal — allows all block placements, skips mob spawning.
struct BlockRegistry;
impl WorldPortalExt for BlockRegistry {
    fn can_place_at(
        &self,
        _block: &haskmc_data::Block,
        _state: &haskmc_data::BlockState,
        _block_accessor: &dyn haskmc_world::world::BlockAccessor,
        _block_pos: &haskmc_util::math::position::BlockPos,
    ) -> bool {
        true
    }

    fn mirror(
        &self,
        block: &haskmc_data::Block,
        state_id: BlockStateId,
        mirror: haskmc_data::Mirror,
    ) -> &'static haskmc_data::BlockState {
        block.mirror(state_id, mirror)
    }

    fn rotate(
        &self,
        block: &haskmc_data::Block,
        state_id: BlockStateId,
        rotation: haskmc_data::Rotation,
    ) -> &'static haskmc_data::BlockState {
        block.rotate(state_id, rotation)
    }

    fn spawn_mobs_for_chunk_generation(
        &self,
        _cache: &mut dyn haskmc_world::generation::proto_chunk::GenerationCache,
        _biome: &'static haskmc_data::chunk::Biome,
        _chunk_x: i32,
        _chunk_z: i32,
    ) {
    }
}

/// Benchmarks the full pipeline for a `GRID_SIZE^2` chunks grid running in parallel
/// across `THREAD_COUNT` rayon threads.
fn bench_concurrent_chunk_generation(c: &mut Criterion) {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(THREAD_COUNT)
        .thread_name(|i| format!("bench-chunk-{i}"))
        .build()
        .expect("Failed to build rayon thread pool");

    let world_gen = Arc::new(get_world_gen(
        SEED,
        Dimension::OVERWORLD,
        false,
        Vec::new(),
        String::new(),
    ));
    let block_registry = Arc::new(BlockRegistry);

    let bench_name = format!(
        "concurrent_full_pipeline_{}_chunks_{}_threads",
        GRID_SIZE * GRID_SIZE,
        THREAD_COUNT
    );

    c.bench_function(&bench_name, |b| {
        b.iter_custom(|iters| {
            let mut total = std::time::Duration::ZERO;

            for _ in 0..iters {
                let wg = world_gen.clone();
                let br = block_registry.clone();

                let start = Instant::now();
                pool.scope(|s| {
                    for cx in 0..GRID_SIZE {
                        for cz in 0..GRID_SIZE {
                            let wg = wg.clone();
                            let br = br.clone();
                            s.spawn(move |_| {
                                black_box(generate_single_chunk(
                                    &wg,
                                    br.as_ref(),
                                    cx,
                                    cz,
                                    StagedChunkEnum::Full,
                                ));
                            });
                        }
                    }
                });
                total += start.elapsed();
            }

            total
        });
    });
}

criterion_group!(benches, bench_concurrent_chunk_generation);
criterion_main!(benches);
