import { useEffect, useState } from "react";
import { FiX, FiMinus, FiSquare, FiCopy } from "react-icons/fi";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { isTauri } from "@tauri-apps/api/core";

export default function TitleBar() {
  const appWindow = getCurrentWindow();

  const [isMaximized, setIsMaximized] = useState(false);

  // Window maximize tracking
  useEffect(() => {
    if (!isTauri) return;
    async function init() {
      try {
        setIsMaximized(await appWindow.isMaximized());
      } catch {}
    }
    init();

    const unlisten = appWindow.onResized(async () => {
      try {
        setIsMaximized(await appWindow.isMaximized());
      } catch (err) {
        console.log("onResize Error", err);
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(() => {});
    };
  }, [appWindow]);

  const minimize = async () => {
    try {
      await appWindow.minimize();
    } catch (err) {
      console.error("Minimize failed:", err);
    }
  };

  const toggleMaximize = async () => {
    try {
      const max = await appWindow.isMaximized();
      max ? await appWindow.unmaximize() : await appWindow.maximize();
      setIsMaximized(!max);
    } catch (err) {
      console.error("Toggle maximize failed:", err);
    }
  };

  const close = async () => {
    try {
      await appWindow.close();
    } catch (err) {
      console.error("Close failed:", err);
    }
  };

  if (!isTauri) return null;

  return (
    <div
      className="fixed top-0 left-0 right-0 z-9999 flex items-center justify-between h-10 bg-black backdrop-blur-md text-white px-4 select-none group border-b border-white/5"
      data-tauri-drag-region
    >
      <h1>LightForth Copilot</h1>
      <div
        className="flex items-center px-2 gap-2 opacity-100 transition-all duration-300 "
        style={{ WebkitAppRegion: "drag" } as React.CSSProperties}
      ></div>

      {/* Hover View — Window Controls */}
      <div className="absolute right-0 flex items-center gap-0 translate-y-1 transition-all duration-200">
        <button
          onClick={minimize}
          className="p-3 hover:bg-white/10 transition-colors"
          style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}
        >
          <FiMinus size={14} />
        </button>

        <button
          onClick={toggleMaximize}
          className="p-3 hover:bg-white/10 transition-colors"
          style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}
        >
          {isMaximized ? <FiCopy size={14} /> : <FiSquare size={14} />}
        </button>

        <button
          onClick={close}
          className="p-3 hover:bg-rose-600 transition-colors rounded-bl-sm"
          style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}
        >
          <FiX size={14} />
        </button>
      </div>
    </div>
  );
}
