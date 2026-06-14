use chrono::Local;
use clap::{Parser, Subcommand};
use inquire::Text;
use inquire::validator::Validation;
use inquire::CustomUserError;
use std::fs;
use std::path::Path;
use std::process::Command;

// ── embedded assets ────────────────────────────────────────────────────────

const TEMPLATE_TYP: &str       = include_str!("../assets/template.typ");
const FONT_REGULAR: &[u8]      = include_bytes!("../assets/fonts/DAVID.TTF");
const FONT_BOLD: &[u8]         = include_bytes!("../assets/fonts/DAVIDBD.TTF");
const COPILOT_MD: &str         = include_str!("../assets/.github/copilot-instructions.md");
const SKILL_MD: &str           = include_str!("../assets/.agents/skills/typst/SKILL.md");
const ACADEMIC_MD: &str        = include_str!("../assets/.agents/skills/typst/academic.md");
const BASICS_MD: &str          = include_str!("../assets/.agents/skills/typst/basics.md");
const DEBUG_MD: &str           = include_str!("../assets/.agents/skills/typst/debug.md");
const STYLING_MD: &str         = include_str!("../assets/.agents/skills/typst/styling.md");
const TYPES_MD: &str           = include_str!("../assets/.agents/skills/typst/types.md");
const LICENSE: &str            = include_str!("../assets/.agents/skills/typst/LICENSE");
const VERIFY_MD: &str          = include_str!("../assets/.agents/skills/typst/agents/typst-verify.md");
const EXAMPLE_HW: &str         = include_str!("../assets/.agents/skills/typst/examples/hebrew-homework.typ");
const EXAMPLE_BASIC: &str      = include_str!("../assets/.agents/skills/typst/examples/basic-document.typ");
const EXAMPLE_STYLED: &str     = include_str!("../assets/.agents/skills/typst/examples/styled-document.typ");
const EXAMPLE_ACADEMIC: &str   = include_str!("../assets/.agents/skills/typst/examples/academic-paper.typ");
const SEARCH_API_PY: &str      = include_str!("../assets/.agents/skills/typst/scripts/search-api.py");
const API_JSON: &str           = include_str!("../assets/.agents/skills/typst/data/api.json");
const API_BM25_JSON: &str      = include_str!("../assets/.agents/skills/typst/data/api-bm25.json");
const API_MAIN_JSON: &str      = include_str!("../assets/.agents/skills/typst/data/api-main.json");
const API_MAIN_BM25_JSON: &str = include_str!("../assets/.agents/skills/typst/data/api-main-bm25.json");
const API_0142_JSON: &str      = include_str!("../assets/.agents/skills/typst/data/api-0.14.2.json");
const API_0142_BM25_JSON: &str = include_str!("../assets/.agents/skills/typst/data/api-0.14.2-bm25.json");

// ── CLI ────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(name = "typst-tmpl", about = "Typst academic homework project initializer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold a new homework project
    New {
        /// Project folder name (prompted if omitted)
        name: Option<String>,
    },
    /// Add a new homework file to the current project
    Hw {
        /// Course title (Hebrew ok)
        title: String,
        /// Homework number
        number: u32,
        /// Output filename (default: HW<number>.typ)
        #[arg(short, long)]
        out: Option<String>,
    },
}

// ── helpers ────────────────────────────────────────────────────────────────

fn not_empty(s: &str) -> Result<Validation, CustomUserError> {
    if s.trim().is_empty() {
        Ok(Validation::Invalid("Cannot be empty".into()))
    } else {
        Ok(Validation::Valid)
    }
}

fn write_text(path: &Path, content: &str) {
    fs::write(path, content.as_bytes()).unwrap_or_else(|e| panic!("write {:?}: {e}", path));
}

fn write_bytes(path: &Path, content: &[u8]) {
    fs::write(path, content).unwrap_or_else(|e| panic!("write {:?}: {e}", path));
}

fn mkdir(path: &Path) {
    fs::create_dir_all(path).unwrap_or_else(|e| panic!("mkdir {:?}: {e}", path));
}

fn authors_typst(authors: &[(String, String)]) -> String {
    authors
        .iter()
        .map(|(name, id)| format!("    (name: \"{name}\", email: \"\", id: \"{id}\")"))
        .collect::<Vec<_>>()
        .join(",\n")
}

fn hw_content(title: &str, number: u32, authors: &[(String, String)]) -> String {
    let d = Local::now();
    format!(
        "#import \"template.typ\": *\n\
         \n\
         #show: project.with(\n\
         \x20\x20title: \"{title}\",\n\
         \x20\x20number: {number},\n\
         \x20\x20authors: (\n\
         {authors}\n\
         \x20\x20),\n\
         \x20\x20date: datetime(year: {year}, month: {month}, day: {day})\n\
         )\n\
         #set text(font: \"David\")\n\
         #set text(size: 9pt)\n\
         #set math.text(size: 12pt)\n\
         \n\
         == שאלה 1\n\
         \n\
         *א.*\n\
         \n",
        authors = authors_typst(authors),
        year  = d.format("%Y"),
        month = d.format("%-m"),
        day   = d.format("%-d"),
    )
}

// ── commands ───────────────────────────────────────────────────────────────

fn cmd_new(name: Option<String>) {
    let folder = match name {
        Some(n) => n,
        None => Text::new("Project folder name:")
            .with_validator(not_empty)
            .prompt()
            .unwrap_or_else(|_| std::process::exit(0)),
    };

    let base = Path::new(&folder);
    if base.exists() {
        eprintln!("Error: '{}' already exists.", folder);
        std::process::exit(1);
    }

    println!("\nAuthor info (saved into every homework file):\n");

    let a1_name = Text::new("Author 1 – name:")
        .with_validator(not_empty)
        .prompt()
        .unwrap_or_else(|_| std::process::exit(0));
    let a1_id = Text::new("Author 1 – student ID:")
        .with_validator(not_empty)
        .prompt()
        .unwrap_or_else(|_| std::process::exit(0));

    let a2_name = Text::new("Author 2 – name (leave blank if solo):")
        .prompt()
        .unwrap_or_else(|_| std::process::exit(0));
    let a2_id = if !a2_name.trim().is_empty() {
        Text::new("Author 2 – student ID:")
            .with_validator(not_empty)
            .prompt()
            .unwrap_or_else(|_| std::process::exit(0))
    } else {
        String::new()
    };

    let mut authors: Vec<(String, String)> = vec![(a1_name, a1_id)];
    if !a2_name.trim().is_empty() {
        authors.push((a2_name, a2_id));
    }

    // Persist authors for `typst-tmpl hw` to reuse
    let authors_cfg = authors
        .iter()
        .map(|(n, id)| format!("{n}|{id}"))
        .collect::<Vec<_>>()
        .join("\n");

    println!("\nCreating {}…", folder);

    mkdir(base);
    mkdir(&base.join("fonts"));
    mkdir(&base.join(".github"));
    mkdir(&base.join(".agents/skills/typst/agents"));
    mkdir(&base.join(".agents/skills/typst/data"));
    mkdir(&base.join(".agents/skills/typst/examples"));
    mkdir(&base.join(".agents/skills/typst/scripts"));

    write_bytes(&base.join("fonts/DAVID.TTF"),   FONT_REGULAR);
    write_bytes(&base.join("fonts/DAVIDBD.TTF"), FONT_BOLD);
    write_text(&base.join("template.typ"),        TEMPLATE_TYP);
    write_text(&base.join(".authors"),            &authors_cfg);
    write_text(&base.join(".gitignore"),          "*.pdf\n*.png\n");

    write_text(&base.join(".github/copilot-instructions.md"),                    COPILOT_MD);
    write_text(&base.join(".agents/skills/typst/SKILL.md"),                      SKILL_MD);
    write_text(&base.join(".agents/skills/typst/academic.md"),                   ACADEMIC_MD);
    write_text(&base.join(".agents/skills/typst/basics.md"),                     BASICS_MD);
    write_text(&base.join(".agents/skills/typst/debug.md"),                      DEBUG_MD);
    write_text(&base.join(".agents/skills/typst/styling.md"),                    STYLING_MD);
    write_text(&base.join(".agents/skills/typst/types.md"),                      TYPES_MD);
    write_text(&base.join(".agents/skills/typst/LICENSE"),                       LICENSE);
    write_text(&base.join(".agents/skills/typst/agents/typst-verify.md"),        VERIFY_MD);
    write_text(&base.join(".agents/skills/typst/examples/hebrew-homework.typ"),  EXAMPLE_HW);
    write_text(&base.join(".agents/skills/typst/examples/basic-document.typ"),   EXAMPLE_BASIC);
    write_text(&base.join(".agents/skills/typst/examples/styled-document.typ"),  EXAMPLE_STYLED);
    write_text(&base.join(".agents/skills/typst/examples/academic-paper.typ"),   EXAMPLE_ACADEMIC);
    write_text(&base.join(".agents/skills/typst/scripts/search-api.py"),         SEARCH_API_PY);
    write_text(&base.join(".agents/skills/typst/data/api.json"),                 API_JSON);
    write_text(&base.join(".agents/skills/typst/data/api-bm25.json"),            API_BM25_JSON);
    write_text(&base.join(".agents/skills/typst/data/api-main.json"),            API_MAIN_JSON);
    write_text(&base.join(".agents/skills/typst/data/api-main-bm25.json"),       API_MAIN_BM25_JSON);
    write_text(&base.join(".agents/skills/typst/data/api-0.14.2.json"),          API_0142_JSON);
    write_text(&base.join(".agents/skills/typst/data/api-0.14.2-bm25.json"),     API_0142_BM25_JSON);

    let git_ok = Command::new("git")
        .args(["init", base.to_str().unwrap()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    println!(
        "\n✓ {folder}/ created\n{}",
        if git_ok { "✓ git initialized" } else { "! git not found — run `git init` manually" }
    );
    println!("\nNext steps:");
    println!("  cd {folder}");
    println!("  typst-tmpl hw \"Course Name\" 1");
    println!("  typst watch HW1.typ");
}

fn cmd_hw(title: String, number: u32, out: Option<String>) {
    let authors: Vec<(String, String)> = match fs::read_to_string(".authors") {
        Ok(cfg) => cfg
            .trim_start_matches('\u{feff}')
            .lines()
            .filter(|l| l.contains('|'))
            .map(|l| {
                let mut parts = l.splitn(2, '|');
                let name = parts.next().unwrap_or("").to_string();
                let id   = parts.next().unwrap_or("").to_string();
                (name, id)
            })
            .collect(),
        Err(_) => {
            eprintln!("No .authors file found. Run `typst-tmpl new` first, or create .authors with lines: Name|StudentID");
            std::process::exit(1);
        }
    };

    let filename = out.unwrap_or_else(|| format!("HW{number}.typ"));
    if Path::new(&filename).exists() {
        eprintln!("Error: '{filename}' already exists.");
        std::process::exit(1);
    }

    write_text(Path::new(&filename), &hw_content(&title, number, &authors));
    println!("Created {filename}");

    let _ = Command::new("code").arg(&filename).status();
}

// ── entry ──────────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name }              => cmd_new(name),
        Commands::Hw { title, number, out } => cmd_hw(title, number, out),
    }
}
