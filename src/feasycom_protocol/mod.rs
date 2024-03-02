use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use anyhow::{Error, Result};
use core::str;
use defmt::Format;

mod indication_macros {
    macro_rules! define_string {
        ($name: ident) => {
            use alloc::string::{String, ToString};
            use core::str::{self, Utf8Error};

            pub type $name = String;

            pub fn parse(value: &[u8]) -> Result<$name, Utf8Error> {
                Ok(str::from_utf8(value)?.to_string())
            }
        };
    }

    macro_rules! define_enum {
        ($name: ident, $($byte: expr => $variant: ident,)*) => {
            use core::fmt::Display;
            use defmt::Format;

            #[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
            pub struct Error;

            impl Display for Error {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "+{} unknown enum value error", stringify!($name).to_ascii_uppercase())
                }
            }

            #[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
            pub enum $name {
                $($variant,)*
            }

            impl TryFrom<&[u8]> for $name {
                type Error = Error;

                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    match value {
                        $($byte => Ok($name::$variant),)*
                        _ => Err(Error),
                    }
                }
            }

            pub fn parse(value: &[u8]) -> Result<$name, Error> {
                $name::try_from(value)
            }
        };
    }

    pub(crate) use define_enum;
    pub(crate) use define_string;
}

#[cfg(feature = "a2dpdev")]
mod a2dpdev;
#[cfg(feature = "a2dpstat")]
mod a2dpstat;
#[cfg(feature = "avrcpstat")]
mod avrcpstat;
#[cfg(feature = "gattdata")]
mod gattdata;
#[cfg(feature = "gattdev")]
mod gattdev;
#[cfg(feature = "gattstat")]
mod gattstat;
#[cfg(feature = "playstat")]
mod playstat;
#[cfg(feature = "sppdata")]
mod sppdata;
#[cfg(feature = "sppdev")]
mod sppdev;
#[cfg(feature = "sppstat")]
mod sppstat;
#[cfg(feature = "trackinfo")]
mod trackinfo;
#[cfg(feature = "trackstat")]
mod trackstat;

#[derive(Debug, Eq, PartialEq, Clone, Format)]
pub enum Indication {
    Ok,
    Err,

    #[cfg(feature = "a2dpstat")]
    A2dpStat(a2dpstat::A2dpStat),
    #[cfg(feature = "a2dpdev")]
    A2dpDev(a2dpdev::A2dpDev),
    #[cfg(feature = "avrcpstat")]
    AvrcpStat(avrcpstat::AvrcpStat),
    #[cfg(feature = "playstat")]
    PlayStat(playstat::PlayStat),
    #[cfg(feature = "trackstat")]
    TrackStat(trackstat::TrackStat),
    #[cfg(feature = "trackinfo")]
    TrackInfo(trackinfo::TrackInfo),

    #[cfg(feature = "sppstat")]
    SppStat(sppstat::SppStat),
    #[cfg(feature = "gattstat")]
    GattStat(gattstat::GattStat),
    #[cfg(feature = "sppdev")]
    SppDev(sppdev::SppDev),
    #[cfg(feature = "gattdev")]
    GattDev(gattdev::GattDev),
    #[cfg(feature = "sppdata")]
    SppData(sppdata::SppData),
    #[cfg(feature = "gattdata")]
    GattData(gattdata::GattData),

    Unsupported(String, String),
}

impl TryFrom<&[u8]> for Indication {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let (indication, params) = match value.iter().position(|c| c == &b'=') {
            Some(index) => (&value[..index], &value[index + 1..]),
            None => (value, &[] as &[u8]),
        };

        Ok(match indication {
            b"OK" => Self::Ok,
            b"ERROR" => Self::Err,

            #[cfg(feature = "a2dpstat")]
            b"+A2DPSTAT" => Self::A2dpStat(a2dpstat::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "a2dpdev")]
            b"+A2DPDEV" => Self::A2dpDev(a2dpdev::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "avrcpstat")]
            b"+AVRCPSTAT" => Self::AvrcpStat(avrcpstat::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "playstat")]
            b"+PLAYSTAT" => Self::PlayStat(playstat::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "trackstat")]
            b"+TRACKSTAT" => Self::TrackStat(trackstat::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "trackinfo")]
            b"+TRACKINFO" => Self::TrackInfo(trackinfo::parse(params).map_err(Error::msg)?),

            #[cfg(feature = "sppstat")]
            b"+SPPSTAT" => Self::SppStat(sppstat::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "gattstat")]
            b"+GATTSTAT" => Self::GattStat(gattstat::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "sppdev")]
            b"+SPPDEV" => Self::SppDev(sppdev::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "gattdev")]
            b"+GATTDEV" => Self::GattDev(gattdev::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "sppdata")]
            b"+SPPDATA" => Self::SppData(sppdata::parse(params).map_err(Error::msg)?),
            #[cfg(feature = "gattdata")]
            b"+GATTDATA" => Self::GattData(gattdata::parse(params).map_err(Error::msg)?),

            _ => Self::Unsupported(
                str::from_utf8(indication).map_err(Error::msg)?.to_string(),
                str::from_utf8(params).map_err(Error::msg)?.to_string(),
            ),
        })
    }
}

impl TryFrom<Vec<u8>> for Indication {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self> {
        value.as_slice().try_into()
    }
}
