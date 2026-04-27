### INPUT ###

raw = input("Enter a list of elements separated by spaces:\n")

### SPLIT INTO LIST ###

lst = raw.split()

### REMOVE DUPLICATES ###

result = []
for item in lst:
    if item not in result:
        result.append(item)

### OUTPUT ###

print(len(result))