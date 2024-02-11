use alloc::string::{String, ToString};
use anyhow::{anyhow, Error, Result};
use core::{str, time::Duration};
use defmt::Format;

macro_rules! impl_try_from_slice_for {
    ($enum: ident, $($byte: expr => $variant: ident,)*) => {
        impl<'a> TryFrom<&'a [u8]> for $enum {
            type Error = Error;

            fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
                match value {
                    $( $byte => Ok($enum::$variant), )*
                    _ => Err(anyhow!("Invalid {} variant {:#?}", stringify!($enum), value))
                }
            }
        }
    };
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub enum A2dpState {
    Unsupported,
    Standby,
    Connecting,
    Connected,
    Streaming,
}

impl_try_from_slice_for!(A2dpState,
    b"0" => Unsupported,
    b"1" => Standby,
    b"2" => Connecting,
    b"3" => Connected,
    b"4" => Streaming,
);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub enum AvrcpState {
    Unsupported,
    Standby,
    Connecting,
}

impl_try_from_slice_for!(AvrcpState,
    b"0" => Unsupported,
    b"1" => Standby,
    b"2" => Connecting,
);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub enum PlayStat {
    Stopped,
    Playing,
    Paused,
    FastForwarding,
    FastRewinding,
}

impl_try_from_slice_for!(PlayStat,
    b"0" => Stopped,
    b"1" => Playing,
    b"2" => Paused,
    b"3" => FastForwarding,
    b"4" => FastRewinding,
);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub enum SppStat {
    Unsupported,
    Standby,
    Connecting,
    Connected,
}

impl_try_from_slice_for!(SppStat,
    b"0" => Unsupported,
    b"1" => Standby,
    b"2" => Connecting,
    b"3" => Connected,
);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub enum GattStat {
    Unsupported,
    Standby,
    Connecting,
    Connected,
}

impl_try_from_slice_for!(GattStat,
    b"0" => Unsupported,
    b"1" => Standby,
    b"2" => Connecting,
    b"3" => Connected,
);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Indication {
    Ok,
    Err,

    A2dpState(A2dpState),
    A2dpDev(String),
    AvrcpState(AvrcpState),
    PlayStat(PlayStat),
    TrackStat {
        play_stat: PlayStat,
        elapsed_time: Duration,
        total_time: Duration,
    },
    TrackInfo {
        title: String,
        artist: String,
        album: String,
    },

    SppStat(SppStat),
    GattStat(GattStat),
    SppDev(String),
    GattDev(String),
    SppData(Option<usize>, String),
    GattData(Option<usize>, String),

    Unsupported(String, Option<String>),
}

impl<'a> TryFrom<&'a [u8]> for Indication {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value == b"OK" {
            return Ok(Self::Ok);
        }

        if value == b"ERROR" {
            return Ok(Self::Err);
        }

        let (indication, params) = match value.iter().position(|c| c == &b'=') {
            Some(index) => (&value[..index], Some(&value[index + 1..])),
            None => (value, None),
        };

        match (indication, params) {
            (b"+A2DPSTATE", Some(params)) => Ok(Self::A2dpState(A2dpState::try_from(params)?)),
            (b"+A2DPDEV", Some(params)) => Ok(Self::A2dpDev(
                str::from_utf8(params).map_err(Error::msg)?.to_string(),
            )),
            (b"+AVRCPSTATE", Some(params)) => Ok(Self::AvrcpState(AvrcpState::try_from(params)?)),
            (b"+PLAYSTAT", Some(params)) => Ok(Self::PlayStat(PlayStat::try_from(params)?)),
            (b"+TRACKSTAT", Some(params)) => {
                let (play_stat, params) =
                    params.split_once(|c| c == &b',').unwrap_or((params, &[]));
                let (elapsed_time, total_time) =
                    params.split_once(|c| c == &b',').unwrap_or((params, &[]));

                Ok(Self::TrackStat {
                    play_stat: PlayStat::try_from(play_stat)?,
                    elapsed_time: Duration::from_millis(
                        str::from_utf8(elapsed_time)
                            .map_err(Error::msg)?
                            .parse()
                            .map_err(Error::msg)?,
                    ),
                    total_time: Duration::from_millis(
                        str::from_utf8(total_time)
                            .map_err(Error::msg)?
                            .parse()
                            .map_err(Error::msg)?,
                    ),
                })
            }
            (b"+TRACKINFO", Some(params)) => {
                let (title, params) = params.split_once(|c| c == &0xFF).unwrap_or((params, &[]));
                let (artist, album) = params.split_once(|c| c == &0xFF).unwrap_or((params, &[]));

                Ok(Self::TrackInfo {
                    title: str::from_utf8(title).map_err(Error::msg)?.to_string(),
                    artist: str::from_utf8(artist).map_err(Error::msg)?.to_string(),
                    album: str::from_utf8(album).map_err(Error::msg)?.to_string(),
                })
            }
            (b"+SPPSTAT", Some(params)) => Ok(Self::SppStat(SppStat::try_from(params)?)),
            (b"+GATTSTAT", Some(params)) => Ok(Self::GattStat(GattStat::try_from(params)?)),
            (b"+SPPDEV", Some(params)) => Ok(Self::SppDev(
                str::from_utf8(params).map_err(Error::msg)?.to_string(),
            )),
            (b"+GATTDEV", Some(params)) => Ok(Self::GattDev(
                str::from_utf8(params).map_err(Error::msg)?.to_string(),
            )),
            (b"+SPPDATA", Some(params)) => match params.iter().position(|c| c == &b',') {
                Some(index) => {
                    let (size, data) = params.split_at(index);
                    Ok(Self::SppData(
                        Some(
                            str::from_utf8(size)
                                .map_err(Error::msg)?
                                .parse()
                                .map_err(Error::msg)?,
                        ),
                        str::from_utf8(data).map_err(Error::msg)?.to_string(),
                    ))
                }
                None => Ok(Self::SppData(
                    None,
                    str::from_utf8(params).map_err(Error::msg)?.to_string(),
                )),
            },
            (b"+GATTDATA", Some(params)) => match params.iter().position(|c| c == &b',') {
                Some(index) => {
                    let (size, data) = params.split_at(index);
                    Ok(Self::GattData(
                        Some(
                            str::from_utf8(size)
                                .map_err(Error::msg)?
                                .parse()
                                .map_err(Error::msg)?,
                        ),
                        str::from_utf8(data).map_err(Error::msg)?.to_string(),
                    ))
                }
                None => Ok(Self::GattData(
                    None,
                    str::from_utf8(params).map_err(Error::msg)?.to_string(),
                )),
            },
            _ => Ok(Self::Unsupported(
                str::from_utf8(indication).map_err(Error::msg)?.to_string(),
                params.map(|params| String::from_utf8_lossy(params).to_string()),
            )),
        }
    }
}
