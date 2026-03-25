import { proxy } from "valtio";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getStoredValue, setStoredValue } from "../lib/storage";

// 1. Define the Tactical State
export const state = proxy({
  isStealth: false,
  isListening: false,
  isExpanded: false,
  isSmartMode: false,
  aiResponse: "",
  aiStatus: "idle", // "idle" | "processing" | "transcribing" | "error"

  // Actions
  async init() {
    const savedStealth = await getStoredValue("stealth_enabled");
    const savedSmart = await getStoredValue("smart_mode_enabled");

    if (savedStealth !== null) {
      state.isStealth = Boolean(savedStealth);
      await invoke("stealth_mode", { mode: state.isStealth });
    }

    if (savedSmart !== null) {
      state.isSmartMode = Boolean(savedSmart);
    }
  },

  async toggleStealth() {
    state.isStealth = !state.isStealth;
    await invoke("stealth_mode", { mode: state.isStealth });
    await setStoredValue("stealth_enabled", state.isStealth);
  },

  async toggleSmartMode() {
    state.isSmartMode = !state.isSmartMode;
    await setStoredValue("smart_mode_enabled", state.isSmartMode);
  },
  async toggleListening() {
    // Optimistic UI update
    const nextState = !state.isListening;
    state.isListening = nextState;

    if (nextState) {
      state.aiResponse = "";
      state.aiStatus = "transcribing";
      await invoke("start_audio");
    } else {
      state.aiStatus = "processing";
      await invoke("stop_audio");
    }
  },
});

listen<string>("ai_response_chunk", (event) => {
  // Replace the current text with the full AI response
  state.aiResponse = event.payload;
});

listen<string>("ai_status", (event) => {
  state.aiStatus = event.payload;
});

//Auto-Hydrate on import
state.init();
