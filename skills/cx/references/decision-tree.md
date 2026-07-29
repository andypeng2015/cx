# cx Decision Tree

Quick decision guide for when and how to use cx.

```
START: Need to understand code/docs?
│
├─ YES → Is it a supported language? (Rust/TS/Python/Go/C/ObjC/C++/Java/Ruby/Lua/Zig/Bash/Solidity/Dart/Elixir/Swift/Markdown)
│   │
│   ├─ NO → Use `read` tool directly (for .yaml, .json, .toml, binary files, etc.)
│   │
│   └─ YES → What's your goal?
│       │
│       ├─ Understand file or directory structure → `cx overview <path> [--full]`
│       │   └─ Need specific symbol/section? → `cx definition --name <X>`
│       │
│       ├─ Find symbol across project → `cx symbols [--kind K] [--name GLOB] [--kinds]`
│       │   └─ Found it? → `cx definition --name <X> --from <file>`
│       │
│       ├─ Before editing/refactoring → `cx references --name <X> [--context]`
│       │   └─ See impact? → Proceed with edit
│       │
│       └─ After context compression → `cx overview <path>` to re-orient
│
└─ NO → Do you need full file context? (imports, comments, non-code)
    │
    ├─ YES → Use `read` tool
    │
    └─ NO → Are you editing the file?
        │
        ├─ YES → Use `read` + `edit` tools
        │
        └─ NO → Use `grep` or `web_search` for text patterns
```
## Quick Lookup Table

| Your Situation | First Command | Next Step |
|----------------|---------------|-----------|
| "What's in this file or dir?" | `cx overview file.rs` or `cx overview dir/` | `cx definition --name <X>` |
| "Where is X defined?" | `cx symbols --name "*X*"` | `cx definition --name X --from <file>` |
| "Who calls X?" | `cx references --name X --context` | Read exact reference line context |
| "How does this module work?" | `cx symbols --file module.py` | `cx definition` for key functions |
| "What kinds of symbols exist?" | `cx symbols --kinds` | `cx symbols --kind <K>` |
| "What's in this Markdown doc?" | `cx overview README.md` | `cx definition --name "Section Title"` |

## Error Recovery Flow

```
cx command fails
│
├─ "unsupported file type" → File is .yaml/.json/.toml → Use `read`
│
├─ "database locked" → Wait 2-3s → Retry → `cx cache clean` if persistent
│
├─ "file not in index" → File outside git root? → Pass `--root <path>` or use `read`
│
├─ "symbol not found" → Try glob: `cx symbols --name "*partial*"`
│
└─ "missing grammar" → `cx lang add <language>`
```
