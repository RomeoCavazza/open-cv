import random
from english_words import english_words_lower_set

def get_random_word():
    words = list(english_words_lower_set)
    word = random.choice(words)
    return word

print(get_random_word())