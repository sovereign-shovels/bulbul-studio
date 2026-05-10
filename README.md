# bulbul-studio

> Desktop studio for Bulbul TTS. 30+ voices, 10 Indic languages, batch + SSML.

**Status:** v0.1 — planning. Not yet released.

**Sovereignty:** sovereign-by-construction. BYO endpoint, BYO key, BYO model.
A local-only configuration is documented and tested.

This is a community project, **not affiliated with Bulbul**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## What this is

Desktop studio for Bulbul TTS. 30+ voices, 10 Indic languages, batch + SSML.

## What this isn't

Not a video editor. Not a music composer. Not a voice cloner (unless Sarvam ships clean cloning APIs in v1.0).

## Install

> Coming with v0.1 release.

## Configure

You bring the model. By default `bulbul-studio` tries to use a local provider:

- For LLM endpoints: Ollama at `http://localhost:11434`
- For voice endpoints: configurable, see [docs/configure.md]

To use any other provider (Claude, GPT, Hermes, OpenRouter, Sarvam, etc.):

```toml
# ~/.config/bulbul-studio/config.toml
[provider]
endpoint = "https://api.your-provider.com/v1"
api_key_env = "YOUR_PROVIDER_KEY"
model = "your-model-name"
```

Anthropic, OpenAI, and Sarvam endpoints all work. Local Ollama, llama.cpp,
LM Studio, and vLLM all work via their OpenAI-compatible endpoints.

## Why this exists

Bulbul v3 is the best Indic TTS available. The only way to use it today is via API calls — which works for engineers but locks out the actual buyers: content creators, podcasters, audiobook producers, e-learning teams. They need a desktop app: pick voice, paste text, get audio. bulbul-studio is that app, with batch mode and SSML for power users.

## What's next

See [PRD-v1.md](./PRD-v1.md) for the full v0.1 → v0.5 → v1.0 plan.

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels)
portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts,
ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm,
llm-diff, claude-bridge, claude-radio, sarvam-cast.
