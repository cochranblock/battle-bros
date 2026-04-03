// This is free and unencumbered software released into the public domain.
// Contributors: GotEmCoach (mcochran), KOVA, Claude Opus 4.6, Mattbusel (XFactor)
//
//! Regulation ingestion and storage.
//! All military regs are public domain — no licensing issues.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A single regulation entry extracted from source text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegEntry {
    /// Domain: ucmj, ar670, fm, ar600, etc.
    pub domain: String,
    /// Section/article identifier (e.g., "Article 92", "AR 670-1 Ch 3-2")
    pub section: String,
    /// The regulation text content.
    pub text: String,
    /// Key phrases extracted for scenario generation.
    pub keywords: Vec<String>,
}

/// Ingest a regulation text file or directory into the corpus.
pub fn ingest(path: &Path, domain: &str) -> anyhow::Result<()> {
    let data_dir = data_path()?;
    std::fs::create_dir_all(&data_dir)?;

    let entries = if path.is_dir() {
        let mut all = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().extension().map(|e| e == "txt" || e == "md").unwrap_or(false) {
                all.extend(parse_reg_file(&entry.path(), domain)?);
            }
        }
        all
    } else {
        parse_reg_file(path, domain)?
    };

    // Append to corpus
    let corpus_path = data_dir.join(format!("{}.json", domain));
    let mut existing: Vec<RegEntry> = if corpus_path.exists() {
        let data = std::fs::read_to_string(&corpus_path)?;
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    };

    let count = entries.len();
    existing.extend(entries);
    std::fs::write(&corpus_path, serde_json::to_string_pretty(&existing)?)?;

    println!("ingest: {} entries from {} → {}", count, path.display(), corpus_path.display());
    Ok(())
}

/// List all regulation domains and their entry counts.
pub fn list_domains() -> anyhow::Result<()> {
    let data_dir = data_path()?;
    if !data_dir.exists() {
        println!("No regulations ingested yet. Run: battle-bros ingest <path> -d <domain>");
        return Ok(());
    }

    println!("== regulation domains ==");
    for entry in std::fs::read_dir(&data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let domain = path.file_stem().unwrap().to_string_lossy();
            let data = std::fs::read_to_string(&path)?;
            let entries: Vec<RegEntry> = serde_json::from_str(&data).unwrap_or_default();
            println!("  {:<12} {} entries", domain, entries.len());
        }
    }
    Ok(())
}

/// Load all entries for a domain (or all domains if "all").
pub fn load_entries(domain: &str) -> anyhow::Result<Vec<RegEntry>> {
    let data_dir = data_path()?;
    let mut entries = Vec::new();

    if !data_dir.exists() {
        return Ok(entries);
    }

    for file in std::fs::read_dir(&data_dir)? {
        let file = file?;
        let path = file.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            let file_domain = path.file_stem().unwrap().to_string_lossy().to_string();
            if domain == "all" || domain == file_domain {
                let data = std::fs::read_to_string(&path)?;
                let file_entries: Vec<RegEntry> = serde_json::from_str(&data).unwrap_or_default();
                entries.extend(file_entries);
            }
        }
    }
    Ok(entries)
}

/// Parse a regulation text file into entries.
/// Uses regex to detect article/section headers and extract content blocks.
fn parse_reg_file(path: &Path, domain: &str) -> anyhow::Result<Vec<RegEntry>> {
    let content = std::fs::read_to_string(path)?;
    let mut entries = Vec::new();

    // Match common military regulation section patterns
    let re_article = Regex::new(r"(?i)^(Article\s+\d+[a-z]?)\s*[.:\-—]\s*(.+)$").unwrap();
    let re_section = Regex::new(r"(?i)^((?:Section|Chapter|Para(?:graph)?)\s+[\d\-\.]+)\s*[.:\-—]\s*(.+)$").unwrap();
    let re_ar = Regex::new(r"(?i)^(AR\s+[\d\-\.]+\s+(?:Ch|Para|Sec)\s+[\d\-\.]+)\s*[.:\-—]\s*(.+)$").unwrap();
    let re_fm = Regex::new(r"(?i)^(FM\s+[\d\-\.]+)\s*[.:\-—]\s*(.+)$").unwrap();
    let re_keyword = Regex::new(r"(?i)\b(shall|must|will not|prohibited|authorized|required|punishable|offense|duty|responsibility)\b").unwrap();

    let mut current_section = String::new();
    let mut current_text = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Check if this line is a new section header
        let header = re_article.captures(trimmed)
            .or_else(|| re_ar.captures(trimmed))
            .or_else(|| re_fm.captures(trimmed))
            .or_else(|| re_section.captures(trimmed));

        if let Some(caps) = header {
            // Save previous section
            if !current_section.is_empty() && !current_text.is_empty() {
                let keywords: Vec<String> = re_keyword
                    .find_iter(&current_text)
                    .map(|m| m.as_str().to_lowercase())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                entries.push(RegEntry {
                    domain: domain.to_string(),
                    section: current_section.clone(),
                    text: current_text.trim().to_string(),
                    keywords,
                });
            }
            current_section = caps[1].to_string();
            current_text = caps[2].to_string();
            current_text.push('\n');
        } else if !trimmed.is_empty() {
            current_text.push_str(trimmed);
            current_text.push('\n');
        }
    }

    // Don't forget last section
    if !current_section.is_empty() && !current_text.is_empty() {
        let keywords: Vec<String> = re_keyword
            .find_iter(&current_text)
            .map(|m| m.as_str().to_lowercase())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        entries.push(RegEntry {
            domain: domain.to_string(),
            section: current_section,
            text: current_text.trim().to_string(),
            keywords,
        });
    }

    Ok(entries)
}

fn data_path() -> anyhow::Result<std::path::PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("no home dir"))?;
    Ok(home.join(".battle-bros/regs"))
}
