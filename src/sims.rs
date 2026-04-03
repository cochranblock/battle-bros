// This is free and unencumbered software released into the public domain.
// Contributors: GotEmCoach (mcochran), KOVA, Claude Opus 4.6, Mattbusel (XFactor)
//
//! Soldier Board simulation engine.
//! Generates regulation scenarios from ingested corpus.
//! Difficulty levels: green (basic recall), amber (application), red (battlefield snap decision).

use crate::regs::{self, RegEntry};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scenario {
    /// Scenario ID
    pub id: usize,
    /// The regulation entry this scenario tests
    pub domain: String,
    pub section: String,
    /// The situation prompt presented to the soldier
    pub prompt: String,
    /// The correct answer/regulation reference
    pub answer: String,
    /// Difficulty: green, amber, red
    pub difficulty: String,
}

/// Run an interactive Soldier Board simulation.
pub fn run_board(domain: &str, count: usize, difficulty: &str) -> anyhow::Result<()> {
    let entries = regs::load_entries(domain)?;

    if entries.is_empty() {
        println!("No regulations loaded for domain '{}'.", domain);
        println!("Ingest regulations first: battle-bros ingest <path> -d {}", domain);
        println!();
        println!("Example regulation sources (all public domain):");
        println!("  - UCMJ: https://uscode.house.gov/view.xhtml?path=/prelim@title10/subtitleA/part2/chapter47");
        println!("  - AR 670-1: Army Publishing Directorate");
        println!("  - FM 7-22: Army Physical Fitness");
        println!();
        println!("Download the text, then:");
        println!("  battle-bros ingest ucmj.txt -d ucmj");
        println!("  battle-bros ingest ar670-1.txt -d ar670");
        return Ok(());
    }

    let scenarios = generate_scenarios(&entries, count, difficulty);

    println!("== SOLDIER BOARD SIMULATION ==");
    println!("Domain: {}  Scenarios: {}  Difficulty: {}", domain, scenarios.len(), difficulty);
    println!("---");

    for s in &scenarios {
        println!();
        println!("[{}] {}", s.id, s.prompt);
        println!();
        println!("  REF: {} — {}", s.section, s.domain);
        println!("  ANS: {}", s.answer);
        println!("---");
    }

    println!();
    println!("{} scenarios complete. Study your weak areas.", scenarios.len());
    Ok(())
}

/// Generate scenarios from regulation entries.
fn generate_scenarios(entries: &[RegEntry], count: usize, difficulty: &str) -> Vec<Scenario> {
    let mut scenarios = Vec::new();
    let take = count.min(entries.len());

    for (i, entry) in entries.iter().take(take).enumerate() {
        let prompt = match difficulty {
            "green" => format!(
                "What does {} state regarding: {}",
                entry.section,
                first_sentence(&entry.text)
            ),
            "amber" => format!(
                "A soldier in your unit {}. What regulation applies and what action do you take?",
                situationalize(&entry.text)
            ),
            "red" | _ => format!(
                "You're in the field. No comms. {}. What's your call? Cite the reg.",
                battlefield_scenario(&entry.text)
            ),
        };

        scenarios.push(Scenario {
            id: i + 1,
            domain: entry.domain.clone(),
            section: entry.section.clone(),
            prompt,
            answer: truncate(&entry.text, 200),
            difficulty: difficulty.to_string(),
        });
    }
    scenarios
}

/// Export scenarios as JSON for AR glass deployment.
pub fn export(output: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(output)?;
    let entries = regs::load_entries("all")?;

    for difficulty in &["green", "amber", "red"] {
        let scenarios = generate_scenarios(&entries, entries.len(), difficulty);
        let path = output.join(format!("{}.json", difficulty));
        std::fs::write(&path, serde_json::to_string_pretty(&scenarios)?)?;
        println!("export: {} scenarios → {}", scenarios.len(), path.display());
    }
    Ok(())
}

// --- helpers ---

fn first_sentence(text: &str) -> String {
    let re = regex::Regex::new(r"^[^.!?]+[.!?]").unwrap();
    re.find(text)
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| truncate(text, 100))
}

fn situationalize(text: &str) -> String {
    let re = regex::Regex::new(r"(?i)(shall|must|will not|prohibited|required)(.{0,80})").unwrap();
    if let Some(caps) = re.captures(text) {
        let action = caps.get(0).unwrap().as_str();
        format!("is in a situation where the regulation says: '{}'", truncate(action, 100))
    } else {
        format!("needs guidance on: {}", truncate(text, 80))
    }
}

fn battlefield_scenario(text: &str) -> String {
    let re = regex::Regex::new(r"(?i)(duty|responsibility|offense|punishable|authorized)(.{0,60})").unwrap();
    if let Some(caps) = re.captures(text) {
        let context = caps.get(0).unwrap().as_str();
        format!("A situation arises involving: '{}'", truncate(context, 80))
    } else {
        format!("You must decide: {}", truncate(text, 80))
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.min(s.len())])
    }
}
