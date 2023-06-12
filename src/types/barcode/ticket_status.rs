use crate::std::fmt;

/// Status of a barcode ticket.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BarcodeTicketStatus {
    NoValidData = 0x00,
    TicketInEscrow = 0x01,
    TicketStacked = 0x02,
    TicketRejected = 0x03,
    Reserved = 0xff,
}

impl From<u8> for BarcodeTicketStatus {
    fn from(val: u8) -> Self {
        match val {
            0x00 => Self::NoValidData,
            0x01 => Self::TicketInEscrow,
            0x02 => Self::TicketStacked,
            0x03 => Self::TicketRejected,
            _ => Self::Reserved,
        }
    }
}

impl From<BarcodeTicketStatus> for u8 {
    fn from(val: BarcodeTicketStatus) -> Self {
        val as u8
    }
}

impl From<&BarcodeTicketStatus> for u8 {
    fn from(val: &BarcodeTicketStatus) -> Self {
        (*val).into()
    }
}

impl From<BarcodeTicketStatus> for &'static str {
    fn from(val: BarcodeTicketStatus) -> Self {
        match val {
            BarcodeTicketStatus::NoValidData => "No valid data",
            BarcodeTicketStatus::TicketInEscrow => "Ticket in escrow",
            BarcodeTicketStatus::TicketStacked => "Ticket stacked",
            BarcodeTicketStatus::TicketRejected => "Ticket rejected",
            BarcodeTicketStatus::Reserved => "Reserved",
        }
    }
}

impl From<&BarcodeTicketStatus> for &'static str {
    fn from(val: &BarcodeTicketStatus) -> Self {
        (*val).into()
    }
}

impl fmt::Display for BarcodeTicketStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}
