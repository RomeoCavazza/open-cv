n = int(input('Enter number of times: '))
for k in range(1, n+1):
    val = int('1' * k)
    print(f'{val} + {val} = {val * val}')