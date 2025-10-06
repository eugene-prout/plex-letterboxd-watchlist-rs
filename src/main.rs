use core::error::Error;
use plex_letterboxd_sync::{
    cli::{
        cli::parse_cli,
        models::{Command, LetterboxdUsername, Mode, PlexCredentials},
    },
    core::models::TmdbId,
    letterboxd::scraper::get_letterboxd_watchlist,
    plex::builder::ServerBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_cli()?;
    println!("{:?}", args);
    match args {
        Command::Check(plex_credentials, tmdb_id) => check_movie(&plex_credentials, &tmdb_id)?,
        Command::Scan(plex_credentials, letterboxd_username, mode) => {
            scan_watchlist(&plex_credentials, &letterboxd_username, mode)?
        }
    };

    Ok(())
}

fn check_movie(plex_credentials: &PlexCredentials, tmdb_id: &TmdbId) -> Result<(), Box<dyn Error>> {
    let server = ServerBuilder::new(plex_credentials.plex_server_url.clone())
        .with_auth(plex_credentials.plex_token.clone())
        .build()?;

    server.ping()?;

    match server.get_movie_by_tmdb_id(&tmdb_id)? {
        Some(movie) => println!("Movie found in Plex: {:?}", movie),
        None => println!("Movie not found in Plex."),
    }

    Ok(())
}

fn scan_watchlist(
    plex_credentials: &PlexCredentials,
    letterboxd_username: &LetterboxdUsername,
    mode: Option<Mode>,
) -> Result<(), Box<dyn Error>> {
    println!(
        "Fetching Letterboxd watchlist for user: {}",
        letterboxd_username.0
    );
    let letterboxd_watchlist = get_letterboxd_watchlist(&letterboxd_username.0)?;

    let server = ServerBuilder::new(plex_credentials.plex_server_url.clone())
        .with_auth(plex_credentials.plex_token.clone())
        .build()?;

    server.ping()?;

    for movie in letterboxd_watchlist {
        let movie_in_plex = server.get_movie_by_tmdb_id(&movie.tmdb_id)?;

        match (movie_in_plex, &mode) {
            (Some(_), Some(Mode::Present)) => {
                println!("{}", movie.title);
            }
            (None, Some(Mode::Missing)) => {
                println!("{}", movie.title);
            }
            _ => {}
        }
    }

    Ok(())
}
