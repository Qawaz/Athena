use blake3::Hasher;

pub fn hash_string(string: String) -> String {
    let mut hasher = Hasher::new();

    hasher.update(&string.as_bytes());

    hasher
        .finalize()
        .to_hex()
        .chars()
        .collect::<String>()
        .to_string()
}