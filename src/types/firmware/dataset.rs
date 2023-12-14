use alloc::vec::Vec;

use crate::{Error, Result};

/// The maximum length of a [FirmwareData] block.
pub const FIRMWARE_DATA_MAX: usize = u32::MAX as usize;
/// Length of a [FirmwareData] section.
pub const FIRMWARE_DATA_SECTION_LEN: usize = 128;

/// Represents the Firmware DATA block in the ITL firmware file.
///
/// The DATA block immediately follows the [FirmwareRam](super::FirmwareRam), and has a
/// variable length.
///
/// The DATA block is sent to the device in 128-byte sections.
///
/// An XOR-checksum is calculated over each byte in the [FirmwareData] block. At the end of
/// transmission of each section, the host will send the device the calculated XOR-checksum byte.
/// The device responds with its calculated XOR-checksum byte.
///
/// If there is a mismatch, abort the firmware update, and retry.
pub struct FirmwareData {
    block: Vec<u8>,
    index: usize,
}

impl FirmwareData {
    /// Creates a new [FirmwareData].
    pub const fn new() -> Self {
        Self {
            block: Vec::new(),
            index: 0,
        }
    }

    /// Creates a new [FirmwareData] from the provided buffer.
    ///
    /// Returns:
    /// - `Ok(FirmwareData)` on success
    /// - `Err(Error)` if `val` length exceeds [FIRMWARE_DATA_MAX].
    // TODO: should we do checks for valid machine instructions? Are there specifications available
    // for the ITL instruction set used on their devices?
    pub fn create(val: &[u8]) -> Result<Self> {
        let len = val.len();
        if len > FIRMWARE_DATA_MAX {
            Err(Error::Firmware(format!(
                "invalid firmware dataset length, have: {len}, max: {FIRMWARE_DATA_MAX}"
            )))
        } else {
            Ok(Self {
                block: val.into(),
                index: 0,
            })
        }
    }

    /// Gets the next [FIRMWARE_SECTION_LEN] bytes of the [FirmwareData], and advances the `index` to
    /// the end of the returned section.
    ///
    /// If there are no more bytes, `None` is returned.
    ///
    /// If [FirmwareData] does not divide into even [FIRMWARE_SECTION_LEN] segments, the remaining
    /// bytes will be returned in the final section.
    ///
    /// Example:
    ///
    /// ```no_run
    /// use ssp::FirmwareData;
    ///
    /// let mut ram_block = FirmwareData::try_from([0xff; 4096]).expect("invalid DATA block size");
    /// while let Some(section) = ram_block.next_section() {
    ///     // socket.write_all(section).expect("failure writing to socket");
    /// }
    /// ```
    pub fn next_section(&mut self) -> Option<&[u8]> {
        if self.index >= self.block.len() {
            None
        } else if self.block.len() - self.index < FIRMWARE_DATA_SECTION_LEN {
            let start = self.index;
            self.index = self.block.len();
            Some(&self.block[start..])
        } else {
            let start = self.index;
            let end = start + FIRMWARE_DATA_SECTION_LEN;
            self.index = end;
            Some(&self.block[start..end])
        }
    }

    /// Gets the current index in the [FirmwareData] block.
    pub const fn index(&self) -> usize {
        self.index
    }
}

impl TryFrom<&[u8]> for FirmwareData {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::create(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for FirmwareData {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for FirmwareData {
    type Error = Error;

    fn try_from(val: [u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_data_sections() -> Result<()> {
        let exp_data_buf = [0xff; 4096];
        let mut sections = 0;
        let mut index = 0;
        let exp_sections = 32;

        let mut data_block = FirmwareData::create(&exp_data_buf)?;

        while let Some(section) = data_block.next_section() {
            assert_eq!(section, &exp_data_buf[index..index + section.len()]);
            assert!(section.len() <= FIRMWARE_DATA_SECTION_LEN);
            index += section.len();
            sections += 1;
        }

        assert_eq!(sections, exp_sections);

        Ok(())
    }

    #[test]
    fn test_data_uneven_sections() -> Result<()> {
        let exp_data_buf = [0xff; 192];
        let mut sections = 0;
        let mut index = 0;
        let exp_sections = 2;

        let mut data_block = FirmwareData::create(&exp_data_buf)?;

        while let Some(section) = data_block.next_section() {
            assert_eq!(section, &exp_data_buf[index..index + section.len()]);
            assert!(section.len() <= FIRMWARE_DATA_SECTION_LEN);
            index += section.len();
            sections += 1;
        }

        assert_eq!(sections, exp_sections);

        Ok(())
    }

    #[test]
    fn test_invalid_data_block() {
        let bad_data_buf = vec![0xff; FIRMWARE_DATA_MAX + 1];
        let bad_slice: &[u8] = bad_data_buf.as_ref();

        assert!(FirmwareData::create(bad_slice).is_err());
        assert!(FirmwareData::try_from(bad_slice).is_err());
    }
}
