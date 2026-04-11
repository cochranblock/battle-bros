<!-- Unlicense — battle-bros -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why. Proves human-piloted AI development — not generated spaghetti.*

> Every entry below maps to real commits. Run `git log --oneline` to verify.

## How to Read This Document

Each entry follows this format:

- **Date**: When the work shipped (not when it was started)
- **What**: Concrete deliverable — binary, feature, fix, architecture change
- **Why**: Business or technical reason driving the decision
- **Commit**: Short hash(es) for traceability
- **AI Role**: What the AI did vs. what the human directed

This document exists because AI-assisted code has a trust problem. Anyone can generate 10,000 lines of spaghetti. This timeline proves that a human pilot directed every decision, verified every output, and shipped working software.

---

## Entries

<!-- Add entries in reverse chronological order. Template:

### YYYY-MM-DD — [Short Title]

**What:** [Concrete deliverable]
**Why:** [Business/technical driver]
**Commit:** `abc1234`
**AI Role:** [What AI generated vs. what human directed/verified]
**Proof:** [Link to artifact, screenshot, or test output]

-->

### 2026-04-09 — Initial Scaffolding

**What:** Single-binary CLI with six subcommands (`sim`, `ingest`, `train`, `list`, `export`, `scrape`). Regulation corpus with 198 entries across four domains (UCMJ, AR 670-1, FM 7-22, AR 600-20). Scenario generation engine producing green/amber/red difficulty Soldier Board drills. Web scraper for public-domain military regulations. JSON export for AR glass deployment.
**Why:** Soldiers going to boards need regulation drills that work offline — in the field, on AR glasses, with no network. Flashcards are passive; battle-bros generates scenarios from real regulation text.
**Commit:** `df36bc1`, `0ce957c`, `00df70e`, `d0f9acb`, `13f71d5`, `55ac986`
**AI Role:** AI generated regulation parsers, scenario templates, and scraper logic. Human directed the product concept (board simulation trainer for AR glasses), chose the regulation domains, defined the CLI surface, verified all scraped content against source regulations, and tested board sim output.
**Proof:** `cargo check` clean. `battle-bros list` shows 198 entries. `battle-bros sim --domain ucmj` generates valid scenarios.
