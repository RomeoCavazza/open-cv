# Day 57 — Objectifs et Enseignements

## Objectifs

- Configurer une architecture réseau client-gateway avec deux machines virtuelles.  
- Mettre en place un serveur DHCP avec kea-dhcp4-server.  
- Configurer le routage avec nftables.  
- Mettre en place un serveur DNS avec bind9.  
- Comprendre les réseaux privés et le routage entre interfaces.

## Enseignements

- **Architecture réseau** : Client, gateway, séparation des interfaces (bridge/NAT pour Internet, privé pour réseau interne)
- **DHCP** : Attribution automatique d'adresses IP, configuration de plages, IP statique pour la gateway
- **Routage** : nftables pour le forwarding, NAT, gestion des flux entre interfaces
- **DNS** : Configuration bind9, zones, records A et CNAME, résolution de noms dans un domaine local
- **Réseaux privés** : Isolation, communication interne, accès Internet via gateway
- **Virtualisation réseau** : Configuration d'interfaces multiples, réseaux internes VirtualBox
- **Outils réseau** : tcpdump pour l'analyse, nmap/netstat pour le diagnostic
