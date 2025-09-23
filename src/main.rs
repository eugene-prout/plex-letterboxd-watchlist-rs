use core::error::Error;
use std::env;

use plex_letterboxd_sync::plex::{builder::ServerBuilder, server::TmdbId};
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (plex_server_url, plex_token, tmdb_id) = match &args[..] {
        [_script_name, url, token, tmdb_id] => (url, token, tmdb_id),
        _ => panic!(
            "Usage: {} <PLEX_SERVER_URL> <PLEX_TOKEN> <TMDB_ID>",
            args[0]
        ),
    };

    let uri = plex_server_url.parse::<Url>()?;

    let server = ServerBuilder::new(uri)
        .with_auth(plex_token.to_owned())
        .build()?;

    let tmdb_id = Into::<TmdbId>::into(tmdb_id.to_owned());
    println!("{:?}", server.get_movie_by_tmdb_id(tmdb_id.clone())?);

    Ok(())
}
