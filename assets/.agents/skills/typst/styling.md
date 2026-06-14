# Styling and Layout

For language basics, see [basics.md](basics.md).

## Standard Page and Text Setup

```typst
#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(font: "David", size: 9pt, lang: "he")
#set math.text(size: 12pt)
#set par(justify: true)
```

## RTL Text Direction

Hebrew is handled automatically when `lang: "he"` is set on `text`. The template already sets this via `set text(font: "Noto Sans Hebrew", lang: "he")`. Per-file overrides:

```typst
#set text(font: "David")   // override font for the file
```

To left-align a specific block (e.g., a math expression) inside an RTL page:

```typst
#align(left, block($rho = -q N_A$))
```

## Page Breaks

```typst
#pagebreak()            // force page break
#pagebreak(weak: true)  // only if not already at page start
```

## Font Configuration

```typst
#set text(font: "David")              // Hebrew serif
#set text(font: "Noto Sans Hebrew")   // Hebrew sans
```

```bash
typst fonts          # list all installed fonts
typst fonts | grep David
```

If a font is missing, install it system-wide or use `--font-path ./fonts`.

## Images

```typst
#image("diagram.png")                           // full width
#image("diagram.png", width: 80%)               // scaled
#image("diagram.png", width: 5cm, height: 4cm, fit: "contain")
```

Paths are relative to the current `.typ` file.

## Horizontal Rule

```typst
#line(length: 100%)
```

## Footnotes

```typst
Some text#footnote[Footnote content here.].

// Inside math:
$ E =^((#footnote()[note text])) m c^2 $
```
