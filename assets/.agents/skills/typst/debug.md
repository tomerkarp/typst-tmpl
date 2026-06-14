# Verifying Typst Output

## Visual Check (PNG export)

```bash
typst compile document.typ "page-{p}.png" -f png
typst compile document.typ "page-{p}.png" -f png --ppi 288   # higher resolution
typst compile document.typ "page-{p}.png" -f png --pages 1-2  # specific pages
```

Then read the PNG files to inspect layout, fonts, math rendering, and page breaks.

## Content Check (HTML export)

```bash
typst compile document.typ /dev/stdout -f html --features html 2>/dev/null
```

Good for checking text content and structure; does not render page-specific features (headers, footers).

## Compilation Errors

| Error | Fix |
|-------|-----|
| "unknown variable" | Typo or missing `#let` |
| "expected X, found Y" | Type mismatch — check function signature |
| "file not found" | Path is relative to the current file's directory |
| "unknown font" | Run `typst fonts` to list installed fonts |
| "can only be used when context is known" | Wrap in `context { ... }` |
| "unexpected argument" | Named args use `:` not `=`: `func(name: value)` |
| set/show rule has no effect | Move rule before the content it targets |

## Quick Inspection with repr

```typst
#repr(some-variable)  // prints internal structure to the document
```
