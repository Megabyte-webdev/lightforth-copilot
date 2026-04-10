import { useSnapshot } from "valtio";
import { state } from "../store/state";
import { useState } from "react";

const SessionControl = () => {
  const snap = useSnapshot(state);
  const [loading, setLoading] = useState(false);

  const onStart = async () => {
    setLoading(true);
    try {
      await state.startSession();
    } catch (e) {
      console.error("Could not start session", e);
    } finally {
      setLoading(false);
    }
  };

  console.log(snap);

  const onStop = async () => {
    if (!snap.sessionId) return;
    setLoading(true);
    try {
      await state.endSession();
    } catch (e) {
      console.error("Could not end session", e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex items-center gap-3">
      {!!snap.sessionId ? (
        <button
          onClick={onStart}
          disabled={loading}
          className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white font-medium rounded-lg transition-colors"
        >
          {loading ? "Starting..." : "Start New Session"}
        </button>
      ) : (
        <button
          onClick={onStop}
          disabled={loading}
          className="px-4 py-2 bg-red-600 hover:bg-red-700 disabled:opacity-50 text-white font-medium rounded-lg transition-colors"
        >
          {loading ? "Ending..." : "Stop & End Session"}
        </button>
      )}
    </div>
  );
};

export default SessionControl;
