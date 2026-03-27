import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { motion, AnimatePresence } from "framer-motion";
import { X, Sparkles, Activity, Command } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

export default function MeetingWidget() {
  const [platform, setPlatform] = useState<string>("Detecting...");
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    let unlistenDetect: any;
    let unlistenEnd: any;

    const init = async () => {
      unlistenDetect = await listen<any>("meeting-detected", (event) => {
        const payload = event.payload;
        const keys = Object.keys(payload);
        const rawTitle = payload[keys[0]];
        const cleanTitle = rawTitle
          .replace(/ - Google Chrome$/i, "")
          .replace(/ - Microsoft Edge$/i, "")
          .split(" - ")[0];

        setPlatform(cleanTitle);
        setVisible(true);
      });

      unlistenEnd = await listen("meeting-ended", () => {
        setVisible(false);
      });
    };

    init();
    return () => {
      if (unlistenDetect) unlistenDetect();
      if (unlistenEnd) unlistenEnd();
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
    <div className="w-full h-full flex items-center justify-center bg-transparent select-none font-sans overflow-hidden">
      <AnimatePresence>
        {visible && (
          <motion.div
            initial={{ y: 40, opacity: 0, scale: 0.9 }}
            animate={{ y: 0, opacity: 1, scale: 1 }}
            exit={{ y: 20, opacity: 0, scale: 0.95 }}
            transition={{ type: "spring", stiffness: 300, damping: 25 }}
            className="relative w-full group"
          >
            {/* Main Container */}
            <div className="relative bg-[#0a0a0b]/90 backdrop-blur-2xl border border-white/10 rounded-[22px] shadow-[0_20px_40px_rgba(0,0,0,0.4)] overflow-hidden">
              {/* Header Bar */}
              <div className="flex items-center justify-between px-4 py-2 bg-white/[0.02] border-b border-white/[0.05]">
                <div className="flex items-center gap-1.5">
                  <div className="w-1.5 h-1.5 rounded-full bg-blue-500 shadow-[0_0_8px_rgba(59,130,246,0.6)]" />
                  <span className="text-[9px] font-bold text-white/40 uppercase tracking-widest">
                    Lightforth Copilot
                  </span>
                </div>
                <button
                  onClick={closeWidget}
                  className="p-1 hover:bg-white/10 rounded-md transition-colors text-white/20 hover:text-red-500"
                >
                  <X size={12} />
                </button>
              </div>

              {/* Body */}
              <div className="p-4">
                <div className="flex items-center gap-3 mb-4">
                  <div className="relative flex-shrink-0">
                    <div className="w-11 h-11 rounded-2xl bg-[#161618] border border-white/5 flex items-center justify-center shadow-inner group-hover:border-blue-500/30 transition-colors">
                      <Activity size={20} className="text-blue-400" />
                    </div>
                    <motion.div
                      animate={{ scale: [1, 1.2, 1] }}
                      transition={{ duration: 2, repeat: Infinity }}
                      className="absolute -top-1 -right-1 w-3 h-3 bg-green-500 rounded-full border-2 border-[#0a0a0b]"
                    />
                  </div>

                  <div className="min-w-0">
                    <p className="text-[10px] font-medium text-white/40 leading-none">
                      Meeting Detected
                    </p>
                    <h3 className="text-lg font-semibold text-white truncate tracking-tight">
                      {platform}
                    </h3>
                  </div>
                </div>

                {/* The "Action Bar" Button */}
                <button
                  onClick={startAssistant}
                  className="relative w-full group/btn flex items-center justify-between px-4 py-2.5 bg-white rounded-xl transition-all duration-300 hover:shadow-[0_0_20px_rgba(255,255,255,0.1)] active:scale-[0.97]"
                >
                  <div className="flex items-center gap-2">
                    <Sparkles
                      size={14}
                      className="text-black transition-transform group-hover/btn:rotate-12"
                    />
                    <span className="text-[11px] font-bold text-black uppercase tracking-wide">
                      Start Copilot
                    </span>
                  </div>

                  {/* Subtle Keyboard Shortcut Hint */}
                  <div className="flex items-center gap-0.5 opacity-30 group-hover/btn:opacity-60 transition-opacity">
                    <Command size={10} className="text-black" />
                    <span className="text-[9px] font-bold text-black">S</span>
                  </div>
                </button>
              </div>

              {/* Decorative Bottom Edge */}
              <div className="h-1 w-full bg-gradient-to-r from-transparent via-blue-500/20 to-transparent" />
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
