use repo_owl::{Language, Repo, pick_repo};

fn lang(name: &str, bytes: u64) -> Language {
    Language { name: name.into(), color: None, bytes }
}

fn repo(name: &str, pushed_at: &str) -> Repo {
    Repo {
        name: name.into(),
        is_fork: false,
        pushed_at: pushed_at.into(),
        languages: vec![lang("Python", 100)],
    }
}

#[test]
fn picks_the_most_recently_pushed_repo() {
    let repos = vec![
        repo("old", "2025-12-28T14:51:04Z"),
        repo("newest", "2026-10-18T21:14:04Z"),
        repo("middle", "2026-09-15T14:04:49Z"),
    ];

    let picked = pick_repo(&repos, "someone", &[]).unwrap();

    assert_eq!(picked.name, "newest");
}

#[test]
fn skips_forks() {
    let mut fork = repo("someone-elses", "2026-10-30T00:00:00Z");
    fork.is_fork = true;
    let repos = vec![fork, repo("mine", "2026-10-01T00:00:00Z")];

    let picked = pick_repo(&repos, "someone", &[]).unwrap();

    assert_eq!(picked.name, "mine");
}

#[test]
fn skips_the_profile_repo_regardless_of_case() {
    let repos = vec![
        repo("nerosight", "2026-11-01T00:00:00Z"),
        repo("XPLAIN", "2026-10-18T21:14:04Z"),
    ];

    let picked = pick_repo(&repos, "Nerosight", &[]).unwrap();

    assert_eq!(picked.name, "XPLAIN");
}

#[test]
fn skips_excluded_repos() {
    let repos = vec![
        repo("dotfiles", "2026-11-01T00:00:00Z"),
        repo("XPLAIN", "2026-10-18T21:14:04Z"),
    ];

    let picked = pick_repo(&repos, "someone", &["dotfiles".to_string()]).unwrap();

    assert_eq!(picked.name, "XPLAIN");
}

#[test]
fn falls_back_past_repos_without_languages() {
    let mut empty = repo("glidepath", "2026-10-20T00:00:00Z");
    empty.languages.clear();
    let repos = vec![empty, repo("XPLAIN", "2026-10-18T21:14:04Z")];

    let picked = pick_repo(&repos, "someone", &[]).unwrap();

    assert_eq!(picked.name, "XPLAIN");
}

#[test]
fn returns_none_when_nothing_qualifies() {
    let mut empty = repo("glidepath", "2026-10-20T00:00:00Z");
    empty.languages.clear();

    assert_eq!(pick_repo(&[empty], "someone", &[]), None);
}
