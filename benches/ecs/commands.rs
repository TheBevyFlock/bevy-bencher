use super::types::*;
use bevy::ecs::{prelude::*, world::CommandQueue};
use criterion::Criterion;
use std::{hint::black_box, iter::repeat};

/// Benchmark that flushes an empty [`CommandQueue`], applying it to the world.
pub fn empty_queue(c: &mut Criterion) {
    c.bench_function("ecs::commands::apply_queue", |b| {
        let mut world = World::new();
        let mut queue = CommandQueue::default();

        b.iter(move || {
            queue.apply(&mut world);
        });
    });
}

/// Benchmark that pushes a command that does nothing to the queue, then applies it.
pub fn nothing(c: &mut Criterion) {
    fn nothing(_: &mut World) {}

    c.bench_function("ecs::commands::empty", |b| {
        let mut world = World::new();
        let mut queue = CommandQueue::default();

        b.iter(move || {
            let mut commands = Commands::new(&mut queue, &world);
            commands.push(black_box(nothing));
            queue.apply(&mut world);
        });
    });
}

/// Benchmark that spawns an entity using [`Commands`].
///
/// This is meant to be compared with [`world::spawn`](super::world::spawn).
pub fn spawn(c: &mut Criterion) {
    c.bench_function("ecs::commands::spawn", |b| {
        let mut world = World::new();
        let mut queue = CommandQueue::default();

        b.iter(move || {
            let mut commands = Commands::new(&mut queue, &world);
            commands.spawn((A(0), B(0)));
            queue.apply(&mut world);
        });
    });
}

/// Benchamrks spawning a batch of entities using [`Commands`].
///
/// This is meant to be compared with [`world::spawn_batch`](super::world::spawn_batch).
pub fn spawn_batch(c: &mut Criterion) {
    c.bench_function("ecs::commands::spawn_batch", |b| {
        let mut world = World::new();
        let mut queue = CommandQueue::default();

        b.iter(move || {
            let mut commands = Commands::new(&mut queue, &world);
            commands.spawn_batch(repeat((A(0), (B(0)))).take(50));
            queue.apply(&mut world);
        });
    });
}
