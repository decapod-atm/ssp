use alloc::vec::Vec;

use crate::{Error, Result};

/// The maximum length of a [FirmwareRam] block.
pub const FIRMWARE_RAM_MAX: usize = u32::MAX as usize;
/// Length of a [FirmwareRam] section.
pub const FIRMWARE_RAM_SECTION_LEN: usize = 128;

/// Represents the Firmware RAM block in the ITL firmware file.
///
/// The RAM block immediately follows the [FirmwareHeader](super::FirmwareHeader), and has a
/// variable length.
///
/// Typical size is 4096 bytes.
///
/// The RAM block is sent to the device in 128-byte sections.
///
/// An XOR-checksum is calculated over each byte in the [FirmwareRam] block. At the end of
/// transmission, the device will return its XOR-checksum byte, which should be checked against
/// the local checksum. If there is a mismatch, abort the firmware update, and retry.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct FirmwareRam {
    block: Vec<u8>,
    index: usize,
}

impl FirmwareRam {
    /// Creates a new [FirmwareRam].
    pub const fn new() -> Self {
        Self {
            block: Vec::new(),
            index: 0,
        }
    }

    /// Creates a new [FirmwareRam] from the provided buffer.
    ///
    /// Returns:
    /// - `Ok(FirmwareRam)` on success
    /// - `Err(Error)` if `val` length exceeds [FIRMWARE_RAM_MAX].
    // TODO: should we do checks for valid machine instructions? Are there specifications available
    // for the ITL instruction set used on their devices?
    pub fn create(val: &[u8]) -> Result<Self> {
        let len = val.len();
        if len > FIRMWARE_RAM_MAX {
            Err(Error::Firmware(format!(
                "invalid firmware RAM length, have: {len}, max: {FIRMWARE_RAM_MAX}"
            )))
        } else {
            Ok(Self {
                block: val.into(),
                index: 0,
            })
        }
    }

    /// Gets the next [FIRMWARE_SECTION_LEN] bytes of the [FirmwareRam], and advances the `index` to
    /// the end of the returned section.
    ///
    /// If there are no more bytes, `None` is returned.
    ///
    /// If [FirmwareRam] does not divide into even [FIRMWARE_SECTION_LEN] segments, the remaining
    /// bytes will be returned in the final section.
    ///
    /// Example:
    ///
    /// ```no_run
    /// use ssp::FirmwareRam;
    ///
    /// let mut ram_block = FirmwareRam::try_from([0xff; 4096]).expect("invalid RAM block size");
    /// while let Some(section) = ram_block.next_section() {
    ///     // socket.write_all(section).expect("failure writing to socket");
    /// }
    /// ```
    pub fn next_section(&mut self) -> Option<&[u8]> {
        if self.index >= self.block.len() {
            None
        } else if self.block.len() - self.index < FIRMWARE_RAM_SECTION_LEN {
            let start = self.index;
            self.index = self.block.len();
            Some(&self.block[start..])
        } else {
            let start = self.index;
            let end = start + FIRMWARE_RAM_SECTION_LEN;
            self.index = end;
            Some(&self.block[start..end])
        }
    }

    /// Gets the current index in the [FirmwareRam] block.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Gets the length of the [FirmwareRam] buffer.
    pub fn len(&self) -> usize {
        self.block.len()
    }

    /// Gets whether the [FirmwareRam] buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.block.is_empty()
    }
}

impl TryFrom<&[u8]> for FirmwareRam {
    type Error = Error;

    fn try_from(val: &[u8]) -> Result<Self> {
        Self::create(val)
    }
}

impl<const N: usize> TryFrom<&[u8; N]> for FirmwareRam {
    type Error = Error;

    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}

impl<const N: usize> TryFrom<[u8; N]> for FirmwareRam {
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
    fn test_ram_sections() -> Result<()> {
        let exp_ram_buf = [0xff; 4096];
        let mut sections = 0;
        let mut index = 0;
        let exp_sections = 32;

        let mut ram_block = FirmwareRam::create(&exp_ram_buf)?;

        while let Some(section) = ram_block.next_section() {
            assert_eq!(section, &exp_ram_buf[index..index + section.len()]);
            assert!(section.len() <= FIRMWARE_RAM_SECTION_LEN);
            index += section.len();
            sections += 1;
        }

        assert_eq!(sections, exp_sections);

        Ok(())
    }

    #[test]
    fn test_ram_uneven_sections() -> Result<()> {
        let exp_ram_buf = [0xff; 192];
        let mut sections = 0;
        let mut index = 0;
        let exp_sections = 2;

        let mut ram_block = FirmwareRam::create(&exp_ram_buf)?;

        while let Some(section) = ram_block.next_section() {
            assert_eq!(section, &exp_ram_buf[index..index + section.len()]);
            assert!(section.len() <= FIRMWARE_RAM_SECTION_LEN);
            index += section.len();
            sections += 1;
        }

        assert_eq!(sections, exp_sections);

        Ok(())
    }

    #[test]
    fn test_invalid_ram_block() {
        let bad_ram_buf = vec![0xff; FIRMWARE_RAM_MAX + 1];
        let bad_slice: &[u8] = bad_ram_buf.as_ref();

        assert!(FirmwareRam::create(bad_slice).is_err());
        assert!(FirmwareRam::try_from(bad_slice).is_err());
    }
}
