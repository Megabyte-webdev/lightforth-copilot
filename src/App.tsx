import { useState, useEffect } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import Dashboard from "./screens/Dashboard";
import Widget from "./screens/Widget";
import MeetingWidget from "./screens/MeetingWidget";
import "./App.css";

function App() {
  const [windowLabel, setWindowLabel] = useState<string | null>(null);

  useEffect(() => {
    // Get the label defined in tauri.conf.json
    const label = getCurrentWebviewWindow().label;
    setWindowLabel(label);
  }, []);

  // Prevent white flicker during identification
  if (windowLabel === null) return <div className="bg-transparent" />;

  return (
    <>
      {windowLabel === "main" && <Dashboard />}
      {windowLabel === "widget" && <Widget />}
      {windowLabel === "meetingWidget" && <MeetingWidget />}
      {/* Add a fallback to see if an unknown label is the culprit */}
      {!["main", "widget", "meetingWidget"].includes(windowLabel) && (
        <div className="bg-transparent text-red-500">
          Unknown Window: {windowLabel}
        </div>
      )}
    </>
  );
}

export default App;
