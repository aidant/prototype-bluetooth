use core::{
    str::{self},
    time::Duration,
};
use defmt::Format;

use super::{Error, PlayStat};

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
