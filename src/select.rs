use crate::model::{Book, Language, Repo};

pub const MAX_BOOKS: usize = 3;

/// Picks the repo the owl reads: the most recently pushed one that isn't a
/// fork, the profile repo, or excluded, and that has some code in it.
pub fn pick_repo<'a>(repos: &'a [Repo], owner: &str, exclude: &[String]) -> Option<&'a Repo> {
    repos
        .iter()
        .filter(|r| !r.is_fork)
        .filter(|r| !r.name.eq_ignore_ascii_case(owner))
        .filter(|r| !exclude.iter().any(|e| e.eq_ignore_ascii_case(&r.name)))
        .filter(|r| r.languages.iter().any(|l| l.bytes > 0))
        .max_by(|a, b| a.pushed_at.cmp(&b.pushed_at))
}

/// Turns a repo's languages into the books the owl reads.
pub fn select_books(languages: &[Language], min_percent: f64) -> Vec<Book> {
    let total: u64 = languages.iter().map(|l| l.bytes).sum();
    if total == 0 {
        return Vec::new();
    }
    let mut sorted: Vec<&Language> = languages.iter().collect();
    sorted.sort_by_key(|l| std::cmp::Reverse(l.bytes));
    sorted
        .into_iter()
        .take(MAX_BOOKS)
        .map(|l| Book {
            name: l.name.clone(),
            color: l.color.clone(),
            percent: l.bytes as f64 * 100.0 / total as f64,
        })
        .enumerate()
        // The top language always stays so the owl never has nothing to read.
        .filter(|(i, b)| *i == 0 || b.percent >= min_percent)
        .map(|(_, b)| b)
        .collect()
}
