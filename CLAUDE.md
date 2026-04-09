# battle-bros

Tiny offline AI for military regulation training. Soldier Board simulations that run on AR glasses — no cloud, no network, no excuses.

Train soldiers to make split-second regulation decisions when no one is there to hold their hand.

## What it is

A single Rust binary that:
- Scrapes public-domain military regulations (UCMJ, AR 670-1, FM 7-22, AR 600-20) into plain text.
- Ingests them into a local corpus at `~/.battle-bros/`, splitting into `RegEntry` records with extracted keywords.
- Generates rapid-fire Soldier Board scenarios from the corpus at three difficulty tiers (green/amber/red).
- Exports scenarios as JSON for on-device AR glass display.
- (Planned) Trains a sub-100MB GGUF model for fully offline Q&A inference.

All military regulations are public domain — no licensing issues with ingesting them.

## Layout

| Path | Purpose |
|------|---------|
| `src/main.rs` | `clap` CLI: `sim`, `ingest`, `train`, `list`, `export`, `scrape` |
| `src/regs.rs` | `RegEntry` type, ingest pipeline, corpus storage |
| `src/sims.rs` | Soldier Board scenario generator, export |
| `src/scraper.rs` | Public-source regulation scraper (reqwest blocking) |
| `src/regs/`, `src/sims/`, `src/models/` | Empty — reserved for future split |
| `docs/` | Seed regulation text: `ucmj.txt`, `ar670_scraped.txt`, `fm7_22.txt`, `ar600_20.txt` |

Corpus lives at `~/.battle-bros/` (created on first ingest). Currently holds 198 regulation entries.

## Build

```
cargo check
cargo build --release
cargo run -- sim --domain ucmj --count 10 --difficulty green
cargo run -- list
cargo run -- scrape --ingest
```

No workspace — this is a standalone single-binary crate. Edition 2024.

## Current state

- CLI wired end-to-end for `sim`, `ingest`, `list`, `export`, `scrape`.
- 198 entries ingested (UCMJ punitive articles, AR 670-1, FM 7-22, AR 600-20).
- Scenario generation works; board sims run.
- `train` subcommand is a stub — GGUF training not yet wired to kova MoE pipeline.
- Scenarios are sequential, not shuffled. No interactive Q&A yet. No score tracking.

## Design rules

- Offline-first. No runtime network calls outside `scrape`.
- Single binary. No external DBs — JSON on disk under `~/.battle-bros/`.
- Public-domain content only.
- Terminology: "Soldier Board", "scenarios", "domains" (ucmj, ar670, fm, ar600).
