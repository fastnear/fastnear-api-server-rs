use near_crypto::PublicKey;

pub(crate) fn public_key_to_implicit_account(public_key: &PublicKey) -> Option<String> {
    if let PublicKey::ED25519(pk) = public_key {
        Some(hex::encode(&pk))
    } else {
        None
    }
}
