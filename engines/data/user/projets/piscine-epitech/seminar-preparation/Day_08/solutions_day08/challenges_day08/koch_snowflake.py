import turtle
t = turtle.Turtle()

def koch(l,n):
    if n == 0:
        t.forward(l)
    else:
        koch(l, n-1)
        t.left(60)
        koch(l, n-1)
        t.right(120)
        koch(l, n-1)
        t.left(60)
        koch(l, n-1)

for _ in range(3):
    koch(240, 4)
    t.right(120)
