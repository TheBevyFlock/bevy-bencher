mod types;

use self::types::*;
use bevy_ecs::prelude::*;
use criterion::{criterion_group, BatchSize, Criterion};
use rand::prelude::*;
use std::{hint::black_box, iter::repeat};

pub fn world_spawn(c: &mut Criterion) {
    c.bench_function("world_spawn", |b| {
        let mut world = World::new();

        b.iter(|| {
            world.spawn((A(0), B(0)));
        });
    });
}

pub fn world_query_iter(c: &mut Criterion) {
    c.bench_function("world_query_iter", |b| {
        let mut world = World::new();

        world.spawn_batch(repeat((A(0), B(0))).take(64));
        world.spawn_batch(repeat(A(0)).take(64));
        world.spawn_batch(repeat(B(0)).take(64));

        b.iter(|| {
            for a in world.query::<&A>().iter(&world) {
                // Pretend we're doing something with `a` so this look is not optimized away.
                black_box(a);
            }
        });
    });
}

pub fn world_get_entity(c: &mut Criterion) {
    c.bench_function("world_get_entity", |b| {
        let mut world = World::new();
        let mut prng = crate::create_prng();
        let ids: Vec<Entity> = world.spawn_batch(repeat((A(0), B(0))).take(64)).collect();

        b.iter_batched(
            || ids.choose(&mut prng).unwrap(),
            |&input| {
                black_box(world.get_entity(input));
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(group, world_spawn, world_query_iter, world_get_entity);
