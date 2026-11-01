use std::{env, fs, path::Path, process::ExitCode};

use repo_owl::{Language, fetch_repos, languages, pick_repo, render, select_books};

const USAGE: &str = "\
repo-owl: draws a pixel owl reading books in your latest repo's languages

Usage:
  repo-owl --user <login> [--out repo-owl.svg] [--min-percent 10] [--exclude a,b] [--token <token>]
  repo-owl --fake \"Python:70,Rust:30\" [--repo-name my-repo] [--out preview.svg]

The token falls back to the GITHUB_TOKEN or GH_TOKEN environment variable.";

#[derive(Default)]
struct Args {
    user: Option<String>,
    out: Option<String>,
    min_percent: Option<String>,
    exclude: Option<String>,
    token: Option<String>,
    fake: Option<String>,
    repo_name: Option<String>,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args = parse_args(env::args().skip(1))?;
    let out = non_empty(args.out).unwrap_or_else(|| "repo-owl.svg".into());
    let min_percent: f64 = match non_empty(args.min_percent) {
        Some(p) => p.parse().map_err(|_| format!("--min-percent must be a number, got '{p}'"))?,
        None => 10.0,
    };

    let (repo_name, languages) = match non_empty(args.fake) {
        Some(spec) => {
            let name = non_empty(args.repo_name).unwrap_or_else(|| "my-repo".into());
            (name, parse_fake(&spec)?)
        }
        None => {
            let user = non_empty(args.user).ok_or("--user is required (or use --fake)")?;
            let token = non_empty(args.token)
                .or_else(|| env::var("GITHUB_TOKEN").ok())
                .or_else(|| env::var("GH_TOKEN").ok())
                .ok_or("no token: pass --token or set GITHUB_TOKEN")?;
            let exclude: Vec<String> = args
                .exclude
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let repos = fetch_repos(&user, &token)?;
            let repo = pick_repo(&repos, &user, &exclude)
                .ok_or_else(|| format!("{user} has no public repos with code for the owl to read"))?;
            (repo.name.clone(), repo.languages.clone())
        }
    };

    let books = select_books(&languages, min_percent);
    let names: Vec<&str> = books.iter().map(|b| b.name.as_str()).collect();
    if let Some(dir) = Path::new(&out).parent().filter(|d| !d.as_os_str().is_empty()) {
        fs::create_dir_all(dir).map_err(|e| format!("can't create {}: {e}", dir.display()))?;
    }
    fs::write(&out, render(&repo_name, &books)).map_err(|e| format!("can't write {out}: {e}"))?;
    println!("Owl is reading {repo_name}: {} -> {out}", names.join(", "));
    Ok(())
}

fn parse_args(mut raw: impl Iterator<Item = String>) -> Result<Args, String> {
    let mut args = Args::default();
    while let Some(flag) = raw.next() {
        if flag == "-h" || flag == "--help" {
            println!("{USAGE}");
            std::process::exit(0);
        }
        let slot = match flag.as_str() {
            "--user" => &mut args.user,
            "--out" => &mut args.out,
            "--min-percent" => &mut args.min_percent,
            "--exclude" => &mut args.exclude,
            "--token" => &mut args.token,
            "--fake" => &mut args.fake,
            "--repo-name" => &mut args.repo_name,
            _ => return Err(format!("unknown argument '{flag}'\n\n{USAGE}")),
        };
        *slot = Some(raw.next().ok_or_else(|| format!("{flag} needs a value"))?);
    }
    Ok(args)
}

/// Parses `"Python:70,Rust:30"` into languages.
fn parse_fake(spec: &str) -> Result<Vec<Language>, String> {
    spec.split(',')
        .map(|part| {
            let (name, bytes) = part
                .rsplit_once(':')
                .ok_or_else(|| format!("--fake entries look like Name:amount, got '{part}'"))?;
            let name = name.trim().to_string();
            let bytes = bytes.trim().parse().map_err(|_| format!("bad amount in '{part}'"))?;
            let color = languages::color(&name);
            Ok(Language { name, color, bytes })
        })
        .collect()
}

/// The action passes empty strings for inputs the user left out.
fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|v| !v.trim().is_empty())
}
