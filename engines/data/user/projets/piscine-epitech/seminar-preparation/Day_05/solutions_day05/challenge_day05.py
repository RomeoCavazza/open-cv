import random
import time

n = 1000000

start = time.time()

random_list = random.sample(range(1,1000001), n)
random_list.sort()
print(random_list)

print(time.time()- start)