//! CRC-16 checksum calculation

/// Seed for the CRC-16 algorithm used in SSP.
pub const CRC_SEED: u16 = 0xffff;
/// Polynomical for the CRC-16 algorithm used in SSP.
///
/// The highest polynomial is omitted.
pub const CRC_POLY: u16 = 0x8005;

/// The final 2 bytes are used for a Cyclic Redundancy Check (CRC). This is provided to detect
/// errors during transmission. The CRC is calculated using a forward CRC-16 algorithm with
/// the polynomial `(X16 + X15 + X2 + 1 = 0b1_1000_0000_0000_0101)`.
///
/// It is calculated on all bytes except STX and initialised using the seed `0xFFFF`.
///
/// The CRC is calculated before byte stuffing
pub fn crc16(data: &[u8]) -> u16 {
    let mut crc = CRC_SEED;

    for &byte in data.iter() {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ CRC_POLY;
            } else {
                crc = saturating_shl(crc, 1);
            }
        }
    }

    crc
}

fn saturating_shl(b: u16, s: u32) -> u16 {
    let (res, o) = b.overflowing_shl(s);

    if o {
        u16::MAX
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssp_crc_example() {
        let data = [0x80, 0x01, 0x11];
        let exp_crc = 0x8265;

        assert_eq!(crc16(data.as_ref()), exp_crc);
    }

    #[test]
    fn test_ssp_crc() {
        let mut data = [
            0x80, 0x11, 0x7E, 0x92, 0x2C, 0xF0, 0xC6, 0x74, 0x40, 0xD1, 0x38, 0xB9, 0x17, 0x18,
            0x4D, 0xFC, 0x76, 0x11, 0xB4, 0x00, 0x00,
        ];
        let exp_crc = 0x66e3u16;
        let exp_crc_bytes = exp_crc.to_be_bytes();

        assert_eq!(
            crc16(data[..data.len() - 2].as_ref()),
            exp_crc,
            "have: 0x{:04x}, expected: 0x{exp_crc:04x}",
            crc16(data[..data.len() - 2].as_ref())
        );

        let len = data.len();
        data[len - 2..].copy_from_slice(exp_crc_bytes.as_ref());

        assert_eq!(crc16(data.as_ref()), 0);
    }

    #[test]
    fn test_encrypted_crc() {
        let data_crc = [0x66, 0xe3];
        let enc_data = [
            0x80,
            0x11,
            0x7E,
            0x92,
            0x2C,
            0xF0,
            0xC6,
            0x74,
            0x40,
            0xD1,
            0x38,
            0xB9,
            0x17,
            0x18,
            0x4D,
            0xFC,
            0x76,
            0x11,
            0xB4,
            data_crc[0],
            data_crc[1],
        ];

        assert_eq!(
            crc16(enc_data[..enc_data.len() - 2].as_ref()),
            u16::from_be_bytes(data_crc)
        );
        assert_eq!(crc16(enc_data.as_ref()), 0x0000);
    }

    #[test]
    fn test_sample() {
        let data_crc = [0xac, 0x1a];
        let enc_data = [
            0x00,
            0x11,
            0x7e,
            0xe5,
            0x65,
            0x07,
            0x0e,
            0x2a,
            0x8f,
            0xab,
            0xf7,
            0xdd,
            0xb3,
            0x87,
            0xe6,
            0x45,
            0xea,
            0xb2,
            0xbb,
            data_crc[0],
            data_crc[1],
        ];
        assert_eq!(
            crc16(enc_data[..enc_data.len() - 2].as_ref()),
            u16::from_be_bytes(data_crc)
        );

        let enc_crc = [0x95, 0xfb];
        let enc_data = [
            0x01, 0x05, 0x00, 0x00, 0x00, 0x07, 0x9c, 0x10, 0x69, 0xd9, 0x37, 0xf4, 0x18, 0x7f,
            enc_crc[0], enc_crc[1],
        ];
        assert_eq!(
            crc16(enc_data[..enc_data.len() - 2].as_ref()),
            u16::from_le_bytes(enc_crc)
        );
    }
}
