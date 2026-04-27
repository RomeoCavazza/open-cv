def bread():
    return "<////////// >"

def lettuce():
    return "~~~~~~~~~~~~"

def tomato():
    return "O O O O O O"

def ham():
    return "============"

def sandwich():
    return [bread(), lettuce(), tomato(), ham(), ham(), bread()]

try:
    order = int(input("How many sandwich do you want? "))
    if order > 0:
        sandwiches = [sandwich() for i in range(order)]
        for i in range(len(sandwiches[0])):
            print("   ".join(s[i] for s in sandwiches))
    else:
        print("I can't do this!")
except ValueError:
    print("I can't do this!")