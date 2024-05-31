mod ecs;

use criterion::criterion_main;
use rand::{rngs::SmallRng, prelude::*};

pub(crate) fn create_prng() -> impl Rng {
    SmallRng::seed_from_u64(0x7df09deb486e920a)
}

criterion_main!(ecs::group);
