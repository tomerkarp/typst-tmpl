# Copilot Instructions

This project contains Hebrew-language academic homework documents written in Typst.

## Document structure

Every document imports and uses the shared `template.typ`:

```typst
#import "template.typ": *

#show: project.with(
  title: "שם הקורס",
  number: 1,
  authors: (
    (name: "שם מלא", id: "123456789", email: ""),
  ),
  date: datetime(year: 2026, month: 1, day: 1)
)
#set text(font: "David")
#set text(size: 9pt)
#set math.text(size: 12pt)
```

## Conventions

- Language: Hebrew (RTL). All prose, section titles, and explanations are in Hebrew.
- Math: Typst math mode (`$ ... $`). Use `integral`, `sum`, `cases`, `therefore`, `underbrace`, etc.
- Sections use `== שאלה N` headings (level 2). Sub-questions use bold `*א.*`, `*ב.*`, etc.
- Bold step labels use `**שלב N – ...**` syntax.
- Use `#pagebreak()` between major questions.
- Inline math references use `$"text"$` for text inside math (e.g., `$"אחרת"$`).
- Accent helpers defined at file top: `#let Hat(x) = $accent(#x, hat)$`, `#let Bar(x) = $accent(#x, macron)$`, `#let Vec(x) = $accent(#x, arrow)$`.

## Math notation used in this project

| Symbol | Typst |
|--------|-------|
| Fourier transform | `X^f (omega)` |
| DTFT | `X^f (theta)` |
| Infinite sum | `sum_(n=-oo)^oo` |
| Integral to infinity | `integral_(-oo)^oo` |
| Cases (piecewise) | `cases(val & cond, 0 & "אחרת")` |
| Therefore | `therefore` |
| Underbrace with label | `underbrace(expr, "label")` |
| Cancel (zero out) | `cancel(V_A)` |
| Absolute value | `abs(x)` |
| Conjugate | `overline(X^f)` |

## Alignment in explanations

Use `#align(left, block(...))` for left-aligned math blocks inside an RTL page:

```typst
#align(left, block($rho_p = -q N_A approx -0.048 [C/cm^3]$))
```

## Footnotes

```typst
$ E_"max" =^((#footnote()[הסבר קצר])) (2 V_(B I))/d $
```

## File layout

- `template.typ` — shared project template (do not modify structure)
- `*.typ` — individual homework files, one per assignment
- `DspExample.typ` — Digital Signal Processing homework example
- `example.typ` — Semiconductors homework example
