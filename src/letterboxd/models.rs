use crate::core::models::TmdbId;

#[derive(Debug)]
pub struct LetterboxdMovie {
    pub title: String,
    pub tmdb_id: TmdbId,
}
