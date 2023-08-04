use crate::{
    impl_command_ops, impl_default, impl_message_from_buf, impl_var_message_ops,
    len::{PAYOUT_BLOCK, PAYOUT_BY_DENOMINATION_COMMAND},
    std::fmt,
    CommandOps, Error, MessageOps, MessageType, PayoutDenomination, PayoutDenominationList,
    PayoutOption, PayoutVec, Result, MAX_PAYOUTS,
};

mod index {
    pub const NUMBER_BLOCKS: usize = 4;
    pub const PAYOUT_BLOCKS: usize = 5;
}

/// PayoutByDenomination - Command (0x46)
///
/// A command to payout the requested quantity of individual denominations.
///
/// ***Requires Protocol Version 6 or above.***
///
/// ***Attempting to use the command with an earlier protocol version will generate a
/// response 0xF4 (parameter out of range).***
///
/// The quantities of denominations to pay are sent as a 2 byte little endian array; the money
/// values as 4 byte little endian array and the country code as a 3 byte ASCII array.
///
/// The host also adds an option byte to the end of the command array ([`PayoutOption`]).
/// This will allow a pre test of the ability to payout the requested levels before actual payout executes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PayoutByDenominationCommand {
    buf: [u8; PAYOUT_BY_DENOMINATION_COMMAND],
}

impl PayoutByDenominationCommand {
    /// Creates a new [PayoutByDenominationCommand] message.
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; PAYOUT_BY_DENOMINATION_COMMAND],
        };

        msg.init();
        msg.set_command(MessageType::PayoutByDenomination);

        msg
    }

    /// Gets the number of [PayoutDenomination] blocks in the command.
    pub const fn number_of_payouts(&self) -> u8 {
        self.buf[index::NUMBER_BLOCKS]
    }

    /// Sets the number of [PayoutDenomination] blocks in the command.
    ///
    /// `num` must be in the valid range: [0, [`MAX_PAYOUTS`]].
    pub fn set_number_of_payouts(&mut self, num: u8) {
        if (0..=MAX_PAYOUTS).contains(&(num as usize)) {
            self.buf[index::NUMBER_BLOCKS] = num;
        }
    }

    /// Sets the number of [PayoutDenomination] blocks in the command.
    ///
    /// `num` must be in the valid range: [0, [`MAX_PAYOUTS`]].
    pub fn with_number_of_payouts(mut self, num: u8) -> Self {
        self.set_number_of_payouts(num);
        self
    }

    /// Gets a list of the [PayoutDenomination]s in the command.
    pub fn payout_denominations(&self) -> Result<PayoutDenominationList> {
        let num = self.number_of_payouts() as usize;
        if num > MAX_PAYOUTS {
            Err(Error::InvalidLength((num, MAX_PAYOUTS)))
        } else if num == 0 {
            Ok(PayoutDenominationList::new())
        } else {
            let start = index::PAYOUT_BLOCKS;
            let end = start + (num * PAYOUT_BLOCK);

            let mut list = PayoutVec::new();

            for block in self.buf[start..end].chunks_exact(PAYOUT_BLOCK) {
                // `push` only fails if `heapless::Vec` is full
                // since we checked valid range above, `push` will not fail
                list.push(PayoutDenomination::try_from(block)?).ok();
            }

            Ok(list.into())
        }
    }

    /// Sets a list of the [PayoutDenomination]s in the command.
    pub fn set_payout_denominations(&mut self, list: &PayoutDenominationList) {
        // length is guarenteed valid because of type constraints
        let num = list.len();
        self.set_number_of_payouts(num as u8);

        let start = index::PAYOUT_BLOCKS;
        let end = start + (num * PAYOUT_BLOCK);

        for (chunk, payout) in self.buf[start..end]
            .chunks_exact_mut(PAYOUT_BLOCK)
            .zip(list.iter())
        {
            // `to_buffer` will not fail because the chunk length is valid
            payout.to_buffer(chunk).ok();
        }
    }

    /// Gets the [PayoutOption].
    pub fn payout_option(&self) -> PayoutOption {
        let num = self.number_of_payouts() as usize;
        let opt_idx = index::PAYOUT_BLOCKS + (num * PAYOUT_BLOCK);

        self.buf[opt_idx].into()
    }

    /// Sets the [PayoutOption].
    ///
    /// **NOTE** user should set the [PayoutDenomination]s before calling this function, otherwise
    /// the value will be overwritten.
    pub fn set_payout_option(&mut self, option: PayoutOption) {
        let num = self.number_of_payouts() as usize;
        let opt_idx = index::PAYOUT_BLOCKS + (num * PAYOUT_BLOCK);

        self.buf[opt_idx] = option.into();
    }

    /// Builder function that sets the [PayoutOption].
    ///
    /// **NOTE** user should set the [PayoutDenomination]s before calling this function, otherwise
    /// the value will be overwritten.
    pub fn with_payout_option(mut self, option: PayoutOption) -> Self {
        self.set_payout_option(option);
        self
    }
}

impl_default!(PayoutByDenominationCommand);
impl_command_ops!(PayoutByDenominationCommand);
impl_message_from_buf!(PayoutByDenominationCommand);
impl_var_message_ops!(PayoutByDenominationCommand);

impl fmt::Display for PayoutByDenominationCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "STX: 0x{:02x} | ", self.stx())?;
        write!(f, "SEQID: {} | ", self.sequence_id())?;
        write!(f, "LEN: 0x{:02x} | ", self.data_len())?;
        write!(f, "Command: {} | ", self.command())?;
        write!(
            f,
            "Number of payout denominations: {} | ",
            self.number_of_payouts()
        )?;
        write!(
            f,
            "Payout denominations: {} | ",
            self.payout_denominations()
                .unwrap_or(PayoutDenominationList::new())
        )?;
        write!(f, "Payout option: {} | ", self.payout_option())?;
        write!(f, "CRC-16: 0x{:04x}", self.checksum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessors() -> Result<()> {
        let mut cmd = PayoutByDenominationCommand::new();
        let max_num = MAX_PAYOUTS as u8;

        // checks that all valid number of payouts are set
        for num in 0..=max_num {
            cmd.set_number_of_payouts(num);
            assert_eq!(cmd.number_of_payouts(), num);
        }

        // attempt to set invalid number of payouts
        let invalid_num = max_num + 1;
        cmd.set_number_of_payouts(invalid_num);

        assert_ne!(cmd.number_of_payouts(), invalid_num);

        let list_num = 3;
        let denom_list = PayoutDenominationList::from([PayoutDenomination::new(); 3]);

        assert_eq!(denom_list.len(), list_num);

        cmd.set_payout_denominations(&denom_list);

        let ret_list = cmd.payout_denominations()?;

        assert_eq!(ret_list, denom_list);
        assert_eq!(ret_list.len(), list_num);

        cmd.set_payout_option(PayoutOption::PayoutAmount);
        assert_eq!(cmd.payout_option(), PayoutOption::PayoutAmount);

        cmd.set_payout_option(PayoutOption::TestPayoutAmount);
        assert_eq!(cmd.payout_option(), PayoutOption::TestPayoutAmount);

        let cmd_with = PayoutByDenominationCommand::new()
            // this call is technically unnecessary, but doesn't hurt to get coverage
            .with_number_of_payouts(list_num as u8)
            // this call overwrites `number`, which is what makes the above call unnecessary
            .with_payout_denominations(&denom_list)
            .with_payout_option(cmd.payout_option());

        assert_eq!(cmd_with, cmd);

        Ok(())
    }
}
