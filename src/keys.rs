//! Keys for encrypted communication over `eSSP`.

use aes::cipher::KeySizeUser;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

use crate::{impl_default, make_key, primes::Generator, rng::Seed};

/// All ITL devices use the same static prefixes in their AES encryption keys,
/// effectively shortening the key-space to 64-bits...
pub const DEFAULT_FIXED_KEY: [u8; 8] = [0x67, 0x45, 0x23, 0x01, 0x67, 0x45, 0x23, 0x01];
pub const DEFAULT_FIXED_KEY_U64: u64 = 0x0123456701234567;

fn pow_mod_big(x: u64, y: u64, n: u64) -> u64 {
    use num_bigint::BigUint;
    use num_traits::{FromPrimitive, ToPrimitive};

    let xb = BigUint::from_u64(x).unwrap();
    let yb = BigUint::from_u64(y).unwrap();
    let nb = BigUint::from_u64(n).unwrap();

    xb.modpow(&yb, &nb).to_u64().unwrap_or(0)
}

make_key!(
    GeneratorKey,
    u64,
    "Generator prime key used for `eSSP` key negotiation."
);

impl GeneratorKey {
    /// Generates a new [GeneratorKey] from a prime [Generator].
    pub fn from_generator(gen: &mut Generator) -> Self {
        gen.new_prime().into()
    }

    /// Generates a new [GeneratorKey] from a seed.
    ///
    /// See notes on generating a sufficiently random seed from the
    /// [Generator](crate::primes::Generator::from_seed) documentation.
    pub fn from_seed(seed: Seed) -> Self {
        Generator::from_seed(seed).new_prime().into()
    }

    /// Generates a new [GeneratorKey] from system entropy.
    #[cfg(feature = "std")]
    pub fn from_entropy() -> Self {
        Generator::from_entropy().new_prime().into()
    }
}

make_key!(
    ModulusKey,
    u64,
    "Modulus prime key used for `eSSP` key negotiation."
);

impl ModulusKey {
    /// Generates a new [ModulusKey] from a prime [Generator].
    pub fn from_generator(gen: &mut Generator) -> Self {
        gen.new_prime().into()
    }

    /// Generates a new [ModulusKey] from a seed.
    ///
    /// See notes on generating a sufficiently random seed from the
    /// [Generator](crate::primes::Generator::from_seed) documentation.
    pub fn from_seed(seed: Seed) -> Self {
        Generator::from_seed(seed).new_prime().into()
    }

    /// Generates a new [ModulusKey] from system entropy.
    #[cfg(feature = "std")]
    pub fn from_entropy() -> Self {
        Generator::from_entropy().new_prime().into()
    }
}

make_key!(
    RandomKey,
    u64,
    "Random number used to generate the [IntermediateKey](crate::IntermediateKey)."
);

impl RandomKey {
    /// Generates a [RandomKey] from a seed.
    ///
    /// See notes on generating a sufficiently random seed from the
    /// [Generator](crate::primes::Generator::from_seed) documentation.
    pub fn from_seed(seed: Seed) -> Self {
        ChaCha20Rng::from_seed(seed).next_u64().into()
    }

    /// Generates a [RandomKey] from system entropy.
    #[cfg(feature = "std")]
    pub fn from_entropy() -> Self {
        rand::thread_rng().next_u64().into()
    }
}

make_key!(
    IntermediateKey,
    u64,
    r"
 Calculated key during `eSSP` key negotiation:

 | **Host IntermediateKey** |
 |:-----------------:|
 | ([GENERATOR](crate::GeneratorKey) ^ [HOST_RND](crate::RandomKey)) mod [MODULUS](crate::ModulusKey) |

 | **Device IntermediateKey** |
 |:-------------------:|
 | ([GENERATOR](crate::GeneratorKey) ^ [DEV_RND](crate::RandomKey)) mod [MODULUS](crate::ModulusKey) |
"
);

impl IntermediateKey {
    pub fn from_keys(gen_key: &GeneratorKey, rnd_key: &RandomKey, mod_key: &ModulusKey) -> Self {
        // (GeneratorKey ^ RandomKey) % ModKey
        pow_mod_big(gen_key.as_inner(), rnd_key.as_inner(), mod_key.as_inner()).into()
    }
}

make_key!(
    EncryptionKey,
    u64,
    r"
 Negotiated key used for `eSSP` encrypted communication.

 | **Host Key** |
 |:------------:|
 | ([DEV_INTERKEY](crate::IntermediateKey) ^ [HOST_RND](crate::RandomKey)) mod [MODULUS](crate::ModulusKey) |

 | **Device Key** |
 |:--------------:|
 | ([HOST_INTERKEY](crate::IntermediateKey) ^ [DEV_RND](crate::RandomKey)) mod [MODULUS](crate::ModulusKey) |
"
);

impl EncryptionKey {
    pub fn from_keys(
        inter_key: &IntermediateKey,
        rnd_key: &RandomKey,
        mod_key: &ModulusKey,
    ) -> Self {
        // (InterKey ^ RandomKey) % ModKey
        pow_mod_big(inter_key.as_inner(), rnd_key.as_inner(), mod_key.as_inner()).into()
    }
}

make_key!(FixedKey, u64, r"Fixed part of the `eSSP` encryption key.");

impl FixedKey {
    /// Creates a new [FixedKey] from the default ITL bytes.
    pub const fn new() -> Self {
        Self::from_inner(DEFAULT_FIXED_KEY_U64)
    }

    /// Generates a random [FixedKey] from a seed.
    ///
    /// See notes on generating a sufficiently random seed from the
    /// [Generator](crate::primes::Generator::from_seed) documentation.
    pub fn from_seed(seed: Seed) -> Self {
        ChaCha20Rng::from_seed(seed).next_u64().into()
    }

    /// Generates a random [FixedKey] from system entropy.
    #[cfg(feature = "std")]
    pub fn from_entropy() -> Self {
        rand::thread_rng().next_u64().into()
    }
}

impl_default!(FixedKey);

pub type AesKey = aes::cipher::Key<aes::Aes128>;
pub type AesBlock =
    aes::cipher::generic_array::GenericArray<u8, <aes::Aes128 as KeySizeUser>::KeySize>;

impl From<EncryptionKey> for AesKey {
    fn from(val: EncryptionKey) -> Self {
        let mut key = Self::from(FixedKey::new());

        key[8..].copy_from_slice(val.as_inner().to_le_bytes().as_ref());

        key
    }
}

impl From<FixedKey> for AesKey {
    fn from(val: FixedKey) -> Self {
        (&val).into()
    }
}

#[rustfmt::skip]
impl From<&FixedKey> for AesKey {
    fn from(val: &FixedKey) -> Self {
        let k = val.as_inner().to_le_bytes();

        Self::clone_from_slice(
            [
                k[0], k[1], k[2], k[3], k[4], k[5], k[6], k[7],
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
            .as_ref(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_key() {
        let exp_fixed_key = DEFAULT_FIXED_KEY_U64.to_le_bytes();

        assert_eq!(DEFAULT_FIXED_KEY[..8].as_ref(), exp_fixed_key.as_ref());

        assert_eq!(
            AesKey::from(FixedKey::from_inner(DEFAULT_FIXED_KEY_U64))[..8].as_ref(),
            [0x67, 0x45, 0x23, 0x01, 0x67, 0x45, 0x23, 0x01].as_ref(),
        );
    }

    #[test]
    fn test_key_exchange() {
        let gen = GeneratorKey::from_seed([0xde; 32]);
        let modulus = ModulusKey::from_seed([0xaf; 32]);

        let host_rnd = RandomKey::from_seed([0xaa; 32]);
        let dev_rnd = RandomKey::from_seed([0xbb; 32]);

        let host_inter = IntermediateKey::from_keys(&gen, &host_rnd, &modulus);
        let dev_inter = IntermediateKey::from_keys(&gen, &dev_rnd, &modulus);

        let host_enc = EncryptionKey::from_keys(&dev_inter, &host_rnd, &modulus);
        let dev_enc = EncryptionKey::from_keys(&host_inter, &dev_rnd, &modulus);

        assert_eq!(host_enc, dev_enc);
    }

    #[test]
    fn test_known_keys() {
        let gen = GeneratorKey::from_inner(0x7fcc_9ee3);
        let modulus = ModulusKey::from_inner(0x7f1c_7181);

        let host_rnd = RandomKey::from_inner(0x7f2b_ceec);
        let host_inter = IntermediateKey::from_inner(0xc04_3f46);

        let dev_inter = IntermediateKey::from_inner(0x634c_0016);

        let encrypt_key = EncryptionKey::from_inner(0x7bf4_9046);

        assert_eq!(
            IntermediateKey::from_keys(&gen, &host_rnd, &modulus),
            host_inter
        );
        assert_eq!(
            EncryptionKey::from_keys(&dev_inter, &host_rnd, &modulus),
            encrypt_key
        );

        let dev_inter = IntermediateKey::from_inner(0x04ba466d);
        let host_rnd = RandomKey::from_inner(0x2d61283d);
        let modulus = ModulusKey::from_inner(0x2d469703);

        assert_eq!(
            EncryptionKey::from_keys(&dev_inter, &host_rnd, &modulus),
            EncryptionKey::from_inner(0x1aeda1fb),
        );
    }
}
