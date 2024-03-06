use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{self, Debug, Display},
    num::ParseIntError,
    str::{self, Utf8Error},
};
use defmt::Format;

#[derive(Debug, Eq, PartialEq, Clone, Format)]
pub struct InvalidVariantError(String, Vec<u8>);

impl Display for InvalidVariantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"+{} invalid variant "{:?}""#, self.0, self.1)
    }
}

macro_rules! define_error {
    ($($name:ident),+ $(,)?) => {
        #[derive(Debug, Eq, PartialEq, Clone)]
        pub enum Error {
            $($name($name)),+
        }

        $(
            impl From<$name> for Error {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }
        )+

        impl Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$name(error) => Display::fmt(error, f)),+
                }
            }
        }
    };
}

define_error!(Utf8Error, ParseIntError, InvalidVariantError);

macro_rules! string_indications {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(Debug, Eq, PartialEq, Clone, Format)]
            pub struct $name(String);

            impl TryFrom<&[u8]> for $name {
                type Error = Utf8Error;

                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    Ok($name(str::from_utf8(value)?.to_string()))
                }
            }
        )?
    };
}

macro_rules! data_indications {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(Debug, Eq, PartialEq, Clone, Format)]
            pub struct $name(String);

            impl TryFrom<&[u8]> for $name {
                type Error = Utf8Error;

                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    let (len, data) = value.split_once(|c| c == &b',').unwrap_or((&[], value));

                    // TODO if len is provided perform check and error

                    Ok($name(str::from_utf8(data)?.to_string()))
                }
            }
        )?
    };
}

macro_rules! enum_indications {
    ($($name:ident { $($byte:literal => $variant:ident),+ $(,)? }),+ $(,)?) => {
        $(
            #[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
            pub enum $name {
                $($variant),+
            }

            impl TryFrom<&[u8]> for $name {
                type Error = InvalidVariantError;

                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    match value {
                        $($byte => Ok($name::$variant),)+
                        _ => Err(InvalidVariantError(stringify!($name).to_string(), value.to_vec())),
                    }
                }
            }
        )+
    };
}

string_indications!(A2dpDev, Addr, GattDev, LeAddr, SppDev, Ver);
data_indications!(GattData, SppData);
enum_indications!(
    A2dpStat {
        b"0" => Unsupported,
        b"1" => Standby,
        b"2" => Connecting,
        b"3" => Connected,
        b"4" => Streaming,
    },
    AvrcpStat {
        b"0" => Unsupported,
        b"1" => Standby,
        b"2" => Connecting,
        b"3" => Connected,
    },
    GattStat {
        b"0" => Unsupported,
        b"1" => Standby,
        b"2" => Connecting,
        b"3" => Connected,
    },
    PlayStat {
        b"0" => Stopped,
        b"1" => Playing,
        b"2" => Paused,
        b"3" => FastForwarding,
        b"4" => FastRewinding,
    },
    SppStat {
        b"0" => Unsupported,
        b"1" => Standby,
        b"2" => Connecting,
        b"3" => Connected,
    },
);

mod trackinfo;
mod trackstat;

pub use trackinfo::TrackInfo;
pub use trackstat::TrackStat;

macro_rules! indications {
    ($($bytes:literal => $type:ident),+ $(,)?) => {
        #[derive(Debug, Eq, PartialEq, Clone, Format)]
        pub enum Indication {
            Ok,
            Err,

            $($type($type),)+

            Unsupported(String, String),
        }

        impl TryFrom<&[u8]> for Indication {
            type Error = Error;

            fn try_from(value: &[u8]) -> Result<Self, Error> {
                let (indication, params) = match value.iter().position(|c| c == &b'=') {
                    Some(index) => (&value[..index], &value[index + 1..]),
                    None => (value, &[] as &[u8]),
                };

                Ok(match indication {
                    b"OK" => Self::Ok,
                    b"ERROR" => Self::Err,

                    $($bytes => Self::$type($type::try_from(params)?),)+

                    _ => Self::Unsupported(
                        str::from_utf8(indication)?.to_string(),
                        str::from_utf8(params)?.to_string(),
                    ),
                })
            }
        }

        impl TryFrom<Vec<u8>> for Indication {
            type Error = Error;

            fn try_from(value: Vec<u8>) -> Result<Self, Error> {
                value.as_slice().try_into()
            }
        }
    };
}

indications!(
    b"+VER" => Ver,
    b"+ADDR" => Addr,
    b"+LEADDR" => LeAddr,
    b"+A2DPSTAT" => A2dpStat,
    b"+A2DPDEV" => A2dpDev,
    b"+AVRCPSTAT" => AvrcpStat,
    b"+PLAYSTAT" => PlayStat,
    b"+TRACKSTAT" => TrackStat,
    b"+TRACKINFO" => TrackInfo,
    b"+SPPSTAT" => SppStat,
    b"+GATTSTAT" => GattStat,
    b"+SPPDEV" => SppDev,
    b"+GATTDEV" => GattDev,
    b"+SPPDATA" => SppData,
    b"+GATTDATA" => GattData,
);
