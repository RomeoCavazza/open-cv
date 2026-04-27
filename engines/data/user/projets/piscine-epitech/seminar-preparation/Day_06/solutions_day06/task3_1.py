def funA(s,n):
    return len(s) >= n

def funB(s,n):
    special_chars = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"
    count = sum(1 for c in s if c in special_chars)
    return count >= n

def funC(s,n):
    count = sum(1 for c in s if c.isdigit())
    return count >= n

string_input = input("Write a string: ")
print(funA(string_input, 1))
print(funB(string_input, 1))
print(funC(string_input, 1))