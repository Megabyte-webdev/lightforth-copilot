import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { LogicalSize } from "@tauri-apps/api/dpi";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import {
  ChevronDown,
  Eye,
  GripVertical,
  X,
  Zap,
  SendHorizontal,
  EyeOff,
} from "lucide-react";
import { useSnapshot } from "valtio";
import { state } from "../store/state";

export default function Widget() {
  const snap = useSnapshot(state);
  const appWindow = getCurrentWebviewWindow();

  // Local state for the text input
  const [input, setInput] = useState("");

  const toggleExpand = async () => {
    if (snap.isExpanded) {
      await appWindow.setSize(new LogicalSize(500, 120));
      state.isExpanded = false;
    } else {
      await appWindow.setSize(new LogicalSize(500, 300)); // Increased height for response room
      state.isExpanded = true;
    }
  };

  const handleSendMessage = async () => {
    if (!input.trim()) return;

    state.aiStatus = "processing";
    const userQuery = input;
    setInput("");

    try {
      // Call your new Rust command
      const response = await invoke("kalosm_ask_ai", {
        prompt: userQuery,
      });

      state.aiResponse = response as string;
    } catch (error) {
      console.error(error);

      state.aiResponse =
        typeof error === "string" ? error : "SYSTEM ERROR: CHECK CONSOLE";
    } finally {
      state.aiStatus = "idle";
    }
  };

  return (
    <div className="flex flex-col items-center h-screen w-screen bg-transparent select-none font-sans overflow-hidden p-4">
      <div className="flex items-center gap-3 mb-2">
        <div
          className={`backdrop-blur-2xl transition-all duration-500 rounded-full px-4 py-2 flex items-center gap-3 shadow-[0_10px_40px_rgba(0,0,0,0.7)] bg-[#111827]/95 border ${
            snap.isStealth ? "border-dashed border-white/40" : "border-white/20"
          }`}
        >
          <button
            onClick={() => state.toggleStealth()}
            className="transition-colors px-1 text-white/80 hover:text-white"
          >
            {snap.isStealth ? <EyeOff size={20} /> : <Eye size={20} />}
          </button>

          <button
            onClick={() => state.toggleListening()}
            className={`px-6 py-1.5 rounded-full text-sm font-semibold tracking-tight transition-all duration-300 ${
              snap.isListening
                ? "bg-red-500/20 text-red-400 border border-red-500/30"
                : "bg-[#1d4ed8] hover:bg-[#2563eb] text-white"
            }`}
          >
            {snap.isListening ? (
              <span className="flex items-center gap-2">
                <span className="w-1.5 h-1.5 bg-red-500 rounded-full animate-pulse" />
                Listening...
              </span>
            ) : (
              "Start Listening"
            )}
          </button>

          <button
            onClick={toggleExpand}
            className="transition-transform duration-300 text-white/60"
            style={{
              transform: snap.isExpanded ? "rotate(180deg)" : "rotate(0deg)",
            }}
          >
            <ChevronDown size={20} />
          </button>

          <div className="w-px h-5 mx-1 bg-white/10" />

          <div
            data-tauri-drag-region
            className="cursor-grab active:cursor-grabbing p-1"
          >
            <GripVertical
              size={18}
              className="text-white/30"
              data-tauri-drag-region
            />
          </div>
        </div>

        <button
          onClick={() => invoke("end_session")}
          className={`backdrop-blur-2xl border rounded-full p-2.5 transition-all shadow-xl bg-[#111827]/95 text-white/60 hover:text-red-400 ${
            snap.isStealth ? "border-dashed border-white/30" : "border-white/20"
          }`}
        >
          <X size={20} />
        </button>
      </div>

      {snap.isExpanded && (
        <div
          className={`w-125 backdrop-blur-2xl p-4 shadow-[0_20px_60px_rgba(0,0,0,0.8)] animate-in fade-in zoom-in duration-200 rounded-2xl bg-[#111827]/95 border ${
            snap.isStealth
              ? "border-2 border-dashed border-white/10"
              : "border border-white/10"
          }`}
        >
          {/* AI Response Area */}
          <div className="mb-4 p-3 rounded-xl min-h-15 max-h-40 overflow-y-auto font-mono text-[11px] transition-all text-white/70 bg-white/5 border border-white/5">
            {snap.aiStatus === "processing" ? (
              <div className="flex items-center gap-2 text-blue-400 animate-pulse">
                <span className="w-2 h-2 bg-blue-400 rounded-full animate-ping" />
                UPLINKING TO COMMAND CENTER...
              </div>
            ) : (
              <div className="whitespace-pre-wrap">
                <span className="opacity-40 mr-2">{">"}</span>
                {snap.aiResponse || "SYSTEM IDLE. AWAITING INPUT..."}
              </div>
            )}
          </div>

          <div className="relative mb-4">
            <input
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSendMessage()}
              placeholder="Ask about screen..."
              className="w-full bg-transparent border border-white/10 rounded-xl p-4 text-[15px] outline-none transition-all text-white placeholder:text-white/30 focus:border-blue-500/50"
            />
          </div>

          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <button
                onClick={() => state.toggleSmartMode()} // Assuming you add this to state
                className={`flex items-center gap-2 border px-3 py-1.5 rounded-full text-[13px] transition-all ${
                  snap.isSmartMode
                    ? "bg-blue-500/20 border-blue-500/40 text-blue-400"
                    : "bg-white/5 border-white/10 text-white/60 hover:bg-white/10"
                }`}
              >
                <Zap
                  size={14}
                  fill={snap.isSmartMode ? "currentColor" : "none"}
                />{" "}
                Smart Mode
              </button>
            </div>

            <button
              onClick={handleSendMessage}
              disabled={snap.aiStatus === "processing"}
              className="p-2.5 rounded-full text-white shadow-lg transition-transform hover:scale-105 bg-[#1d4ed8] disabled:opacity-50 disabled:scale-100"
            >
              <SendHorizontal size={18} />
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
