Parfait, on fait le “super récap EPITECH DevOps arc 1” 

---

##  1. Bilan Jour 1 → 4 (version simple & claire)

###  Jour 1 – **Serveur Linux sécurisé** 

Tu apprends à :

* Installer Debian proprement (sans interface graphique, partitions / /home /var /swap)
* Créer & gérer des **utilisateurs / groupes** (marvin, zaphod, H2G2)
* Configurer **SSH sécurisé** : port custom, clés, pas de login root
* Mettre en place **Fail2ban** (bloquer les IP qui bruteforcent)
* Configurer un **firewall nftables** qui n’ouvre que les ports nécessaires

 Compétence clé : **installer et durcir un serveur comme en production.**

---

###  Jour 2 – **Réseau complet (mini-Internet)**

Tu construis un vrai réseau :

* Une **gateway** (ta box perso) avec deux interfaces réseau
* Un **serveur DHCP** (Kea) qui distribue des IP à tes clients
* Un **serveur DNS** (Bind9) avec ton domaine local (`.lan`)
* Du **routing + NAT** via nftables
* Tu utilises **tcpdump, netstat, nmap** pour voir comment ça circule

 Compétence clé : **comprendre comment les machines se parlent (IP, DHCP, DNS, routeur).**

---

###  Jour 3 – **Serveur Web Full-Stack**

Tu transformes ta machine en **hébergeur web** :

* Installation des toolchains : `curl`, `php`, `composer`, `symfony`, `node`, `mariadb-server`
* Mise en place d’**Apache2 sur port 8080**
* Config & exposition de **MariaDB** + user SQL dédié
* Installation de **phpMyAdmin** (admin DB)
* Déploiement d’un **frontend** dans `/var/www/frontend`

 Compétence clé : **déployer un site web complet (front + DB) sur ton propre serveur.**

---

###  Jour 4 – **Infra “production ready”** 

Tu passes en mode **DevOps sérieux** :

* Déploiement du **backend Symfony** dans `/var/www/backend`, servi par Apache (`/api`)
* Sécurisation de la zone **admin** via Basic Auth Apache (`/admin`)
* Renforcement du **firewall nftables** (Deny all / Allow only needed)
* Script de **backup de la base** (`/backup/backup.sh`)
* **Cron** pour lancer le backup automatiquement toutes les heures
* Activation du **HTTPS** sur l’admin avec un certificat auto-signé

 Compétence clé : **sécurité applicative, backups, HTTPS, automatisation.**

---

###  Bilan global en une phrase

> En 4 jours, tu as appris à installer, sécuriser et administrer un serveur Linux, construire un réseau complet, déployer une stack web (front + back + DB) et la rendre production-ready avec firewall, backup automatique et HTTPS.

---

##  2. Boîte à outils DevOps complète J1 → J4

###  OS & Système

* **Debian 13**
* Partitionnement : `/`, `/home`, `/var`, `swap`
* `useradd`, `groupadd`, `passwd`, `chown`, `chmod`
* `systemctl`, `service`, logs `/var/log/*`

---

###  Sécurité & Accès

* **SSH (OpenSSH)** : `sshd_config`, port custom, clés, no-root
* **Fail2ban** : jail SSH, bannissement IP auto
* **nftables** : firewall + NAT + routing
* **Basic Auth Apache** : `.htaccess`, `.htpasswd`
* **HTTPS** : certificats auto-signés, Apache SSL

---

###  Réseau

* **VirtualBox** (réseaux bridged / host-only)
* Commandes : `ip`, `netstat`, `tcpdump`, `nmap`
* **DHCP** : `kea-dhcp4`
* **DNS** : `bind9` (zone, A, CNAME)
* Gateway / NAT / routing entre 2 interfaces

---

###  Web & Backend

* **Apache2** : virtual hosts, port 8080, docroots
* **PHP ≥ 8.0**
* **Symfony CLI**
* **Composer**
* **Node.js** (pour front JS)
* Frontend statique servi depuis `/var/www/frontend`
* Backend Symfony servi depuis `/var/www/backend` (`/api`)

---

###  Base de données

* **MariaDB Server**
* `mysql_secure_installation`
* Création de users & privilèges (`data-backend`)
* **phpMyAdmin** pour admin graphique
* Backups via `mysqldump` (+ compression)

---

### ⚙️ Automatisation & Maintenance

* **Autograder** (logique CI automatisée)
* **Scripts Bash** (backup)
* **Cron** (`crontab` root → exécution horaire)
* Sauvegardes locales compressées dans `/backup/`

---

##  3. Schéma d’architecture final (texte)

Voici une vue d’ensemble de ce que tu as en J1→J4 :

```text
                 INTERNET / CLIENT
                        |
                        v
                [ IP publique / DNS ]
                        |
                 (Jour 2 - Gateway)
                        |
                 ┌─────────────────┐
                 │  Serveur Debian │
                 │ (Jour 1→4)      │
                 └─────────────────┘
                        |
        ┌───────────────┼────────────────┐
        |                                   |
        v                                   v
 [ Firewall nftables ]               [ SSH sécurisé ]
  (ports ouverts minimum)        (clés, fail2ban, no-root)
        |
        v
   ┌─────────────── Web layer (Jour 3–4) ────────────────┐
   │                                                    │
   │         ┌─────────────────────────────┐            │
   │         │         Apache2             │            │
   │         │  - port 8080               │            │
   │         │  - VirtualHosts            │            │
   │         │  - HTTP + HTTPS (admin)    │            │
   │         └─────────────┬──────────────┘            │
   │                       │                           │
   │     /frontend         │          /backend (API)   │
   │   (JS front)          │        (Symfony/PHP)      │
   └───────────────────────┼───────────────────────────┘
                           |
                           v
                ┌──────────────────────┐
                │     MariaDB          │
                │  (base de données)   │
                └──────────────────────┘
                           |
                           v
             ┌───────────────────────────────┐
             │   /backup/backup.sh (Bash)    │
             │   + Cron root (toutes les h)  │
             │   → dumps DB compressés       │
             └───────────────────────────────┘
```

* **Firewall** filtre tout ce qui entre/sort
* **Apache** sert :

  * le **frontend JS** sur `/`
  * le **backend API** sur `/api`
  * l’**admin sécurisée** sur `/admin` (Basic Auth + HTTPS)
* **MariaDB** stocke les données
* **Backup + Cron** assurent la résilience des données
* **SSH + Fail2ban** assurent l’accès admin propre et sécurisé

---

Si tu veux, on peut maintenant :

* mapper **cette architecture directement sur “Veyl.io Lab”** (nginx + Next + FastAPI + Postgres)
