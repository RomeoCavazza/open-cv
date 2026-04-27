### INPUT ###

text = """
La Cigale, ayant chanté
Tout l'été,
Se trouva fort dépourvue
Quand la bise fut venue :
Pas un seul petit morceau
De mouche ou de vermisseau.
Elle alla crier famine
Chez la Fourmi sa voisine,
La priant de lui prêter
Quelque grain pour subsister
Jusqu'à la saison nouvelle.
" Je vous paierai, lui dit-elle,
Avant l'Oût, foi d'animal,
Intérêt et principal. "
La Fourmi n'est pas prêteuse :
C'est là son moindre défaut.
Que faisiez-vous au temps chaud ?
Dit-elle à cette emprunteuse.
- Nuit et jour à tout venant
Je chantais, ne vous déplaise.
- Vous chantiez ? j'en suis fort aise.
Eh bien ! dansez maintenant.
"""

### NORMALIZATION ###

text_lower = text.lower()

letters_only = []
for ch in text_lower:
    if 'a' <= ch <= 'z':
        letters_only.append(ch)

### LETTER COUNTING ###

letter_counts = {}
for ch in letters_only:
    if ch not in letter_counts:
        letter_counts[ch] = 1
    else:
        letter_counts[ch] += 1

### OUTPUT ###

print("=== LETTER FREQUENCIES ===")
for ch in sorted(letter_counts.keys()):
    print(f"{ch}: {letter_counts[ch]}")
