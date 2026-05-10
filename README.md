# bulbul-studio

> Desktop studio for Bulbul TTS. 30+ voices, 10 Indic languages, batch + SSML.

**Status:** v0.1 — ready to use.

**Sovereignty:** sovereign-by-construction. BYO endpoint, BYO key, BYO model.

This is a community project, **not affiliated with Sarvam AI**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## What this is

Bulbul v3 is the best Indic TTS available. The only way to use it today is via API calls — which works for engineers but locks out the actual buyers: content creators, podcasters, audiobook producers, e-learning teams.

bulbul-studio is a desktop app: pick voice, paste text, get audio. No code required.

## What this isn't

- No video editing
- No music composition
- No voice cloning until v1.0 (and only if upstream supports it cleanly)

See [PRD-v1.md](./PRD-v1.md) for the full anti-scope definition.

---

## Install

### From source

**Prerequisites:**
- [Node.js](https://nodejs.org/) 20+
- [Rust](https://rustup.rs/) 1.75+

```bash
git clone https://github.com/sovereign-shovels/bulbul-studio.git
cd bulbul-studio

# Install dependencies
npm install

# Build desktop app
npm run tauri build

# Or run in dev mode
npm run tauri dev
```

The built app will be in `src-tauri/target/release/bundle/`.

---

## Configure

Get a free API key from [Sarvam AI Dashboard](https://dashboard.sarvam.ai/). Then:

```bash
export BULBUL_API_KEY="your-key-here"
```

Or set it in your config file:

```toml
# ~/.config/bulbul-studio/config.toml
[provider]
endpoint = "https://api.sarvam.ai/text-to-speech"
api_key_env_var = "BULBUL_API_KEY"
language = "hi-IN"
speaker = "meera"
model = "bulbul:v3"
```

**Supported languages:** `hi-IN`, `ta-IN`, `te-IN`, `bn-IN`, `mr-IN`, `gu-IN`, `kn-IN`, `ml-IN`, `pa-IN`, `en-IN`

**Speakers:** Vary by language. Common options include `meera`, `arjun`, and others per language. Check the Sarvam dashboard for available voices.

### Environment variables

```bash
export BULBUL_STUDIO_LANGUAGE="ta-IN"
export BULBUL_STUDIO_SPEAKER="meera"
```

---

## Usage

1. Launch the app.
2. Enter text in the text area.
3. Select language and speaker.
4. Click **Generate Audio**.
5. Preview the audio in the built-in player.
6. Click **Save to File** to export as WAV.

---

## Why this exists

Indic content creators are the densest underserved AI substrate in the country. Sarvam ships APIs, not desktop tools. The gap is structural and unlikely to close from upstream.

See [PRD-v1.md](./PRD-v1.md) for the full problem statement and rationale.

## What's next

- **v0.5:** Batch mode for long docs, multi-track timeline, subtitle export
- **v1.0:** Video dubbing pipeline, voice cloning, audiobook publishing flow

See [PRD-v1.md](./PRD-v1.md) for the full roadmap.

---

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels) portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts, ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm, llm-diff, claude-bridge, claude-radio, sarvam-cast.
