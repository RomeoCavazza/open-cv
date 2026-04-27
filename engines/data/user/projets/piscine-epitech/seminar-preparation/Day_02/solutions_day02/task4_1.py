terms = 1_000_000
pi_approx = 0

for k in range(terms):
    pi_approx += ((-1)**k) / (2*k + 1)

pi_approx *= 4

print(f"{pi_approx:.6f}")
