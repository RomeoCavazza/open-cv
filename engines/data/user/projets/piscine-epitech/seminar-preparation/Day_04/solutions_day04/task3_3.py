### INPUT MODE ###

mode = input("Do you want to encrypt or decrypt? (e/d): ").strip().lower()
if mode not in ("e", "d"):
    print("Invalid choice. Use 'e' for encrypt or 'd' for decrypt.")
    raise SystemExit(1)

### INPUT MESSAGE ###

message = input("Enter your message: ")

### INPUT KEYWORD ###

keyword = input("Enter the keyword: ").lower()
if not keyword.isalpha():
    print("Keyword must only contain letters (a-z).")
    raise SystemExit(1)

### NORMALIZATION ###

message_lower = message.lower()

### HELPER FUNCTION TO SHIFT A LETTER ###

def shift_letter(letter, shift, encrypt=True):
    if 'a' <= letter <= 'z':
        pos = ord(letter) - ord('a')
        if encrypt:
            new_pos = (pos + shift) % 26
        else:
            new_pos = (pos - shift) % 26
        return chr(ord('a') + new_pos)
    else:
        return letter

### ENCRYPTION/DECRYPTION LOOP ###

result_chars = []
key_index = 0
for ch in message_lower:
    if 'a' <= ch <= 'z':
        key_letter = keyword[key_index % len(keyword)]
        shift = ord(key_letter) - ord('a')
        result_chars.append(shift_letter(ch, shift, encrypt=(mode == 'e')))
        key_index += 1
    else:
        result_chars.append(ch)

### RESULT ###

result = "".join(result_chars)
print(result)
