def check_penalties(n):
    if n >= 12:
        print("You loose!")
        return True
    else:
        return False

n = int(input("Choose a number: "))

if check_penalties(n):
    print("Game over...")
else:
    print("You can keep playing!")