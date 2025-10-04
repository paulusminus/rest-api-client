use futures_util::future::BoxFuture;
use futures_util::{FutureExt, TryFutureExt};
use lipl_core::{
    LiplRepo, Lyric, LyricPost, Playlist, PlaylistPost, Result, Summary, Uuid, error::reqwest_error,
};
pub use rest_json_client::{ApiClient, ApiClientBuilder};
pub use rest_json_client::{Authentication, BasicAuthentication};

const LYRIC: &str = "lyric";
const PLAYLIST: &str = "playlist";
const FULL: &str = "full=true";

pub struct LiplApiClient {
    api_client: ApiClient,
}

impl From<ApiClient> for LiplApiClient {
    fn from(value: ApiClient) -> Self {
        Self { api_client: value }
    }
}

impl LiplApiClient {
    pub fn try_new(prefix: &str, auth: Authentication) -> Result<Self> {
        ApiClientBuilder::new(prefix)
            .authentication(auth)
            .build()
            .map_err(reqwest_error)
            .map(LiplApiClient::from)
    }
}

impl LiplRepo for LiplApiClient {
    fn get_lyrics(&self) -> BoxFuture<'_, Result<Vec<Lyric>>> {
        async move {
            self.api_client
                .get(&format!("{LYRIC}?{FULL}"))
                .map_err(reqwest_error)
                .await
        }
        .boxed()
    }

    fn get_lyric_summaries(&self) -> BoxFuture<'_, Result<Vec<Summary>>> {
        async move { self.api_client.get(LYRIC).map_err(reqwest_error).await }.boxed()
    }

    fn get_lyric(&self, uuid: Uuid) -> BoxFuture<'_, Result<Lyric>> {
        async move {
            self.api_client
                .get(&format!("{LYRIC}/{uuid}"))
                .map_err(reqwest_error)
                .await
        }
        .boxed()
    }

    fn upsert_lyric(&self, lyric: Lyric) -> BoxFuture<'_, Result<Lyric>> {
        async move {
            self.api_client
                .post(&format!("{LYRIC}/{}", lyric.id), LyricPost::from(lyric))
                .map_err(reqwest_error)
                .await
        }
        .boxed()
    }

    fn delete_lyric(&self, uuid: Uuid) -> BoxFuture<'_, Result<()>> {
        async move {
            self.api_client
                .delete(&format!("{LYRIC}/{uuid}"))
                .map_err(reqwest_error)
                .await
        }
        .boxed()
    }

    fn get_playlists(&self) -> BoxFuture<'_, Result<Vec<Playlist>>> {
        async move {
            self.api_client
                .get(&format!("{PLAYLIST}?{FULL}"))
                .map_err(reqwest_error)
                .await
        }
        .boxed()
    }

    fn get_playlist_summaries(&self) -> BoxFuture<'_, Result<Vec<Summary>>> {
        async move { self.api_client.get(PLAYLIST).map_err(reqwest_error).await }.boxed()
    }

    fn get_playlist(&self, uuid: Uuid) -> BoxFuture<'_, Result<Playlist>> {
        async move {
            self.api_client
                .get(&format!("{PLAYLIST}/{uuid}"))
                .map_err(reqwest_error)
                .await
        }
        .boxed()
    }

    fn upsert_playlist(&self, playlist: Playlist) -> BoxFuture<'_, Result<Playlist>> {
        async move {
            self.api_client
                .post(
                    &format!("{PLAYLIST}/{}", playlist.id),
                    PlaylistPost::from(playlist),
                )
                .map_err(reqwest_error)
                .await
        }
        .boxed()
    }

    fn delete_playlist(&self, uuid: Uuid) -> BoxFuture<'_, Result<()>> {
        async move {
            self.api_client
                .delete(&format!("{PLAYLIST}/{uuid}"))
                .map_err(reqwest_error)
                .await
        }
        .boxed()
    }

    fn stop(&self) -> BoxFuture<'_, Result<()>> {
        futures_util::future::ready(Ok(())).boxed()
    }
}
