def bread():
    return "<////////// >"

def lettuce():
    return "~~~~~~~~~~~~"

def tomato():
    return "O O O O O O"

def ham():
    return "============"

def sandwich(veggie=False):
    if veggie:
        return [bread(), lettuce(), tomato(), lettuce(), tomato(), bread()]
    else:
        return [bread(), lettuce(), tomato(), ham(), ham(), bread()]

try:
    order = int(input("How many sandwich do you want? "))
    ingredient = input("Ham or Veggie? ")
    if order > 0 and ingredient in ("Ham", "Veggie"):
        veggie = ingredient == "Veggie"
        sandwiches = [sandwich(veggie) for _ in range(order)]
        for i in range(len(sandwiches[0])):
            print("   ".join(s[i] for s in sandwiches))
    else:
        print("I can't do this!")
except ValueError:
    print("I can't do this!")