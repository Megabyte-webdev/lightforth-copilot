// src/store/state.ts  (the file you posted)
import { proxy } from "valtio";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getStoredValue, setStoredValue } from "../lib/storage";
import { startSession } from "../hooks/session";

export const state = proxy({
  /* ---- Session plumbing --------------------------------------- */
  sessionId: null as string | null, // ← ← **our real ID**
  isStealth: false,
  isListening: false,
  isExpanded: false,
  isSmartMode: false,
  aiResponse: "",
  aiStatus: "idle",

  /* ---- Actions ------------------------------------------------- */
  async init() {
    const savedStealth = await getStoredValue("stealth_enabled");
    const savedSmart = await getStoredValue("smart_mode_enabled");
    const savedSession = await getStoredValue("session_id");

    if (savedStealth !== null) {
      state.isStealth = Boolean(savedStealth);
      await invoke("stealth_mode", { mode: state.isStealth });
    }

    if (savedSmart !== null) state.isSmartMode = Boolean(savedSmart);

    // 🛡️ Strict Check: Prevent "undefined" or null-strings from breaking the UI
    if (
      savedSession &&
      savedSession !== "undefined" &&
      savedSession !== "null"
    ) {
      state.sessionId = String(savedSession);
    } else {
      state.sessionId = null;
      // Clean up the store if it contains junk
      if (savedSession === "undefined") {
        await setStoredValue("session_id", null);
      }
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

  async startSession() {
    try {
      const id = await startSession();

      // Ensure 'id' is a valid string and not an empty result
      if (id && id !== "undefined") {
        state.sessionId = String(id);
        await setStoredValue("session_id", id);

        await invoke("start_session");
        state.isListening = true;
      } else {
        console.error("StartSession hook returned an invalid ID:", id);
      }
    } catch (error) {
      console.error("Failed to start session:", error);
    }
  },
  async endSession() {
    if (!state.sessionId) return;
    await invoke("end_session");
    state.sessionId = null;
    await setStoredValue("session_id", null);
    state.isListening = false;
  },

  async analyze(sessionId: string, transcript: string) {
    const tip: string = await invoke("analyze_session", {
      session_id: sessionId,
      transcript,
    });
    // push it back to the DB (optional)
    await invoke("push_insight", { session_id: sessionId, text: tip });
    return tip;
  },
});

listen<string>("ai_response_chunk", (e) => {
  state.aiResponse = e.payload;
});

listen<string>("ai_status", (e) => {
  state.aiStatus = e.payload;
});

state.init(); // auto‑hydrate on import
