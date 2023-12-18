pub(crate) mod dataset;
pub(crate) mod header;
pub(crate) mod ram;

pub use dataset::*;
pub use header::*;
pub use ram::*;

pub const FIRMWARE_ACK: u8 = 0x32;

/// Parses an ITL firmware file into [FirmwareHeader], [FirmwareRam], and [FirmwareData]
/// structures.
#[cfg(feature = "std")]
pub fn parse_firmware_file(
    file_path: &str,
) -> crate::Result<(FirmwareHeader, FirmwareRam, FirmwareData)> {
    use crate::Error;
    use std::{fs, io::Read, mem};

    let md = fs::metadata(file_path)
        .map_err(|err| Error::Firmware(format!("error getting firmware file metadata: {err}")))?;

    let mut f = fs::File::open(file_path)
        .map_err(|err| Error::Firmware(format!("error opening firmware file: {err}")))?;

    let file_len = md.len() as usize;
    let mut file_buf = Vec::with_capacity(file_len);

    f.read_to_end(&mut file_buf)
        .map_err(|err| Error::Firmware(format!("error reading firmware file: {err}")))?;

    let file_slice: &[u8] = file_buf.as_ref();
    let header = FirmwareHeader::try_from(file_slice)?;

    let ram_start = mem::size_of::<FirmwareHeader>();
    let ram_end = ram_start + header.ram_len() as usize;

    if ram_end > file_len {
        Err(Error::Firmware(format!(
            "invalid RAM block length, have: {ram_end}, max: {file_len}"
        )))
    } else {
        let ram = FirmwareRam::try_from(file_buf[ram_start..ram_end].as_ref())?;
        let data = FirmwareData::try_from(file_buf[ram_end..].as_ref())?;

        Ok((header, ram, data))
    }
}
