# Backlog

Prioritized stack. Top = next work. Max 20 items.

1. `[feature]` Randomized scenario order — shuffle entries before generating scenarios, currently sequential
2. `[feature]` Interactive stdin Q&A mode — present one scenario at a time, wait for answer, score correct/incorrect, tally at end
3. `[feature]` Score tracking — save results to ~/.battle-bros/scores.json, show improvement over time with `battle-bros scores`
4. `[feature]` AR 27-10 domain — Military Justice procedures, complements UCMJ articles
5. `[feature]` Manual for Courts-Martial scraper — the big reference for board prep, add MCM domain
6. `[feature]` Tiny GGUF model training — wire to kova MoE pipeline, train regulation Q&A model sub-100MB for offline inference
7. `[feature]` AR glass export format — Meta Ray-Ban SDK compatible JSON, optimized for on-device display
8. `[feature]` Timed mode — 30-second countdown per scenario, simulates board pressure
9. `[feature]` Weak area tracking — track which articles soldier gets wrong most, prioritize those in future sims
10. `[feature]` Multi-soldier mode — track scores per soldier name, leaderboard
11. `[fix]` Amber/red scenario prompts need more context — currently too short, extract better situation phrases from regulation text
12. `[feature]` Add regex to illbethejudgeofthat Cargo.toml for legal doc parsing
13. `[feature]` Cross-domain scenarios — "What reg covers this?" without domain hint, tests broader knowledge
14. `[feature]` FM 6-22 (Army Leadership) domain — leadership competencies, counseling, evaluation
15. `[feature]` STP 21-1 (Soldier's Manual) domain — common tasks every soldier must know
