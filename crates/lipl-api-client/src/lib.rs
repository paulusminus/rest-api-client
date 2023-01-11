use async_trait::async_trait;
use futures_util::TryFutureExt;
use lipl_core::{LiplRepo, Lyric, LyricPost, Playlist, PlaylistPost, Result, Summary, Uuid};
use json_api_client::{ApiClient};
pub use json_api_client::{Authentication, BasicAuthentication};

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

#[async_trait]
impl LiplRepo for LiplApiClient {
    async fn get_lyrics(&self) -> Result<Vec<Lyric>> {
        self.api_client.get(&format!("{LYRIC}?{FULL}"))
            .err_into()
            .await
    }

    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>> {
        self.api_client.get(LYRIC)
            .err_into()
            .await
    }

    async fn get_lyric(&self, uuid: Uuid) -> Result<Lyric> {
        self.api_client.get(&format!("{LYRIC}/{uuid}"))
            .err_into()
            .await
    }

    async fn post_lyric(&self, lyric: Lyric) -> Result<Lyric> {
        self.api_client.post(&format!("{LYRIC}/{}", lyric.id), LyricPost::from(lyric))
            .err_into()
            .await
    }

    async fn delete_lyric(&self, uuid: Uuid) -> Result<()> {
        self.api_client.delete(&format!("{LYRIC}/{uuid}"))
            .err_into()
            .await
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        self.api_client.get(&format!("{PLAYLIST}?{FULL}"))
            .err_into()
            .await
    }

    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        self.api_client.get(PLAYLIST)
            .err_into()
            .await
    }

    async fn get_playlist(&self, uuid: Uuid) -> Result<Playlist> {
        self.api_client.get(&format!("{PLAYLIST}/{uuid}"))
            .err_into()
            .await
    }

    async fn post_playlist(&self, playlist: Playlist) -> Result<Playlist> {
        self.api_client.post(&format!("{PLAYLIST}/{}", playlist.id), PlaylistPost::from(playlist))
            .err_into()
            .await
    }

    async fn delete_playlist(&self, uuid: Uuid) -> Result<()> {
        self.api_client.delete(&format!("{PLAYLIST}/{uuid}"))
            .err_into()
            .await
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}