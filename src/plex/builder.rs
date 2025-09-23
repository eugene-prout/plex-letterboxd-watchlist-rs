use std::error::Error;

use reqwest::{
    blocking,
    header::{self, HeaderMap, HeaderValue},
};
use secrecy::{ExposeSecret, SecretString};
use url::Url;

use super::server::Server;

#[derive(Debug)]
pub struct ServerBuilder {
    url: Url,
    token: Option<SecretString>,
}

impl ServerBuilder {
    pub fn new(url: Url) -> Self {
        Self { url, token: None }
    }

    pub fn with_auth(self, token: impl Into<SecretString>) -> Self {
        Self {
            url: self.url,
            token: Some(token.into()),
        }
    }

    pub fn build(self) -> Result<Server, Box<dyn Error>> {
        let Some(token) = self.token else {
            return Err("no authentication method provided".into());
        };

        let mut headers = HeaderMap::with_capacity(2);

        let mut auth_value = HeaderValue::from_str(token.expose_secret())?;
        auth_value.set_sensitive(true);
        headers.insert("X-Plex-Token", auth_value);
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let client = blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Server::new(self.url, client))
    }
}
