use std::error::Error;

use itertools::Itertools;

use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[derive(Debug)]
struct Movie {
    title: String,
    tmdb_id: i32,
}

fn get_web_page(address: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::get(address)?.text()?;
    Ok(response)
}

struct LetterboxdWatchlistWebPage {
    next: Option<String>,
}

impl Iterator for LetterboxdWatchlistWebPage {
    type Item = Document;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Figure out why this needs to be cloned.
        let next_url = self.next.clone()?;
        let page = get_web_page(&next_url).ok()?;

        let movie_document = Document::from(page.as_str());

        self.next = movie_document
            .find(Name("a").and(Class("next")))
            .exactly_one()
            .ok()
            .and_then(|x| x.attr("href"))
            .map(|s| "https://letterboxd.com".to_owned() + s);

        Some(movie_document)
    }
}

fn parse_movie_page(movie_document: &Document) -> Result<Movie, Box<dyn Error>> {
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
        .parse::<i32>()?;

    Ok(Movie { title, tmdb_id })
}

fn parse_watchlist_page(watchlist_page_url: &Document) -> Result<Vec<Movie>, Box<dyn Error>> {
    let mut watchlist_movies: Vec<Movie> = Vec::new();

    for node in watchlist_page_url.find(Name("li").and(Class("griditem"))) {
        // TODO: Understand why lifetimes are stopping me from using exactly_one here.
        let movie_link = node
            .find(Name("div").and(Class("react-component")))
            .next()
            .and_then(|div| div.attr("data-target-link"))
            .ok_or("Failed to find movie title from link.")?;

        let film_page = "https://letterboxd.com".to_owned() + movie_link;

        let movie_contents = get_web_page(&film_page)?;

        let movie_doc = Document::from(movie_contents.as_str());
        let movie = parse_movie_page(&movie_doc)?;

        watchlist_movies.push(movie);
    }

    Ok(watchlist_movies)
}

fn main() {
    let watchlist_pages = LetterboxdWatchlistWebPage {
        next: Some("https://letterboxd.com/<LETTERBOXD_USERNAME>/watchlist/page/0/".to_string()),
    };

    let movies = watchlist_pages
        .flat_map(|page| parse_watchlist_page(&page).unwrap())
        .collect::<Vec<Movie>>();

    for movie in movies {
        println!("{movie:?}");
    }
}
