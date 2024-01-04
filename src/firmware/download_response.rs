use crate::{
    impl_default, impl_message_from_buf, impl_message_ops, impl_response_display,
    impl_response_ops, len::DOWNLOAD_DATA_PACKET_RESPONSE, message::MessageOps, MessageType,
};

/// DownloadDataPacket - Response (0x74)
///
/// Represents a response to an [DownloadDataPacketCommand](crate::DownloadDataPacketCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DownloadDataPacketResponse {
    buf: [u8; DOWNLOAD_DATA_PACKET_RESPONSE],
}

impl DownloadDataPacketResponse {
    /// Creates a new [DownloadDataPacketResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; DOWNLOAD_DATA_PACKET_RESPONSE],
        };

        msg.init();

        msg
    }
}

impl_default!(DownloadDataPacketResponse);
impl_message_from_buf!(DownloadDataPacketResponse);
impl_message_ops!(DownloadDataPacketResponse, MessageType::ProgramFirmware);
impl_response_ops!(DownloadDataPacketResponse);
impl_response_display!(DownloadDataPacketResponse);
