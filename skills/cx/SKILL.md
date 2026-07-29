---
name: cx
description: "Semantic code navigation with `cx` CLI. Use when you need to understand code structure before reading files, find symbol definitions, trace references before refactoring, or explore large codebases efficiently. Triggers: 'cx overview', 'cx symbols', 'cx definition', 'cx references', 'code structure', 'find function', 'where is X defined', 'who calls X', 'semantic navigation'."
---

# cx

Use `cx` to navigate code semantically **before** reading full files. This saves tokens and focuses attention on relevant code.

## Escalation Hierarchy

**Always follow this order**: overview → symbols → definition / references → read

| Goal | Command | Token Cost |
|------|---------|------------|
| Understand file or directory structure | `cx overview <path> [--full]` | ~200 tokens |
| Find symbols across project | `cx symbols [--kind K] [--name GLOB] [--kinds]` | ~300 tokens |
| Read specific function/type/heading | `cx definition --name <name> [--from PATH]` | ~500 tokens |
| Find semantic usages of a symbol | `cx references --name <name> [--context]` | ~200 tokens |
| Full file context (last resort) | `read <file>` | ~2000+ tokens |

## Quick Reference

```bash
# File or directory structure
cx overview PATH                    # Symbol table of contents for a file, or symbol list for a dir
cx overview DIR --full              # Include kind, range, signature for directory files

# Symbol search across project
cx symbols [--kind K] [--name GLOB] [--file PATH]
cx symbols --kinds [--file PATH]   # List distinct symbol kinds with counts

# Symbol body (function/type definition or heading section)
cx definition --name NAME [--from PATH] [--kind K] [--max-lines N]

# Symbol usages (call sites, references)
cx references --name NAME [--file PATH] [--context]

# Pagination & Filters
--limit N                           # Max results to return
--offset N                          # Skip first N results
--all                               # Bypass default limit
--no-tests                          # Exclude test files/symbols
--root PATH                         # Project root (default: git root)
--json                              # Emit JSON output

# Language management
cx lang list                        # Show installed grammars
cx lang add LANG [LANG...]          # Install grammars (rust, typescript, python, go, swift, etc.)

# Cache management
cx cache path                       # Show index cache location
cx cache clean                      # Delete cached index
```

**Environment Variables**:
- `CX_CACHE_DIR`: Override index/grammar cache location (useful in sandboxed environments)

**Short aliases**: `cx o`, `cx s`, `cx d`, `cx r`

**Symbol kinds**: `fn`, `struct`, `enum`, `trait`, `type`, `const`, `class`, `interface`, `module`, `event`, `field`, `heading` *(Note: `method` is represented as `fn`)*

## When to Use cx vs Read

### ✅ Use cx when:
- **Before reading a file/directory**: run `cx overview` first to understand structure
- **Before editing a function/heading**: use `cx definition --name X` to capture exact text
- **Before refactoring**: use `cx references --name X [--context]` to understand impact
- **Exploring a codebase**: use `cx symbols` first, then narrow with `cx definition`
- **Navigating Markdown docs**: use `cx overview doc.md` to see heading tree, then `cx definition --name Heading` for section content
- **After context compression**: re-orient with `cx overview`

## Complete Usage Inventory

`cx references` reports **semantic symbol references**. It does not enumerate binary
entrypoints, manifests, documentation, tests, or plain-text path mentions.

When the request asks for **all usages**, **entrypoints**, or **integration impact**:

1. Run `cx references --name <name> --context` for semantic call sites.
2. Classify the target: library symbol, binary entrypoint, configuration value, or file path.
3. Search the relevant non-symbol surfaces with `rg`; limit the search to known files or
   directories and exclude generated output.
4. Report results in separate groups: **semantic references**, **runtime/manifest entrypoints**,
   and **documentation/test/plain-text mentions**. Do not present a text match as a call site.

```bash
# Rust binary entrypoints and file-path mentions
rg -n --glob '!target/**' 'src/main\.rs|\[\[bin\]\]' Cargo.toml README.md docs tests

# Generic documentation and test mentions for a symbol
rg -n --glob '!node_modules/**' --glob '!vendor/**' '\bNAME\b' README.md docs tests
```

If the repository has no relevant manifest, documentation, or test paths, say so explicitly;
do not broaden the search to unrelated parent directories.

### ❌ Don't use cx when:
- File is **not a supported language** (Supported: Rust, TypeScript/JS, Python, Go, C, Objective-C, C++, Java, Ruby, Lua, Zig, Bash, Solidity, Dart, Elixir, Swift, Markdown)
- File is **YAML, JSON, TOML** — cx returns `unsupported file type`
- Target is an **anonymous function, JSX inline component, dynamic dispatch, or non-symbol region** — cx only indexes named symbols
- You need **full file context** (imports, non-code prose)
- You need to **edit the file** — cx is read-only navigation

## Pagination & Result Limits

Default result limits:
- `overview`: Unlimited
- `definition`: 3
- `symbols`: 100
- `references`: 50

When results are truncated, `stderr` shows: `cx: 3/32 definitions for "X" | --from PATH to narrow | --offset 3 for more | --all`.
- Use `--file`/`--from` and `--kind` to narrow scope.
- Use `--offset N` for subsequent pages, or `--all` for all results.
- When limited, `--json` returns `{total, offset, limit, results: [...]}`. When unlimited, `--json` returns a bare array `[...]`.

## First-Run Checks (Once Per Session)

1. **Is cx installed?** Run `command -v cx`. If absent, ask user to install (`brew install cx` or `cargo install cx-cli`).
2. **Are language grammars installed?** Run `cx overview .` as initial probe. If grammars are missing, cx prints self-diagnosing instructions:
   ```
   cx: no language grammars installed
   Detected languages in this project:
     typescript (37 files)
     markdown (7 files)
   Install with: cx lang add typescript markdown
   ```

## 🔴 CHECKPOINT: Before Running cx

**STOP and verify**:
1. Target file is a **supported programming language or Markdown** (not .yaml, .json, .toml)
2. You're in a **git repository** (or specify `--root <PATH>`)
3. Required **language grammar is installed** (`cx lang list` to check)


## Failure Modes & Recovery

### Error: `cx: unsupported file type: .yaml`
**Cause**: cx only parses supported source code and Markdown files
**Fix**: Use `read` tool directly for .yaml/.json/.toml files

### Error: `cx: database locked, waiting...`
**Cause**: Another cx process holds the index lock
**Fix**:
1. Wait 2-3 seconds and retry
2. If persistent: `cx cache clean` to reset
3. Last resort: kill stale cx processes

### Error: `cx: file not in index: <path>`
**Cause**: File not yet indexed (new file or outside project root)
**Fix**: Ensure file is within git root or pass `--root <PATH>`

### Error: `cx: missing grammar for <language>`
**Cause**: Language grammar not installed
**Fix**: `cx lang add <language>` (e.g., `cx lang add rust typescript swift`)

### Error: `cx: symbol not found: <name>`
**Cause**: Symbol doesn't exist or name is misspelled
**Fix**:
1. Try `cx symbols --name "*partial*"` with glob pattern
2. Check `cx overview <file>` to see available symbols
3. Verify spelling and case sensitivity

## Anti-Patterns (Don't Do This)

| ❌ Wrong | ✅ Right | Why |
|----------|----------|-----|
| `cx overview config.yaml` | `read config.yaml` | cx doesn't parse YAML/JSON |
| `cx definition --name main` without context | `cx definition --name main --from src/app.py` | Ambiguous names need `--from` disambiguation |
| Reading full file before checking structure | `cx overview file.py` → then targeted `cx definition` | Saves 80%+ tokens |
| `cx symbols` with exact name | `cx symbols --name "*handler*"` | Use glob patterns for discovery |
| Ignoring `cx lang list` errors | Install missing grammars first (`cx lang add <lang>`) | cx silently skips unsupported files |

## Workflow Examples

### Example 1: Understand a new codebase
```bash
# Step 1: Overview project directory
cx overview .

# Step 2: Find all functions
cx symbols --kind fn

# Step 3: Look at entry point
cx definition --name main --from src/main.rs

# Step 4: Trace dependencies
cx references --name main --context
```

### Example 2: Safe refactoring
```bash
# Step 1: Find all usages before renaming (with source lines)
cx references --name old_function_name --context

# Step 2: Read the definition
cx definition --name old_function_name

# Step 3: Now edit with confidence
```

### Example 3: Markdown documentation navigation
```bash
# Step 1: View section heading hierarchy
cx overview README.md

# Step 2: Extract specific section content
cx definition --name "Installation" --from README.md
```

## 📚 Reference Documents

| Document | Purpose |
|----------|--------|
| `references/decision-tree.md` | Visual decision tree for when/how to use cx |
| `references/output-examples.md` | Real output samples from each command |

## Output Format Reference

See `references/output-examples.md` for real output samples from each command. Key points:
- Default format: **TOON** (compact, line-based)
- Use `--json` flag for machine-parseable JSON
- Line numbers are 1-indexed
- File paths are relative to project root

## Installation

Install `cx` via Homebrew, Cargo, or shell script:

```bash
# Option 1: Homebrew (macOS/Linux)
brew tap ind-igo/cx && brew install cx

# Option 2: Cargo
cargo install cx-cli

# Option 3: Shell Installer Script
curl -sL https://raw.githubusercontent.com/ind-igo/cx/master/install.sh | sh
```

If `cx` reports a missing grammar:

```bash
cx lang add rust typescript python go swift dart
```

## 🛑 STOP: Limitations

- **Read-only**: cx cannot modify files, only navigate
- **Language support**: Limited to installed grammars (17 supported languages)
- **No config/markup parsing**: .yaml, .json, .toml are unsupported (.md is supported via headings)
- **Git-dependent**: Uses git root as default project boundary (override with `--root`)
- **Index latency**: New files may need a moment to appear in index

## 🔴 CHECKPOINT: Common Pitfalls

**Before running any cx command, verify:**

1. **File type check**: Is it a supported language or Markdown? (not .yaml/.json/.toml)
2. **Git root**: Are you in a git repository or specifying `--root`?
3. **Grammar installed**: Run `cx lang list` to verify language support
4. **Symbol name**: Use glob patterns (`*partial*`) for discovery, not exact names

**If command fails:**
- Check error message against "Failure Modes & Recovery" section
- Try `--json` flag for machine-parseable output
- Fall back to `read` tool if cx doesn't support the file type
