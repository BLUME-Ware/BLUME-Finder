import type { Messages } from "./en";

const files = (n: number) => `${n} ${n > 1 ? "fichiers" : "fichier"}`;

export const fr: Messages = {
  searchPlaceholder:
    "Décrire ce qui est recherché : « contrat de location », « billets de train Toulouse »…",
  foldersLabel: "Dossiers indexés",
  addFolder: "Ajouter un dossier",
  folderChip: (name, count) => `${name} · ${files(count)}`,
  removeFolderTitle: "Retirer ce dossier de l'index (les fichiers ne sont pas touchés)",
  removeFolderLabel: (name) => `Retirer ${name} de l'index`,
  folderBeingRead: "Ce dossier est en cours de lecture",
  progress: (seen, current) => `Lecture en cours… ${files(seen)} vus. ${current}`,
  ready: (name, s) => {
    let text = `« ${name} » est prêt : ${files(s.seen)}`;
    if (s.changed) {
      text += `, dont ${s.read} ${s.read > 1 ? "lus" : "lu"} en profondeur`;
      if (s.nameOnly > 0) text += ` et ${s.nameOnly} trouvables par leur nom seulement`;
    } else {
      text += ", rien n'a changé";
    }
    return `${text}.`;
  },
  cannotRead: (name, error) => `Impossible de lire « ${name} » : ${error}`,
  removed: (name) => `« ${name} » est retiré de l'index. Les fichiers n'ont pas été touchés.`,
  cannotRemove: (error) => `Impossible de retirer ce dossier : ${error}`,
  cannotReadIndex: (error) => `Impossible de lire l'index : ${error}`,
  noResults: (query) =>
    `Aucun résultat pour « ${query} ». Les scans et les images ne sont trouvés que par leur nom.`,
  searchFailed: (error) => `Recherche impossible : ${error}`,
  nameOnlyHit: "Trouvé par son nom (le contenu de ce type de fichier n'est pas lu).",
  open: "Ouvrir",
  reveal: "Afficher dans le dossier",
  actionFailed: (error) => `Action impossible : ${error}`,
  bytes: (n) => `${n} o`,
  kilobytes: (n) => `${n} Ko`,
  megabytes: (n) => `${n} Mo`,
};
