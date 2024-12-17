use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

pub fn sort(arr: &mut [String]) {
    let seed: u64 = get_random_seed();

    let mut hashed_strings: Vec<(u64, String)> = arr.iter()
        .map(|s| (random_hash(s, seed), s.clone()))
        .collect();

    hashed_strings.sort_by(|a, b| a.0.cmp(&b.0));

    for (i, (_, value)) in hashed_strings.iter().enumerate() {
        arr[i] = value.clone();
    }
}

fn random_hash<T: Hash>(t: &T, seed: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(seed);
    t.hash(&mut hasher);
    hasher.finish()
}

// Simple implementation of random seed by system time
// not cryptographically secure but works
fn get_random_seed() -> u64 {
    let start = SystemTime::now();
    let duration = start.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_secs() ^ u64::MAX
}
