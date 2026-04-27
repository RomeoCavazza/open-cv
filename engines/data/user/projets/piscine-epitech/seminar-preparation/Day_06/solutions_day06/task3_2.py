def fun1(s, n):
    return len(s) >= n

def fun2(s, n):
    special_chars = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"
    count = sum(1 for c in s if c in special_chars)
    return count >= n

def fun3(s, n):
    count = sum(1 for c in s if c.isdigit())
    return count >= n    

def passcheck(func, n, password):
    return func(password, n)

mysecretpassword = input("Write a string: ")
print(passcheck(fun1, 16, mysecretpassword))
print(passcheck(fun2, 3, mysecretpassword))
print(passcheck(fun3, 1, mysecretpassword))