# Rapport de mission – Gmail Automation & Dashboard Web

---

##  Objectif

Mettre en place une automatisation complète avec **n8n + Gmail + OpenAI**, et l’exposer via une **interface web moderne et responsive**.  
L’idée : récupérer les e-mails de la journée, générer un résumé automatique et les afficher clairement dans un dashboard local.  

---

## ️ Étapes du projet

### 1. Workflow n8n
- Connexion à Gmail via OAuth2.  
- Récupération des e-mails de la journée (expéditeur, objet, date).  
- Génération d’un résumé avec un LLM (OpenAI GPT-3.5).  
- Sauvegarde en JSON (`mails-today.json`) dans un dossier partagé.  

> ⚡ Ce workflow a été le cœur du projet.  
J’ai passé une journée entière dessus : il fallait coordonner plusieurs sorties, écrire des expressions en JavaScript et gérer un merge entre les données brutes et le résumé IA.  

### 2. Dockerisation
- J’ai dockerisé n8n pour simplifier le setup et assurer la reproductibilité.  
- Le projet tourne en **localhost:5678** (n8n) et **localhost:8080** (interface web).  
- Un volume local permet de stocker et lire automatiquement le JSON généré.  

### 3. Développement Web
- Une interface web simple, responsive et lisible.  
- Affiche : expéditeur, objet, date/heure.  
- Intègre un **résumé global des mails** généré par l’IA.  
- Ajout d’un bouton **reload** (connecté à un webhook n8n) pour rafraîchir les données sans relancer le conteneur.  

> Cette partie m’a pris une grosse soirée : j’ai utilisé **Cursor** pour coder rapidement l'interface en choississant le simple combo HTML/CSS/JS, façon MVP.

### 4. Documentation et packaging
- J’ai pris une dernière journée pour finaliser le **README**, les instructions d’installation et les tests de reproductibilité.  
- Tout est documenté : configuration Gmail OAuth2, intégration OpenAI, quickstart Docker, déclenchement via webhook, etc.

 Environ **3 jours de travail au total** :  
- 1 jour pour le workflow n8n,  
- 1 soirée pour le front,  
- 1 jour pour la doc, tests et packaging.  

---

##  Difficultés rencontrées

1. **Branchement des données dans n8n**  
   - Pas trivial de fusionner les sorties Gmail + résumé IA.  
   - J’ai dû écrire plusieurs expressions JS pour que le JSON reste propre.  

2. **Coordination front ↔ back**  
   - Le bouton « reload » devait appeler un webhook pour rafraîchir les données.  
   - La synchronisation des fichiers JSON avec Docker a nécessité des ajustements (permissions + volumes).  

3. **Dockerisation**  
   - Quelques conflits de droits d’accès au dossier `data/` dans une image dockerisé (a nécessité plusieurs essais)

---

##  Ressenti personnel

J’ai pris beaucoup de plaisir à réaliser cette mission.  
- C’était un vrai **challenge technique**, surtout la partie merge dans n8n et le branchement du front.
- J’ai appris à mieux documenter mes projets et à penser en termes de **setup reproductible**.  

C’était aussi particulièrement agréable de créer ma propre interface et de mener ce mini-projet “presque full-stack”, 
entre front, back, API et dockerisation. 

Il ne manque finalement qu’un vrai déploiement (par exemple via Vercel) et l’ajout d’une page de paiement avec Stripe 
pour aller encore plus loin — une pratique que je compte clairement garder et faire progresser dans mes futurs projets.

Ce fut une mission marquante pour moi, et une très bonne pièce dans mon portfolio. 
