mod commands;
mod types;
mod world;

use criterion::criterion_group;

criterion_group!(
    group,
    commands::empty_queue,
    commands::nothing,
    commands::spawn,
    commands::spawn_batch,
    world::spawn,
    world::spawn_batch,
    world::despawn,
    world::query_iter,
    world::get_entity
);
