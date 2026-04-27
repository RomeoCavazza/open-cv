### INPUT MESSAGE ###

message = input("Enter a clear message: ")

### INPUT KEY ###

key_raw = input("Enter a key (1–25): ")
try:
    key = int(key_raw)
except ValueError:
    print("Key must be an integer.")
    raise SystemExit(1)

if key < 1 or key > 25:
    print("Key must be between 1 and 25.")
    raise SystemExit(1)

### NORMALIZATION ###

message_lower = message.lower()

### ENCRYPTION LOOP ###

encrypted_chars = []

for ch in message_lower:
    if 'a' <= ch <= 'z':
        original_pos = ord(ch) - ord('a')
        shifted_pos = (original_pos + key) % 26
        new_ch = chr(ord('a') + shifted_pos)
        encrypted_chars.append(new_ch)
    else:
        encrypted_chars.append(ch)

### BUILD ENCRYPTED MESSAGE ###

encrypted_message = "".join(encrypted_chars)

### OUTPUT ###

print("Encrypted message:", encrypted_message)
