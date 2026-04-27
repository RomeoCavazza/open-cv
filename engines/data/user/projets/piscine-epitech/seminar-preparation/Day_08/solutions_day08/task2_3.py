import turtle
t = turtle.Turtle()
t.color("black")
    
def draw_polygon(sides,length):
    for _ in range(sides):
        t.forward(length) 
        t.left(360 / sides)

s = int(input("How many sides do you want your polygon to have: "))
l = int(input("How long do you want your polygon to be: "))
draw_polygon(s,l)