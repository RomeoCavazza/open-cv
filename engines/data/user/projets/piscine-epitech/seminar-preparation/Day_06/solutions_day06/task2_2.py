### CLEANING FUNCTION ###

def clean_string(text):
    cleaned = ""
    for char in text:
        if char.isalnum():
            cleaned += char.lower()
    return cleaned

### RECURSIVE PALINDROME ###

def is_palindrome(s):
    if len(s) <= 1:
        return True
    if s[0] != s[-1]:
        return False
    return is_palindrome(s[1:-1])

### MAIN PROGRAM ###

user_input = input("Enter a sentence: ")
normalized = clean_string(user_input)
print(is_palindrome(normalized))