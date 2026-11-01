//! Fetches a user's public repos and their languages from GitHub's GraphQL API.

use serde::Deserialize;
use serde_json::json;

use crate::model::{Language, Repo};

const QUERY: &str = "query($login: String!) {
  user(login: $login) {
    repositories(first: 50, privacy: PUBLIC, ownerAffiliations: [OWNER],
                 orderBy: {field: PUSHED_AT, direction: DESC}) {
      nodes {
        name
        isFork
        pushedAt
        languages(first: 100, orderBy: {field: SIZE, direction: DESC}) {
          edges { size node { name color } }
        }
      }
    }
  }
}";

pub fn fetch_repos(login: &str, token: &str) -> Result<Vec<Repo>, String> {
    let response: Response = ureq::post("https://api.github.com/graphql")
        .set("Authorization", &format!("bearer {token}"))
        .set("User-Agent", "repo-owl")
        .send_json(json!({ "query": QUERY, "variables": { "login": login } }))
        .map_err(|e| format!("GitHub API request failed: {e}"))?
        .into_json()
        .map_err(|e| format!("GitHub API returned unexpected JSON: {e}"))?;

    if let Some(errors) = response.errors {
        let messages: Vec<String> = errors.into_iter().map(|e| e.message).collect();
        return Err(format!("GitHub API error: {}", messages.join("; ")));
    }
    let user = response
        .data
        .and_then(|d| d.user)
        .ok_or_else(|| format!("GitHub user '{login}' not found"))?;

    Ok(user
        .repositories
        .nodes
        .into_iter()
        .map(|r| Repo {
            name: r.name,
            is_fork: r.is_fork,
            pushed_at: r.pushed_at.unwrap_or_default(),
            languages: r
                .languages
                .edges
                .into_iter()
                .map(|e| Language { name: e.node.name, color: e.node.color, bytes: e.size })
                .collect(),
        })
        .collect())
}

#[derive(Deserialize)]
struct Response {
    data: Option<Data>,
    errors: Option<Vec<GraphQlError>>,
}

#[derive(Deserialize)]
struct GraphQlError {
    message: String,
}

#[derive(Deserialize)]
struct Data {
    user: Option<User>,
}

#[derive(Deserialize)]
struct User {
    repositories: Nodes<RepoNode>,
}

#[derive(Deserialize)]
struct Nodes<T> {
    nodes: Vec<T>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RepoNode {
    name: String,
    is_fork: bool,
    pushed_at: Option<String>,
    languages: Edges,
}

#[derive(Deserialize)]
struct Edges {
    edges: Vec<LanguageEdge>,
}

#[derive(Deserialize)]
struct LanguageEdge {
    size: u64,
    node: LanguageNode,
}

#[derive(Deserialize)]
struct LanguageNode {
    name: String,
    color: Option<String>,
}
