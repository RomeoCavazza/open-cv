### TARGETS ###

words = ["cat", "garden", "mice"]
patterns = []

for word in words:
    patterns.append(word)
    patterns.append(word[::-1])

### NORMALIZATION ###

sentences = [
    "the CataCat attaCk a Cat",
    "thE Cat's tactic wAS tO surpRISE thE mIce iN tHE gArdeN"
]

sentences_lowered = []
for s in sentences:
    sentences_lowered.append(s.lower())

### COUNTING STRATEGY ###

def count_occurrences(text, sub):
    count = 0
    start = 0
    while True:
        pos = text.find(sub, start)
        if pos == -1:
            break
        count += 1
        start = pos + len(sub)
    return count

### SENTENCE SCANNING ###

for sentence in sentences_lowered:
    total = 0
    for pattern in patterns:
        total += count_occurrences(sentence, pattern)
    print(sentence, "→", total)