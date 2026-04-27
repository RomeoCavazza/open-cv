### PROPER WAY ###

simple = input("Write a string: ")
double = ""

for i in simple:
    double = double + i*2
print(double)

### OUTPUT ### 
# ttaaxxii

### COMMON MISTAKES ###

string = input("Write a string: ")
print (string*2)

### OUTPUT ### 
# taxitaxi

single = input("Write a string: ")
for i in single:
    print(i* 2)

### OUTPUT ###
# tt
# aa
# xx
# ii