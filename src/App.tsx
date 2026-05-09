import { useEffect, useState } from "react";
import { initI18n, SupportedLang } from "./i18n";
import { useAppStore } from "./store/useAppStore";
import Popup from "./components/Popup";
import Settings from "./components/Settings";
import { listen } from "@tauri-apps/api/event";

function applyTheme(theme: string) {
  const root = document.documentElement;
  if (theme === "dark") {
    root.classList.add("dark");
  } else if (theme === "light") {
    root.classList.remove("dark");
  } else {
    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    if (prefersDark) {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  }
}

function App() {
  const [ready, setReady] = useState(false);
  const { loadSettings, settings } = useAppStore();

  const hash = window.location.hash.replace("#/", "") || "popup";

  useEffect(() => {
    async function boot() {
      await loadSettings();
    }
    boot();
  }, [loadSettings]);

  useEffect(() => {
    if (!settings) return;
    const lang = (settings.language || "en") as SupportedLang;
    initI18n(lang).then(() => setReady(true));
  }, [settings]);

  useEffect(() => {
    if (!settings) return;
    applyTheme(settings.theme);
  }, [settings?.theme]);

  useEffect(() => {
    const unlisten = listen("file-organized", (event) => {
      console.log("File organized:", event.payload);
      useAppStore.getState().loadLogs();
      useAppStore.getState().loadStats();
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  if (!ready) {
    return (
      <div className="flex h-full items-center justify-center bg-surface text-text">
        <div className="animate-pulse text-sm">Mouzi...</div>
      </div>
    );
  }

  return (
    <div className="h-full w-full">
      {hash === "settings" ? <Settings /> : <Popup />}
    </div>
  );
}

export default App;
