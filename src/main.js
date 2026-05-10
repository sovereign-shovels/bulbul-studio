import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';

const textInput = document.getElementById('text-input');
const languageSelect = document.getElementById('language');
const speakerInput = document.getElementById('speaker');
const generateBtn = document.getElementById('generate-btn');
const saveBtn = document.getElementById('save-btn');
const status = document.getElementById('status');
const player = document.getElementById('player');

let currentAudio = null;

async function loadConfig() {
  try {
    const cfg = await invoke('get_config');
    if (cfg.language) languageSelect.value = cfg.language;
    if (cfg.speaker) speakerInput.value = cfg.speaker;
  } catch (e) {
    console.error('Failed to load config:', e);
  }
}

generateBtn.addEventListener('click', async () => {
  const text = textInput.value.trim();
  if (!text) {
    status.textContent = 'Please enter some text.';
    return;
  }

  generateBtn.disabled = true;
  saveBtn.disabled = true;
  status.textContent = 'Generating audio...';
  player.style.display = 'none';

  try {
    const result = await invoke('generate_speech', {
      text,
      language: languageSelect.value,
      speaker: speakerInput.value,
    });

    currentAudio = new Uint8Array(result.audio_bytes);
    const blob = new Blob([currentAudio], { type: 'audio/wav' });
    const url = URL.createObjectURL(blob);

    player.src = url;
    player.style.display = 'block';
    status.textContent = 'Audio generated. Press play to preview.';
    saveBtn.disabled = false;
  } catch (e) {
    status.textContent = `Error: ${e}`;
  } finally {
    generateBtn.disabled = false;
  }
});

saveBtn.addEventListener('click', async () => {
  if (!currentAudio) return;

  try {
    const path = await save({
      defaultPath: 'bulbul-output.wav',
      filters: [{ name: 'WAV', extensions: ['wav'] }],
    });

    if (path) {
      await invoke('save_audio', { path, audio_bytes: Array.from(currentAudio) });
      status.textContent = `Saved to ${path}`;
    }
  } catch (e) {
    status.textContent = `Save error: ${e}`;
  }
});

loadConfig();
