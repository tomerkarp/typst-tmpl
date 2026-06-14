# Copilot Instructions

This project contains Hebrew-language academic homework documents written in Typst.

## Suggestion scope (important)

Keep inline suggestions SHORT and local. The author is writing math and prose by
hand and only wants help with the next small piece — not whole solutions.

- Complete at most the **current line or the current math expression**. Stop at the
  closing `$`, the end of the line, or the next blank line.
- Do **not** generate an entire `*א.*` / `*ב.*` sub-answer, a full derivation, or a
  whole `== שאלה N` block from a heading or a one-line prompt.
- Do **not** invent the rest of a problem, its given values, or its final result.
  The author supplies the problem; you only help type what they've started.
- Prefer finishing a half-typed expression (e.g. completing `sum_(n=` → `sum_(n=0)^(N-1)`)
  or the rest of a known identity over proposing new steps.
- When unsure, suggest less. One correct line beats a long speculative block.

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
- `HW*.typ` — individual homework files, one per assignment
