use crate::core::models::TmdbId;

use url::Url;

#[derive(Debug)]
pub struct PlexCredentials {
    pub plex_server_url: Url,
    pub plex_token: String,
}

#[derive(Debug)]
pub struct LetterboxdUsername(pub String);

#[derive(Debug)]
pub enum Command {
    Check(PlexCredentials, TmdbId),
    Scan(PlexCredentials, LetterboxdUsername, Option<Mode>),
}

#[derive(Debug)]
pub enum Mode {
    Present,
    Missing,
}
