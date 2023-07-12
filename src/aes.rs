//! AES implementation from the ITL SSP SDK
//!
//! Minor modifications for style, and clarity, otherwise a direct translation from the original C.

use crate::{Error, Result};

pub const MAX_KEY_LENGTH: usize = 16;
pub const AES_KEY: usize = 16;
pub const AES_BLOCK: usize = 16;
pub const NUMBER_ROUNDS: usize = 10;

pub const WORD_ALL: u32 = 0xffffffff;
pub const WORD_MSB: u32 = 0x80000000;
pub const WORD_ZERO: u32 = 0x00000000;
pub const WORD_ONE: u32 = 0x00000001;

/// Define the GF2^8 irreducible field polynomial 0x11B = x^8 + x^4 + x^3 + x + 1
pub const GF2_8_FIELD_POLYNOMIAL: u8 = 0x1B;

/// forward S-box = SubBytes() transformation
#[rustfmt::skip]
pub static FORWARD_S_BOX: [u8; 256] = [
  0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5,
  0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
  0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0,
  0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
  0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC,
  0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
  0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A,
  0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
  0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0,
  0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
  0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B,
  0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
  0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85,
  0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
  0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5,
  0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
  0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17,
  0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
  0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88,
  0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
  0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C,
  0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
  0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9,
  0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
  0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6,
  0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
  0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E,
  0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
  0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94,
  0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
  0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68,
  0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16
];

/// Inverse S-box = InvSubBytes() transformation
#[rustfmt::skip]
pub static INVERSE_S_BOX: [u8; 256] = [
  0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38,
  0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
  0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87,
  0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
  0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D,
  0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
  0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2,
  0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
  0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16,
  0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
  0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA,
  0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
  0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A,
  0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
  0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02,
  0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
  0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA,
  0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
  0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85,
  0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
  0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89,
  0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
  0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20,
  0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
  0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31,
  0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
  0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D,
  0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
  0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0,
  0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
  0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26,
  0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D
];

/// Round constants = [2^i in GF(2^8)]
#[rustfmt::skip]
pub static RCON: [u8; 10] = [
  0x01, 0x02, 0x04, 0x08, 0x10,
  0x20, 0x40, 0x80, 0x1B, 0x36
];

pub const fn gf2_8_field_mult_by_2(a: u8) -> u8 {
    // mult by 2, if MSB was 1 then reduce by field polynomial
    let r = [a << 1, (a << 1) ^ GF2_8_FIELD_POLYNOMIAL];

    // this is for SPA resistance
    r[((a & 0x80) != 0x0) as usize]
}

/// Performs a forward S-box lookup.
pub fn forward_sub_byte(input: usize) -> u8 {
    FORWARD_S_BOX[input % FORWARD_S_BOX.len()]
}

/// Performs an inverse S-box lookup.
pub fn inverse_sub_byte(input: usize) -> u8 {
    INVERSE_S_BOX[input % INVERSE_S_BOX.len()]
}

/// Get the zeroth byte (big-endian) of a 32-bit word.
pub const fn byte_0(w: u32) -> u8 {
    ((w >> 24) & 0xff) as u8
}

/// Get the first byte (big-endian) of a 32-bit word.
pub const fn byte_1(w: u32) -> u8 {
    ((w >> 16) & 0xff) as u8
}

/// Get the second byte (big-endian) of a 32-bit word.
pub const fn byte_2(w: u32) -> u8 {
    ((w >> 8) & 0xff) as u8
}

/// Get the third byte (big-endian) of a 32-bit word.
pub const fn byte_3(w: u32) -> u8 {
    (w & 0xff) as u8
}

/// Concatenates 4 x 8-bit words to one 32-bit word (big-endian).
pub const fn concat_4_bytes(b: [u8; 4]) -> u32 {
    u32::from_be_bytes(b)
}

/// Splits a 32-bit word into 4 x 8-bit words (big-endian).
pub const fn split_into_4_bytes(w: u32) -> [u8; 4] {
    w.to_be_bytes()
}

fn split_into_4_bytes_inplace(w: u32, out: &mut [u8]) {
    if out.len() < 4 {
        return;
    }

    for (o, &b) in out[..4].iter_mut().zip(w.to_be_bytes().iter()) {
        *o = b;
    }
}

/// Performs the forward S-box substitution.
pub const fn forward_mix_col(state_in: u32) -> u32 {
    // t = a[0] + a[1] + a[2] + a[3]
    let t = byte_0(state_in) ^ byte_1(state_in) ^ byte_2(state_in) ^ byte_3(state_in);

    // v = a[0] + a[1]
    // v = x*v
    // r[0] = a[0] + v + t
    let mut v = byte_0(state_in) ^ byte_1(state_in);
    v = gf2_8_field_mult_by_2(v);
    let state_out = ((byte_0(state_in) ^ v ^ t) as u32) << 24;

    // v = a[1] + a[2]
    // v = x*v
    // r[1] = a[1] + v + t
    v = byte_1(state_in) ^ byte_2(state_in);
    v = gf2_8_field_mult_by_2(v);
    let state_out = state_out ^ (((byte_1(state_in) ^ v ^ t) as u32) << 16);

    // v = a[2] + a[3]
    // v = x*v
    // r[2] = a[2] + v + t
    v = byte_2(state_in) ^ byte_3(state_in);
    v = gf2_8_field_mult_by_2(v);
    let state_out = state_out ^ (((byte_2(state_in) ^ v ^ t) as u32) << 8);

    // v = a[3] + a[0]
    // v = x*v
    // r[3] = a[3] + v + t
    v = byte_3(state_in) ^ byte_0(state_in);
    v = gf2_8_field_mult_by_2(v);
    // return result
    state_out ^ ((byte_3(state_in) ^ v ^ t) as u32)
}

/// Performs the inverse S-box substitution.
pub const fn inverse_mix_col(state_in: u32) -> u32 {
    // u = x*(x*(a[0] + a[2]))
    let u = gf2_8_field_mult_by_2(gf2_8_field_mult_by_2(byte_0(state_in) ^ byte_2(state_in)));

    // v = v*(v*(a[1] + a[3]))
    let v = gf2_8_field_mult_by_2(gf2_8_field_mult_by_2(byte_1(state_in) ^ byte_3(state_in)));

    // a[0] = a[0] + u
    // a[1] = a[1] + v
    // a[2] = a[2] + u
    // a[3] = a[3] + v
    let state_out = u32::from_be_bytes([
        (byte_0(state_in) ^ u),
        (byte_1(state_in) ^ v),
        (byte_2(state_in) ^ u),
        (byte_3(state_in) ^ v),
    ]);

    // MixCol( a )
    // return result
    forward_mix_col(state_out)
}

/// Represents a context for performing AES cryptographic operations.
#[repr(C)]
#[derive(zeroize::Zeroize)]
pub struct AesContext {
    enc_round_keys: [u32; 44],
}

impl AesContext {
    /// Creates a new [AesContext].
    pub const fn new() -> Self {
        Self {
            enc_round_keys: [0u32; 44],
        }
    }

    /// Derives and sets the round keys for AES operations.
    pub fn set_key(&mut self, key: &[u8; AES_KEY]) {
        // get mutable reference to encryption round keys
        let mut enc_round_key = self.enc_round_keys.as_mut();

        // enc_round_key[0..3] is the original key
        for (round_key, k) in enc_round_key[..4].iter_mut().zip(key.chunks_exact(4)) {
            // unwrap will not panic because `k`'s length is guaranteed to be 4
            *round_key = concat_4_bytes(k.try_into().unwrap());
        }

        // derive enc_round_key[4..43] from enc_round_key[0..3]
        for rcon in RCON.iter() {
            // enc_round_key[i mod 4 == 0] = .. where i = this i * 4 (means i from FIPS-197 Fig.11)
            // 32-bit w[i-4]
            enc_round_key[4] = enc_round_key[0]
                ^ u32::from_be_bytes([
                    // 32-bit { s-box( enc_round_key[i-1]-a1 )
                    (forward_sub_byte(byte_1(enc_round_key[3]) as usize) ^ rcon),
                    // s-box( enc_round_key[i-1]-a2 )
                    forward_sub_byte(byte_2(enc_round_key[3]) as usize),
                    // s-box( enc_round_key[i-1]-a3 )
                    forward_sub_byte(byte_3(enc_round_key[3]) as usize),
                    // s-box( enc_round_key[i-1]-a0 ) } = subword(rotword(w[i-1]))
                    forward_sub_byte(byte_0(enc_round_key[3]) as usize),
                ]);

            // enc_round_key[i mod 4 != 0] = w[i-4] XOR w[i-1], where i = this i * 4 (means i from FIPS-197 Fig.11)
            enc_round_key[5] = enc_round_key[1] ^ enc_round_key[4];
            enc_round_key[6] = enc_round_key[2] ^ enc_round_key[5];
            enc_round_key[7] = enc_round_key[3] ^ enc_round_key[6];

            // next 4 32-bit words
            enc_round_key = &mut enc_round_key[4..];
        }
    }

    /// Encrypts a 16-byte block in-place using the AES function.
    pub fn encrypt_16_byte_block(&self, plain: &[u8; AES_BLOCK], cipher: &mut [u8]) {
        assert!(cipher.len() >= AES_BLOCK);

        let mut round_key = self.enc_round_keys.as_ref();

        // read 16 plain text bytes into four 32-bit words (FIPS-197: state = in )
        //
        // the following `unwrap` calls will never panic because the byte length is guaranteed
        // correct.
        let mut cx0 = concat_4_bytes(plain[..4].try_into().unwrap());
        let mut cx1 = concat_4_bytes(plain[4..8].try_into().unwrap());
        let mut cx2 = concat_4_bytes(plain[8..12].try_into().unwrap());
        let mut cx3 = concat_4_bytes(plain[12..].try_into().unwrap());

        // XOR it with encryption round_key[0..3] (FIPS-197: AddRoundKey(state))
        cx0 ^= round_key[0];
        cx1 ^= round_key[1];
        cx2 ^= round_key[2];
        cx3 ^= round_key[3];

        // do encryption rounds 1..9
        // forward S-box, ShiftRows() via input structure, MixCol, XOR round_key
        round_key = &round_key[4..];

        let (mut cy0, mut cy1, mut cy2, mut cy3);

        for _i in 0..9 {
            cy0 = round_key[0]
                ^ forward_mix_col(u32::from_be_bytes([
                    forward_sub_byte(byte_0(cx0) as usize),
                    forward_sub_byte(byte_1(cx1) as usize),
                    forward_sub_byte(byte_2(cx2) as usize),
                    forward_sub_byte(byte_3(cx3) as usize),
                ]));

            cy1 = round_key[1]
                ^ forward_mix_col(u32::from_be_bytes([
                    forward_sub_byte(byte_0(cx1) as usize),
                    forward_sub_byte(byte_1(cx2) as usize),
                    forward_sub_byte(byte_2(cx3) as usize),
                    forward_sub_byte(byte_3(cx0) as usize),
                ]));

            cy2 = round_key[2]
                ^ forward_mix_col(u32::from_be_bytes([
                    forward_sub_byte(byte_0(cx2) as usize),
                    forward_sub_byte(byte_1(cx3) as usize),
                    forward_sub_byte(byte_2(cx0) as usize),
                    forward_sub_byte(byte_3(cx1) as usize),
                ]));

            cy3 = round_key[3]
                ^ forward_mix_col(u32::from_be_bytes([
                    forward_sub_byte(byte_0(cx3) as usize),
                    forward_sub_byte(byte_1(cx0) as usize),
                    forward_sub_byte(byte_2(cx1) as usize),
                    forward_sub_byte(byte_3(cx2) as usize),
                ]));

            round_key = &round_key[4..];

            // copy cy --> cx for input for next round
            cx0 = cy0;
            cx1 = cy1;
            cx2 = cy2;
            cx3 = cy3;
        }

        // final forward S-box round inluding ShiftRows() via input structure
        cy0 = round_key[0]
            ^ u32::from_be_bytes([
                forward_sub_byte(byte_0(cx0) as usize),
                forward_sub_byte(byte_1(cx1) as usize),
                forward_sub_byte(byte_2(cx2) as usize),
                forward_sub_byte(byte_3(cx3) as usize),
            ]);

        cy1 = round_key[1]
            ^ u32::from_be_bytes([
                forward_sub_byte(byte_0(cx1) as usize),
                forward_sub_byte(byte_1(cx2) as usize),
                forward_sub_byte(byte_2(cx3) as usize),
                forward_sub_byte(byte_3(cx0) as usize),
            ]);

        cy2 = round_key[2]
            ^ u32::from_be_bytes([
                forward_sub_byte(byte_0(cx2) as usize),
                forward_sub_byte(byte_1(cx3) as usize),
                forward_sub_byte(byte_2(cx0) as usize),
                forward_sub_byte(byte_3(cx1) as usize),
            ]);

        cy3 = round_key[3]
            ^ u32::from_be_bytes([
                forward_sub_byte(byte_0(cx3) as usize),
                forward_sub_byte(byte_1(cx0) as usize),
                forward_sub_byte(byte_2(cx1) as usize),
                forward_sub_byte(byte_3(cx2) as usize),
            ]);

        split_into_4_bytes_inplace(cy0, cipher[..4].as_mut());
        split_into_4_bytes_inplace(cy1, cipher[4..8].as_mut());
        split_into_4_bytes_inplace(cy2, cipher[8..12].as_mut());
        split_into_4_bytes_inplace(cy3, cipher[12..].as_mut());
    }

    /// Decrypts a 16-byte block in-place using the AES function.
    pub fn decrypt_16_byte_block(&self, cipher: &[u8; AES_BLOCK], plain: &mut [u8]) {
        assert!(plain.len() >= AES_BLOCK);

        // get decryption round keys --> changed: get encryption round keys
        let mut round_key_offset = 40;
        let mut round_key = self.enc_round_keys[round_key_offset..].as_ref();

        // read 16 cipher text bytes into four 32-bit words (FIPS-197: state = in )
        //
        // the following `unwrap` calls will never panic because the length is guaranteed correct.
        let mut cx0 = concat_4_bytes(cipher[..4].try_into().unwrap());
        let mut cx1 = concat_4_bytes(cipher[4..8].try_into().unwrap());
        let mut cx2 = concat_4_bytes(cipher[8..12].try_into().unwrap());
        let mut cx3 = concat_4_bytes(cipher[12..].try_into().unwrap());

        // XOR it with enc_round_key[43..40]
        cx0 ^= round_key[0];
        cx1 ^= round_key[1];
        cx2 ^= round_key[2];
        cx3 ^= round_key[3];

        let (mut cy0, mut cy1, mut cy2, mut cy3);

        // do decryption rounds 9..1
        // inverse S-box, inverse ShiftRows() via input structure, inverseMixCol, XOR round_key
        round_key_offset -= 4;
        round_key = self.enc_round_keys[round_key_offset..].as_ref();
        for _i in 0..9 {
            cy0 = inverse_mix_col(
                round_key[0]
                    ^ u32::from_be_bytes([
                        inverse_sub_byte(byte_0(cx0) as usize),
                        inverse_sub_byte(byte_1(cx3) as usize),
                        inverse_sub_byte(byte_2(cx2) as usize),
                        inverse_sub_byte(byte_3(cx1) as usize),
                    ]),
            );

            cy1 = inverse_mix_col(
                round_key[1]
                    ^ u32::from_be_bytes([
                        inverse_sub_byte(byte_0(cx1) as usize),
                        inverse_sub_byte(byte_1(cx0) as usize),
                        inverse_sub_byte(byte_2(cx3) as usize),
                        inverse_sub_byte(byte_3(cx2) as usize),
                    ]),
            );

            cy2 = inverse_mix_col(
                round_key[2]
                    ^ u32::from_be_bytes([
                        inverse_sub_byte(byte_0(cx2) as usize),
                        inverse_sub_byte(byte_1(cx1) as usize),
                        inverse_sub_byte(byte_2(cx0) as usize),
                        inverse_sub_byte(byte_3(cx3) as usize),
                    ]),
            );

            cy3 = inverse_mix_col(
                round_key[3]
                    ^ u32::from_be_bytes([
                        inverse_sub_byte(byte_0(cx3) as usize),
                        inverse_sub_byte(byte_1(cx2) as usize),
                        inverse_sub_byte(byte_2(cx1) as usize),
                        inverse_sub_byte(byte_3(cx0) as usize),
                    ]),
            );

            round_key_offset -= 4;
            round_key = self.enc_round_keys[round_key_offset..].as_ref();

            // copy cy --> cx for input for next round
            cx0 = cy0;
            cx1 = cy1;
            cx2 = cy2;
            cx3 = cy3;
        }

        // final round:
        cy0 = round_key[0]
            ^ u32::from_be_bytes([
                inverse_sub_byte(byte_0(cx0) as usize),
                inverse_sub_byte(byte_1(cx3) as usize),
                inverse_sub_byte(byte_2(cx2) as usize),
                inverse_sub_byte(byte_3(cx1) as usize),
            ]);

        cy1 = round_key[1]
            ^ u32::from_be_bytes([
                inverse_sub_byte(byte_0(cx1) as usize),
                inverse_sub_byte(byte_1(cx0) as usize),
                inverse_sub_byte(byte_2(cx3) as usize),
                inverse_sub_byte(byte_3(cx2) as usize),
            ]);

        cy2 = round_key[2]
            ^ u32::from_be_bytes([
                inverse_sub_byte(byte_0(cx2) as usize),
                inverse_sub_byte(byte_1(cx1) as usize),
                inverse_sub_byte(byte_2(cx0) as usize),
                inverse_sub_byte(byte_3(cx3) as usize),
            ]);

        cy3 = round_key[3]
            ^ u32::from_be_bytes([
                inverse_sub_byte(byte_0(cx3) as usize),
                inverse_sub_byte(byte_1(cx2) as usize),
                inverse_sub_byte(byte_2(cx1) as usize),
                inverse_sub_byte(byte_3(cx0) as usize),
            ]);

        // write result into plain text byte array
        split_into_4_bytes_inplace(cy0, plain[..4].as_mut());
        split_into_4_bytes_inplace(cy1, plain[4..8].as_mut());
        split_into_4_bytes_inplace(cy2, plain[8..12].as_mut());
        split_into_4_bytes_inplace(cy3, plain[12..].as_mut());
    }
}

/// Encrypts a plaintext data buffer using AES in ECB mode.
///
/// Returns the resulting ciphertext. Plaintext must be a multiple of 16.
pub fn aes_encrypt<const N: usize>(key: &[u8; AES_KEY], plain_data: &[u8; N]) -> Result<[u8; N]> {
    if N % AES_BLOCK != 0 {
        Err(Error::Aes(format!(
            "plaintext must be a multiple of {AES_BLOCK}, have: {N}"
        )))
    } else {
        let mut aes_ctx = AesContext::new();
        aes_ctx.set_key(key);

        let mut cipher = [0u8; N];

        let plain_blocks = plain_data.chunks_exact(AES_BLOCK);
        let ciph_blocks = cipher.chunks_exact_mut(AES_BLOCK);

        for (plain_block, ciph_block) in plain_blocks.zip(ciph_blocks) {
            aes_ctx.encrypt_16_byte_block(plain_block.try_into().unwrap(), ciph_block);
        }

        Ok(cipher)
    }
}

/// Encrypts a plaintext data buffer using AES in ECB mode in-place.
///
/// Returns the `Ok` on success, `Err` on failure. Ciphertext must be a multiple of 16.
pub fn aes_encrypt_inplace(
    key: &[u8; AES_KEY],
    plain_data: &[u8],
    cipher_data: &mut [u8],
) -> Result<()> {
    let plain_len = plain_data.len();
    let cipher_len = cipher_data.len();

    if plain_data.len() % AES_BLOCK != 0 || plain_len != cipher_len {
        Err(Error::Aes(format!(
            "plaintext must be a multiple of {AES_BLOCK}, have: {plain_len}"
        )))
    } else {
        let mut aes_ctx = AesContext::new();
        aes_ctx.set_key(key);

        let plain_blocks = plain_data.chunks_exact(AES_BLOCK);
        let ciph_blocks = cipher_data.chunks_exact_mut(AES_BLOCK);

        for (plain_block, ciph_block) in plain_blocks.zip(ciph_blocks) {
            aes_ctx.encrypt_16_byte_block(plain_block.try_into().unwrap(), ciph_block);
        }

        Ok(())
    }
}

/// Decrypts a ciphertext data buffer using AES in ECB mode.
///
/// Returns the resulting plaintext. Ciphertext must be a multiple of 16.
pub fn aes_decrypt<const N: usize>(key: &[u8; AES_KEY], cipher_data: &[u8; N]) -> Result<[u8; N]> {
    if N % AES_BLOCK != 0 {
        Err(Error::Aes(format!(
            "ciphertext must be a multiple of {AES_BLOCK}, have: {N}"
        )))
    } else {
        let mut aes_ctx = AesContext::new();
        aes_ctx.set_key(key);

        let mut plain = [0u8; N];

        let ciph_blocks = cipher_data.chunks_exact(AES_BLOCK);
        let plain_blocks = plain.chunks_exact_mut(AES_BLOCK);

        for (ciph_block, plain_block) in ciph_blocks.zip(plain_blocks) {
            aes_ctx.decrypt_16_byte_block(ciph_block.try_into().unwrap(), plain_block);
        }

        Ok(plain)
    }
}

/// Decrypts a ciphertext data buffer using AES in ECB mode in-place.
///
/// Returns the `Ok` on success, `Err` on failure. Ciphertext must be a multiple of 16.
pub fn aes_decrypt_inplace(
    key: &[u8; AES_KEY],
    cipher_data: &[u8],
    plain_data: &mut [u8],
) -> Result<()> {
    let cipher_len = cipher_data.len();
    let plain_len = plain_data.len();

    if cipher_len % AES_BLOCK != 0 || cipher_len != plain_len {
        Err(Error::Aes(format!(
            "ciphertext must be a multiple of {AES_BLOCK}, have: {cipher_len}"
        )))
    } else {
        let mut aes_ctx = AesContext::new();
        aes_ctx.set_key(key);

        let ciph_blocks = cipher_data.chunks_exact(AES_BLOCK);
        let plain_blocks = plain_data.chunks_exact_mut(AES_BLOCK);

        for (ciph_block, plain_block) in ciph_blocks.zip(plain_blocks) {
            aes_ctx.decrypt_16_byte_block(ciph_block.try_into().unwrap(), plain_block);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let word = 0x1122_3344;
        let exp_bytes = [0x11, 0x22, 0x33, 0x44];
        let manual_bytes = [
            (word >> 24) as u8,
            (word >> 16) as u8,
            (word >> 8) as u8,
            word as u8,
        ];

        assert_eq!(split_into_4_bytes(word), exp_bytes);
        assert_eq!(split_into_4_bytes(word), manual_bytes);
        assert_eq!(manual_bytes, exp_bytes);
    }

    #[test]
    #[rustfmt::skip]
    fn test_block_encryption() {
        let mut aes = AesContext::new();
        let key = [0xff; AES_KEY];

        aes.set_key(&key);

        let plain = [0x11; AES_BLOCK];
        let exp_cipher = [
            0xf1, 0x9f, 0xd2, 0xd2, 0xba, 0x1c, 0x22, 0xe1,
            0x6d, 0xc1, 0xfe, 0x1b, 0x4b, 0x43, 0xd5, 0x30
        ]; 

        let mut cipher = [0u8; AES_BLOCK];
        aes.encrypt_16_byte_block(&plain, cipher.as_mut());

        assert_eq!(cipher, exp_cipher, "cipher: {:x?}, expected: {:x?}", cipher, exp_cipher);
    }

    #[test]
    #[rustfmt::skip]
    fn test_block_decryption() {
        let mut aes = AesContext::new();
        let key = [0xff; AES_KEY];

        aes.set_key(&key);

        let cipher = [
            0xf1, 0x9f, 0xd2, 0xd2, 0xba, 0x1c, 0x22, 0xe1,
            0x6d, 0xc1, 0xfe, 0x1b, 0x4b, 0x43, 0xd5, 0x30
        ]; 
        let exp_plain = [0x11; AES_BLOCK];

        let mut plain = [0u8; AES_BLOCK];
        aes.decrypt_16_byte_block(&cipher, plain.as_mut());

        assert_eq!(plain, exp_plain, "plain: {:x?}, expected: {:x?}", plain, exp_plain);
    }

    #[test]
    #[rustfmt::skip]
    fn test_encryption() -> Result<()> {
        let key = [0xff; AES_KEY];

        let plain = [0x11; AES_BLOCK * 2];
        let exp_cipher = [
            0xf1, 0x9f, 0xd2, 0xd2, 0xba, 0x1c, 0x22, 0xe1,
            0x6d, 0xc1, 0xfe, 0x1b, 0x4b, 0x43, 0xd5, 0x30,
            0xf1, 0x9f, 0xd2, 0xd2, 0xba, 0x1c, 0x22, 0xe1,
            0x6d, 0xc1, 0xfe, 0x1b, 0x4b, 0x43, 0xd5, 0x30
        ]; 

        let cipher = aes_encrypt(&key, &plain)?;

        assert_eq!(cipher, exp_cipher, "cipher: {:x?}, expected: {:x?}", cipher, exp_cipher);

        // Check that plaintext that is not a multiple of AES_BLOCK is rejected
        assert!(aes_encrypt(&key, &[0u8; 17]).is_err());

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_decryption() -> Result<()> {
        let key = [0xff; AES_KEY];

        let cipher = [
            0xf1, 0x9f, 0xd2, 0xd2, 0xba, 0x1c, 0x22, 0xe1,
            0x6d, 0xc1, 0xfe, 0x1b, 0x4b, 0x43, 0xd5, 0x30,
            0xf1, 0x9f, 0xd2, 0xd2, 0xba, 0x1c, 0x22, 0xe1,
            0x6d, 0xc1, 0xfe, 0x1b, 0x4b, 0x43, 0xd5, 0x30
        ]; 
        let exp_plain = [0x11; AES_BLOCK * 2];

        let plain = aes_decrypt(&key, &cipher)?;

        assert_eq!(plain, exp_plain, "plain: {:x?}, expected: {:x?}", plain, exp_plain);

        // Check that ciphertext that is not a multiple of AES_BLOCK is rejected
        assert!(aes_decrypt(&key, &[0u8; 17]).is_err());

        Ok(())
    }
}
