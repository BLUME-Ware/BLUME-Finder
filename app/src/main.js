// Interface de Blume Finder. Aucune ressource extérieure, aucun innerHTML :
// tout le texte venant des fichiers est inséré comme texte, jamais comme code.
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const HL_START = "\u0001";
const HL_END = "\u0002";

const champ = document.getElementById("recherche");
const listeDossiers = document.getElementById("liste-dossiers");
const boutonAjouter = document.getElementById("ajouter");
const etat = document.getElementById("etat");
const resultats = document.getElementById("resultats");

let dossiers = [];
let occupe = 0;          // nombre d'indexations en cours
let numeroRecherche = 0; // ignore les réponses périmées
let minuteur = null;

function dire(texte, erreur = false) {
  etat.textContent = texte;
  etat.classList.toggle("erreur", erreur);
}

function nomCourt(chemin) {
  const morceaux = chemin.split(/[\\/]/).filter(Boolean);
  return morceaux.length ? morceaux[morceaux.length - 1] : chemin;
}

function pluriel(n, un, plusieurs) {
  return `${n} ${n > 1 ? plusieurs : un}`;
}

function taille(octets) {
  if (octets < 1024) return `${octets} o`;
  if (octets < 1024 * 1024) return `${Math.round(octets / 1024)} Ko`;
  return `${(octets / 1024 / 1024).toFixed(1)} Mo`;
}

function date(secondes) {
  if (!secondes) return "";
  return new Date(secondes * 1000).toLocaleDateString("fr-FR", {
    day: "numeric", month: "short", year: "numeric",
  });
}

// ---------- dossiers ----------

function afficherDossiers() {
  listeDossiers.replaceChildren();
  for (const d of dossiers) {
    const li = document.createElement("li");
    li.className = "dossier";
    li.title = d.path;

    const nom = document.createElement("span");
    nom.className = "nom";
    nom.textContent = `${nomCourt(d.path)} · ${pluriel(d.files, "fichier", "fichiers")}`;

    const retirer = document.createElement("button");
    retirer.type = "button";
    retirer.textContent = "×";
    retirer.title = "Retirer ce dossier de l'index (les fichiers ne sont pas touchés)";
    retirer.setAttribute("aria-label", `Retirer ${nomCourt(d.path)} de l'index`);
    retirer.addEventListener("click", () => retirerDossier(d.path));

    li.append(nom, retirer);
    listeDossiers.append(li);
  }
}

async function chargerDossiers() {
  dossiers = await invoke("list_folders");
  afficherDossiers();
}

async function indexer(chemin) {
  occupe += 1;
  boutonAjouter.disabled = true;
  try {
    const rapport = await invoke("index_folder", { path: chemin });
    await chargerDossiers();
    resumer(chemin, rapport);
  } catch (e) {
    dire(`Impossible de lire « ${nomCourt(chemin)} » : ${e}`, true);
  } finally {
    occupe -= 1;
    if (occupe === 0) boutonAjouter.disabled = false;
  }
}

function resumer(chemin, r) {
  const lus = r.indexed - r.name_only;
  let texte = `« ${nomCourt(chemin)} » est prêt : ${pluriel(r.seen, "fichier", "fichiers")}`;
  if (r.indexed > 0) {
    texte += `, dont ${pluriel(Math.max(lus, 0), "lu en profondeur", "lus en profondeur")}`;
    if (r.name_only > 0) texte += ` et ${r.name_only} trouvables par leur nom seulement`;
  } else {
    texte += ", rien n'a changé";
  }
  texte += ".";
  dire(texte);
  lancerRecherche();
}

async function ajouterDossier() {
  const chemin = await invoke("choose_folder");
  if (chemin) await indexer(chemin);
}

async function retirerDossier(chemin) {
  try {
    await invoke("forget_folder", { path: chemin });
    await chargerDossiers();
    dire(`« ${nomCourt(chemin)} » est retiré de l'index. Les fichiers n'ont pas été touchés.`);
    lancerRecherche();
  } catch (e) {
    dire(`Impossible de retirer ce dossier : ${e}`, true);
  }
}

// ---------- recherche ----------

function extrait(texte) {
  const p = document.createElement("p");
  p.className = "extrait";
  let reste = texte;
  while (reste.length) {
    const debut = reste.indexOf(HL_START);
    if (debut === -1) {
      p.append(document.createTextNode(reste));
      break;
    }
    const fin = reste.indexOf(HL_END, debut);
    if (fin === -1) {
      p.append(document.createTextNode(reste.replace(HL_START, "")));
      break;
    }
    if (debut > 0) p.append(document.createTextNode(reste.slice(0, debut)));
    const m = document.createElement("mark");
    m.textContent = reste.slice(debut + 1, fin);
    p.append(m);
    reste = reste.slice(fin + 1);
  }
  return p;
}

function carte(hit) {
  const el = document.createElement("article");
  el.className = "carte";

  const titre = document.createElement("h2");
  titre.textContent = hit.name;

  const chemin = document.createElement("p");
  chemin.className = "chemin";
  const meta = [taille(hit.size), date(hit.mtime)].filter(Boolean).join(" · ");
  chemin.textContent = `${hit.path}  ·  ${meta}`;

  el.append(titre, chemin);

  if (hit.snippet) {
    el.append(extrait(hit.snippet));
  } else {
    const p = document.createElement("p");
    p.className = "extrait nom-seul";
    p.textContent = "Trouvé par son nom (le contenu de ce type de fichier n'est pas lu).";
    el.append(p);
  }

  const actions = document.createElement("div");
  actions.className = "actions";
  const ouvrir = document.createElement("button");
  ouvrir.type = "button";
  ouvrir.textContent = "Ouvrir";
  ouvrir.addEventListener("click", () => agir("open_file", hit.path));
  const montrer = document.createElement("button");
  montrer.type = "button";
  montrer.textContent = "Afficher dans le dossier";
  montrer.addEventListener("click", () => agir("reveal_file", hit.path));
  actions.append(ouvrir, montrer);
  el.append(actions);
  return el;
}

async function agir(commande, chemin) {
  try {
    await invoke(commande, { path: chemin });
  } catch (e) {
    dire(`Action impossible : ${e}`, true);
  }
}

async function lancerRecherche() {
  const requete = champ.value.trim();
  const mon = ++numeroRecherche;
  if (!requete) {
    resultats.replaceChildren();
    if (!dossiers.length && !occupe) {
      const p = document.createElement("p");
      p.className = "vide";
      p.textContent = "Ajouter un dossier pour commencer. Blume Finder lit les fichiers une seule fois, localement, puis permet de les retrouver par leur contenu.";
      resultats.append(p);
    }
    return;
  }
  try {
    const hits = await invoke("search", { query: requete });
    if (mon !== numeroRecherche) return;
    resultats.replaceChildren();
    if (!hits.length) {
      const p = document.createElement("p");
      p.className = "vide";
      p.textContent = `Aucun résultat pour « ${requete} ». Les scans et les images ne sont trouvés que par leur nom.`;
      resultats.append(p);
      return;
    }
    for (const h of hits) resultats.append(carte(h));
  } catch (e) {
    if (mon === numeroRecherche) dire(`Recherche impossible : ${e}`, true);
  }
}

champ.addEventListener("input", () => {
  clearTimeout(minuteur);
  minuteur = setTimeout(lancerRecherche, 150);
});

champ.addEventListener("keydown", (e) => {
  if (e.key === "Escape") {
    champ.value = "";
    lancerRecherche();
  }
});

boutonAjouter.addEventListener("click", ajouterDossier);

// ---------- démarrage ----------

listen("progress", (evenement) => {
  const p = evenement.payload;
  dire(`Lecture en cours… ${pluriel(p.seen, "fichier", "fichiers")} vus. ${nomCourt(p.current)}`);
});

(async function demarrer() {
  champ.focus();
  try {
    await chargerDossiers();
  } catch (e) {
    dire(`Impossible de lire l'index : ${e}`, true);
    return;
  }
  lancerRecherche();
  // Au lancement, on remet à jour les dossiers déjà connus (seuls les fichiers modifiés sont relus).
  for (const d of dossiers) await indexer(d.path);
})();
