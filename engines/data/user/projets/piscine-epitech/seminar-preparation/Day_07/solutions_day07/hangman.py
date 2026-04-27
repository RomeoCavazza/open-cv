import random

### PICK A WORD ###

words = ["apple", "garden", "python", "rocket", "window"]
secret_word = random.choice(words).upper()

### INIT GAME STATE ###

mask = ["_"] * len(secret_word)
penalties = 0
LIMIT = 12
print(" ".join(mask), "/", penalties, "penalty")

while penalties < LIMIT and "_" in mask:
    guess = input("Type a letter or a word: ").upper()
    if guess == "":
        continue

    # CASE 1 : SINGLE LETTER
    if len(guess) == 1:
        letter = guess
        found = False
        for i in range(len(secret_word)):
            if secret_word[i] == letter:
                mask[i] = letter
                found = True
        if found:
            print("Good job! Found", letter)
        else:
            print("No", letter, "here")
            penalties += 1

    # CASE 2 : FULL WORD GUESS
    else:
        if guess == secret_word:
            mask = list(secret_word)
            break
        else:
            print("Wrong word! +5 penalties")
            penalties += 5
    print(" ".join(mask), "/", penalties, "penalties")

### END OF GAME ###

if "_" not in mask:
    print("YOU WIN! The word was:", secret_word)
    print("Total penalties:", penalties)
else:
    print("YOU LOOSE!")
    print("The word was:", secret_word)
