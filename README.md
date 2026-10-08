# Blume Finder

Search files by their content, not just their name.

Blume Finder runs locally and reads files without ever modifying them. It indexes the folders it is pointed at, then finds documents from a description of what they contain, with the passage where the words appear.

> Status: V1 in development. Public release planned for November 3, 2026.

## How it works

The folders to index are chosen in the app. Each file is read once, its text is extracted, and the result is stored in a local index (a single SQLite file in the application data folder). A search matches the words of the query, tolerating plurals, accents and partial words, against both file names and file contents. Results show the matching passage with the words highlighted. Folders are re-read at launch, and only files that changed since the last run are processed again.

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

## Running it

Requirements: [Rust](https://rustup.rs) and [Node.js](https://nodejs.org). On Mac, install the Xcode command line tools first (`xcode-select --install`). On Windows, the WebView2 runtime is required (already present on recent versions).

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

Every change passes the same checks locally and in continuous integration: formatting, clippy without warnings, tests, dependency audit and the no-network check.

```
scripts/check-all.sh
```

These checks also require [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) (`cargo install --locked cargo-deny`) and [ShellCheck](https://www.shellcheck.net). Once enabled with `git config core.hooksPath .githooks`, the pre-commit hook runs them before each commit.

## Not included yet

Search by meaning (finding a document from an idea rather than exact words), OCR for scans and images, signed releases. These are directions, not commitments.

## License

MIT. Issues and questions: GitHub issues, or contact@blumeware.fr.

---

# Blume Finder (français)

Recherche de fichiers par leur contenu, et pas seulement par leur nom.

Blume Finder fonctionne en local et lit les fichiers sans jamais les modifier. Il indexe les dossiers désignés, puis retrouve les documents à partir d'une description de ce qu'ils contiennent, en affichant le passage où apparaissent les mots recherchés.

> Statut : V1 en développement. Publication publique prévue le 3 novembre 2026.

## Fonctionnement

Les dossiers à indexer sont choisis dans l'application. Chaque fichier est lu une fois, son texte est extrait, et le résultat est enregistré dans un index local (un seul fichier SQLite dans le dossier de données de l'application). Une recherche compare les mots de la requête, en tolérant pluriels, accents et mots partiels, aux noms de fichiers et à leur contenu. Les résultats montrent le passage correspondant, avec les mots surlignés. Les dossiers sont relus au lancement, et seuls les fichiers modifiés depuis le dernier passage sont retraités.

Formats pris en charge : texte brut, Markdown, CSV, HTML, PDF avec couche texte, Word, PowerPoint, Excel et OpenDocument. Les scans et les images ne sont trouvés que par leur nom, faute d'OCR pour le moment.

## Comportement et garanties

- Lecture seule. Aucun fichier n'est déplacé, renommé, modifié ou supprimé. Retirer un dossier de l'index ne touche pas à ses fichiers.
- Traitement local. Le moteur de recherche (`core/`) ne dépend d'aucune bibliothèque réseau. L'application ne demande aucune permission réseau, sa politique de sécurité de contenu bloque les connexions extérieures, et elle n'embarque aucun client HTTP.
- Exclus par défaut : fichiers et dossiers cachés, fichiers de clés et de secrets (`.env`, `.pem`, `.key`, trousseaux, bases de mots de passe), exports d'identifiants et de mots de passe, codes de récupération de double authentification, applications et dossiers de dépendances.
- L'index contient le texte des fichiers indexés. Il est stocké localement, avec des droits réservés au propriétaire sur Mac et Linux. Le supprimer efface tout ce que l'application connaît.

Ces points sont vérifiables dans le code, avec les trois éléments listés dans la section anglaise : les dépendances du moteur, la politique de sécurité de contenu et les permissions accordées à l'interface.

## Lancement

Prérequis : [Rust](https://rustup.rs) et [Node.js](https://nodejs.org). Sur Mac, installer d'abord les outils en ligne de commande Xcode (`xcode-select --install`). Sous Windows, le runtime WebView2 est nécessaire (déjà présent sur les versions récentes). Les commandes sont celles de la section « Running it » ci-dessus.

## Développement

Chaque modification passe les mêmes contrôles en local et en intégration continue : formatage, clippy sans avertissement, tests, audit des dépendances et contrôle d'absence de réseau. Les commandes sont celles de la section « Development » ci-dessus.

## Pas encore inclus

La recherche par le sens (retrouver un document à partir d'une idée plutôt que de mots exacts), l'OCR pour les scans et les images, les versions signées. Ce sont des orientations, pas des engagements.

## Licence

MIT. Questions et signalements : tickets GitHub, ou contact@blumeware.fr.
