use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_message_from_buf,
    impl_var_message_ops,
    len::{DATA_PACKET_LINE, DOWNLOAD_DATA_PACKET_COMMAND},
    CommandOps, Error, MessageOps, MessageType, Result,
};

mod index {
    pub const BLOCK: usize = 4;
    pub const BLOCK_END: usize = 8;
    pub const LINE: usize = 8;
    pub const DOWNLOAD_DATA_PACKET: usize = 9;
}

const DATA_PACKET_META_LEN: usize = 6;

/// DownloadDataPacket - Command (0x74)
///
/// Downloads a data packet to the SSP device as part of the remote update process.
///
/// Uses the `block` and `line` numbers to specify the address offset of the packet.
///
/// Special case block number `0xffffffff` is used for the firmware header packet.
///
/// Blocks are zero-indexed, and subtracted from `0xfffffffe`.
///
/// Lines are zero-indexed, and the valid range is `0-255`. Each line is `128` bytes or less.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DownloadDataPacketCommand {
    buf: [u8; DOWNLOAD_DATA_PACKET_COMMAND],
}

impl DownloadDataPacketCommand {
    /// Creates a new [DownloadDataPacketCommand] message.
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// # use ssp::MessageOps;
    /// let data_cmd = ssp::DownloadDataPacketCommand::new();
    /// // base data section w/ no data packet "line": CMD + BLOCK (u32) + LINE (u8)
    /// assert_eq!(data_cmd.data_len(), 6);
    /// // data packet "line" length
    /// assert_eq!(data_cmd.data_packet().len(), 0);
    /// ```
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DOWNLOAD_DATA_PACKET_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::DownloadDataPacket);
        msg.set_data_len(DATA_PACKET_META_LEN as u8);

        msg
    }

    /// Creates a new [DownloadDataPacketCommand] message.
    ///
    /// Parameters:
    ///
    /// - `block`: block number
    ///   - Header block = `0xffff_ffff`
    ///   - Block zero: `0xffff_fffe`
    ///   - Block one: `0xffff_fffd`
    /// - `line`: zero-indexed "line" number
    /// - `data`: data packet "line" buffer
    ///
    /// Examples:
    ///
    /// ```
    /// # use ssp;
    /// // Create a dummy header block
    /// let header_block = 0xffff_ffffu32;
    /// let header_data = [0xff; ssp::len::DATA_PACKET_LINE];
    /// let header_packet = ssp::DownloadDataPacketCommand::create(
    ///     header_block,
    ///     0,
    ///     &header_data,
    /// ).unwrap();
    /// assert_eq!(header_packet.block(), header_block);
    /// assert_eq!(header_packet.line(), 0);
    /// assert_eq!(header_packet.data_packet(), header_data.as_ref());
    ///
    /// // Create a dummy data block - zero block
    /// let data_block = 0xffff_fffeu32;
    /// let data = [0xff; ssp::len::DATA_PACKET_LINE];
    /// let data_packet = ssp::DownloadDataPacketCommand::create(
    ///     data_block,
    ///     0,
    ///     &data,
    /// ).unwrap();
    /// assert_eq!(data_packet.block(), data_block);
    /// assert_eq!(data_packet.line(), 0);
    /// assert_eq!(data_packet.data_packet(), data.as_ref());
    /// ```
    pub fn create(block: u32, line: u8, data: &[u8]) -> Result<Self> {
        let mut msg = Self {
            buf: [0u8; DOWNLOAD_DATA_PACKET_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::DownloadDataPacket);
        msg.set_block(block);
        msg.set_line(line);
        msg.set_data_packet(data)?;

        Ok(msg)
    }

    /// Gets the block number of the [DownloadDataPacketCommand].
    ///
    /// - Header block = `0xffff_ffff`
    /// - Block zero: `0xffff_fffe`
    /// - Block one: `0xffff_fffd`
    /// - ...
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let data_cmd = ssp::DownloadDataPacketCommand::new();
    /// assert_eq!(data_cmd.block(), 0);
    /// ```
    pub fn block(&self) -> u32 {
        u32::from_be_bytes([
            self.buf[index::BLOCK],
            self.buf[index::BLOCK + 1],
            self.buf[index::BLOCK + 2],
            self.buf[index::BLOCK + 3],
        ])
    }

    /// Sets the block number of the [DownloadDataPacketCommand].
    ///
    /// - Header block = `0xffff_ffff`
    /// - Block zero: `0xffff_fffe`
    /// - Block one: `0xffff_fffd`
    /// - ...
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let mut data_cmd = ssp::DownloadDataPacketCommand::new();
    /// assert_eq!(data_cmd.block(), 0);
    ///
    /// let block = 0xffff_ffff;
    /// data_cmd.set_block(block);
    /// assert_eq!(data_cmd.block(), block);
    /// ```
    pub fn set_block(&mut self, block: u32) {
        self.buf[index::BLOCK..index::BLOCK_END].copy_from_slice(block.to_be_bytes().as_ref());
    }

    /// Gets the line number of the [DownloadDataPacketCommand].
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let data_cmd = ssp::DownloadDataPacketCommand::new();
    /// assert_eq!(data_cmd.line(), 0);
    /// ```
    pub const fn line(&self) -> u8 {
        self.buf[index::LINE]
    }

    /// Sets the line number of the [DownloadDataPacketCommand].
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let mut data_cmd = ssp::DownloadDataPacketCommand::new();
    /// assert_eq!(data_cmd.line(), 0);
    ///
    /// let line = 1;
    /// data_cmd.set_line(line);
    /// assert_eq!(data_cmd.line(), line);
    /// ```
    pub fn set_line(&mut self, line: u8) {
        self.buf[index::LINE] = line;
    }

    /// Gets the data packet bytes.
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let data_cmd = ssp::DownloadDataPacketCommand::new();
    /// assert_eq!(data_cmd.data_packet(), &[]);
    /// ```
    pub fn data_packet(&self) -> &[u8] {
        let start = index::DOWNLOAD_DATA_PACKET;
        let end = start + self.data_len().saturating_sub(DATA_PACKET_META_LEN);
        &self.buf[start..end]
    }

    /// Sets the data packet "line" for the [DownloadDataPacketCommand].
    ///
    /// The packet buffer length must be in the range: `[0,128]` inclusive.
    ///
    /// Example:
    ///
    /// ```
    /// # use ssp;
    /// let mut data_cmd = ssp::DownloadDataPacketCommand::new();
    /// let data = [0xff; 128];
    /// data_cmd.set_data_packet(&data);
    /// assert_eq!(data_cmd.data_packet(), data);
    /// ```
    pub fn set_data_packet(&mut self, data: &[u8]) -> Result<()> {
        let len = data.len();
        if len > DATA_PACKET_LINE {
            Err(Error::InvalidDataLength((len, DATA_PACKET_LINE)))
        } else {
            let start = index::DOWNLOAD_DATA_PACKET;
            let end = start + len;
            let rem = DATA_PACKET_LINE - len;

            self.buf[start..end].copy_from_slice(data);
            self.buf[end..end + rem].copy_from_slice([0u8; DATA_PACKET_LINE][..rem].as_ref());
            self.set_data_len(6 + len as u8);

            Ok(())
        }
    }
}

impl_default!(DownloadDataPacketCommand);
impl_command_display!(DownloadDataPacketCommand);
impl_message_from_buf!(DownloadDataPacketCommand);
impl_var_message_ops!(DownloadDataPacketCommand);
impl_command_ops!(DownloadDataPacketCommand);
