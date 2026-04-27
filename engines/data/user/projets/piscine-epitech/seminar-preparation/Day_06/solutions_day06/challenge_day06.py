import time

x = 42
y = 84
# y = 168

def power(x,y):
    if y == 0:
        return 1
    if y >= 1:
        return x * power(x,y-1)

start = time.time()

print(power(x, y))

print(time.time() - start)