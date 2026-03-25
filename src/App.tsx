import { useState, useEffect } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import Dashboard from "./screens/Dashboard";
import Widget from "./screens/Widget";
import "./App.css";

function App() {
  const [windowLabel, setWindowLabel] = useState<string | null>(null);

  useEffect(() => {
    // Get the label defined in tauri.conf.json (e.g., "main" or "widget")
    const label = getCurrentWebviewWindow().label;
    setWindowLabel(label);
  }, []);

  // Prevent flicker while the app identifies the window
  if (windowLabel === null) return <div className="bg-transparent" />;

  // Render the correct screen based on the window identity
  return <>{windowLabel === "widget" ? <Widget /> : <Dashboard />}</>;
}

export default App;
