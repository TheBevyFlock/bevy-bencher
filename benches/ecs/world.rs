use super::types::*;
use bevy::ecs::prelude::*;
use criterion::{BatchSize, Criterion};
use rand::prelude::*;
use std::{hint::black_box, iter::repeat};

/// Benchmarks spawning an entity into a [`World`].
pub fn spawn(c: &mut Criterion) {
    c.bench_function("ecs::world::spawn", |b| {
        let mut world = World::new();

        b.iter(|| {
            world.spawn((A(0), B(0)));
        });
    });
}

/// Benchamrks spawning a batch of entities into a [`World`].
pub fn spawn_batch(c: &mut Criterion) {
    c.bench_function("ecs::world::spawn_batch", |b| {
        let mut world = World::new();

        b.iter(|| {
            world.spawn_batch(repeat((A(0), (B(0)))).take(50));
        });
    });
}

/// Benchmarks despawning an entity in a [`World`].
pub fn despawn(c: &mut Criterion) {
    c.bench_function("ecs::world::despawn", |b| {
        let mut world = World::new();

        let unsafe_world_cell = world.as_unsafe_world_cell();

        b.iter_batched(
            || {
                // SAFETY: The `&mut World` returned by this is always dropped before the routine
                // accesses the world.
                let world = unsafe { unsafe_world_cell.world_mut() };

                world.spawn_batch(repeat((A(0), B(0))).take(64))
            },
            |input| {
                // SAFETY: The `&mut World` return by this is always dropped before the setup
                // acceses the world.
                let world = unsafe { unsafe_world_cell.world_mut() };

                for entity in input {
                    world.despawn(entity);
                }
            },
            BatchSize::SmallInput,
        );
    });
}

/// Benchmarks iterating over all matching entities within a [`World`].
pub fn query_iter(c: &mut Criterion) {
    c.bench_function("ecs::world::query_iter", |b| {
        let mut world = World::new();

        // Spawn some with `A`, some with `A` and `B`, and some without `A`.
        world.spawn_batch(repeat((A(0), B(0))).take(64));
        world.spawn_batch(repeat(A(0)).take(64));
        world.spawn_batch(repeat(B(0)).take(64));

        b.iter(|| {
            for a in world.query::<&A>().iter(&world) {
                // Pretend we're doing something with `a` so this is not optimized away.
                black_box(a);
            }
        });
    });
}

pub fn get_entity(c: &mut Criterion) {
    c.bench_function("ecs::world::get_entity", |b| {
        let mut world = World::new();
        let mut prng = crate::create_prng();

        // Spawn 64 entities and store their IDs in a list.
        let ids: Vec<Entity> = world.spawn_batch(repeat((A(0), B(0))).take(64)).collect();

        b.iter_batched(
            // Pick a random ID from the list, outside of the benchmark.
            || ids.choose(&mut prng).unwrap(),
            |&input| {
                // Get the entity from the world. The input is random to hopefully escape any
                // locality optimizations. We use `.get_entity()` instead of `.entity()` to avoid
                // an extra `.unwrap()` call, though the result will never be `None`.
                black_box(world.get_entity(input));
            },
            BatchSize::SmallInput,
        );
    });
}
