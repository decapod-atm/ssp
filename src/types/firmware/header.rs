use core::mem;

use crate::{Error, Result};

/// Length of the firmware header magic bytes.
pub const FIRMWARE_MAGIC_LEN: usize = 3;
/// Firmware header magic bytes (`b"ITL"`).
pub const FIRMWARE_MAGIC: [u8; FIRMWARE_MAGIC_LEN] = [b'I', b'T', b'L'];
/// Firmware header magic bytes (integer representation).
pub const FIRMWARE_MAGIC_NUM: u32 = 0x49544c;
/// Firmware header data length.
///
/// The firmware header is defined to be 128 bytes long, but only the first eleven bytes are given
/// meaning in the specification. The remaining bytes are reserved as `data`, as they may still
/// contain meaningful information.
pub const FIRMWARE_HDR_DATA_LEN: usize = 117;
/// Length of the firmware file header.
pub const FIRMWARE_HEADER_LEN: usize = 128;
/// Reserved field of unspecified firmware header data.
pub const FIRMWARE_RESERVED_LEN: usize = 3;

/// Represents the header of a firmware file used to upgrade devices over SSP.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirmwareHeader {
    magic: [u8; FIRMWARE_MAGIC_LEN],
    _reserved: [u8; FIRMWARE_RESERVED_LEN],
    file_code: u8,
    ram_len: u32,
    data: [u8; FIRMWARE_HDR_DATA_LEN],
}

impl FirmwareHeader {
    /// Creates a new [FirmwareHeader].
    pub const fn new() -> Self {
        Self {
            magic: FIRMWARE_MAGIC,
            _reserved: [0; FIRMWARE_RESERVED_LEN],
            file_code: 0,
            ram_len: 0,
            data: [0; FIRMWARE_HDR_DATA_LEN],
        }
    }

    /// Gets the [FirmwareHeader] magic bytes.
    pub fn magic(&self) -> u32 {
        u32::from_be_bytes([0, self.magic[0], self.magic[1], self.magic[2]])
    }

    /// Gets the [FirmwareHeader] file code for the firmware data.
    pub const fn file_code(&self) -> u8 {
        self.file_code
    }

    /// Gets the length of the RAM code.
    pub const fn ram_len(&self) -> u32 {
        self.ram_len
    }

    /// Gets the [FirmwareHeader] data section.
    pub const fn data(&self) -> &[u8] {
        &self.data
    }

    /// Converts the [FirmwareHeader] to a byte buffer.
    ///
    /// **NOTE**: `buf` must be at least [FIRMWARE_HEADER_LEN] bytes.
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<()> {
        let len = buf.len();
        if len < FIRMWARE_HEADER_LEN {
            Err(Error::Firmware(format!("invalid header destination buffer length, have: {len}, expected: {FIRMWARE_HEADER_LEN}")))
        } else {
            let mut idx = 0;
            buf[..FIRMWARE_MAGIC_LEN].copy_from_slice(&self.magic);

            idx += FIRMWARE_MAGIC_LEN;
            let res_end = idx + FIRMWARE_RESERVED_LEN;
            buf[idx..res_end].copy_from_slice(&self._reserved);

            idx = res_end;

            buf[idx] = self.file_code;
            idx += mem::size_of::<u8>();

            let ram_end = idx + mem::size_of::<u32>();
            buf[idx..ram_end].copy_from_slice(self.ram_len.to_be_bytes().as_ref());

            idx = ram_end;
            let data_end = idx + FIRMWARE_HDR_DATA_LEN;

            buf[idx..data_end].copy_from_slice(&self.data);

            Ok(())
        }
    }
}

impl TryFrom<&FirmwareHeader> for [u8; FIRMWARE_HEADER_LEN] {
    type Error = Error;

    fn try_from(val: &FirmwareHeader) -> Result<Self> {
        let mut buf = [0u8; FIRMWARE_HEADER_LEN];
        val.to_bytes(&mut buf)?;
        Ok(buf)
    }
}

impl TryFrom<FirmwareHeader> for [u8; FIRMWARE_HEADER_LEN] {
    type Error = Error;

    fn try_from(val: FirmwareHeader) -> Result<Self> {
        (&val).try_into()
    }
}

impl TryFrom<&[u8]> for FirmwareHeader {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        let len = val.len();
        if len < FIRMWARE_HEADER_LEN {
            Err(Error::Firmware(format!(
                "invalid header length, have: {len}, expected: {FIRMWARE_HEADER_LEN}"
            )))
        } else if val[..FIRMWARE_MAGIC_LEN] != FIRMWARE_MAGIC {
            let bad_magic = &val[..FIRMWARE_MAGIC_LEN];
            let magic = &FIRMWARE_MAGIC;
            Err(Error::Firmware(format!(
                "bad magic bytes, have: {bad_magic:x?}, expected: {magic:x?}"
            )))
        } else {
            let mut idx = FIRMWARE_MAGIC_LEN;
            let magic = FIRMWARE_MAGIC;

            // these conversions to fixed-length arrays should never fail, but just in case...
            let res_end = idx.saturating_add(FIRMWARE_RESERVED_LEN);
            let _reserved: [u8; FIRMWARE_RESERVED_LEN] = val[idx..res_end]
                .try_into()
                .map_err(|err| Error::Firmware(format!("invalid reserved data: {err}")))?;

            idx = res_end;

            let file_code = val[idx];

            idx += mem::size_of::<u8>();
            let ram_len = u32::from_be_bytes([val[idx], val[idx + 1], val[idx + 2], val[idx + 3]]);

            idx += mem::size_of::<u32>();
            let data_end = idx + FIRMWARE_HDR_DATA_LEN;
            // these conversions to fixed-length arrays should never fail, but just in case...
            let data: [u8; FIRMWARE_HDR_DATA_LEN] = val[idx..data_end]
                .try_into()
                .map_err(|err| Error::Firmware(format!("invalid firmware data: {err}")))?;

            Ok(Self {
                magic,
                _reserved,
                file_code,
                ram_len,
                data,
            })
        }
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for FirmwareHeader {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for FirmwareHeader {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_header_parse() -> Result<()> {
        let hdr_buf = [
            // magic
            b'I', b'T', b'L',
            // reserved
            0, 0, 0,
            // file code
            0x03,
            // RAM length
            0x01, 0x02, 0x03, 0x04,
            // data
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0,
        ];

        let exp_hdr = FirmwareHeader {
            magic: FIRMWARE_MAGIC,
            _reserved: [0u8; FIRMWARE_RESERVED_LEN],
            file_code: 0x03,
            ram_len: 0x01020304,
            data: [0u8; FIRMWARE_HDR_DATA_LEN],
        };

        assert_eq!(FirmwareHeader::try_from(&hdr_buf)?, exp_hdr);
        assert_eq!(<[u8; FIRMWARE_HEADER_LEN]>::try_from(&exp_hdr)?, hdr_buf);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_header_bad_magic() -> Result<()> {
        let hdr_buf = [
            // magic
            b'b', b'4', b'd',
            // reserved
            0, 0, 0,
            // file code
            0x03,
            // RAM length
            0x01, 0x02, 0x03, 0x04,
            // data
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0,
        ];

        assert!(FirmwareHeader::try_from(&hdr_buf).is_err());

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_header_short_buf() -> Result<()> {
        let mut hdr_buf = [
            // magic
            b'b', b'4', b'd',
            // reserved
            0, 0, 0,
            // file code
            0x03,
            // RAM length
            0x01, 0x02, 0x03, 0x04,
        ];

        assert!(FirmwareHeader::try_from(&hdr_buf).is_err());
        assert!(FirmwareHeader::new().to_bytes(&mut hdr_buf).is_err());

        Ok(())
    }
}
