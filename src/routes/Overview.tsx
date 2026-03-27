import { invoke } from "@tauri-apps/api/core";

export default function Overview() {
  return (
    <div className="animate-in fade-in duration-300">
      <header className="flex justify-between items-center mb-8">
        <div>
          <h1 className="text-3xl font-bold text-white">Dashboard</h1>
          <p className="text-slate-400">Welcome back to your command center.</p>
        </div>
        <button
          onClick={() => invoke("start_session")}
          className="bg-blue-600 px-6 py-2 rounded-lg hover:bg-blue-500 font-semibold shadow-lg shadow-blue-900/20 transition-colors"
        >
          Start New Session
        </button>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-slate-900 border border-slate-800 p-6 rounded-2xl">
          <p className="text-slate-400 text-sm">Overall Progress</p>
          <h3 className="text-2xl font-bold text-white">74%</h3>
        </div>
        {/* Additional cards here */}
      </div>
    </div>
  );
}
