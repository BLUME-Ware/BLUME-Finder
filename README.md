# Blume Finder

Search files by their content, not just their name.

Blume Finder runs locally and reads files without ever modifying them. It indexes the folders it is pointed at, then finds documents from a description of what they contain, with the passage where the words appear.

> Status: V1 in development. Public release planned for November 3, 2026.

## How it works

On first launch, the app offers to read the Documents, Desktop and Downloads folders; macOS then asks for access to each of them, and the offer can be declined. Indexed folders are managed in the Blume Finder menu, under Indexed Folders (Command-comma), so the window keeps only the search. A folder inside an indexed folder is already covered, and adding a parent folder takes over the indexed folders it contains. Each file is read once, its text is extracted, and the result is stored in a local index (a single SQLite file in the application data folder). A search matches the words of the query, tolerating plurals, accents and partial words, against both file names and file contents. When no file contains every word, the files containing some of them are shown, with a notice saying so. While the search field is empty, examples of everyday searches are typed in it. Results show the matching passage with the words highlighted. Folders are re-read at launch, and only files that changed since the last run are processed again. The interface is in English or French, following the system language. On macOS, the window is translucent, with the desktop blurred behind it.

Supported formats: plain text, Markdown, CSV, HTML, PDF with a text layer, Word, PowerPoint, Excel and OpenDocument. Scans and images are matched by file name only, as there is no OCR yet.

## Behavior and guarantees

- Read-only. Files are never moved, renamed, edited or deleted. Removing a folder from the index leaves its files untouched.
- Local processing. The search engine (`core/`) depends on no network library. The app requests no network permission, its content security policy blocks external connections, and it ships no HTTP client.
- Excluded by default: hidden files and folders, key and secret files (`.env`, `.pem`, `.key`, keychains, password databases), credential and password exports, two-factor recovery codes, applications and dependency folders.
- The index holds the text of the indexed files. It is stored locally, with owner-only permissions on Mac and Linux. Deleting it removes everything the app knows.

These points can be checked in the code:

```
cargo tree -p blume-finder-core          # dependencies of the engine
app/src-tauri/tauri.conf.json            # content security policy
app/src-tauri/capabilities/default.json  # permissions granted to the interface
scripts/check-no-network.sh              # automated check, run on every change
```

## Installing on macOS

Blume Finder 0.1.0 runs on macOS 13 or later, on Apple Silicon and Intel Macs. The disk image is published on the [releases page](https://github.com/BLUME-Ware/BLUME-Finder/releases). Once the image is opened, the app is dragged into the Applications folder.

The app is not notarized by Apple, which requires a paid developer account. On first launch, macOS therefore refuses to open it and shows a message saying that Apple could not verify it. To allow it, once only:

1. Open the app a first time and close the message with "Done".
2. Open System Settings, then Privacy & Security.
3. In the Security section, next to the message about Blume Finder, choose "Open Anyway", then confirm with the account password.
4. In the window that follows, choose "Open Anyway" again.

The following launches open the app directly. The source code of the published version is the one tagged on GitHub, and the app can also be built from it, as described below.

## Running it

Requirements: [Rust](https://rustup.rs) and [Node.js](https://nodejs.org) 22 or 24. On Mac, install the Xcode command line tools first (`xcode-select --install`). On Windows, the WebView2 runtime is required (already present on recent versions).

```
cd app
npm install
npm run dev        # opens the app
npm run build      # builds an installable app
```

The engine can also be used on its own:

```
cargo build --release
target/release/blume index ~/Documents
target/release/blume search "rental contract"
target/release/blume folders
target/release/blume stats
```

## Development

Every change passes the same checks locally and in continuous integration: formatting, clippy without warnings, tests, dependency audit, interface type checking and build, and the no-network check.

```
scripts/check-all.sh
```

These checks also require [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) (`cargo install --locked cargo-deny`) and [ShellCheck](https://www.shellcheck.net). Once enabled with `git config core.hooksPath .githooks`, the pre-commit hook runs them before each commit.

## Not included yet

The full roadmap, the guarantees expected from the next versions and where the project stands are in [`docs/ROADMAP.md`](docs/ROADMAP.md).

Search by meaning, finding a document from an idea rather than its exact words, is planned as version 0.2.0, with a model running on the computer. The Windows release is planned as version 0.3.0. OCR for scans and images, and signed releases, are directions, not commitments.

## License

MIT. Issues and questions: GitHub issues, or contact@blumeware.fr.

---

# Blume Finder (français)

Recherche de fichiers par leur contenu, et pas seulement par leur nom.

Blume Finder fonctionne en local et lit les fichiers sans jamais les modifier. Il indexe les dossiers désignés, puis retrouve les documents à partir d'une description de ce qu'ils contiennent, en affichant le passage où apparaissent les mots recherchés.

> Statut : V1 en développement. Publication publique prévue le 3 novembre 2026.

## Fonctionnement

Au premier lancement, l'application propose de lire les dossiers Documents, Bureau et Téléchargements ; macOS demande alors l'accès à chacun d'eux, et la proposition peut être refusée. Les dossiers indexés se gèrent dans le menu Blume Finder, sous Dossiers indexés (Commande-virgule), si bien que la fenêtre ne garde que la recherche. Un dossier situé dans un dossier indexé est déjà couvert, et ajouter un dossier parent reprend les dossiers indexés qu'il contient. Chaque fichier est lu une fois, son texte est extrait, et le résultat est enregistré dans un index local (un seul fichier SQLite dans le dossier de données de l'application). Une recherche compare les mots de la requête, en tolérant pluriels, accents et mots partiels, aux noms de fichiers et à leur contenu. Quand aucun fichier ne contient tous les mots, ceux qui en contiennent une partie sont affichés, avec une mention qui le précise. Tant que le champ de recherche est vide, des exemples de recherches courantes s'y écrivent. Les résultats montrent le passage correspondant, avec les mots surlignés. Les dossiers sont relus au lancement, et seuls les fichiers modifiés depuis le dernier passage sont retraités. L'interface est en anglais ou en français, selon la langue du système. Sur macOS, la fenêtre est translucide, avec le bureau flouté derrière elle.

Formats pris en charge : texte brut, Markdown, CSV, HTML, PDF avec couche texte, Word, PowerPoint, Excel et OpenDocument. Les scans et les images ne sont trouvés que par leur nom, faute d'OCR pour le moment.

## Comportement et garanties

- Lecture seule. Aucun fichier n'est déplacé, renommé, modifié ou supprimé. Retirer un dossier de l'index ne touche pas à ses fichiers.
- Traitement local. Le moteur de recherche (`core/`) ne dépend d'aucune bibliothèque réseau. L'application ne demande aucune permission réseau, sa politique de sécurité de contenu bloque les connexions extérieures, et elle n'embarque aucun client HTTP.
- Exclus par défaut : fichiers et dossiers cachés, fichiers de clés et de secrets (`.env`, `.pem`, `.key`, trousseaux, bases de mots de passe), exports d'identifiants et de mots de passe, codes de récupération de double authentification, applications et dossiers de dépendances.
- L'index contient le texte des fichiers indexés. Il est stocké localement, avec des droits réservés au propriétaire sur Mac et Linux. Le supprimer efface tout ce que l'application connaît.

Ces points sont vérifiables dans le code, avec les trois éléments listés dans la section anglaise : les dépendances du moteur, la politique de sécurité de contenu et les permissions accordées à l'interface.

## Installation sur macOS

Blume Finder 0.1.0 fonctionne sur macOS 13 ou plus récent, sur les Mac Apple Silicon et Intel. L'image disque est publiée sur la [page des versions](https://github.com/BLUME-Ware/BLUME-Finder/releases). Une fois l'image ouverte, l'application se glisse dans le dossier Applications.

L'application n'est pas notarisée par Apple, ce qui demande un compte développeur payant. Au premier lancement, macOS refuse donc de l'ouvrir et affiche un message indiquant qu'Apple n'a pas pu la vérifier. Pour l'autoriser, une seule fois :

1. Ouvrir l'application une première fois et fermer le message avec « Terminé ».
2. Ouvrir Réglages Système, puis Confidentialité et sécurité.
3. Dans la section Sécurité, à côté du message concernant Blume Finder, choisir « Ouvrir quand même », puis confirmer avec le mot de passe du compte.
4. Dans la fenêtre qui suit, choisir de nouveau « Ouvrir quand même ».

Les lancements suivants ouvrent directement l'application. Le code source de la version publiée est celui marqué sur GitHub, et l'application peut aussi être construite à partir de lui, comme décrit ci-dessous.

## Lancement

Prérequis : [Rust](https://rustup.rs) et [Node.js](https://nodejs.org) 22 ou 24. Sur Mac, installer d'abord les outils en ligne de commande Xcode (`xcode-select --install`). Sous Windows, le runtime WebView2 est nécessaire (déjà présent sur les versions récentes). Les commandes sont celles de la section « Running it » ci-dessus.

## Développement

Chaque modification passe les mêmes contrôles en local et en intégration continue : formatage, clippy sans avertissement, tests, audit des dépendances, vérification des types et construction de l'interface, et contrôle d'absence de réseau. Les commandes sont celles de la section « Development » ci-dessus.

## Pas encore inclus

La feuille de route complète, les garanties attendues des prochaines versions et l'état du projet sont décrits, en anglais, dans [`docs/ROADMAP.md`](docs/ROADMAP.md).

La recherche par le sens, qui retrouve un document à partir d'une idée plutôt que de ses mots exacts, est prévue pour la version 0.2.0, avec un modèle qui fonctionne sur l'ordinateur. La version pour Windows est prévue en 0.3.0. L'OCR pour les scans et les images et les versions signées sont des orientations, pas des engagements.

## Licence

MIT. Questions et signalements : tickets GitHub, ou contact@blumeware.fr.
