use alloc::string::String;
use core::time::Duration;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;

static APP_STATE_CHANNEL: Channel<ThreadModeRawMutex, AppState, 1> = Channel::new();

pub async fn app_state_set(app_state: AppState) {
    APP_STATE_CHANNEL.send(app_state).await;
}

pub async fn app_state_get() -> AppState {
    APP_STATE_CHANNEL.receive().await
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AppState {
    pub song_title: Option<String>,
    pub song_artist: Option<String>,
    pub song_album: Option<String>,

    pub playback_elapsed_time: Option<Duration>,
    pub playback_total_time: Option<Duration>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            song_title: Default::default(),
            song_artist: Default::default(),
            song_album: Default::default(),
            playback_elapsed_time: Default::default(),
            playback_total_time: Default::default(),
        }
    }
}
