use std::error::Error;

use itertools::Itertools;
use reqwest::blocking::Client;
use url::Url;

use crate::plex::models::{MediaRoot, MovieMetadata, SearchRoot};

#[derive(Debug)]
pub struct Server {
    pub url: Url,
    client: Client, // TODO: Make this a more flexible way of calling a server.
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TmdbId(String);

impl From<String> for TmdbId {
    fn from(id: String) -> Self {
        TmdbId(id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlexId(String);

impl From<String> for PlexId {
    fn from(id: String) -> Self {
        PlexId(id)
    }
}

impl Server {
    pub(crate) fn new(url: Url, client: Client) -> Self {
        Self { url, client }
    }

    pub fn ping(&self) -> Result<(), Box<dyn Error>> {
        let endpoint = self.url.join("/identity")?;

        self.client.get(endpoint).send()?.error_for_status()?;

        Ok(())
    }

    pub fn get_movie_by_tmdb_id(
        &self,
        tmdb_id: TmdbId,
    ) -> Result<Option<MovieMetadata>, Box<dyn Error>> {
        let movie_id = self
            .get_movie_id(&tmdb_id)
            .map_err(|e| format!("Failed to load movie {} from TMDB: {:?}", tmdb_id.0, e))?;

        let endpoint = self.url.join("/library/sections/1/all")?;

        let resp = self
            .client
            .get(endpoint)
            .query(&[("includeGuids", "1"), ("guid", &movie_id.0)])
            .send()?;

        let parsed = resp.json::<MediaRoot>()?;

        match &parsed.media_container.metadata[..] {
            [] => Ok(None),
            [movie] => Ok(Some(movie.clone())),
            _ => Err("Too many results for Plex guid".into()),
        }
    }

    fn get_movie_id(&self, tmdb_id: &TmdbId) -> Result<PlexId, Box<dyn Error>> {
        let endpoint = self.url.join("/library/metadata/1/matches")?;

        let resp = self
            .client
            .get(endpoint)
            .query(&[("manual", "1"), ("title", &format!("tmdb-{}", tmdb_id.0))])
            .send()?;

        let parsed = resp.json::<SearchRoot>()?;

        Ok(parsed
            .media_container
            .search_result
            .into_iter()
            .exactly_one()?
            .guid
            .into())
    }
}
