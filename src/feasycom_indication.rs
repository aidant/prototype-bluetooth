use alloc::vec::Vec;
use core::time::Duration;
use defmt::Format;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub enum PLAYSTAT {
    Stopped,
    Playing,
    Paused,
    FastForwarding,
    FastRewinding,
}

impl TryFrom<&str> for PLAYSTAT {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(PLAYSTAT::Stopped),
            "1" => Ok(PLAYSTAT::Playing),
            "2" => Ok(PLAYSTAT::Paused),
            "3" => Ok(PLAYSTAT::FastForwarding),
            "4" => Ok(PLAYSTAT::FastRewinding),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub struct TRACKINFO<'a> {
    pub title: &'a str,
    pub artist: &'a str,
    pub album: &'a str,
}

impl<'a> TryFrom<&'a str> for TRACKINFO<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let params = value.split(",").collect::<Vec<&str>>();

        if params.len() == 3 {
            Ok(Self {
                title: params[0],
                artist: params[1],
                album: params[2],
            })
        } else {
            Ok(Self {
                title: params[0],
                artist: params[1],
                album: params[2],
            })
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub struct TRACKSTAT {
    pub PLAYSTAT: PLAYSTAT,
    pub elapsed_time: Duration,
    pub total_time: Duration,
}

impl TryFrom<&str> for TRACKSTAT {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let params = value.split(",").collect::<Vec<&str>>();

        if params.len() == 3 {
            Ok(Self {
                PLAYSTAT: PLAYSTAT::try_from(params[0]).unwrap(),
                elapsed_time: Duration::from_millis(params[1].parse().unwrap()),
                total_time: Duration::from_millis(params[2].parse().unwrap()),
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub enum Indication<'a> {
    OK,
    ERR,

    PLAYSTAT(PLAYSTAT),
    TRACKINFO(TRACKINFO<'a>),
    TRACKSTAT(TRACKSTAT),

    Unknown(&'a str, &'a str),
}

impl<'a> TryFrom<&'a str> for Indication<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value == "OK" {
            return Ok(Indication::OK);
        }

        if value == "ERROR" {
            return Ok(Indication::ERR);
        }

        if !value.starts_with("+") {
            return Err(());
        }

        let pos = value.find("=").unwrap();

        let indication_type = &value[1..pos];
        let indication_params = &value[(pos + 1)..];

        match indication_type {
            "PLAYSTAT" => Ok(Indication::PLAYSTAT(
                PLAYSTAT::try_from(indication_params).unwrap(),
            )),
            "TRACKINFO" => Ok(Indication::TRACKINFO(
                TRACKINFO::try_from(indication_params).unwrap(),
            )),
            "TRACKSTAT" => Ok(Indication::TRACKSTAT(
                TRACKSTAT::try_from(indication_params).unwrap(),
            )),
            _ => Ok(Indication::Unknown(indication_type, indication_params)),
        }
    }
}
