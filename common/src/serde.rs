use hex;
use secp256k1::{RecoveryId, Signature};
use serde::ser::*;

pub fn serialize_signature<S>(
    (signature, recovery_id): &(Signature, RecoveryId),
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // A helper struct to go from `[u8; 64]` to `&[u8]` so that `hex::encode`
    // accepts it.
    struct RawBytes64([u8; 64]);

    impl AsRef<[u8]> for RawBytes64 {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    let bytes = signature.serialize();
    let s = hex::encode(RawBytes64(bytes));
    serializer.serialize_str(format!("0x{}{:x}", s, recovery_id.serialize() + 27).as_str())
}
