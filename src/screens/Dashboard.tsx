import { JSX, useState } from "react";
import TitleBar from "../layout/TitleBar";
import Sidebar, { ViewType } from "../layout/dashboard/Sidebar";
import Overview from "../routes/Overview";
import SeasonSummary from "../routes/SeasonSummary";
import Sheets from "../routes/Sheets";

export default function Dashboard() {
  const [activeView, setActiveView] = useState<ViewType>("overview");

  const views: Record<ViewType, JSX.Element> = {
    overview: <Overview />,
    season: <SeasonSummary />,
    sheets: <Sheets />,
  };

  return (
    <div className="pt-5 flex flex-col h-screen bg-[#020617] text-slate-200 overflow-hidden select-none">
      <TitleBar />

      <div className="flex flex-1 overflow-hidden relative">
        <Sidebar activeView={activeView} onViewChange={setActiveView} />

        <main className="flex-1 overflow-y-auto relative custom-scrollbar">
          {/* Responsive Container:
             - p-4 on small windows
             - p-8 on medium
             - p-12 on large/ultra-wide
             - max-w-7xl keeps the content from stretching too far on 4K monitors
          */}
          <div className="p-4 md:p-8 lg:p-12 max-w-7xl mx-auto w-full transition-all duration-300">
            {views[activeView]}
          </div>
        </main>
      </div>
    </div>
  );
}
