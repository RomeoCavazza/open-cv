def mask_word(word):
    length = len(word)
    mask = ["_"] * length
    return " ".join(mask)

word = input("Write a word: ")
print(mask_word(word))
