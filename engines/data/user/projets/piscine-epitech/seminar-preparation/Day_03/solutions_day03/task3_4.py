sentence = input("Type a string: ")
words = sentence.split()
letters = []

for word in words:
    letters.append(word[0])

result = "".join(letters)
print (result)