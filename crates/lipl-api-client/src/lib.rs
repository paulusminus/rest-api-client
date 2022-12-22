use anyhow::anyhow;
use async_trait::async_trait;
use lipl_core::{LiplRepo, Lyric, LyricPost, Playlist, PlaylistPost, Summary, Uuid};
use json_api_client::{ApiClient, Error};
pub use json_api_client::{Authentication, BasicAuthentication};
use anyhow::Result as Result;

const LYRIC: &str = "lyric";
const PLAYLIST: &str = "playlist";
const FULL: &str = "full=true";

#[derive(Clone)]
pub struct LiplApiClient {
    api_client: ApiClient,
}

impl LiplApiClient {
    pub fn new(prefix: &str, auth: Authentication) -> Self {
        Self {
            api_client: ApiClient::new(prefix, auth)
        }
    }
}

fn to_anyhow(error: Error) -> anyhow::Error {
    anyhow!(error)
}

#[async_trait]
impl LiplRepo for LiplApiClient {
    async fn get_lyrics(&self) -> Result<Vec<Lyric>> {
        self.api_client.get(&format!("{LYRIC}?{FULL}"))
            .await
            .map_err(to_anyhow)
    }

    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>> {
        self.api_client.get(LYRIC)
            .await
            .map_err(to_anyhow)
    }

    async fn get_lyric(&self, uuid: Uuid) -> Result<Lyric> {
        self.api_client.get(&format!("{LYRIC}/{uuid}"))
            .await
            .map_err(to_anyhow)
    }

    async fn post_lyric(&self, lyric: Lyric) -> Result<Lyric> {
        self.api_client.post(&format!("{LYRIC}/{}", lyric.id), LyricPost::from(lyric))
            .await
            .map_err(to_anyhow)
    }

    async fn delete_lyric(&self, uuid: Uuid) -> Result<()> {
        self.api_client.delete(&format!("{LYRIC}/{uuid}"))
            .await
            .map_err(to_anyhow)
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        self.api_client.get(&format!("{PLAYLIST}?{FULL}"))
            .await
            .map_err(to_anyhow)
    }

    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        self.api_client.get(PLAYLIST)
            .await
            .map_err(to_anyhow)
    }

    async fn get_playlist(&self, uuid: Uuid) -> Result<Playlist> {
        self.api_client.get(&format!("{PLAYLIST}/{uuid}"))
            .await
            .map_err(to_anyhow)
    }

    async fn post_playlist(&self, playlist: Playlist) -> Result<Playlist> {
        self.api_client.post(&format!("{PLAYLIST}/{}", playlist.id), PlaylistPost::from(playlist))
            .await
            .map_err(to_anyhow)
    }

    async fn delete_playlist(&self, uuid: Uuid) -> Result<()> {
        self.api_client.delete(&format!("{PLAYLIST}/{uuid}"))
            .await
            .map_err(to_anyhow)
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}