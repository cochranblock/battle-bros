# battle-bros

Tiny offline AI for military regulation training. Soldier Board simulations. Runs on AR glasses. No cloud. No excuses.

Train soldiers to make split-second regulation decisions when no one is there to hold their hand in the middle of a battlefield.

## Install

```
git clone https://github.com/cochranblock/battle-bros
cd battle-bros
cargo build --release
```

Binary lands at `target/release/battle-bros`.

## Usage

```
# Run a 10-scenario Soldier Board (default)
battle-bros

# UCMJ-only, 20 scenarios, hardest difficulty
battle-bros sim --domain ucmj --count 20 --difficulty red

# Scrape public-domain regs and auto-ingest
battle-bros scrape --ingest

# Ingest your own regulation text
battle-bros ingest path/to/reg.txt --domain ar600

# List loaded domains and scenario counts
battle-bros list

# Export scenarios as JSON for AR glass deployment
battle-bros export --output export/
```

## Domains

Currently loaded: **UCMJ** (punitive articles), **AR 670-1** (wear and appearance), **FM 7-22** (holistic health and fitness), **AR 600-20** (Army command policy). 198 entries total.

All content is public domain — US military regulations are not copyrighted.

## Subcommands

| Cmd | What it does |
|-----|--------------|
| `sim` | Rapid-fire regulation scenarios. `--domain`, `--count`, `--difficulty green\|amber\|red` |
| `ingest` | Load a regulation text file or directory into `~/.battle-bros/` |
| `scrape` | Pull regulations from public sources. `--ingest` to auto-load after |
| `list` | Show domains and entry counts |
| `export` | Dump scenarios as JSON for AR glass deployment |
| `train` | (stub) Train a sub-100MB GGUF model from the corpus |

## Why

Soldiers going to boards need to know regulations cold. Flashcards are passive. Web apps don't work in the field. battle-bros generates fresh scenarios from the actual regulation text and runs on hardware a soldier can wear.

## License

Unlicense (public domain). See `UNLICENSE`.
