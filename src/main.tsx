import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { invoke } from "@tauri-apps/api/core";

// Ensure Tauri is available globally for console access
if (typeof window !== 'undefined') {
  (window as any).__invoke__ = invoke;
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
