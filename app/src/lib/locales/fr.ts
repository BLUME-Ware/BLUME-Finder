import type { Messages } from "./en";

const files = (n: number) => `${n} ${n > 1 ? "fichiers" : "fichier"}`;

export const fr: Messages = {
  searchLabel: "Rechercher",
  consentText:
    "Blume Finder peut lire vos Documents, votre Bureau et vos Téléchargements pour retrouver vos fichiers par leur contenu. Rien ne quitte cet ordinateur.",
  consentAllow: "Autoriser",
  consentLater: "Plus tard",
  searchTips: [
    "contrat de location",
    "facture électricité mars",
    "billets de train Toulouse",
    "devis toiture",
    "attestation de domicile",
    "ordonnance médecin",
  ],
  foldersLabel: "Dossiers indexés",
  knownFolders: { documents: "Documents", desktop: "Bureau", downloads: "Téléchargements" },
  addFolder: "Ajouter un dossier",
  folderChip: (name, count) => `${name} · ${files(count)}`,
  removeFolderTitle: "Retirer ce dossier de l'index (les fichiers ne sont pas touchés)",
  removeFolderLabel: (name) => `Retirer ${name} de l'index`,
  folderBeingRead: "Ce dossier est en cours de lecture",
  updatingStart: "Mise à jour de l'index…",
  updating: (seen) => `Mise à jour de l'index… ${files(seen)}`,
  indexReady: (folders, count) =>
    `Prêt · ${folders} ${folders > 1 ? "dossiers" : "dossier"}, ${files(count)}`,
  menu: {
    about: (app) => `À propos de ${app}`,
    folders: "Dossiers indexés…",
    services: "Services",
    hide: (app) => `Masquer ${app}`,
    hideOthers: "Masquer les autres",
    showAll: "Tout afficher",
    quit: (app) => `Quitter ${app}`,
    edit: "Édition",
    undo: "Annuler",
    redo: "Rétablir",
    cut: "Couper",
    copy: "Copier",
    paste: "Coller",
    selectAll: "Tout sélectionner",
    window: "Fenêtre",
    minimize: "Placer dans le Dock",
    zoom: "Zoom",
    close: "Fermer la fenêtre",
  },
  close: "Fermer",
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
  partialMatch: "Aucun fichier ne contient tous les mots ; voici ceux qui en contiennent une partie.",
  showMore: (n) => (n === 1 ? "Voir 1 autre résultat" : `Voir les ${n} autres résultats`),
  searchFailed: (error) => `Recherche impossible : ${error}`,
  nameOnlyHit: "Trouvé par son nom (le contenu de ce type de fichier n'est pas lu).",
  open: "Ouvrir",
  reveal: "Afficher dans le dossier",
  actionFailed: (error) => `Action impossible : ${error}`,
  bytes: (n) => `${n} o`,
  kilobytes: (n) => `${n} Ko`,
  megabytes: (n) => `${n} Mo`,
};
