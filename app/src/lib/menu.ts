import { Menu, MenuItem, PredefinedMenuItem, Submenu } from "@tauri-apps/api/menu";
import type { PredefinedMenuItemOptions } from "@tauri-apps/api/menu";
import { t } from "./i18n";

const APP = "Blume Finder";

// Tauri's default menu has its labels hard-coded in English, so the macOS menu is built here from
// the catalogs. The predefined items keep their system actions and shortcuts.
export async function setMacMenu(openFolders: () => void) {
  const item = (kind: PredefinedMenuItemOptions["item"], text?: string) =>
    PredefinedMenuItem.new({ item: kind, text });
  const separator = () => item("Separator");
  const m = t.menu;

  const app = await Submenu.new({
    text: APP,
    items: [
      await item({ About: null }, m.about(APP)),
      await separator(),
      await MenuItem.new({ text: m.folders, accelerator: "CmdOrCtrl+,", action: openFolders }),
      await separator(),
      await item("Services", m.services),
      await separator(),
      await item("Hide", m.hide(APP)),
      await item("HideOthers", m.hideOthers),
      await item("ShowAll", m.showAll),
      await separator(),
      await item("Quit", m.quit(APP)),
    ],
  });
  const edit = await Submenu.new({
    text: m.edit,
    items: [
      await item("Undo", m.undo),
      await item("Redo", m.redo),
      await separator(),
      await item("Cut", m.cut),
      await item("Copy", m.copy),
      await item("Paste", m.paste),
      await item("SelectAll", m.selectAll),
    ],
  });
  const window = await Submenu.new({
    text: m.window,
    items: [
      await item("Minimize", m.minimize),
      await item("Maximize", m.zoom),
      await separator(),
      await item("CloseWindow", m.close),
    ],
  });
  const menu = await Menu.new({ items: [app, edit, window] });
  await menu.setAsAppMenu();
}
