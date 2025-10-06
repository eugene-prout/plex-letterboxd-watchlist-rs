use std::error::Error;

use itertools::Itertools;

use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

use crate::letterboxd::models::LetterboxdMovie;

pub fn get_letterboxd_watchlist(
    letterboxd_username: &str,
) -> Result<impl Iterator<Item = LetterboxdMovie>, Box<dyn Error>> {
    let first_page = format!(
        "https://letterboxd.com/{}/watchlist/page/0/",
        letterboxd_username
    );

    let watchlist_iterator = LetterboxdWatchlistWebPage {
        next: Some(first_page),
    };

    let all_movies = watchlist_iterator
        .collect::<Result<Vec<Document>, Box<dyn Error>>>()?
        .into_iter()
        .map(|page| parse_watchlist_page(&page))
        .collect::<Result<Vec<Vec<LetterboxdMovie>>, Box<dyn Error>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    Ok(all_movies.into_iter())
}

struct LetterboxdWatchlistWebPage {
    next: Option<String>,
}

impl Iterator for LetterboxdWatchlistWebPage {
    type Item = Result<Document, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Figure out why this needs to be cloned.
        let next_url = self.next.clone()?;

        let page = reqwest::blocking::get(&next_url)
            .and_then(|resp| resp.text())
            .map(|text| Document::from(text.as_str()));

        let movie_document = match page {
            Ok(doc) => doc,
            Err(e) => return Some(Err(Box::new(e) as Box<dyn Error>)),
        };

        self.next = movie_document
            .find(Name("a").and(Class("next")))
            .exactly_one()
            .ok()
            .and_then(|x| x.attr("href"))
            .map(|s| "https://letterboxd.com".to_owned() + s);

        Some(Ok(movie_document))
    }
}

fn parse_movie_page(movie_document: &Document) -> Result<LetterboxdMovie, Box<dyn Error>> {
    let title = movie_document
        .find(
            Name("h1")
                .and(Class("primaryname"))
                .descendant(Name("span").and(Class("name"))),
        )
        .exactly_one()
        .map_err(|e| e.to_string())?
        .text();

    let tmdb_link = movie_document
        .find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .filter(|href| href.starts_with("https://www.themoviedb.org/movie/"))
        .exactly_one()
        .map_err(|e| e.to_string())?;

    let tmdb_regex = Regex::new(r"https:\/\/www\.themoviedb\.org\/movie\/([0-9]+)\/").unwrap();

    let tmdb_id = tmdb_regex
        .captures(tmdb_link)
        .ok_or("Failed to parse TMDB link")?
        .get(1)
        .ok_or("Failed to parse TMDB link")?
        .as_str()
        .to_owned();

    Ok(LetterboxdMovie {
        title,
        tmdb_id: tmdb_id.into(),
    })
}

fn parse_watchlist_page(
    watchlist_page_url: &Document,
) -> Result<Vec<LetterboxdMovie>, Box<dyn Error>> {
    let mut watchlist_movies: Vec<LetterboxdMovie> = Vec::new();

    for node in watchlist_page_url.find(Name("li").and(Class("griditem"))) {
        // TODO: Understand why lifetimes are stopping me from using exactly_one here.
        let movie_link = node
            .find(Name("div").and(Class("react-component")))
            .next()
            .and_then(|div| div.attr("data-target-link"))
            .ok_or("Failed to find movie title from link.")?;

        let film_page = format!("https://letterboxd.com{}", movie_link);

        dbg!(&film_page);

        let movie_contents = reqwest::blocking::get(&film_page)?.text()?;

        let movie_doc = Document::from(movie_contents.as_str());
        let movie = parse_movie_page(&movie_doc)?;

        watchlist_movies.push(movie);
    }

    Ok(watchlist_movies)
}
