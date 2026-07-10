use rand::{Rng};
use rand::rngs::OsRng;

pub fn generate_password(length: usize, charset: &[u8]) -> String {
    let mut rng = OsRng;
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}