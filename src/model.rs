/// A language used in a repo, as GitHub reports it.
#[derive(Debug, Clone, PartialEq)]
pub struct Language {
    pub name: String,
    /// GitHub's official color, like `#3572A5`. Some languages have none.
    pub color: Option<String>,
    pub bytes: u64,
}

/// One book in the scene, standing for one language.
#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub name: String,
    pub color: Option<String>,
    /// Share of the repo's code, 0–100.
    pub percent: f64,
}

/// A public repo and the languages in it.
#[derive(Debug, Clone, PartialEq)]
pub struct Repo {
    pub name: String,
    pub is_fork: bool,
    /// ISO 8601 timestamp, so it sorts correctly as a string.
    pub pushed_at: String,
    pub languages: Vec<Language>,
}
