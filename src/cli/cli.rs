use url::Url;

use crate::{
    cli::models::{Command, LetterboxdUsername, PlexCredentials},
    core::models::TmdbId,
};

use std::{env, error::Error};

pub fn parse_cli() -> Result<Command, Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    let (_exec_name, subcommand, rest) = match &args[..] {
        [exec_name, subcommand, rest @ ..] => (exec_name, subcommand, rest),
        _ => panic!("Usage: {} <check/scan> <args...>", args[0]),
    };

    let command_params = match subcommand.as_str() {
        "check" => {
            if let [uri, token, tmdb_id] = &rest[..] {
                let plex_credentials = parse_plex_credentials(uri, token)?;
                let tmdb_id = Into::<TmdbId>::into(tmdb_id.to_owned());

                Command::Check(plex_credentials, tmdb_id)
            } else {
                panic!(
                    "Usage: {} check <PLEX_SERVER_URL> <PLEX_TOKEN> <TMDB_ID>",
                    args[0]
                )
            }
        }
        "scan" => match &rest[..] {
            [uri, token, username] => {
                let plex_credentials = parse_plex_credentials(uri, token)?;
                let letterboxd_username = LetterboxdUsername(username.to_owned());

                Command::Scan(plex_credentials, letterboxd_username, None)
            }
            [uri, token, username, mode] => {
                let plex_credentials = parse_plex_credentials(uri, token)?;
                let letterboxd_username = LetterboxdUsername(username.to_owned());
                let mode = match mode.as_str() {
                    "--present" => Some(crate::cli::models::Mode::Present),
                    "--missing" => Some(crate::cli::models::Mode::Missing),
                    _ => panic!(
                        "Usage: {} scan <PLEX_SERVER_URL> <PLEX_TOKEN> <LETTERBOXD_USERNAME> [--present/--missing]",
                        args[0]
                    ),
                };

                Command::Scan(plex_credentials, letterboxd_username, mode)
            }
            _ => panic!(
                "Usage: {} scan <PLEX_SERVER_URL> <PLEX_TOKEN> <LETTERBOXD_USERNAME> [--present/--missing]",
                args[0]
            ),
        },
        _ => panic!("Unknown subcommand: {}", subcommand),
    };

    Ok(command_params)
}

fn parse_plex_credentials(uri: &str, token: &str) -> Result<PlexCredentials, Box<dyn Error>> {
    Ok(PlexCredentials {
        plex_server_url: uri.parse::<Url>()?,
        plex_token: token.to_owned(),
    })
}
