# typst-tmpl

A single-binary CLI that scaffolds Hebrew Typst academic homework projects — no Node, no npm, no git clone. Everything (template, David fonts, Claude Code skills, Copilot instructions) is baked into the executable at compile time.

## Install

```powershell
cargo install --path .
```

This puts `typst-tmpl` on your PATH (`~/.cargo/bin`). Build the toolchain with Rust ≥ 1.85.

## Usage

### Start a new project

```powershell
typst-tmpl new my-dsp-course
```

Prompts for author names and student IDs (1 or 2 authors), then creates:

```
my-dsp-course/
├── template.typ                  # shared title-page template
├── fonts/                        # David + David Bold (bundled)
│   ├── DAVID.TTF
│   └── DAVIDBD.TTF
├── .authors                      # saved authors (reused by `hw`)
├── .gitignore
├── .github/copilot-instructions.md
└── .agents/skills/typst/         # Claude Code skill
```

Git is initialized automatically.

### Add a homework file

From inside a project:

```powershell
typst-tmpl hw "עיבוד אותות דיגיטלי" 2
# → creates HW2.typ with today's date and your authors, opens it in VS Code
```

Custom filename:

```powershell
typst-tmpl hw "מוליכים למחצה" 8 --out Semi8.typ
```

### Compile

The David fonts ship in each project's `fonts/` folder:

```powershell
typst compile --font-path fonts HW2.typ
typst watch   --font-path fonts HW2.typ
```

## Updating the template

Edit anything under `assets/`, then reinstall:

```powershell
cargo install --path . --force
```

New projects pick up the changes immediately. (Existing projects are independent copies.)
