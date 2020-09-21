use secp256k1::PublicKey;

pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
    use tiny_keccak::{Hasher, Keccak};
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    hasher.finalize(&mut output);
    output
}

pub fn hash_message(msg: &Vec<u8>) -> [u8; 32] {
    const PREFIX: &str = "\x19Ethereum Signed Message:\n";
    let mut eth_msg = format!("{}{}", PREFIX, msg.len()).into_bytes();
    eth_msg.extend_from_slice(msg.as_slice());
    keccak256(&eth_msg)
}

pub fn public_key_to_address(public_key: PublicKey) -> Vec<u8> {
    let uncompressed = public_key.serialize();
    debug_assert_eq!(uncompressed[0], 0x04);
    let hash = keccak256(&uncompressed[1..]);
    hash[12..].into()
}

pub fn checksum_address(address: Vec<u8>) -> String {
    let s = hex::encode(&address);
    let unchecksummed = s.as_bytes();
    let hash = keccak256(unchecksummed);

    let mut result = String::with_capacity(42);
    result.push('0');
    result.push('x');
    result.extend(unchecksummed.into_iter().enumerate().map(|(i, c)| {
        let mut hash_byte = hash[i / 2];
        if i % 2 == 0 {
            hash_byte = hash_byte >> 4;
        } else {
            hash_byte = hash_byte & 0xf;
        }
        if *c > 57 && hash_byte > 7 {
            char::from(c - 32)
        } else {
            char::from(*c)
        }
    }));

    result
}
