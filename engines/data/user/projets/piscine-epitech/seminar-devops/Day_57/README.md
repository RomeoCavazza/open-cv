# Day 57 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: (à compléter si nécessaire)

## Objectifs
- Configurer une architecture réseau client-gateway avec deux machines virtuelles.  
- Mettre en place un serveur DHCP avec kea-dhcp4-server.  
- Configurer le routage avec nftables.  
- Mettre en place un serveur DNS avec bind9.  
- Comprendre les réseaux privés et le routage entre interfaces.  

## Actions
- **ex00 - Client machine** : Installation Debian 13 avec interface graphique (KDE), une interface réseau en réseau privé, utilisée pour tester DHCP et DNS
- **ex01 - Gateway machine** : Installation Debian 13 sans GUI, deux interfaces réseau (première en bridge/NAT, deuxième en réseau privé partagé avec vm-client), machine principale pour l'autograder
- **ex02 - Network name** : Changement du nom de la machine gateway en `vn-gateway`
- **ex03 - Packages** : Installation des packages nécessaires (tcpdump, kea-dhcp4-server, bind9, nmap, netstat)
- **ex04 - DHCP** : Configuration kea-dhcp4-server sur vm-gateway avec réseau 192.168.42.0/24, netmask 255.255.255.0, IP gateway 192.168.42.254 (statique), range DHCP 192.168.42.100-150 (50 hôtes), test avec vm-client
- **ex05 - Router** : Configuration nftables dans /etc/nftables.conf pour le routage (interface bridge en DHCP, interface privée sur même réseau que vm-client, autoriser tout le trafic sortant du réseau privé), permettre à vm-client d'accéder à Internet via la gateway
- **ex06 - DNS** : Configuration bind9 avec domaine epi42.lan (les deux machines dans ce domaine), records DNS : gateway.epi42.lan => 192.168.42.254, dhcp.epi42.lan (alias de gateway.epi42.lan)

## Leçons
- **Architecture réseau** : Client, gateway, séparation des interfaces (bridge/NAT pour Internet, privé pour réseau interne)
- **DHCP** : Attribution automatique d'adresses IP, configuration de plages, IP statique pour la gateway
- **Routage** : nftables pour le forwarding, NAT, gestion des flux entre interfaces
- **DNS** : Configuration bind9, zones, records A et CNAME, résolution de noms dans un domaine local
- **Réseaux privés** : Isolation, communication interne, accès Internet via gateway
- **Virtualisation réseau** : Configuration d'interfaces multiples, réseaux internes VirtualBox
- **Outils réseau** : tcpdump pour l'analyse, nmap/netstat pour le diagnostic
