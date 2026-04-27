# Day 13 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day13/](solutions_day13/)

## Objectifs
- Structurer des pages HTML5 avec balises sémantiques (`header`, `nav`, `main`, `section`, `article`, `footer`).  
- Mettre en place une navigation interne (ancres) et des menus repliables (`details`/`summary`).  
- Organiser le contenu en sections multi-colonnes/logiques.  
- Introduire un CV/portfolio responsive avec framework CSS externe (Materialize via CDN) et assets.  
- Construire un formulaire de contact accessible.  

## Actions
- Task 01: squelette sémantique (header + nav simple, 2 sections, 6 articles, footer avec `address` et lien `tel:`).  
- Task 02: navigation imbriquée avec `details/summary` + ancres sur sections/articles; duplication structurée des articles; footer identique.  
- Task 03: ajout d’une `section-3` et distribution d’articles (1→8) sur plusieurs sous-sections; hiérarchie de titres `h1→h3`.  
- Task 04: raffinement des sections/sous-sections (répartition 1→6) et menu repliable; maintien de la sémantique et du footer.  
- Task 05: CV numérique avec Materialize CSS (CDN) — en-tête, cartes (`card`), grille responsive, images (`automation.png`, `hyprland.jpg`, `osint.jpg`, `piscine.png`, `kicad.png`, `nvidia.avif`), liens externes (LinkedIn, GitHub), formulaire (`email`, `textarea`, bouton), JS (jQuery + Materialize).  

## Leçons
- **HTML5 sémantique**: hiérarchie claire `h1→h4`, sections logiques, `footer` avec `address`.  
- **Navigation**: ancres internes + `details/summary` pour accessibilité et compacité.  
- **Organisation**: séparer en sous-sections plutôt que listes longues; éviter la surcharge d’une seule section.  
- **Framework CSS**: usage via CDN (Materialize) pour prototyper rapidement; penser aux performances (CDN, cache) et au fallback.  
- **Assets**: fournir `alt` descriptifs aux images, soigner la compression (PNG/JPG/AVIF).  
- **Formulaires**: champs typés (`email`), labels associés, structure simple et accessible.  
