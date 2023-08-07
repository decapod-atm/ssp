use parking_lot::{Mutex, MutexGuard};

use crate::{ChannelValue, Error, Result};

#[cfg(feature = "nv200")]
pub const MAX_CHANNELS: usize = 24;
#[cfg(not(feature = "nv200"))]
pub const MAX_CHANNELS: usize = 16;

pub const CHANNEL_TIMEOUT_MS: u128 = 2_500;
pub const CHANNEL_TIMEOUT_ATTEMPTS: u64 = 10_000;

pub type Channels = [ChannelValue; MAX_CHANNELS];

/// Device channels for storing notes.
///
/// By default, all channels have a zero-value.
///
/// Channels are configured after making a call to the device that returns the number of channels,
/// and their respective values.
pub static CHANNELS: Mutex<Channels> = Mutex::new([
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    ChannelValue::from_inner(0),
    #[cfg(feature = "nv200")]
    ChannelValue::from_inner(0),
    #[cfg(feature = "nv200")]
    ChannelValue::from_inner(0),
    #[cfg(feature = "nv200")]
    ChannelValue::from_inner(0),
    #[cfg(feature = "nv200")]
    ChannelValue::from_inner(0),
    #[cfg(feature = "nv200")]
    ChannelValue::from_inner(0),
    #[cfg(feature = "nv200")]
    ChannelValue::from_inner(0),
    #[cfg(feature = "nv200")]
    ChannelValue::from_inner(0),
]);

/// Acquires a lock on the global [Channels].
///
/// Use operating system timers to measure elapsed time. Fail if timeout expires.
#[cfg(feature = "std")]
pub fn lock_channels() -> Result<MutexGuard<'static, Channels>> {
    use crate::std::time;

    let now = time::Instant::now();

    while now.elapsed().as_millis() < CHANNEL_TIMEOUT_MS {
        if let Some(chan_lock) = CHANNELS.try_lock() {
            return Ok(chan_lock);
        }
    }

    Err(Error::Timeout("channels lock".into()))
}

/// Acquires a lock on the global [Channels].
///
/// No access to operating system timers, so approximate with a number of attempts.
#[cfg(not(feature = "std"))]
pub fn lock_channels() -> Result<MutexGuard<'static, Channels>> {
    let mut attempts = 0;
    while attempts < CHANNEL_TIMEOUT_ATTEMPTS {
        if let Some(chan_lock) = CHANNELS.try_lock() {
            return Ok(chan_lock);
        }

        attempts += 1;
    }

    Err(Error::Timeout("channels lock".into()))
}

/// Gets a reference to the number of configured channels.
///
/// Example:
///
/// ```rust, no_run
/// # fn main() -> ssp::Result<()> {
/// let chan_lock = ssp::lock_channels()?;
/// let _channels = ssp::channels(&chan_lock)?;
///
/// # Ok(())
/// # }
/// ```
pub fn channels<'a>(chan_lock: &'a MutexGuard<'static, Channels>) -> Result<&'a [ChannelValue]> {
    // Find the first unconfigured channel.
    // If no channels are configured, a zero-length slice is returned.
    let idx = chan_lock
        .iter()
        .position(|p| p.as_inner() == 0)
        .unwrap_or(0);
    Ok(chan_lock[..idx].as_ref())
}

/// Gets a mutable reference to the number of configured channels.
///
/// Example:
///
/// ```rust, no_run
/// # fn main() -> ssp::Result<()> {
/// let mut chan_lock = ssp::lock_channels()?;
/// let _channels = ssp::channels_mut(&mut chan_lock)?;
///
/// # Ok(())
/// # }
/// ```
pub fn channels_mut<'a>(
    chan_lock: &'a mut MutexGuard<'static, Channels>,
) -> Result<&'a mut [ChannelValue]> {
    // Find the first unconfigured channel.
    // If no channels are configured, a zero-length slice is returned.
    let idx = chan_lock
        .iter_mut()
        .position(|p| p.as_inner() == 0)
        .unwrap_or(0);
    Ok(chan_lock[..idx].as_mut())
}

/// Used to set the configured channels after a device call that returns the number and
/// value of configured channels, e.g. [SetupRequest](crate::setup_request).
///
/// Example:
///
/// ```rust, no_run
/// # fn main() -> ssp::Result<()> {
/// // Pretend this was a response to an actual request...
/// let res = ssp::SetupRequestResponse::new();
/// let channels = res.channel_values()?;
///
/// ssp::configure_channels(channels.as_ref())?;
/// # Ok(())
/// # }
pub fn configure_channels(channels: &[ChannelValue]) -> Result<()> {
    let mut chan_lock = lock_channels()?;

    configure_channels_with_lock(&mut chan_lock, channels)
}

/// While holding a mutable lock on global channels, used to set the configured channels after a device
/// call that returns the number and value of configured channels, e.g. [SetupRequest](crate::setup_request).
///
/// Example:
///
/// ```rust, no_run
/// # fn main() -> ssp::Result<()> {
/// // Pretend this was a response to an actual request...
/// let res = ssp::SetupRequestResponse::new();
/// let channels = res.channel_values()?;
///
/// let mut chan_lock = ssp::lock_channels()?;
/// ssp::configure_channels_with_lock(&mut chan_lock, channels.as_ref())?;
/// # Ok(())
/// # }
pub fn configure_channels_with_lock(
    chan_lock: &mut MutexGuard<'static, Channels>,
    channels: &[ChannelValue],
) -> Result<()> {
    let len = channels.len();

    if len <= MAX_CHANNELS {
        chan_lock[..len]
            .iter_mut()
            .zip(channels.iter())
            .for_each(|(c, &s)| *c = s);

        Ok(())
    } else {
        Err(Error::InvalidLength((len, MAX_CHANNELS)))
    }
}

/// Gets the [ChannelValue] of the given channel index.
///
/// Channels are one-indexed, since many events use zero as a special value when returning the
/// channel index. For example, [Read](crate::ResponseStatus::Read) uses zero to indicate a note is
/// still being processed, and non-zero values to indicate the channel a validated note moves into.
///
/// Example:
///
/// ```rust, no_run
/// # fn main() -> ssp::Result<()> {
/// // Channels are one-indexed, zero is a special value
/// let _val = ssp::channel_value(1)?;
/// # Ok(())
/// # }
pub fn channel_value(channel: usize) -> Result<ChannelValue> {
    channel_value_with_lock(&lock_channels()?, channel)
}

/// While holding a lock on global channels, gets the [ChannelValue] of the given channel index.
///
/// Channels are one-indexed, since many events use zero as a special value when returning the
/// channel index. For example, [Read](crate::ResponseStatus::Read) uses zero to indicate a note is
/// still being processed, and non-zero values to indicate the channel a validated note moves into.
///
/// Example:
///
/// ```rust, no_run
/// # fn main() -> ssp::Result<()> {
/// // Channels are one-indexed, zero is a special value
/// let _val = ssp::channel_value(1)?;
/// # Ok(())
/// # }
pub fn channel_value_with_lock(
    chan_lock: &MutexGuard<'static, Channels>,
    channel: usize,
) -> Result<ChannelValue> {
    match channel {
        0 => Ok(ChannelValue::default()),
        c => {
            let val = chan_lock
                .get(c - 1)
                .ok_or(Error::InvalidLength((c, MAX_CHANNELS)))?;

            Ok(*val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configure_channels() -> Result<()> {
        let exp_channels = [
            ChannelValue::from(1),
            ChannelValue::from(5),
            ChannelValue::from(10),
            ChannelValue::from(20),
            ChannelValue::from(50),
            ChannelValue::from(100),
        ];
        let exp_len = exp_channels.len();

        // Before configuring, check all channels are zero-valued
        for c in 1..=MAX_CHANNELS {
            assert_eq!(channel_value(c)?.as_inner(), 0);
        }

        {
            let mut chan_lock = lock_channels()?;

            assert!(channels(&chan_lock)?.is_empty());
            assert!(channels_mut(&mut chan_lock)?.is_empty());
        }

        configure_channels(exp_channels.as_ref())?;

        let mut chan_lock = lock_channels()?;
        assert_eq!(chan_lock[..exp_len].as_ref(), exp_channels.as_ref());

        // check that "channel" zero still returns the default value
        assert_eq!(channel_value_with_lock(&chan_lock, 0)?.as_inner(), 0);

        // check that the configured channels return their expected values
        for c in 1..=exp_len {
            assert_eq!(channel_value_with_lock(&chan_lock, c)?, exp_channels[c - 1]);
        }

        let channels = channels(&chan_lock)?;
        assert_eq!(channels, exp_channels.as_ref());

        let channels = channels_mut(&mut chan_lock)?;
        assert_eq!(channels, exp_channels.as_ref());

        // reset to default values for other tests
        configure_channels_with_lock(
            &mut chan_lock,
            [
                ChannelValue::default(),
                ChannelValue::default(),
                ChannelValue::default(),
                ChannelValue::default(),
                ChannelValue::default(),
                ChannelValue::default(),
            ]
            .as_ref(),
        )?;

        for c in 0..MAX_CHANNELS {
            assert_eq!(channel_value_with_lock(&chan_lock, c)?.as_inner(), 0);
        }

        Ok(())
    }
}
