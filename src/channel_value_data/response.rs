use crate::{
    impl_default, impl_message_from_buf, impl_response_ops, impl_var_message_ops, len, std::fmt,
    ChannelValue, ChannelValueList, Error, MessageOps, MessageType, ResponseOps, Result, Vec,
};

mod index {
    pub const NUM_CHANNELS: usize = 4;
    pub const CHANNEL_VALUES: usize = NUM_CHANNELS + 1;
}

/// ChannelValueData - Response (0x0A)
///
/// Represents a response to an [ChannelValueDataCommand](crate::ChannelValueDataCommand) message.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChannelValueDataResponse {
    buf: [u8; len::CHANNEL_VALUE_DATA_RESPONSE],
}

impl ChannelValueDataResponse {
    /// Creates a new [ChannelValueDataResponse] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::CHANNEL_VALUE_DATA_RESPONSE],
        };

        msg.init();

        msg
    }

    /// Gets the number of channels.
    pub fn num_channels(&self) -> usize {
        self.buf[index::NUM_CHANNELS].into()
    }

    /// Gets the [ChannelValue] for each channel.
    pub fn channel_values(&self) -> Result<ChannelValueList> {
        self.is_valid()?;

        let channel_values_end = self.channel_values_end();

        Ok(self.buf[index::CHANNEL_VALUES..channel_values_end]
            .iter()
            .map(|&v| ChannelValue::from([v]))
            .collect::<Vec<ChannelValue>>()
            .into())
    }

    fn channel_values_end(&self) -> usize {
        index::CHANNEL_VALUES + self.num_channels()
    }

    /// Gets whether the message contains a valid length and number of channels.
    ///
    /// Because the buffer has variable length, and fields that are multiples of the number of
    /// channels, there are combinations that result in overflowing the total buffer length.
    ///
    /// For example, LEN=255 & NUM_FIELDS=255 obviously overflows the buffer since the first
    /// CHANNEL_VALUES field is NUM_FIELDS*LEN_CHANNEL_VALUE(1) = 255.
    pub fn is_valid(&self) -> Result<()> {
        use crate::message::index as msg_index;

        let end_range = len::HEADER + self.data_len();
        let values_end = self.channel_values_end();

        if (msg_index::DATA..=end_range).contains(&values_end) {
            Ok(())
        } else {
            Err(Error::InvalidDataLength((values_end, end_range)))
        }
    }
}

impl_default!(ChannelValueDataResponse);
impl_message_from_buf!(ChannelValueDataResponse);
impl_var_message_ops!(ChannelValueDataResponse, MessageType::ChannelValueData);
impl_response_ops!(ChannelValueDataResponse);

impl fmt::Display for ChannelValueDataResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.is_valid() {
            Ok(_) => {
                let stx = self.stx();
                let seqid = self.sequence_id();
                let len = self.data_len();
                let status = self.response_status();
                let num_channels = self.num_channels();
                let channel_values = self.channel_values().unwrap();
                let crc = self.checksum();

                write!(f, "STX: 0x{stx:02x} | SEQID: {seqid} | LEN: 0x{len:02x} | Response status: {status} | Number of channels: {num_channels} | Channel values: {channel_values} | CRC-16: 0x{crc:04x}")
            }
            Err(err) => {
                write!(f, "Invalid ChannelValueDataResponse message: {err}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Result;

    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_unit_data_response() -> Result<()> {
        use crate::{STX, ResponseStatus};

        let channel_value_data_response_bytes = [
            // STX(0x7f)
            STX,
            // SEQID
            0x80,
            // LEN
            0x06, 
            // OK response
            0xf0,
            // Number of channels
            0x04,
            // Channel values
            0x05, 0x0a, 0x14, 0x32,
            // CRC-16
            0xb8, 0xba,
        ];

        let exp_response = ResponseStatus::Ok;
        let exp_num_channels = 4;
        let exp_channel_values = [
            ChannelValue::from(5),
            ChannelValue::from(10),
            ChannelValue::from(20),
            ChannelValue::from(50),
        ];

        let response = ChannelValueDataResponse::try_from(channel_value_data_response_bytes)?;

        assert_eq!(response.len(), channel_value_data_response_bytes.len());
        assert_eq!(response.data_len(), channel_value_data_response_bytes.len() - len::METADATA);

        assert_eq!(response.response_status(), exp_response);
        assert_eq!(response.num_channels(), exp_num_channels);
        assert_eq!(response.channel_values()?.as_ref(), exp_channel_values.as_ref());

        Ok(())
    }
}
