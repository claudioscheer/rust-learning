use rand::Rng;

pub fn random_n() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=100);
}
