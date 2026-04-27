import turtle
t = turtle.Turtle()
t.hideturtle()
turtle.tracer(0)

r = 120
petals = 36
step = 360 / petals

for _ in range(petals):
    t.circle(r, 360)
    t.right(step)

turtle.update()
turtle.exitonclick()