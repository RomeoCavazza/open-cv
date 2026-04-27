import random

def roll():
    number = [1,2,3,4,5,6]
    item = random.choice(number)
    return item

print(roll())