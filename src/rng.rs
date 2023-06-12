use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;

const PBKDF_ITER: u32 = 4096;

/// Seed for the CSPRNG.
///
/// This seed should be reasonably random itself.
pub type Seed = [u8; 32];

/// Creates a seed from data and a nonce.
///
/// Extends any entropy present in `data` and `nonce` by running many iterations of the
/// [PBKDF2](https://docs.rs/pbkdf2/latest) algorithm.
pub fn seed(data: &[u8], salt: &[u8]) -> Seed {
    pbkdf2_hmac_array::<Sha256, 32>(data, salt, PBKDF_ITER)
}
