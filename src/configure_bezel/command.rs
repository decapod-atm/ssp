use crate::{
    impl_command_ops, impl_default, impl_message_from_buf, impl_message_ops, len, std::fmt,
    BezelConfigStorage, Blue, CommandOps, Green, MessageOps, MessageType, Red, RGB,
};

pub mod index {
    pub const RED: usize = 4;
    pub const GREEN: usize = 5;
    pub const BLUE: usize = 6;
    pub const STORAGE: usize = 7;
}

/// Configure Bezel - Command (0x54)
///
/// Four byte command that sets the colour of the bezel to a specified RGB colour. If the
/// validator does not have a bezel that can be modified in this way, 0xF2 (Unknown
/// command) will be returned.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConfigureBezelCommand {
    buf: [u8; len::CONFIGURE_BEZEL_COMMAND],
}

impl ConfigureBezelCommand {
    /// Creates a new [ConfigureBezelCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::CONFIGURE_BEZEL_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::ConfigureBezel);

        msg
    }

    /// Gets the [RGB] color settings.
    pub fn rgb(&self) -> RGB {
        self.buf[index::RED..=index::BLUE].into()
    }

    /// Sets the [RGB] color settings.
    pub fn set_rgb(&mut self, rgb: RGB) {
        self.buf[index::RED..=index::BLUE].copy_from_slice(rgb.as_bytes().as_ref())
    }

    /// Gets the [Red] color setting.
    pub fn red(&self) -> Red {
        self.buf[index::RED].into()
    }

    /// Sets the [Red] color setting.
    pub fn set_red(&mut self, red: Red) {
        self.buf[index::RED] = red.into();
    }

    /// Gets the [Green] color setting.
    pub fn green(&self) -> Green {
        self.buf[index::GREEN].into()
    }

    /// Sets the [Green] color setting.
    pub fn set_green(&mut self, green: Green) {
        self.buf[index::GREEN] = green.into();
    }

    /// Gets the [Blue] color setting.
    pub fn blue(&self) -> Blue {
        self.buf[index::BLUE].into()
    }

    /// Sets the [Blue] color setting.
    pub fn set_blue(&mut self, blue: Blue) {
        self.buf[index::BLUE] = blue.into();
    }

    /// Gets the [BezelConfigStorage] setting for where color configuration is stored in memory.
    pub fn config_storage(&self) -> BezelConfigStorage {
        self.buf[index::STORAGE].into()
    }

    /// Sets the [BezelConfigStorage] setting for where color configuration is stored in memory.
    pub fn set_config_storage(&mut self, config: BezelConfigStorage) {
        self.buf[index::STORAGE] = config.into();
    }
}

impl_default!(ConfigureBezelCommand);
impl_message_from_buf!(ConfigureBezelCommand);
impl_message_ops!(ConfigureBezelCommand);
impl_command_ops!(ConfigureBezelCommand);

impl fmt::Display for ConfigureBezelCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stx = self.stx();
        let seqid = self.sequence_id();
        let len = self.data_len();
        let cmd = self.command();
        let rgb = self.rgb();
        let storage = self.config_storage();
        let crc = self.checksum();

        write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Command: {cmd} | RGB: {rgb} | Bezel config storage: {storage} | CRC-16: 0x{crc:04x}")
    }
}
