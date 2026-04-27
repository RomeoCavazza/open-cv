### INPUT ###

ciphertext = input("Enter the Caesar-ciphered text: ")

### NORMALIZATION ###

ciphertext_lower = ciphertext.lower()

### FUNCTION TO DECRYPT WITH A GIVEN KEY ###

def decrypt_with_key(text, key):
    result = []
    for ch in text:
        if 'a' <= ch <= 'z':
            original_pos = ord(ch) - ord('a')
            shifted_pos = (original_pos - key) % 26
            new_ch = chr(ord('a') + shifted_pos)
            result.append(new_ch)
        else:
            result.append(ch)
    return "".join(result)

### TRY ALL KEYS ###

print("\n=== POSSIBLE DECRYPTIONS ===")
for key in range(1, 26):
    decrypted = decrypt_with_key(ciphertext_lower, key)
    print(f"Key {key:2d}: {decrypted}")
