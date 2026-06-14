---
name: typst
description: 'Typst document creation for Hebrew academic homework. Use when: working with .typ files, writing math-heavy RTL homework in Hebrew, or modifying template.typ.'
---

# Typst — Homework Workflow

## Start a New Homework File

```powershell
.\new-hw.ps1 "Course Title" <number>
# Example: .\new-hw.ps1 "עיבוד אותות דיגיטלי" 2
```

Or copy [examples/hebrew-homework.typ](examples/hebrew-homework.typ) and adjust the header.

## Compile

```bash
typst compile document.typ          # → PDF
typst watch document.typ            # auto-recompile on save
typst compile document.typ "page-{p}.png" -f png   # visual check
```

## Document Header (always identical)

```typst
#import "template.typ": *

#show: project.with(
  title: "Course Title",
  number: 1,
  authors: (
    (name: "יואב קדם", email: "", id: "207512682"),
    (name: "תומר קרפובסקי", id: "211566864", email: "")
  ),
  date: datetime(year: 2026, month: 6, day: 13)
)
#set text(font: "David")
#set text(size: 9pt)
#set math.text(size: 12pt)
```

## Document Structure

```typst
== שאלה 1

*א.*

Explanation text. $ X^f (omega) = integral_(-oo)^oo x(t) e^(-j omega t) d t $

**שלב 1 – Step label:** followed by content.

*ב.*

$ therefore Y^f [k] = 1/T X^f ((2pi k)/T) $

#pagebreak()

== שאלה 2
```

## Math Quick Reference

| What | Typst |
|------|-------|
| Display equation | `$ expr $` |
| Aligned equations | `$ a &= b \ &= c $` |
| Fraction | `a/b` or `frac(a, b)` |
| Integral | `integral_(-oo)^oo ... d t` |
| Infinite sum | `sum_(n=-oo)^oo` |
| Piecewise | `cases(1 & 0 <= n <= N-1, 0 & "אחרת")` |
| Underbrace | `underbrace(expr, =1 #h(2pt))` |
| Cancel | `cancel(V_A)` |
| Conjugate | `overline(X^f)` |
| Absolute value | `abs(x)` |
| Therefore | `therefore` |
| Accents | `accent(x, hat)`, `accent(x, arrow)`, `accent(x, macron)` |
| Text in math | `"word"` |
| Footnote in math | `=^((#footnote()[note]))` |

## Left-align Math in RTL

```typst
#align(left, block($rho_p = -q N_A approx -0.048 [C/cm^3]$))
```

## Accent Helpers (define at top of file)

```typst
#let Hat(x) = $accent(#x, hat)$
#let Bar(x) = $accent(#x, macron)$
#let Vec(x) = $accent(#x, arrow)$
```

## Common Errors

| Error | Fix |
|-------|-----|
| "unknown variable" | Typo or missing `#let` |
| "expected X, found Y" | Type mismatch — check function signature |
| "unknown font" | Run `typst fonts` to see what's installed |
| "can only be used when context is known" | Wrap in `context { ... }` |
| set/show rule has no effect | Move it before the content it targets |

## API Search

```bash
python3 .agents/skills/typst/scripts/search-api.py "integral"
python3 .agents/skills/typst/scripts/search-api.py "accent" --kind function
```

## More Detail

| Topic | File |
|-------|------|
| Syntax, imports, functions, loops | [basics.md](basics.md) |
| Data types, arrays, dictionaries | [types.md](types.md) |
| Fonts, RTL, page breaks, images | [styling.md](styling.md) |
| Math equations, numbering | [academic.md](academic.md) |
| Verify compiled output (PNG/HTML) | [debug.md](debug.md) |
