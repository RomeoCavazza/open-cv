# Day 12 — Notes

- Objectifs: [OBJECTIVES.md](OBJECTIVES.md)
- Solutions: [solutions_day12/](solutions_day12/)

## Objectifs
- Découvrir HTML5 sémantique et structurer des pages (header/nav/main/aside/footer).  
- Créer une mini‑encyclopédie « CodingPedia » (Index, PHP, SQL).  
- Appliquer du style CSS externe et interne.  
- Intégrer formulaires, tableaux, médias et éléments sémantiques (`article`, `section`, `abbr`, `code`, etc.).  

## Actions
- `task03/index.html` : page d’accueil minimale (balises de base, navigation).  
- `task08` (version finale stylée) :  
  - `index.html` avec `<link rel="stylesheet" href="codingpedia.css">`, blocs `.box`, et animation d’un `div.moving_square`.  
  - Formulaire complet : `required`, `type="email"`, `type="number"` avec `min/max`, `pattern` pour téléphone, `fieldset`/`legend`, `select`, `textarea`.  
  - `php.html` : contenu éditorial (historique PHP), tableau « Course planning », style local (`abbr`, `table,th,td`, `pre code`).  
  - `sql.html` : contenu éditorial (historique SQL), tableau « Course planning », exemple SQL formaté en `<pre><code>`.  
  - `codingpedia.css` : styles communs (ex. `.box { border/margin/padding }`).  

## Leçons
- HTML5 sémantique : utiliser `header`, `nav`, `main`, `article`, `aside`, `footer` pour une structure claire et accessible.  
- Formulaires robustes : contraintes côté client (`required`, `pattern`, bornes numériques) pour prévenir des erreurs basiques.  
- Séparation des préoccupations : CSS externe pour les styles communs ; styles inline/`<style>` seulement quand nécessaire et localisés.  
- Accessibilité et SEO : attributs `lang`, `meta charset`, `meta viewport`, `title`, `alt` (si images).  
- Bonnes pratiques CSS : factoriser des classes réutilisables (ex. `.box`) et éviter la duplication ; valider les contrastes et la lisibilité.  
- Contenus techniques : isoler le code dans `<pre><code>` ; pour les abréviations, utiliser `<abbr title="...">`.  
