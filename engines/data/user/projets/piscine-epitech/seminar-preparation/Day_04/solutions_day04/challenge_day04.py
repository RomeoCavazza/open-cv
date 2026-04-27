### INPUT ###

raw = input("Enter an integer and a string: \n").rstrip()

### SPLIT INTO (INT, STRING) WITH MAXSPLIT=1 ###

parts = raw.split(maxsplit=1)
if not parts:
    print("Invalid input.")
    raise SystemExit(1)

int_part = parts[0]
str_part = parts[1] if len(parts) > 1 else ""

### VALIDATE INTEGER ###

try:
    n = int(int_part)
except ValueError:
    print("First token must be an integer.")
    raise SystemExit(1)

if n == 0:
    raise SystemExit(0)

vowels = set("aeiouyAEIOUY")
has_vowel = any(ch in vowels for ch in str_part)

### DECISION LOGIC ###

if has_vowel:
    print(n)
elif n >= 42:
    print(n)
else:
    print(str_part)
