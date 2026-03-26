import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { motion, AnimatePresence } from "framer-motion";
import { Zap, Mic, X } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export default function MeetingWidget() {
  const [platform, setPlatform] = useState<string>("Detecting...");
  const [visible, setVisible] = useState(false);
  const [expanded, setExpanded] = useState(false);

  useEffect(() => {
    const appWindow = getCurrentWebviewWindow();
    let unlisten: any;

    const init = async () => {
      await listen<string>("meeting-detected", (event) => {
        console.log("RECEIVED:", event.payload);
        setPlatform(event.payload || "Meeting");
        setVisible(true);
      });
    };

    init();
    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const closeWidget = async () => {
    setVisible(false);
    await invoke("dismiss_meeting");
  };
  const startAssistant = async () => {
    await invoke("start_session");
    closeWidget();
  };
  return (
    /* This outer div ensures the React component actually occupies the window space */
    <div className="w-screen h-screen overflow-hidden bg-transparent flex items-end justify-right p-4">
      <AnimatePresence>
        {visible && (
          <motion.div
            initial={{ opacity: 0, scale: 0.9 }}
            animate={{ opacity: 1, scale: 1 }}
            className="w-full h-full p-3 text-white"
          >
            <div className="backdrop-blur-2xl bg-[#0b0f19]/90 border border-white/10 rounded-2xl shadow-2xl p-4">
              <div className="flex items-center justify-between mb-3">
                <div className="text-[10px] text-white/40 tracking-widest uppercase">
                  Tactical Overlay
                </div>
                <button onClick={closeWidget} className="text-white/40">
                  <X size={14} />
                </button>
              </div>

              <div className="flex items-center gap-3 mb-4">
                <div className="w-10 h-10 rounded-xl bg-blue-500/20 flex items-center justify-center">
                  <Zap className="text-blue-400" size={18} />
                </div>
                <div>
                  <div className="text-sm font-bold capitalize">
                    {platform?.split(" ")[0]}
                  </div>
                  <div className="text-[11px] text-green-400 animate-pulse">
                    Live Meeting Detected
                  </div>
                </div>
              </div>

              {!expanded ? (
                <button
                  onClick={startAssistant}
                  className="w-full py-2 rounded-lg bg-blue-600 hover:bg-blue-500 text-sm font-medium transition-all"
                >
                  Start AI Assistant
                </button>
              ) : (
                <div className="flex items-center gap-2 text-red-400 text-xs">
                  <Mic size={14} className="animate-bounce" /> Listening for
                  insights...
                </div>
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
