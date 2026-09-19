# cx Output Examples

These outputs were captured from a small sample project. Its `src/main.rs` contains:

```rust
fn hello() {}
fn main() { hello(); }
```

Its `GUIDE.md` contains `# Guide`, `## Setup`, and `Use cx.` on separate lines.

## Directory overview

`cx overview src`

```
[1]{file,symbols}:
  src/main.rs,"hello, main"
```

## Markdown headings

`cx overview GUIDE.md`

```
[2]{name,kind,range,signature}:
  Guide,heading,"1-3",# Guide
  Setup,heading,"2-3",## Setup
```

## Symbol kinds

`cx symbols --kinds`

```
[2]{kind,count}:
  fn,2
  heading,2
```

## Unlimited JSON

`cx symbols --kind fn --json --all`

```json
[
  {
    "file": "src/main.rs",
    "name": "hello",
    "kind": "fn",
    "signature": "fn hello()"
  },
  {
    "file": "src/main.rs",
    "name": "main",
    "kind": "fn",
    "signature": "fn main()"
  }
]
```

## Truncated JSON

`cx symbols --kind fn --json --limit 1`

```json
{
  "total": 2,
  "offset": 0,
  "limit": 1,
  "results": [
    {
      "file": "src/main.rs",
      "name": "hello",
      "kind": "fn",
      "signature": "fn hello()"
    }
  ]
}
```

## Definition

`cx definition --name hello`

```
file: src/main.rs
line: 1
---
fn hello() {}
```

## References with context

`cx references --name hello --context`

```
[2]{file,line,caller,context}:
  src/main.rs,1,hello,"fn hello() {}"
  src/main.rs,2,main,"fn main() { hello(); }"
```

## Output Format Notes

- TOON is the default; use `--json` for JSON.
- Line numbers and ranges are 1-indexed. Paths are relative to the project root.
- JSON uses `{total, offset, limit, results}` only when truncated or when an offset is applied; otherwise it is an array.
- References match identifier names, including declarations. `caller` names the enclosing symbol; it does not classify a match as a function call.
