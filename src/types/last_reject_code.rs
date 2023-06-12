use crate::std::fmt;

/// Last reject code processed by the device.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LastRejectCode {
    NoteAccepted = 0x00,
    NotLengthIncorrect,
    InvalidNote,
    ChannelInhibited = 0x06,
    SecondNoteInserted,
    HostRejectedNote,
    InvalidNoteRead = 0x0a,
    NoteTooLong,
    ValidatorDisabled,
    MechanismSlow,
    StrimmingAttempt,
    FraudChannelReject,
    NoNotesInserted,
    PeakDetectFail,
    TwistedNoteDetected,
    EscrowTimeout,
    BarcodeScanFail,
    IncorrectNoteWidth = 0x19,
    NoteTooShort,
    Reserved = 0xff,
}

impl From<u8> for LastRejectCode {
    fn from(b: u8) -> Self {
        match b {
            0x00 => Self::NoteAccepted,
            0x01 => Self::NotLengthIncorrect,
            0x02..=0x05 | 0x09 => Self::InvalidNote,
            0x06 => Self::ChannelInhibited,
            0x07 => Self::SecondNoteInserted,
            0x08 => Self::HostRejectedNote,
            0x0a | 0x15..=0x18 => Self::InvalidNoteRead,
            0x0b => Self::NoteTooLong,
            0x0c => Self::ValidatorDisabled,
            0x0d => Self::MechanismSlow,
            0x0e => Self::StrimmingAttempt,
            0x0f => Self::FraudChannelReject,
            0x10 => Self::NoNotesInserted,
            0x11 => Self::PeakDetectFail,
            0x12 => Self::TwistedNoteDetected,
            0x13 => Self::EscrowTimeout,
            0x14 => Self::BarcodeScanFail,
            0x19 => Self::IncorrectNoteWidth,
            0x1a => Self::NoteTooShort,
            _ => Self::Reserved,
        }
    }
}

impl From<LastRejectCode> for u8 {
    fn from(m: LastRejectCode) -> Self {
        m as u8
    }
}

impl From<&LastRejectCode> for u8 {
    fn from(m: &LastRejectCode) -> Self {
        (*m).into()
    }
}

impl From<LastRejectCode> for &'static str {
    fn from(l: LastRejectCode) -> Self {
        match l {
            LastRejectCode::NoteAccepted => "Note accepted",
            LastRejectCode::NotLengthIncorrect => "Not length incorrect",
            LastRejectCode::InvalidNote => "Invalid note",
            LastRejectCode::ChannelInhibited => "Channel inhibited",
            LastRejectCode::SecondNoteInserted => "Second note inserted",
            LastRejectCode::HostRejectedNote => "Host rejected note",
            LastRejectCode::InvalidNoteRead => "Invalid note read",
            LastRejectCode::NoteTooLong => "Note too long",
            LastRejectCode::ValidatorDisabled => "Validator disabled",
            LastRejectCode::MechanismSlow => "Mechanism slow/stalled",
            LastRejectCode::StrimmingAttempt => "Strimming attempt",
            LastRejectCode::FraudChannelReject => "Fraud channel reject",
            LastRejectCode::NoNotesInserted => "No notes inserted",
            LastRejectCode::PeakDetectFail => "Peak detect fail",
            LastRejectCode::TwistedNoteDetected => "Twisted note detected",
            LastRejectCode::EscrowTimeout => "Escrow time-out",
            LastRejectCode::BarcodeScanFail => "Barcode scan fail",
            LastRejectCode::IncorrectNoteWidth => "Incorrect note width",
            LastRejectCode::NoteTooShort => "Note too short",
            LastRejectCode::Reserved => "Reserved",
        }
    }
}

impl From<&LastRejectCode> for &'static str {
    fn from(l: &LastRejectCode) -> Self {
        (*l).into()
    }
}

impl fmt::Display for LastRejectCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}
