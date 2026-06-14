<div align="center">

# 📐 typst-tmpl

**A single-binary CLI that scaffolds Hebrew Typst academic homework projects.**

No Node. No npm. No `git clone`. Template, David fonts, Claude Code skills, and
GitHub Copilot instructions are all baked into the executable at compile time.

[![Built with Rust](https://img.shields.io/badge/built_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Typst](https://img.shields.io/badge/Typst-ready-239DAD)](https://typst.app/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](#-license)

</div>

---

## ✨ Features

- 🦀 **Single static binary** — install once, run from anywhere, zero runtime dependencies
- 🔤 **Fonts bundled** — David + David Bold ship inside every project, embedded in the binary
- 🤖 **AI-ready** — generates Claude Code skills and a GitHub Copilot instructions file for `.typ` files
- 🧮 **Hebrew + math first** — RTL template and homework header tuned for academic problem sets
- 👥 **Remembers your authors** — enter names once per project, reused on every new homework
- 📅 **Auto-dated** — today's date is filled in automatically

## 📦 Install

> Requires the [Rust toolchain](https://www.rust-lang.org/tools/install) (≥ 1.85).

```powershell
git clone https://github.com/tomerkarp/typst-tmpl
cd typst-tmpl
cargo install --path .
```

This drops `typst-tmpl` into `~/.cargo/bin` (already on your `PATH`).

## 🚀 Usage

### 1. Start a new project

```powershell
typst-tmpl new my-dsp-course
```

You'll be prompted for author names and student IDs (1 or 2 authors). The tool then scaffolds:

```text
my-dsp-course/
├── template.typ                    # shared title-page template
├── fonts/                          # David + David Bold (bundled)
│   ├── DAVID.TTF
│   └── DAVIDBD.TTF
├── .authors                        # saved authors (reused by `hw`)
├── .gitignore
├── .github/
│   └── copilot-instructions.md     # Copilot context for .typ files
└── .agents/skills/typst/           # Claude Code skill
```

…and runs `git init` for you. ✅

### 2. Add a homework file

From inside the project:

```powershell
typst-tmpl hw "עיבוד אותות דיגיטלי" 2
```

> Creates `HW2.typ` with today's date and your saved authors, then opens it in VS Code.

Want a custom filename?

```powershell
typst-tmpl hw "מוליכים למחצה" 8 --out Semi8.typ
```

### 3. Compile

The David fonts live in each project's `fonts/` folder, so point Typst at them:

```powershell
typst compile --font-path fonts HW2.typ
typst watch   --font-path fonts HW2.typ
```

## 🔧 Commands

| Command | Description |
| ------- | ----------- |
| `typst-tmpl new [name]` | Scaffold a new homework project (prompts for name if omitted) |
| `typst-tmpl hw <title> <number>` | Add a new homework file using saved authors |
| `typst-tmpl hw <title> <number> --out <file>` | …with a custom filename |
| `typst-tmpl --help` | Show help |

## 🛠️ Updating the template

Edit anything under [`assets/`](assets/), then reinstall:

```powershell
cargo install --path . --force
```

New projects pick up the changes immediately. Existing projects are independent copies and stay untouched.

## 📄 License

MIT
