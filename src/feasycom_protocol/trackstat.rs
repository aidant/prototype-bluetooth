use core::{
    fmt::Display,
    num::ParseIntError,
    str::{self, Utf8Error},
    time::Duration,
};
use defmt::Format;

use super::playstat::{self, PlayStat};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    Utf8Error(Utf8Error),
    ParseIntError(ParseIntError),
    PlayStat(playstat::Error),
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<playstat::Error> for Error {
    fn from(value: playstat::Error) -> Self {
        Self::PlayStat(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Utf8Error(utf8_error) => utf8_error.fmt(f),
            Self::ParseIntError(parse_int_error) => parse_int_error.fmt(f),
            Self::PlayStat(play_stat_error) => play_stat_error.fmt(f),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub struct TrackStat {
    pub play_stat: PlayStat,
    pub elapsed_time: Duration,
    pub total_time: Duration,
}

impl TryFrom<&[u8]> for TrackStat {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (play_stat, value) = value.split_once(|c| c == &b',').unwrap_or((value, &[]));
        let (elapsed_time, total_time) = value.split_once(|c| c == &b',').unwrap_or((value, &[]));

        Ok(Self {
            play_stat: PlayStat::try_from(play_stat)?,
            elapsed_time: Duration::from_millis(str::from_utf8(elapsed_time)?.parse()?),
            total_time: Duration::from_millis(str::from_utf8(total_time)?.parse()?),
        })
    }
}

pub fn parse(value: &[u8]) -> Result<TrackStat, Error> {
    TrackStat::try_from(value)
}
