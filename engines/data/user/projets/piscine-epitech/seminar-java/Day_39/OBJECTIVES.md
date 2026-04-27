# Day 39 — Objectifs et Enseignements

## Objectifs

- Découvrir la Reflection en Java.  
- Comprendre les annotations et leur utilisation.  
- Inspecter les classes à l'exécution avec l'API Reflection.

## Enseignements

- **Reflection** : Inspection et manipulation de classes à l'exécution.  
- **Class<T>** : Objet représentant une classe, obtenu via .class ou getClass().  
- **getDeclaredMethods()** : Récupère uniquement les méthodes déclarées (pas héritées).  
- **getDeclaredFields()** : Récupère uniquement les champs déclarés.  
- **Annotations** : Métadonnées ajoutées au code, lues via Reflection.  
- **Javadoc** : Documentation avec @param, @return, @throws, génération HTML automatique.  
- **Métaprogrammation** : Code qui manipule du code, puissant mais à utiliser avec précaution.
