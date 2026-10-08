# Sécurité

Blume Finder lit des fichiers personnels : les signalements de sécurité sont traités en priorité.

## Signaler une faille

Écrire à contact@blumeware.fr, ou utiliser le signalement privé de GitHub (onglet Security, « Report a vulnerability »). Merci de ne pas publier la faille avant qu'une correction soit disponible. Une réponse est donnée sous quelques jours.

## Périmètre

Sont considérés comme des failles : toute écriture, modification ou suppression d'un fichier de l'utilisateur, toute connexion réseau, l'indexation d'un fichier de secrets, l'exécution de code provenant du contenu d'un fichier, et tout contournement de la politique de sécurité de contenu de l'application.

## Garanties vérifiables

Le moteur n'a aucune dépendance réseau (`scripts/check-no-network.sh`, exécuté à chaque modification), l'application n'a pas d'autre permission que `core:default`, et l'index est stocké localement avec des droits réservés au propriétaire sur Mac et Linux.

---

# Security (English)

Blume Finder reads personal files, so security reports get priority. Report a vulnerability to contact@blumeware.fr or through GitHub private vulnerability reporting, and please do not disclose it before a fix is available.

In scope: any write, change or deletion of a user file, any network connection, indexing of a secrets file, code execution from file content, and any bypass of the app's content security policy. The no-network property of the engine is checked on every change by `scripts/check-no-network.sh`.
