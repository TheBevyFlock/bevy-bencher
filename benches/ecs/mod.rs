mod types;
mod world;

use criterion::criterion_group;

criterion_group!(
    group,
    world::spawn,
    world::spawn_batch,
    world::despawn,
    world::query_iter,
    world::get_entity
);
