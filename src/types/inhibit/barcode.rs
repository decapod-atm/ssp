use crate::{anti_bool_enum, std::fmt, tuple_struct};

anti_bool_enum!(
    CurrencyEnabled,
    "Status flag for whether currency is accepted by the device (0 = enabled, 1 = disabled)."
);
anti_bool_enum!(
    BarcodeEnabled,
    "Status flag for whether barcodes are accepted by the device (0 = enabled, 1 = disabled)."
);
anti_bool_enum!(ChannelEnabled, "Status flag for whether a channel position is enabled on the device (0 = enabled, 1 = disabled).");

tuple_struct!(
    BarcodeCurrencyInhibit,
    u8,
    r"
 Status flags for barcode/currency mode, and which channels are enabled by the device.

 All fields are anti-boolean bitflags (0 = enabled, 1 = disabled).
"
);

impl BarcodeCurrencyInhibit {
    /// Gets the [CurrencyEnabled] setting.
    pub fn currency_enabled(&self) -> CurrencyEnabled {
        (self.0 & 0b1).into()
    }

    /// Sets the [CurrencyEnabled] setting.
    pub fn set_currency_enabled(&mut self, enable: CurrencyEnabled) {
        if bool::from(enable) {
            self.0 |= u8::from(enable);
        } else {
            self.0 &= !1;
        }
    }

    /// Gets the [BarcodeEnabled] setting.
    pub fn barcode_enabled(&self) -> BarcodeEnabled {
        ((self.0 >> 1) & 0b1).into()
    }

    /// Sets the [BarcodeEnabled] setting.
    pub fn set_barcode_enabled(&mut self, enable: BarcodeEnabled) {
        if bool::from(enable) {
            self.0 |= u8::from(enable) << 1;
        } else {
            self.0 &= !(1 << 1);
        }
    }

    /// Gets the [ChannelEnabled] setting for Channel 0.
    pub fn channel_0_enabled(&self) -> ChannelEnabled {
        (self.0 >> 2).into()
    }

    /// Sets the [ChannelEnabled] setting for Channel 0.
    pub fn set_channel_0_enabled(&mut self, enable: ChannelEnabled) {
        if bool::from(enable) {
            self.0 |= u8::from(enable) << 2;
        } else {
            self.0 &= !(1 << 2);
        }
    }

    /// Gets the [ChannelEnabled] setting for Channel 1.
    pub fn channel_1_enabled(&self) -> ChannelEnabled {
        (self.0 >> 3).into()
    }

    /// Sets the [ChannelEnabled] setting for Channel 1.
    pub fn set_channel_1_enabled(&mut self, enable: ChannelEnabled) {
        if bool::from(enable) {
            self.0 |= u8::from(enable) << 3;
        } else {
            self.0 &= !(1 << 3);
        }
    }

    /// Gets the [ChannelEnabled] setting for Channel 2.
    pub fn channel_2_enabled(&self) -> ChannelEnabled {
        (self.0 >> 4).into()
    }

    /// Sets the [ChannelEnabled] setting for Channel 2.
    pub fn set_channel_2_enabled(&mut self, enable: ChannelEnabled) {
        if bool::from(enable) {
            self.0 |= u8::from(enable) << 4;
        } else {
            self.0 &= !(1 << 4);
        }
    }

    /// Gets the [ChannelEnabled] setting for Channel 3.
    pub fn channel_3_enabled(&self) -> ChannelEnabled {
        (self.0 >> 5).into()
    }

    /// Sets the [ChannelEnabled] setting for Channel 3.
    pub fn set_channel_3_enabled(&mut self, enable: ChannelEnabled) {
        if bool::from(enable) {
            self.0 |= u8::from(enable) << 5;
        } else {
            self.0 &= !(1 << 5);
        }
    }

    /// Gets the [ChannelEnabled] setting for Channel 4.
    pub fn channel_4_enabled(&self) -> ChannelEnabled {
        (self.0 >> 6).into()
    }

    /// Sets the [ChannelEnabled] setting for Channel 4.
    pub fn set_channel_4_enabled(&mut self, enable: ChannelEnabled) {
        if bool::from(enable) {
            self.0 |= u8::from(enable) << 6;
        } else {
            self.0 &= !(1 << 6);
        }
    }

    /// Gets the [ChannelEnabled] setting for Channel 5.
    pub fn channel_5_enabled(&self) -> ChannelEnabled {
        (self.0 >> 7).into()
    }

    /// Sets the [ChannelEnabled] setting for Channel 5.
    pub fn set_channel_5_enabled(&mut self, enable: ChannelEnabled) {
        if bool::from(enable) {
            self.0 |= u8::from(enable) << 7;
        } else {
            self.0 &= !(1 << 7);
        }
    }
}

impl fmt::Display for BarcodeCurrencyInhibit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let currency = self.currency_enabled();
        let barcode = self.barcode_enabled();
        let channel_0 = self.channel_0_enabled();
        let channel_1 = self.channel_1_enabled();
        let channel_2 = self.channel_2_enabled();
        let channel_3 = self.channel_3_enabled();
        let channel_4 = self.channel_4_enabled();
        let channel_5 = self.channel_5_enabled();

        write!(f, "Currency({currency}) : Barcode({barcode}) : Channel 0 ({channel_0}) : Channel 1 ({channel_1}) : Channel 2 ({channel_2}) : Channel 3 ({channel_3}) : Channel 4 ({channel_4}) : Channel 5 ({channel_5})")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barcode_currency_inhibit() {
        let currency_on_no_channels = 0b1111_1110;
        let currency_on_all_channels = 0b0000_0010;

        let barcode_on_no_channels = 0b1111_1101;

        let currency_barcode_on_no_channels = 0b1111_1100;
        let currency_barcode_on_all_channels = 0b0000_0000;

        let inh_currency_on_no_channels = BarcodeCurrencyInhibit::from(currency_on_no_channels);
        let inh_currency_on_all_channels = BarcodeCurrencyInhibit::from(currency_on_all_channels);

        let inh_barcode_on_no_channels = BarcodeCurrencyInhibit::from(barcode_on_no_channels);

        let inh_currency_barcode_on_no_channels =
            BarcodeCurrencyInhibit::from(currency_barcode_on_no_channels);
        let inh_currency_barcode_on_all_channels =
            BarcodeCurrencyInhibit::from(currency_barcode_on_all_channels);

        assert_eq!(
            inh_currency_on_no_channels.currency_enabled(),
            CurrencyEnabled::Set
        );
        assert_eq!(
            inh_currency_on_no_channels.barcode_enabled(),
            BarcodeEnabled::Unset
        );
        assert_eq!(
            inh_currency_on_no_channels.channel_0_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_on_no_channels.channel_1_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_on_no_channels.channel_2_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_on_no_channels.channel_3_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_on_no_channels.channel_4_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_on_no_channels.channel_5_enabled(),
            ChannelEnabled::Unset
        );

        assert_eq!(
            inh_currency_on_all_channels.currency_enabled(),
            CurrencyEnabled::Set
        );
        assert_eq!(
            inh_currency_on_all_channels.barcode_enabled(),
            BarcodeEnabled::Unset
        );
        assert_eq!(
            inh_currency_on_all_channels.channel_0_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_on_all_channels.channel_1_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_on_all_channels.channel_2_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_on_all_channels.channel_3_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_on_all_channels.channel_4_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_on_all_channels.channel_5_enabled(),
            ChannelEnabled::Set
        );

        assert_eq!(
            inh_barcode_on_no_channels.currency_enabled(),
            CurrencyEnabled::Unset
        );
        assert_eq!(
            inh_barcode_on_no_channels.barcode_enabled(),
            BarcodeEnabled::Set
        );
        assert_eq!(
            inh_barcode_on_no_channels.channel_0_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_barcode_on_no_channels.channel_1_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_barcode_on_no_channels.channel_2_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_barcode_on_no_channels.channel_3_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_barcode_on_no_channels.channel_4_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_barcode_on_no_channels.channel_5_enabled(),
            ChannelEnabled::Unset
        );

        assert_eq!(
            inh_currency_barcode_on_no_channels.currency_enabled(),
            CurrencyEnabled::Set
        );
        assert_eq!(
            inh_currency_barcode_on_no_channels.barcode_enabled(),
            BarcodeEnabled::Set
        );
        assert_eq!(
            inh_currency_barcode_on_no_channels.channel_0_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_barcode_on_no_channels.channel_1_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_barcode_on_no_channels.channel_2_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_barcode_on_no_channels.channel_3_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_barcode_on_no_channels.channel_4_enabled(),
            ChannelEnabled::Unset
        );
        assert_eq!(
            inh_currency_barcode_on_no_channels.channel_5_enabled(),
            ChannelEnabled::Unset
        );

        assert_eq!(
            inh_currency_barcode_on_all_channels.currency_enabled(),
            CurrencyEnabled::Set
        );
        assert_eq!(
            inh_currency_barcode_on_all_channels.barcode_enabled(),
            BarcodeEnabled::Set
        );
        assert_eq!(
            inh_currency_barcode_on_all_channels.channel_0_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_barcode_on_all_channels.channel_1_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_barcode_on_all_channels.channel_2_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_barcode_on_all_channels.channel_3_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_barcode_on_all_channels.channel_4_enabled(),
            ChannelEnabled::Set
        );
        assert_eq!(
            inh_currency_barcode_on_all_channels.channel_5_enabled(),
            ChannelEnabled::Set
        );
    }

    // All bitfields default to set (0).
    //
    // So, unset them (set to 1), set them (set to 0), and unset them (set to 1).
    //
    // This ensures the bitfield operations are working correctly.

    #[test]
    fn test_currency_bitfield() {
        let mut currency_barcode_inh = BarcodeCurrencyInhibit::default();

        currency_barcode_inh.set_currency_enabled(CurrencyEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.currency_enabled(),
            CurrencyEnabled::Unset
        );

        currency_barcode_inh.set_currency_enabled(CurrencyEnabled::Set);
        assert_eq!(
            currency_barcode_inh.currency_enabled(),
            CurrencyEnabled::Set
        );

        currency_barcode_inh.set_currency_enabled(CurrencyEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.currency_enabled(),
            CurrencyEnabled::Unset
        );
    }

    #[test]
    fn test_barcode_bitfield() {
        let mut currency_barcode_inh = BarcodeCurrencyInhibit::default();

        currency_barcode_inh.set_barcode_enabled(BarcodeEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.barcode_enabled(),
            BarcodeEnabled::Unset
        );

        currency_barcode_inh.set_barcode_enabled(BarcodeEnabled::Set);
        assert_eq!(currency_barcode_inh.barcode_enabled(), BarcodeEnabled::Set);

        currency_barcode_inh.set_barcode_enabled(BarcodeEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.barcode_enabled(),
            BarcodeEnabled::Unset
        );
    }

    #[test]
    fn test_channel0_bitfield() {
        let mut currency_barcode_inh = BarcodeCurrencyInhibit::default();

        currency_barcode_inh.set_channel_0_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_0_enabled(),
            ChannelEnabled::Unset
        );

        currency_barcode_inh.set_channel_0_enabled(ChannelEnabled::Set);
        assert_eq!(
            currency_barcode_inh.channel_0_enabled(),
            ChannelEnabled::Set
        );

        currency_barcode_inh.set_channel_0_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_0_enabled(),
            ChannelEnabled::Unset
        );
    }

    #[test]
    fn test_channel1_bitfield() {
        let mut currency_barcode_inh = BarcodeCurrencyInhibit::default();

        currency_barcode_inh.set_channel_1_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_1_enabled(),
            ChannelEnabled::Unset
        );

        currency_barcode_inh.set_channel_1_enabled(ChannelEnabled::Set);
        assert_eq!(
            currency_barcode_inh.channel_1_enabled(),
            ChannelEnabled::Set
        );

        currency_barcode_inh.set_channel_1_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_1_enabled(),
            ChannelEnabled::Unset
        );
    }

    #[test]
    fn test_channel2_bitfield() {
        let mut currency_barcode_inh = BarcodeCurrencyInhibit::default();

        currency_barcode_inh.set_channel_2_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_2_enabled(),
            ChannelEnabled::Unset
        );

        currency_barcode_inh.set_channel_2_enabled(ChannelEnabled::Set);
        assert_eq!(
            currency_barcode_inh.channel_2_enabled(),
            ChannelEnabled::Set
        );

        currency_barcode_inh.set_channel_2_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_2_enabled(),
            ChannelEnabled::Unset
        );
    }

    #[test]
    fn test_channel3_bitfield() {
        let mut currency_barcode_inh = BarcodeCurrencyInhibit::default();

        currency_barcode_inh.set_channel_3_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_3_enabled(),
            ChannelEnabled::Unset
        );

        currency_barcode_inh.set_channel_3_enabled(ChannelEnabled::Set);
        assert_eq!(
            currency_barcode_inh.channel_3_enabled(),
            ChannelEnabled::Set
        );

        currency_barcode_inh.set_channel_3_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_3_enabled(),
            ChannelEnabled::Unset
        );
    }

    #[test]
    fn test_channel4_bitfield() {
        let mut currency_barcode_inh = BarcodeCurrencyInhibit::default();

        currency_barcode_inh.set_channel_4_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_4_enabled(),
            ChannelEnabled::Unset
        );

        currency_barcode_inh.set_channel_4_enabled(ChannelEnabled::Set);
        assert_eq!(
            currency_barcode_inh.channel_4_enabled(),
            ChannelEnabled::Set
        );

        currency_barcode_inh.set_channel_4_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_4_enabled(),
            ChannelEnabled::Unset
        );
    }

    #[test]
    fn test_channel5_bitfield() {
        let mut currency_barcode_inh = BarcodeCurrencyInhibit::default();

        currency_barcode_inh.set_channel_5_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_5_enabled(),
            ChannelEnabled::Unset
        );

        currency_barcode_inh.set_channel_5_enabled(ChannelEnabled::Set);
        assert_eq!(
            currency_barcode_inh.channel_5_enabled(),
            ChannelEnabled::Set
        );

        currency_barcode_inh.set_channel_5_enabled(ChannelEnabled::Unset);
        assert_eq!(
            currency_barcode_inh.channel_5_enabled(),
            ChannelEnabled::Unset
        );
    }
}
