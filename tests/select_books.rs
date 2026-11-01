use repo_owl::{Language, select_books};

fn lang(name: &str, bytes: u64) -> Language {
    Language { name: name.into(), color: None, bytes }
}

fn names(langs: &[Language], min_percent: f64) -> Vec<String> {
    select_books(langs, min_percent).into_iter().map(|b| b.name).collect()
}

#[test]
fn drops_languages_under_the_cutoff() {
    let langs = [lang("Python", 800), lang("HTML", 150), lang("CSS", 50)];

    assert_eq!(names(&langs, 10.0), ["Python", "HTML"]);
}

#[test]
fn keeps_at_most_three_largest_first() {
    let langs = [
        lang("CSS", 200),
        lang("Rust", 300),
        lang("Shell", 150),
        lang("Python", 350),
    ];

    assert_eq!(names(&langs, 10.0), ["Python", "Rust", "CSS"]);
}

#[test]
fn always_keeps_the_top_language() {
    let mut langs: Vec<Language> = (0..12).map(|i| lang(&format!("L{i}"), 100)).collect();
    langs[3].bytes = 101;

    assert_eq!(names(&langs, 10.0), ["L3"]);
}

#[test]
fn reports_each_books_share() {
    let langs = [lang("Python", 750), lang("Rust", 250)];

    let books = select_books(&langs, 10.0);

    assert_eq!(books[0].percent, 75.0);
    assert_eq!(books[1].percent, 25.0);
}

#[test]
fn no_code_means_no_books() {
    assert!(select_books(&[lang("Python", 0)], 10.0).is_empty());
}
