use alloc::string::{String, ToString};
use core::str::{self, Utf8Error};
use defmt::Format;

#[derive(Debug, Eq, PartialEq, Clone, Format)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
}

impl TryFrom<&[u8]> for TrackInfo {
    type Error = Utf8Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (title, value) = value.split_once(|c| c == &0xFF).unwrap_or((value, &[]));
        let (artist, album) = value.split_once(|c| c == &0xFF).unwrap_or((value, &[]));

        Ok(Self {
            title: str::from_utf8(title)?.to_string(),
            artist: str::from_utf8(artist)?.to_string(),
            album: str::from_utf8(album)?.to_string(),
        })
    }
}

pub fn parse(value: &[u8]) -> Result<TrackInfo, Utf8Error> {
    TrackInfo::try_from(value)
}
