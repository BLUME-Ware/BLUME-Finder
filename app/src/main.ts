import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";
import { locale } from "./lib/i18n";

document.documentElement.lang = locale;
// Only the macOS window is translucent (tauri.macos.conf.json); elsewhere the page keeps its
// opaque background.
if (navigator.userAgent.includes("Mac")) document.documentElement.classList.add("mac");

const target = document.getElementById("app");
if (target) mount(App, { target });
