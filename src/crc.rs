//! CRC-16 checksum calculation

/// The final 2 bytes are used for a Cyclic Redundancy Check (CRC). This is provided to detect
/// errors during transmission. The CRC is calculated using a forward CRC-16 algorithm with
/// the polynomial `(X16 + X15 + X2 + 1 = 0b1_1000_0000_0000_0101)`.
///
/// It is calculated on all bytes except STX and initialised using the seed `0xFFFF`.
///
/// The CRC is calculated before byte stuffing
pub const SSP_CRC_ALG: crc::Algorithm<u16> = crc::Algorithm {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0x0000,
    residue: 0x0000,
};

pub fn crc16(data: &[u8]) -> u16 {
    let crc = crc::Crc::<u16>::new(&SSP_CRC_ALG);

    let mut digest = crc.digest();
    digest.update(data);
    digest.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssp_crc() {
        let data = [
            0x80, 0x11, 0x7E, 0x92, 0x2C, 0xF0, 0xC6, 0x74, 0x40, 0xD1, 0x38, 0xB9, 0x17, 0x18,
            0x4D, 0xFC, 0x76, 0x11, 0xB4,
        ];
        let expected_crc = u16::from_le_bytes([0xe3, 0x66]);

        assert_eq!(
            crc16(data.as_ref()),
            expected_crc,
            "have: 0x{:04x}, expected: 0x{expected_crc:04x}",
            crc16(data.as_ref())
        );
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
}
