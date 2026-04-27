terms = 500_000 
pi_approx = 3.0
sign = 1

for n in range(2, 2*terms, 2):
    pi_approx += sign * (4 / (n * (n+1) * (n+2)))
    sign *= -1   

print(f"{pi_approx:.6f}")