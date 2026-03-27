import {
  Activity,
  LayoutDashboard,
  FileText,
  ChevronLeft,
  ChevronRight,
} from "lucide-react";
import { useState, useEffect } from "react";

export type ViewType = "overview" | "season" | "sheets";

interface SidebarProps {
  activeView: ViewType;
  onViewChange: (view: ViewType) => void;
}

export default function Sidebar({ activeView, onViewChange }: SidebarProps) {
  const [isCollapsed, setIsCollapsed] = useState(false);

  useEffect(() => {
    const handleResize = () => {
      if (window.innerWidth < 768) {
        setIsCollapsed(true);
      } else {
        setIsCollapsed(false);
      }
    };

    handleResize();

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  const menuItems = [
    { id: "overview" as const, label: "Dashboard", icon: LayoutDashboard },
    { id: "season" as const, label: "Season Summary", icon: Activity },
    { id: "sheets" as const, label: "Current Sheet", icon: FileText },
  ];

  return (
    <aside
      className={`relative border-r border-slate-800 flex flex-col transition-all duration-300 ease-in-out bg-[#020617] z-20 ${
        isCollapsed ? "w-20" : "w-64"
      }`}
    >
      <button
        onClick={() => setIsCollapsed(!isCollapsed)}
        className="absolute -right-3 top-12 z-30 flex h-6 w-6 items-center justify-center rounded-full border border-slate-800 bg-[#020617] text-slate-400 hover:text-white hover:bg-slate-800 transition-all shadow-md group"
      >
        {isCollapsed ? (
          <ChevronRight size={14} className="group-hover:scale-110" />
        ) : (
          <ChevronLeft size={14} className="group-hover:scale-110" />
        )}
      </button>

      <div
        className={`p-6 flex items-center ${isCollapsed ? "justify-center" : "justify-between"}`}
      >
        {!isCollapsed && (
          <h2 className="text-xl font-bold text-blue-400 tracking-tight animate-in fade-in zoom-in-95 duration-300">
            Copilot
          </h2>
        )}
        {isCollapsed && (
          <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center animate-in fade-in zoom-in-95">
            <span className="font-black text-white text-xs">LC</span>
          </div>
        )}
      </div>

      <nav className="flex flex-col gap-1.5 px-3">
        {menuItems.map((item) => (
          <button
            key={item.id}
            onClick={() => onViewChange(item.id)}
            title={isCollapsed ? item.label : ""}
            className={`flex items-center rounded-lg transition-all duration-200 group ${
              isCollapsed ? "justify-center p-2.5" : "gap-3 p-2.5"
            } ${
              activeView === item.id
                ? "bg-blue-600/10 text-blue-400"
                : "text-slate-400 hover:bg-slate-800/40 hover:text-slate-200"
            }`}
          >
            <item.icon
              size={24}
              className={`shrink-0 transition-transform duration-300 ${
                activeView === item.id
                  ? "text-blue-400"
                  : "group-hover:scale-110"
              }`}
            />
            {!isCollapsed && (
              <span className="font-medium text-sm whitespace-nowrap overflow-hidden animate-in fade-in slide-in-from-left-4 duration-300">
                {item.label}
              </span>
            )}
          </button>
        ))}
      </nav>

      <div className="mt-auto p-4 border-t border-slate-800/50">
        <div
          className={`flex items-center gap-3 ${isCollapsed ? "justify-center" : ""}`}
        >
          <div className="w-8 h-8 shrink-0 rounded-full bg-linear-to-br from-blue-500 to-indigo-600 flex items-center justify-center text-[10px] font-bold text-white shadow-inner">
            AM
          </div>
          {!isCollapsed && (
            <div className="flex flex-col min-w-0 animate-in fade-in slide-in-from-left-4">
              <span className="text-xs font-semibold text-slate-200 truncate">
                Afolabi Mubarak
              </span>
              <span className="text-[10px] text-slate-500 uppercase tracking-wider">
                Software Engineer
              </span>
            </div>
          )}
        </div>
      </div>
    </aside>
  );
}
