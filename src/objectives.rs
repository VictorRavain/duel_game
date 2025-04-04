use rand::Rng;

pub fn generate_objectives(count: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(0..=100)).collect()
}
