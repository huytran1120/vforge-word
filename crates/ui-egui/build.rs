//! Bake the contributor credits into the binary as static Rust tables, and embed the craft-i18n interface
//! catalogs. Nothing is read at run time.
//!
//! - contributors/contributors.json: commit stats per GitHub username, with a mark for whose people.toml
//!   entry is verified.
//! - contributors/people.toml: the names people chose for themselves. A name is used only when the
//!   JSON says the entry was verified (`people_entry`).
//!
//! A missing or malformed file gives empty tables (or no names) and a warning.
//!
//! - `CRAFT_I18N_DIR`, a craft-i18n checkout (storytold/craft-i18n, at the SHA in
//!   `craft-i18n.rev`): `craft_i18n.rs` holds `CRAFT_I18N`, one (locale, shared catalog, VForge Word
//!   catalog) entry per locale that has a `vforge-word.tsv`. Unset, or not a checkout (a warning),
//!   it is empty and the interface is English only; with `CRAFT_I18N_REQUIRED=1` (releases) that
//!   is an error instead. Recipe: craft-i18n's `docs/integration.md`.

use std::fmt::Write as _;

use serde_json::Value;

fn main() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut dir = manifest_dir.join("../contributors");
    if !dir.exists() {
        dir = manifest_dir.join("../../contributors");
    }
    let json = read(&dir.join("contributors.json"), "the About window shows no contributors", |t| {
        serde_json::from_str::<Value>(t).map_err(|e| e.to_string())
    })
    .unwrap_or(Value::Null);
    let people = read(&dir.join("people.toml"), "contributors are shown by username only", |t| t.parse::<toml::Table>().map_err(|e| e.to_string()))
        .unwrap_or_default();
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default()).join("credits.rs");
    if let Err(e) = std::fs::write(&out, generate(&json, &people)) {
        println!("cargo::warning=writing {}: {e}", out.display());
    }
    let out = out.with_file_name("craft_i18n.rs");
    if let Err(e) = std::fs::write(&out, craft_i18n()) {
        println!("cargo::error=writing {}: {e}", out.display());
    }
}

/// The catalog this app reads in each locale: `locales/<locale>/vforge-word.tsv`.
const APP_CATALOG: &str = "vforge-word.tsv";
/// Upstream craft-i18n catalog name before rebranding.
const LEGACY_APP_CATALOG: &str = "wordcraft.tsv";

/// `CRAFT_I18N` from `CRAFT_I18N_DIR`: the `Embedded` list the craft-i18n crate reads.
fn craft_i18n() -> String {
    println!("cargo::rerun-if-env-changed=CRAFT_I18N_DIR");
    println!("cargo::rerun-if-env-changed=CRAFT_I18N_REQUIRED");
    let required = std::env::var("CRAFT_I18N_REQUIRED").is_ok_and(|v| v.trim() == "1");
    let default_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../craft-i18n");
    let i18n_dir = std::env::var_os("CRAFT_I18N_DIR")
        .map(std::path::PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .or_else(|| if default_dir.is_dir() { Some(default_dir) } else { None });
    let entries = match i18n_dir {
        // Not canonicalized: on Windows that gives `\\?\C:\…`, which `include_str!` can't read.
        Some(dir) => {
            let locales_dir = dir.join("locales");
            match locales(&locales_dir).or_else(|e| fallback_locales(&locales_dir).ok_or(e)) {
                Ok(entries) => entries,
                Err(e) => {
                    let level = if required { "error" } else { "warning" };
                    println!("cargo::{level}=CRAFT_I18N_DIR: {e}; the interface is English only");
                    Vec::new()
                }
            }
        }
        None => {
            if required {
                println!("cargo::error=CRAFT_I18N_REQUIRED=1 but CRAFT_I18N_DIR is unset: a release must embed the translations");
            }
            Vec::new()
        }
    };
    let mut o = String::from("/// (locale, shared catalog, VForge Word's catalog), one entry per locale; `\"\"` for a missing file.\n");
    o.push_str("pub static CRAFT_I18N: &craft_i18n::Embedded = &[\n");
    for (code, shared, app) in entries {
        // `{:?}` escapes the path (Windows backslashes) as a Rust string literal.
        let shared = shared.map_or_else(|| "\"\"".to_string(), |p| format!("include_str!({p:?})"));
        let _ = writeln!(o, "    ({code:?}, {shared}, include_str!({app:?})),");
    }
    o.push_str("];\n");
    o
}

/// A locale's code, its `common.tsv` if any, and its app catalog.
type Locale = (String, Option<std::path::PathBuf>, std::path::PathBuf);

/// Every locale under `dir` with this app's catalog, by code: (code, its `common.tsv` if any, its
/// app catalog). An error when `dir` isn't a craft-i18n `locales/` directory.
fn locales(dir: &std::path::Path) -> Result<Vec<Locale>, String> {
    // Build scripts run in the crate's directory and `include_str!` resolves against `OUT_DIR`, so
    // a relative path would mean two different places.
    if !dir.is_absolute() {
        return Err(format!("{} is not an absolute path", dir.display()));
    }
    let list = std::fs::read_dir(dir).map_err(|e| format!("{} is not a craft-i18n checkout ({e})", dir.display()))?;
    println!("cargo::rerun-if-changed={}", dir.display());
    let mut out = Vec::new();
    for entry in list.flatten() {
        let path = entry.path();
        let Some(code) = path.file_name().and_then(std::ffi::OsStr::to_str).map(str::to_string) else { continue };
        let app = path.join(APP_CATALOG);
        if !app.is_file() {
            continue;
        }
        println!("cargo::rerun-if-changed={}", app.display());
        let shared = Some(path.join("common.tsv")).filter(|p| p.is_file());
        if let Some(p) = &shared {
            println!("cargo::rerun-if-changed={}", p.display());
        }
        out.push((code, shared, app));
    }
    if out.is_empty() {
        return Err(format!("{} has no locale with a {APP_CATALOG}", dir.display()));
    }
    out.sort();
    Ok(out)
}

/// When CI checks out upstream `storytold/craft-i18n` over `craft-i18n/`, restore the repository's
/// `craft-i18n/locales` into `OUT_DIR` via git, or fall back to upstream `wordcraft.tsv` rebranded
/// in `OUT_DIR`.
fn fallback_locales(dir: &std::path::Path) -> Option<Vec<Locale>> {
    if !dir.is_absolute() {
        return None;
    }
    let has_legacy = std::fs::read_dir(dir).ok()?.flatten().any(|e| e.path().join(LEGACY_APP_CATALOG).is_file());
    if !has_legacy {
        return None;
    }
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR")?);
    let git_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.git");
    if git_dir.exists() {
        let work_tree = out_dir.join("git-craft-i18n");
        let _ = std::fs::create_dir_all(&work_tree);
        let status = std::process::Command::new("git")
            .env("GIT_INDEX_FILE", work_tree.join(".index"))
            .arg("--git-dir")
            .arg(&git_dir)
            .arg("--work-tree")
            .arg(&work_tree)
            .args(["checkout", "-f", "HEAD", "--", "craft-i18n/locales"])
            .status();
        if status.is_ok_and(|s| s.success())
            && let Ok(entries) = locales(&work_tree.join("craft-i18n/locales"))
        {
            return Some(entries);
        }
    }
    let converted = out_dir.join("fallback-craft-i18n/locales");
    let list = std::fs::read_dir(dir).ok()?;
    for entry in list.flatten() {
        let path = entry.path();
        let Some(code) = path.file_name().and_then(std::ffi::OsStr::to_str) else { continue };
        let legacy = path.join(LEGACY_APP_CATALOG);
        if !legacy.is_file() {
            continue;
        }
        let dest = converted.join(code);
        let _ = std::fs::create_dir_all(&dest);
        if let Ok(text) = std::fs::read_to_string(&legacy) {
            let rebranded = text.replace("WordCraft", "VForge Word").replace("wordcraft", "vforge-word");
            let _ = std::fs::write(dest.join(APP_CATALOG), rebranded);
        }
        let shared = path.join("common.tsv");
        if shared.is_file()
            && let Ok(text) = std::fs::read_to_string(&shared)
        {
            let rebranded = text.replace("WordCraft", "VForge Word").replace("wordcraft", "vforge-word");
            let _ = std::fs::write(dest.join("common.tsv"), rebranded);
        }
    }
    locales(&converted).ok()
}

fn read<T>(path: &std::path::Path, fallback: &str, parse: impl Fn(&str) -> Result<T, String>) -> Option<T> {
    println!("cargo::rerun-if-changed={}", path.display());
    let r = std::fs::read_to_string(path).map_err(|e| e.to_string()).and_then(|t| parse(&t));
    if let Err(e) = &r {
        println!("cargo::warning={}: {e}; {fallback}", path.display());
    }
    r.ok()
}

/// A non-empty string field of someone's people.toml entry, as a Rust `Option<&str>` literal.
fn name(v: Option<&str>) -> String {
    match v.map(str::trim).filter(|s| !s.is_empty()) {
        Some(x) => format!("Some({x:?})"),
        None => "None".into(),
    }
}

fn s(v: &Value, k: &str) -> String {
    format!("{:?}", v.get(k).and_then(Value::as_str).unwrap_or(""))
}

fn n(v: &Value, k: &str) -> u64 {
    v.get(k).and_then(Value::as_u64).unwrap_or(0)
}

fn rows(json: &Value, k: &str) -> Vec<Value> {
    json.get(k).and_then(Value::as_array).cloned().unwrap_or_default()
}

fn generate(json: &Value, people: &toml::Table) -> String {
    let mut o = String::new();
    let _ = writeln!(o, "pub static TOTAL_COMMITS: u64 = {};", n(json, "commits"));
    let _ = writeln!(o, "pub static CONTRIBUTORS: &[Contributor] = &[");
    for c in rows(json, "contributors") {
        let login = c.get("login").and_then(Value::as_str).unwrap_or("");
        if login.is_empty() {
            continue;
        }
        // Names only from a verified, self-chosen people.toml entry.
        let entry = people.get("people").and_then(|p| p.get(login)).filter(|_| c.get("people_entry").and_then(Value::as_bool) == Some(true));
        let field = |k: &str| entry.and_then(|e| e.get(k)).and_then(toml::Value::as_str);
        let synced = entry.and_then(|e| e.get("sync_github_name")).and_then(toml::Value::as_bool) == Some(true);
        let display = field("display_name")
            .filter(|s| !s.trim().is_empty())
            .or_else(|| if synced { c.get("github_name").and_then(Value::as_str) } else { None });
        let _ = writeln!(
            o,
            "    Contributor {{ login: {login:?}, display_name: {}, real_name: {}, prs: {}, commits: {}, lines_added: {}, lines_deleted: {}, binary_added: {}, binary_deleted: {}, first_commit: {}, last_commit: {} }},",
            name(display),
            name(field("real_name")),
            n(&c, "prs"),
            n(&c, "commits"),
            n(&c, "lines_added"),
            n(&c, "lines_deleted"),
            n(&c, "binary_added"),
            n(&c, "binary_deleted"),
            s(&c, "first_commit"),
            s(&c, "last_commit"),
        );
    }
    let _ = writeln!(o, "];");
    let _ = writeln!(o, "pub static MODELS: &[Model] = &[");
    for m in rows(json, "models") {
        let _ = writeln!(
            o,
            "    Model {{ company: {}, model: {}, version: {}, commits: {}, lines_added: {}, lines_deleted: {} }},",
            s(&m, "company"),
            s(&m, "model"),
            s(&m, "version"),
            n(&m, "commits"),
            n(&m, "lines_added"),
            n(&m, "lines_deleted"),
        );
    }
    let _ = writeln!(o, "];");
    o
}
