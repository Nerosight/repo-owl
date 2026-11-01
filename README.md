# repo-owl

Every night, repo-owl finds the public repo you most recently pushed to and draws a pixel-art owl reading it. Each book is one of that repo's languages (up to three). The owl holds the biggest one, and the rest sit stacked on the branch beside it. It blinks, turns pages, and tilts its head now and then.

Written in Rust. Runs as a GitHub Action, so you never have to build it yourself.

## Put one on your profile

1. Create a public repo named exactly like your username (e.g. `octocat/octocat`). Its README shows at the top of your profile.
2. Add `.github/workflows/repo-owl.yml`:

   ```yaml
   name: repo-owl
   on:
     schedule:
       - cron: "17 3 * * *" # nightly
     workflow_dispatch: # adds a "Run workflow" button
   permissions:
     contents: write
   jobs:
     draw:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4
         - uses: Nerosight/repo-owl@v0
   ```

3. Add the owl to your `README.md`, then run the workflow once from the Actions tab:

   ```html
   <img src="repo-owl.svg" alt="An owl reading my latest repo" width="384">
   ```

## Options

| Input | Default | What it does |
|---|---|---|
| `username` | the repo owner | Whose repos the owl reads |
| `output` | `repo-owl.svg` | Where the SVG is written |
| `min-percent` | `10` | Languages under this share don't get a book (the biggest always does) |
| `exclude-repos` | | Comma-separated repos the owl skips, e.g. `dotfiles,notes` |
| `token` | `github.token` | Used for the API and for pushing the SVG |

The owl skips forks, your profile repo, and repos with no detected languages. When it skips a repo, it reads the next most recent one.

## Adding a language logo

Draw a 12×12 PNG and save it as `assets/logos/<language>.png`. The name is the language as GitHub spells it, lowercased with spaces turned into dashes, `+` into `plus`, and `#` into `sharp` (`jupyter-notebook.png`, `cplusplus.png`, `csharp.png`). Transparent pixels show the book's cover color. A language without a logo gets GitHub's color for it plus a short label.

The owl, branch, moon, page-turn frames, and 3×5 font are PNGs in `assets/` too. Repaint them in any pixel editor (Piskel is free in the browser).

## Developing

```sh
cargo test
cargo run -- --fake "Python:70,Rust:30" --repo-name my-repo --out preview.svg   # no API needed
GITHUB_TOKEN=$(gh auth token) cargo run -- --user <login> --out preview.svg
```
