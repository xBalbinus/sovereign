use sov_modules_api::Hasher;

/// Derives token address from `token_name`, `sender` and `salt`.
pub fn create_token_address<C: sov_modules_api::Context>(
    token_name: &str,
    sender: &[u8],
    salt: u64,
) -> C::Address {
    let mut hasher = C::Hasher::new();
    hasher.update(sender.as_ref());
    hasher.update(token_name.as_bytes());
    hasher.update(&salt.to_le_bytes());

    let hash = hasher.finalize();
    C::Address::from(hash)
}
