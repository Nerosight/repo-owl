//! What repo-owl knows about specific languages, in one place.

struct Known {
    name: &'static str,
    /// Spine label, when the first three letters would read badly.
    label: Option<&'static str>,
    /// GitHub's color, used by `--fake`, which has no API to ask.
    color: &'static str,
}

const KNOWN: &[Known] = &[
    Known { name: "Python", label: Some("PY"), color: "#3572A5" },
    Known { name: "Rust", label: Some("RS"), color: "#dea584" },
    Known { name: "JavaScript", label: Some("JS"), color: "#f1e05a" },
    Known { name: "TypeScript", label: Some("TS"), color: "#3178c6" },
    Known { name: "HTML", label: None, color: "#e34c26" },
    Known { name: "CSS", label: None, color: "#663399" },
    Known { name: "Jupyter Notebook", label: Some("NB"), color: "#DA5B0B" },
    Known { name: "Go", label: None, color: "#00ADD8" },
    Known { name: "Java", label: None, color: "#b07219" },
    Known { name: "C++", label: None, color: "#f34b7d" },
    Known { name: "Kotlin", label: Some("KT"), color: "#A97BFF" },
    Known { name: "Shell", label: Some("SH"), color: "#89e051" },
    Known { name: "PowerShell", label: Some("PS"), color: "#012456" },
    Known { name: "Markdown", label: Some("MD"), color: "#083fa1" },
    Known { name: "Haskell", label: Some("HS"), color: "#5e5086" },
    Known { name: "Ruby", label: Some("RB"), color: "#701516" },
    Known { name: "Swift", label: Some("SWF"), color: "#F05138" },
    Known { name: "Dockerfile", label: Some("DKR"), color: "#384d54" },
    Known { name: "Makefile", label: Some("MK"), color: "#427819" },
];

fn known(name: &str) -> Option<&'static Known> {
    KNOWN.iter().find(|k| k.name == name)
}

/// Short spine label; anything without one uses its first three letters.
pub fn label(name: &str) -> String {
    if let Some(label) = known(name).and_then(|k| k.label) {
        return label.to_string();
    }
    name.to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '#')
        .take(3)
        .collect()
}

/// GitHub's color for a language, if repo-owl knows it offline.
pub fn color(name: &str) -> Option<String> {
    known(name).map(|k| k.color.to_string())
}
