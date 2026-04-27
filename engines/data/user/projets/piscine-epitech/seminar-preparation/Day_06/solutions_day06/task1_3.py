def bread():
    return "<////////// >"

def lettuce():
    return "~~~~~~~~~~~~"

def tomato():
    return "O O O O O O"

def ham():
    return "============"

def sandwich():
    return [
        bread(),
        lettuce(),
        tomato(),
        ham(),
        ham(),
        bread()
    ]

sandwiches = [sandwich() for i in range(4)]

for i in range(len(sandwiches[0])):
    print("   ".join(s[i] for s in sandwiches))