number = int(input("Type a number: "))

if (number == 42):
    print ("a")
elif (number <= 21):
    print ("b")
elif (number % 2 == 0):
    print ("c")
elif (number / 2 < 21):
    print ("d")
elif (number % 2 != 0 and number >= 45):
    print ("e")
else:
    print ("f")