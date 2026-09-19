# cx Setup and Recovery

## First-Run Checks

```bash
command -v cx
cx overview .
cx lang list
```

Run `cx overview .` as the initial project probe. If a grammar is missing, cx
prints detected languages and the exact `cx lang add ...` command to run.

## Installation

```bash
brew tap ind-igo/cx && brew install cx
cargo install cx-cli
curl -sL https://raw.githubusercontent.com/ind-igo/cx/master/install.sh | sh
```

Install required grammars as needed:

```bash
cx lang add rust typescript python go swift dart
```

Set `CX_CACHE_DIR` when the default cache location is not writable in a sandbox.

## Failure Modes

### `cx: unsupported file type: .yaml`

Use normal read tools for YAML, JSON, TOML, binary files, and non-symbol regions.

### `cx: database locked, waiting...`

Let the other cx process finish, then retry. If the lock persists, inspect running
cx processes. Only run `cx cache clean` after they have exited.

### `cx: file not in index: <path>`

Ensure the file is within the git root, or pass `--root <path>` for the intended
project boundary.

### Missing grammar diagnostics

Indexing reports missing grammars with an install hint; references may report
`cx: <language> grammar not installed`. Run the suggested `cx lang add <language>`
command, then rerun the query.

### `cx: no matches`

Search with a glob such as `cx symbols --name "*partial*"`, inspect the file with
`cx overview`, and verify spelling and case before retrying.
