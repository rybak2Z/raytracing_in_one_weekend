use rand::{self, rngs::ThreadRng, Rng};

static mut RNG: Option<ThreadRng> = None;

/// This function should be called before any of the other functions of this
/// module are called.
pub fn initialize() {
    unsafe {
        RNG = Some(rand::thread_rng());
    }
}

/// Before calling this function, "initialize" should have been called before.
pub fn random() -> f32 {
    unsafe {
        // The unwrap works as "initialize" should have been called at this point
        RNG.as_mut().unwrap().gen()
    }
}

/// Before calling this function, "initialize" should have been called before.
pub fn random_range(min: f32, max: f32) -> f32 {
    unsafe {
        // The unwrap works as "initialize" should have been called at this point
        RNG.as_mut().unwrap().gen_range(min..max)
    }
}
