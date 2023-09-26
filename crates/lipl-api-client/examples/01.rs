use lipl_api_client::{ApiClientBuilder, Authentication, LiplApiClient};
use lipl_core::{error::reqwest_error, HasSummary, LiplRepo, Result};

const PREFIX: &str = "https://lipl.paulmin.nl/api/v1/";
const USERNAME: &str = "paul";
const PASSWORD: &str = "CumGranoSalis";

trait VecExt {
    fn display_titles(self, name: &str, seperator: &str) -> String;
}

impl<I> VecExt for I
where
    I: IntoIterator,
    I::Item: HasSummary,
{
    fn display_titles(self, name: &str, separator: &str) -> String {
        let s = self
            .into_iter()
            .map(|t| t.summary().title)
            .collect::<Vec<_>>()
            .join(separator);
        format!("{name}:{separator}{s}")
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let auth = Authentication::new_basic(USERNAME, PASSWORD);
    let client = ApiClientBuilder::new(PREFIX)
        .authentication(auth)
        .build()
        .map_err(reqwest_error)
        .map(LiplApiClient::from)?;

    let lyric_titles = client
        .get_lyric_summaries()
        .await
        .map(|s| s.display_titles("Lyrics", "\n- "))?;
    println!("{lyric_titles}");

    println!();

    let playlist_titles = client
        .get_playlist_summaries()
        .await
        .map(|s| s.display_titles("Playlists", "\n- "))?;
    println!("{playlist_titles}");

    Ok(())
}
