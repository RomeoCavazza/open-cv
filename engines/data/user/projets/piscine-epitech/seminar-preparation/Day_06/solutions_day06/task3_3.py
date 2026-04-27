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

fun_1 = passcheck(fun1, 16, mysecretpassword)
fun_2 = passcheck(fun2, 3, mysecretpassword)
fun_3 = passcheck(fun3, 1, mysecretpassword)

print(fun_1)
print(fun_2)
print(fun_3)

if (fun_1 == True) and (fun_2 == True) and (fun_3 == True):
    print("Password accepted")
else:
    print("Password too weak")