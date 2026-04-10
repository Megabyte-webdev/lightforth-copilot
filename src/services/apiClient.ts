import axios, { AxiosInstance } from "axios";

type Config = {
  baseURL?: string;
  headers?: Record<string, string>;
};

export const createApiClient = (cfg: Config = {}): AxiosInstance => {
  const client = axios.create({
    baseURL: cfg.baseURL ?? "http://localhost:3000/api", // your Hono API prefix
    headers: {
      "Content-Type": "application/json",
      ...cfg.headers,
    },
    timeout: 30_000, // 30 s default
  });

  // ----- Optional auto‑refresh / token‑inject -----
  client.interceptors.request.use(
    (config) => {
      // If you decide to store a JWT in localStorage, put it here
      // const token = localStorage.getItem("jwt");
      // if (token) config.headers?.Authorization = `Bearer ${token}`;
      return config;
    },
    (err) => Promise.reject(err),
  );

  // ----- Global error handler -----
  client.interceptors.response.use(
    (res) => res,
    async (err) => {
      const { response } = err;
      // Auto log the error, show a toast, or retry
      console.error("[API error] ", response?.status, response?.data);
      // If you return the response you can surface it in the UI
      return Promise.reject(err);
    },
  );

  return client;
};

export const api = createApiClient();
