import { api } from "../services/apiClient";
import { extractErrorMessage } from "../utils/formatters";

export const startSession = async () => {
  try {
    const res = await api.post<{ sessionId: string }>("/sessions/init", {
      userId: "demo-user",
      platform: "Zoom",
      meetingTitle: "Backend Engineer Interview",
    });
    return res.data.sessionId;
  } catch (error) {
    alert(extractErrorMessage(error));
  }
};

export const endSession = async (sessionId: string): Promise<void> => {
  await api.post(`/sessions/${sessionId}/end`);
};

export const analyzeSession = async (
  sessionId: string,
  transcript: string,
): Promise<string> => {
  const res = await api.post<{ insight: string }>(
    `/sessions/${sessionId}/analyze`,
    {
      transcript,
    },
  );
  return res.data.insight;
};

export const pushInsight = async (
  sessionId: string,
  insight: string,
): Promise<void> => {
  await api.patch(`/sessions/${sessionId}/insights`, { insight });
};
