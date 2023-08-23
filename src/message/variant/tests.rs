use super::*;
use crate::message::MessageOps;

#[test]
fn test_variant_set_inhibits_destructure() -> Result<()> {
    let mut exp_msg = SetInhibitsResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_set_inhibits_response());
    assert_eq!(var.as_set_inhibits_response()?, &exp_msg);
    assert_eq!(var.into_set_inhibits_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_channel_value_data_destructure() -> Result<()> {
    let mut exp_msg = ChannelValueDataResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_channel_value_data_response());
    assert_eq!(var.as_channel_value_data_response()?, &exp_msg);
    assert_eq!(var.into_channel_value_data_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_configure_bezel_destructure() -> Result<()> {
    let mut exp_msg = ConfigureBezelResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_configure_bezel_response());
    assert_eq!(var.as_configure_bezel_response()?, &exp_msg);
    assert_eq!(var.into_configure_bezel_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_dataset_version_destructure() -> Result<()> {
    let mut exp_msg = DatasetVersionResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_dataset_version_response());
    assert_eq!(var.as_dataset_version_response()?, &exp_msg);
    assert_eq!(var.into_dataset_version_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_disable_destructure() -> Result<()> {
    let mut exp_msg = DisableResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_disable_response());
    assert_eq!(var.as_disable_response()?, &exp_msg);
    assert_eq!(var.into_disable_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_display_off_destructure() -> Result<()> {
    let mut exp_msg = DisplayOffResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_display_off_response());
    assert_eq!(var.as_display_off_response()?, &exp_msg);
    assert_eq!(var.into_display_off_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_display_on_destructure() -> Result<()> {
    let mut exp_msg = DisplayOnResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_display_on_response());
    assert_eq!(var.as_display_on_response()?, &exp_msg);
    assert_eq!(var.into_display_on_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_empty_destructure() -> Result<()> {
    let mut exp_msg = EmptyResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_empty_response());
    assert_eq!(var.as_empty_response()?, &exp_msg);
    assert_eq!(var.into_empty_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_encryption_reset_destructure() -> Result<()> {
    let mut exp_msg = EncryptionResetResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_encryption_reset_response());
    assert_eq!(var.as_encryption_reset_response()?, &exp_msg);
    assert_eq!(var.into_encryption_reset_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_enable_destructure() -> Result<()> {
    let mut exp_msg = EnableResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_enable_response());
    assert_eq!(var.as_enable_response()?, &exp_msg);
    assert_eq!(var.into_enable_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_enable_payout_destructure() -> Result<()> {
    let mut exp_msg = EnablePayoutResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_enable_payout_response());
    assert_eq!(var.as_enable_payout_response()?, &exp_msg);
    assert_eq!(var.into_enable_payout_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_event_ack_destructure() -> Result<()> {
    let mut exp_msg = EventAckResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_event_ack_response());
    assert_eq!(var.as_event_ack_response()?, &exp_msg);
    assert_eq!(var.into_event_ack_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_get_barcode_data_destructure() -> Result<()> {
    let mut exp_msg = GetBarcodeDataResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_get_barcode_data_response());
    assert_eq!(var.as_get_barcode_data_response()?, &exp_msg);
    assert_eq!(var.into_get_barcode_data_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_get_barcode_inhibit_destructure() -> Result<()> {
    let mut exp_msg = GetBarcodeInhibitResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_get_barcode_inhibit_response());
    assert_eq!(var.as_get_barcode_inhibit_response()?, &exp_msg);
    assert_eq!(var.into_get_barcode_inhibit_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_get_barcode_reader_configuration_destructure() -> Result<()> {
    let mut exp_msg = GetBarcodeReaderConfigurationResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_get_barcode_reader_configuration_response());
    assert_eq!(
        var.as_get_barcode_reader_configuration_response()?,
        &exp_msg
    );
    assert_eq!(
        var.into_get_barcode_reader_configuration_response()?,
        exp_msg
    );

    Ok(())
}

#[test]
fn test_variant_set_barcode_inhibit_destructure() -> Result<()> {
    let mut exp_msg = SetBarcodeInhibitResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_set_barcode_inhibit_response());
    assert_eq!(var.as_set_barcode_inhibit_response()?, &exp_msg);
    assert_eq!(var.into_set_barcode_inhibit_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_set_barcode_reader_configuration_destructure() -> Result<()> {
    let mut exp_msg = SetBarcodeReaderConfigurationResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_set_barcode_reader_configuration_response());
    assert_eq!(
        var.as_set_barcode_reader_configuration_response()?,
        &exp_msg
    );
    assert_eq!(
        var.into_set_barcode_reader_configuration_response()?,
        exp_msg
    );

    Ok(())
}

#[test]
fn test_variant_hold_destructure() -> Result<()> {
    let mut exp_msg = HoldResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_hold_response());
    assert_eq!(var.as_hold_response()?, &exp_msg);
    assert_eq!(var.into_hold_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_host_protocol_version_destructure() -> Result<()> {
    let mut exp_msg = HostProtocolVersionResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_host_protocol_version_response());
    assert_eq!(var.as_host_protocol_version_response()?, &exp_msg);
    assert_eq!(var.into_host_protocol_version_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_last_reject_code_destructure() -> Result<()> {
    let mut exp_msg = LastRejectCodeResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_last_reject_code_response());
    assert_eq!(var.as_last_reject_code_response()?, &exp_msg);
    assert_eq!(var.into_last_reject_code_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_poll_destructure() -> Result<()> {
    let mut exp_msg = PollResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_poll_response());
    assert_eq!(var.as_poll_response()?, &exp_msg);
    assert_eq!(var.into_poll_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_poll_with_ack_destructure() -> Result<()> {
    let mut exp_msg = PollWithAckResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_poll_with_ack_response());
    assert_eq!(var.as_poll_with_ack_response()?, &exp_msg);
    assert_eq!(var.into_poll_with_ack_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_reject_destructure() -> Result<()> {
    let mut exp_msg = RejectResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_reject_response());
    assert_eq!(var.as_reject_response()?, &exp_msg);
    assert_eq!(var.into_reject_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_serial_number_destructure() -> Result<()> {
    let mut exp_msg = SerialNumberResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_serial_number_response());
    assert_eq!(var.as_serial_number_response()?, &exp_msg);
    assert_eq!(var.into_serial_number_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_set_encryption_key_destructure() -> Result<()> {
    let mut exp_msg = SetEncryptionKeyResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_set_encryption_key_response());
    assert_eq!(var.as_set_encryption_key_response()?, &exp_msg);
    assert_eq!(var.into_set_encryption_key_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_set_generator_destructure() -> Result<()> {
    let mut exp_msg = SetGeneratorResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_set_generator_response());
    assert_eq!(var.as_set_generator_response()?, &exp_msg);
    assert_eq!(var.into_set_generator_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_set_modulus_destructure() -> Result<()> {
    let mut exp_msg = SetModulusResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_set_modulus_response());
    assert_eq!(var.as_set_modulus_response()?, &exp_msg);
    assert_eq!(var.into_set_modulus_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_request_key_exchange_destructure() -> Result<()> {
    let mut exp_msg = RequestKeyExchangeResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_request_key_exchange_response());
    assert_eq!(var.as_request_key_exchange_response()?, &exp_msg);
    assert_eq!(var.into_request_key_exchange_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_setup_request_destructure() -> Result<()> {
    let mut exp_msg = SetupRequestResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_setup_request_response());
    assert_eq!(var.as_setup_request_response()?, &exp_msg);
    assert_eq!(var.into_setup_request_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_smart_empty_destructure() -> Result<()> {
    let mut exp_msg = SmartEmptyResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_smart_empty_response());
    assert_eq!(var.as_smart_empty_response()?, &exp_msg);
    assert_eq!(var.into_smart_empty_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_sync_destructure() -> Result<()> {
    let mut exp_msg = SyncResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_sync_response());
    assert_eq!(var.as_sync_response()?, &exp_msg);
    assert_eq!(var.into_sync_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_unit_data_destructure() -> Result<()> {
    let mut exp_msg = UnitDataResponse::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_unit_data_response());
    assert_eq!(var.as_unit_data_response()?, &exp_msg);
    assert_eq!(var.into_unit_data_response()?, exp_msg);

    Ok(())
}

#[test]
fn test_variant_wrapped_encrypted_destructure() -> Result<()> {
    let mut exp_msg = WrappedEncryptedMessage::new();
    exp_msg.calculate_checksum();

    let exp_msg_type = exp_msg.message_type();
    let var = MessageVariant::from_buf(exp_msg.buf(), exp_msg_type)?;

    assert!(var.is_wrapped_encrypted_message());
    assert_eq!(var.as_wrapped_encrypted_message()?, &exp_msg);
    assert_eq!(var.into_wrapped_encrypted_message()?, exp_msg);

    Ok(())
}
