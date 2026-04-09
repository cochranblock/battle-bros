# Backlog

Prioritized stack. Top = next work. Max 20 items.

1. `[feature]` Interactive stdin Q&A mode — present one scenario at a time, wait for answer, score correct/incorrect, tally at end
2. `[feature]` Randomized scenario order — shuffle entries before generating scenarios, currently sequential
3. `[feature]` Score tracking — save results to ~/.battle-bros/scores.json, show improvement over time with `battle-bros scores`
4. `[feature]` Weak area tracking — track which articles soldier gets wrong most, prioritize those in future sims
5. `[feature]` Timed mode — 30-second countdown per scenario, simulates board pressure
6. `[fix]` Amber/red scenario prompts need more context — currently too short, extract better situation phrases from regulation text
7. `[feature]` Tiny GGUF model training — wire to kova MoE pipeline, train regulation Q&A model sub-100MB for offline inference
8. `[feature]` AR glass export format — Meta Ray-Ban SDK compatible JSON, optimized for on-device display
9. `[feature]` AR 27-10 domain — Military Justice procedures, complements UCMJ articles
10. `[feature]` Manual for Courts-Martial scraper — the big reference for board prep, add MCM domain
11. `[feature]` FM 6-22 (Army Leadership) domain — leadership competencies, counseling, evaluation
12. `[feature]` STP 21-1 (Soldier's Manual) domain — common tasks every soldier must know
13. `[feature]` Cross-domain scenarios — "What reg covers this?" without domain hint, tests broader knowledge
14. `[feature]` Multi-soldier mode — track scores per soldier name, leaderboard
15. `[feature]` Split src/regs.rs, src/sims.rs into the existing empty regs/, sims/ module dirs once either exceeds 300 lines
16. `[feature]` `battle-bros review <article>` — print full regulation text for a specific article, for study mode
17. `[fix]` Scraper retry/backoff on flaky public mirrors — currently single-shot reqwest blocking
18. `[feature]` Voice input/output hook — for hands-free AR glass use, push-to-talk scenario delivery
19. `[feature]` Daily drill mode — `battle-bros daily` picks 5 scenarios biased toward weak areas, one-shot
20. `[feature]` Quiz export to Anki deck format (.apkg) — for soldiers who still want flashcards
