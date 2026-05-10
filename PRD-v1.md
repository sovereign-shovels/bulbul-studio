---
repo: bulbul-studio
rank: 2
score: 0.81
sprint: 2
substrate_anchor: Bulbul
build_estimate: "3–5 weeks for v0.1"
status: planned
---

# PRD v1.0 — bulbul-studio

> **One-liner:** Desktop studio for Bulbul TTS. 30+ voices, 10 Indic languages, batch + SSML.
>
> **Substrate:** Indian content creators, podcasters, audiobook makers, YouTubers needing Indic TTS
> **Launch channels:** Indian content-creator Twitter/X, r/IndiaTech, r/IndianGaming, LinkedIn India creator economy
> **Build estimate (v0.1):** 3–5 weeks for v0.1

---

## What problem does this solve

Bulbul v3 is the best Indic TTS available. The only way to use it today is via API calls — which works for engineers but locks out the actual buyers: content creators, podcasters, audiobook producers, e-learning teams. They need a desktop app: pick voice, paste text, get audio. bulbul-studio is that app, with batch mode and SSML for power users.

## Why this is a shovel and not a product

Indic content creators are the densest underserved AI substrate in the country. Sarvam ships APIs, not desktop tools. The gap is structural and unlikely to close from upstream. Buildable in weeks. Scope-evolves into a full audio production tool.

---

## v0.1 — what ships

Tauri/Electron desktop: pick voice + language + paste text → audio file. Batch mode for long docs. Basic SSML support. Bulbul v3 by default; pluggable adapters for Coqui-Indic and Indic-Parler as fallbacks.

### Acceptance criteria for v0.1

A v0.1 release is publishable to GitHub when ALL of these are true:

- [ ] Core functionality described above works on the primary developer machine.
- [ ] At least one local-only configuration is documented and tested (no cloud required).
- [ ] BYO endpoint / BYO key configuration is documented.
- [ ] README explains: what it is, who it's for, how to install, how to configure, what it doesn't do.
- [ ] LICENSE present (Apache 2.0 unless overridden).
- [ ] No hardcoded keys or vendor URLs anywhere.
- [ ] No telemetry / phone-home.
- [ ] At least one passing test for the main code path.
- [ ] CI green.
- [ ] AGENTS.md compliance reviewed.

## v0.5 — first major evolution

Multi-track timeline editor. Voice mixing. Subtitle export aligned to audio.

## v1.0 — fuller scope

Video dubbing pipeline. Voice cloning support if Sarvam exposes it. Audiobook publishing flow.

---

## Architecture sketch

### Stack

Tauri preferred (smaller binaries, better cross-platform), TS frontend, Rust backend for audio encoding. ffmpeg bundled for output formats.

### Provider abstraction

The shovel MUST expose a provider abstraction even if v0.1 only uses one
provider. Suggested shape:

```
interface Provider {
  name: string;
  endpoint: URL;
  apiKeyEnvVar: string;
  call(input: ProviderInput): Promise<ProviderOutput>;
}
```

The default config in v0.1 must point to a free, local provider where
applicable, and document how to swap in any other.

### Configuration

Configuration order of precedence (highest to lowest):

1. Command-line flags
2. Environment variables (prefix: `BULBUL_STUDIO_*`)
3. User config file (`~/.config/bulbul-studio/config.toml` on Linux/Mac, equivalent on Windows)
4. Default config (shipped, but never with secrets)

---

## Anti-scope (do NOT build)

No video editing. No music composition. No voice cloning until v1.0 (and only if upstream supports it cleanly).

---

## Tombstone risk and mitigation

**Risk:** Sarvam shipping their own desktop studio. Probability low — they ship APIs and dashboards, not desktop apps.

**Mitigation:** Ship fast (v0.1 in 3–5 weeks for v0.1). Build community early
(launch on Indian content-creator Twitter/X, r/IndiaTech, r/IndianGaming, LinkedIn India creator economy). Even if upstream absorbs the feature, accumulated
stars and the community are the audience-build payoff.

**Kill signal:** ElevenLabs or another global TTS provider matching Bulbul's Indic quality at competitive pricing.

If the kill signal triggers, the maintainer must announce within one week and
either (a) refocus on a remaining gap, (b) merge gracefully into upstream if
they're receptive, or (c) mark the repo as archived with a clear pointer to the
replacement.

---

## Launch plan

### Pre-launch checklist

- [ ] Repo on GitHub at `github.com/sovereign-shovels/bulbul-studio`
- [ ] README polished (see template in `_templates/`)
- [ ] At least 3 issues / discussions seeded (real ones, not placeholder)
- [ ] LICENSE, CODE_OF_CONDUCT, CONTRIBUTING present
- [ ] Demo asset (gif, screenshot, or short video — depending on category)
- [ ] First-launch post drafted for primary launch channel

### Day-1 launch

Post to: Indian content-creator Twitter/X, r/IndiaTech, r/IndianGaming, LinkedIn India creator economy

Subject template (adjust per channel):
- Show HN: `Show HN: bulbul-studio – Desktop studio for Bulbul TTS. 30+ voices, 10 Indic languages, batch + SSML.`
- Reddit: `[OSS] Desktop studio for Bulbul TTS. 30+ voices, 10 Indic languages, batch + SSML.` with full post explaining the gap and the build
- Twitter/X: thread leading with the demo gif

### Week-1 follow-up

- Respond to every issue and comment within 24h.
- Ship at least one bugfix release based on launch feedback.
- Cross-post to secondary channels.

### Month-1 review

- Assess star velocity and community formation.
- If kill signal triggered, follow tombstone protocol above.
- If trajectory is healthy, plan v0.5.

---

## Cross-references

- Constitution: [[AGENTS]]
- Public README: [[README]]
- Progress frontmatter: [[progress]]
- Internal knowledge graph: [[knowledge-graph]]
