use std::cell::{OnceCell, UnsafeCell};

use primitive_types::U256;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

static mut SHARED_RAND: OnceCell<UnsafeCell<ChaCha20Rng>> = OnceCell::new();

#[allow(static_mut_refs)]
pub fn get_rng() -> &'static mut ChaCha20Rng {
    unsafe {
        _ = SHARED_RAND.set(ChaCha20Rng::from_os_rng().into());
        SHARED_RAND.get_mut().unwrap().get_mut()
    }
}

#[allow(static_mut_refs)]
pub fn set_seed(seed: U256) {
    let seed_bytes = seed.to_little_endian();
    unsafe {
        let seeded_rng = ChaCha20Rng::from_seed(seed_bytes);
        if SHARED_RAND.set(seeded_rng.clone().into()).is_err() {
            *SHARED_RAND.get_mut().unwrap().get_mut() = ChaCha20Rng::from_seed(seed_bytes);
        }
    }
}

pub fn get_seed() -> U256 {
    let seed_bytes = get_rng().get_seed();
    U256::from_little_endian(&seed_bytes)
}
