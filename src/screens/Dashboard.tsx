"use client";
import { invoke } from "@tauri-apps/api/core";
import TitleBar from "../layout/TitleBar";

export default function Dashboard() {
  return (
    <div>
      <TitleBar />
      <div className="flex flex-col items-center justify-center h-screen bg-[#020617] text-white">
        <h1 className="text-4xl font-bold mb-8">Lightforth Copilot</h1>
        <button
          onClick={() => invoke("start_session")}
          className="bg-blue-600 px-8 py-3 rounded-xl hover:bg-blue-500 transition-all font-bold"
        >
          Start Session
        </button>
      </div>
    </div>
  );
}
