use fxhash::hash64;

pub fn hash_key(key: &[u8]) -> u64 {
    hash64(key)
}
