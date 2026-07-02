use rand::{thread_rng, Rng};

fn main() {
    let length = 16; 
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                  abcdefghijklmnopqrstuvwxyz\
                  0123456789!@#$%^&*()_-+=<>?";

    let mut rng = thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx] as char
        })
        .collect();

    println!("Senha gerada: {}", password);
}
