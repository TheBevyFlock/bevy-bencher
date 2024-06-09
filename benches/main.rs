mod ecs;

use criterion::criterion_main;
use rand::{prelude::*, rngs::SmallRng};

pub(crate) fn create_prng() -> impl Rng {
    // A small and fast psuedo-random number generator. This is not reproducible across 32-bit
    // and 64-bit platforms, but that shouldn't be a problem here. The seed was chosen somewhat
    // randomly, but modified to have a nice distribution of 1s and 0s.
    SmallRng::seed_from_u64(0x7df09deb486e920a)
}

criterion_main!(ecs::group);
