import { LazyStore } from "@tauri-apps/plugin-store";

const store = new LazyStore("settings.bin");

export const setStoredValue = async (key: string, value: any) => {
  await store.set(key, value);
  await store.save(); // Crucial: Writes to disk immediately
};

export const getStoredValue = async (key: string) => {
  return await store.get(key);
};
