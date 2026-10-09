import { mount } from "svelte";
import "./app.css";
import App from "./App.svelte";
import { locale } from "./lib/i18n";

document.documentElement.lang = locale;

const target = document.getElementById("app");
if (target) mount(App, { target });
