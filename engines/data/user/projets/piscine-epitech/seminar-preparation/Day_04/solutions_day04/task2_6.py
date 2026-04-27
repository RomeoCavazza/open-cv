### INPUT ###

raw = input("Enter an integer n (>= 2): ").strip()

### VALIDATION ###

try:
    n = int(raw)
except ValueError:
    print("Invalid input: please enter an integer.")
    raise SystemExit(1)

if n < 2:
    print("Nothing to do (n must be >= 2).")
    raise SystemExit(0)

### RANGE DEFINITION ###

start_i = 2
end_i_inclusive = n // 2

### MAIN LOOP OVER BASES ###

for i in range(start_i, end_i_inclusive + 1):

    multiples = []
    m = i
    while m < n:
        multiples.append(m)
        m += i

    multiples.reverse()  

    ### OUTPUT PER LINE ###

    print(" ".join(str(x) for x in multiples))
