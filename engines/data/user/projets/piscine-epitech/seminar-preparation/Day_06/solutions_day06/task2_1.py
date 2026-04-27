def sum(n):
    if n == 0:
        return n
    return n + sum(n-1)

n = int(input("Type a number: "))
print(sum(n))