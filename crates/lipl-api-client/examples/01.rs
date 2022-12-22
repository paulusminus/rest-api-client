use std::{fmt::Display};

use lipl_api_client::LiplApiClient;
use json_api_client::{BasicAuthentication, Authentication};
use lipl_core::{HasSummary, LiplRepo, reexport::anyhow::Result};

const PREFIX: &str = "https://lipl.paulmin.nl/api/v1/";
const USERNAME: &str = "paul";
const PASSWORD: &str = "CumGranoSalis";

trait VecExt {
    fn display_titles(&self, name: &str, seperator: &str);
}

impl<T> VecExt for Result<Vec<T>> where T: Display + HasSummary {
    fn display_titles(&self, name: &str, separator: &str) {
        if self.is_ok() {
            let s = self.as_ref()
                .unwrap()
                .into_iter()
                .map(|t| t.summary().title)
                .collect::<Vec<_>>()
                .join(separator);
            println!("{name}: {s}");
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let auth = Authentication::Basic(
        BasicAuthentication::new(USERNAME, PASSWORD),
    );
    let client = LiplApiClient::new(PREFIX, auth);

    client.get_lyric_summaries().await.display_titles("Lyrics", ", ");
    client.get_playlist_summaries().await.display_titles("Playlists", ", ");

    Ok(())
}
