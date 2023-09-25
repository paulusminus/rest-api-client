use async_trait::async_trait;
use futures_util::TryFutureExt;
use json_api_client::ApiClient;
pub use json_api_client::{Authentication, BasicAuthentication};
use lipl_core::{
    error::reqwest_error, LiplRepo, Lyric, LyricPost, Playlist, PlaylistPost, Result, Summary, Uuid,
};

const LYRIC: &str = "lyric";
const PLAYLIST: &str = "playlist";
const FULL: &str = "full=true";

pub struct LiplApiClient {
    api_client: ApiClient,
}

impl LiplApiClient {
    pub fn try_new(prefix: &str, auth: Authentication) -> Result<Self> {
        ApiClient::try_new(prefix, auth, None)
        .map_err(reqwest_error)
        .map(|api_client| Self { api_client })
    }
}

#[async_trait]
impl LiplRepo for LiplApiClient {
    async fn get_lyrics(&self) -> Result<Vec<Lyric>> {
        self.api_client
            .get(&format!("{LYRIC}?{FULL}"))
            .map_err(reqwest_error)
            .await
    }

    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>> {
        self.api_client.get(LYRIC).map_err(reqwest_error).await
    }

    async fn get_lyric(&self, uuid: Uuid) -> Result<Lyric> {
        self.api_client
            .get(&format!("{LYRIC}/{uuid}"))
            .map_err(reqwest_error)
            .await
    }

    async fn upsert_lyric(&self, lyric: Lyric) -> Result<Lyric> {
        self.api_client
            .post(&format!("{LYRIC}/{}", lyric.id), LyricPost::from(lyric))
            .map_err(reqwest_error)
            .await
    }

    async fn delete_lyric(&self, uuid: Uuid) -> Result<()> {
        self.api_client
            .delete(&format!("{LYRIC}/{uuid}"))
            .map_err(reqwest_error)
            .await
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        self.api_client
            .get(&format!("{PLAYLIST}?{FULL}"))
            .map_err(reqwest_error)
            .await
    }

    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        self.api_client.get(PLAYLIST).map_err(reqwest_error).await
    }

    async fn get_playlist(&self, uuid: Uuid) -> Result<Playlist> {
        self.api_client
            .get(&format!("{PLAYLIST}/{uuid}"))
            .map_err(reqwest_error)
            .await
    }

    async fn upsert_playlist(&self, playlist: Playlist) -> Result<Playlist> {
        self.api_client
            .post(
                &format!("{PLAYLIST}/{}", playlist.id),
                PlaylistPost::from(playlist),
            )
            .map_err(reqwest_error)
            .await
    }

    async fn delete_playlist(&self, uuid: Uuid) -> Result<()> {
        self.api_client
            .delete(&format!("{PLAYLIST}/{uuid}"))
            .map_err(reqwest_error)
            .await
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}
