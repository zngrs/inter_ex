use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub fn get_from_seed(seed: u64, length: i32, min: u32, max: u32) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    for _ in 0..length {
        let random_number: u32 = rng.gen_range(min..=max);
        arr.push(random_number as i32);
    }
    return arr;
}
