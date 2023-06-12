//! Keys for encrypted communication over `eSSP`.

use aes::cipher::KeySizeUser;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

use crate::{impl_default, make_key, primes::Generator, rng::Seed};

/// All ITL devices use the same static prefixes in their AES encryption keys,
/// effectively shortening the key-space to 64-bits...
pub const DEFAULT_FIXED_KEY: [u8; 8] = [0x67, 0x45, 0x23, 0x01, 0x67, 0x45, 0x23, 0x01];

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
        let gk = BigUint::from(gen_key.as_inner());
        let rk = BigUint::from(rnd_key.as_inner());
        let mk = BigUint::from(mod_key.as_inner());

        // (GeneratorKey ^ RandomKey) % ModKey
        //
        // The `unwrap` call will never panic because of the modulo operation.
        //
        // The result will always by representable as `u64`, as long as the modulus is a valid
        // `u64`.
        gk.modpow(&rk, &mk).to_u64().unwrap().into()
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
        let ik = BigUint::from(inter_key.as_inner());
        let rk = BigUint::from(rnd_key.as_inner());
        let mk = BigUint::from(mod_key.as_inner());

        // (InterKey ^ RandomKey) % ModKey
        //
        // The `unwrap` call will never panic because of the modulo operation.
        //
        // The result will always by representable as `u64`, as long as the modulus is a valid
        // `u64`.
        ik.modpow(&rk, &mk).to_u64().unwrap().into()
    }
}

make_key!(FixedKey, u64, r"Fixed part of the `eSSP` encryption key.");

impl FixedKey {
    /// Creates a new [FixedKey] from the default ITL bytes.
    pub const fn new() -> Self {
        Self::from_inner(u64::from_le_bytes(DEFAULT_FIXED_KEY))
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
        let exp_fixed_key = 0x0123456701234567u64.to_le_bytes();

        assert_eq!(DEFAULT_FIXED_KEY[..8].as_ref(), exp_fixed_key.as_ref());
    }
}
