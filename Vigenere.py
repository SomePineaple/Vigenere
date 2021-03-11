#!/usr/bin/env python

mode = input('Enter a mode, either encrypt or decrypt: ')

text = ''

if mode == 'encrypt':
    text = input("Enter text to encrypt: ")
elif mode == 'decrypt':
    text = input('Enter some text to decrypt: ')
else:
    print('Improper mode')
    quit(1)

key = input("Enter a key (ie 'TESTKEY'): ")

print(mode + 'ing')


def shift_string(original, shift_amount):
    shifted = ""
    for i in range(len(original)):
        shifted += original[i - shift_amount]
    return shifted


def get_alphabets(original):
    generated_alphabets = []
    for i in range(len(original)):
        generated_alphabets.append(shift_string(original, i))
    return generated_alphabets


def get_letter_of_alphabet(letter, alphabet):
    for i in range(0, len(alphabet)):
        if str(letter) == alphabet[i]:
            return i
    return 69


alphabets = get_alphabets('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnop'
                          'qrstuvwxyz1234567890!@#$%^&*()\'" ,.:')


def get_encrypted_letter(original, key_letter):
    alphabet = alphabets[get_letter_of_alphabet(key_letter, alphabets[0])]
    new_letter = alphabet[get_letter_of_alphabet(original, alphabets[0])]
    return new_letter


def get_decrypted_letter(encrypted, key_letter):
    alphabet = alphabets[get_letter_of_alphabet(key_letter, alphabets[0])]
    new_letter = alphabets[0][get_letter_of_alphabet(encrypted, alphabet)]
    return new_letter


def encrypt(message, message_key):
    encrypted_message = ''
    for i in range(0, len(message)):
        original_char = message[i]
        key_letter = message_key[i % len(message_key)]
        new_char = get_encrypted_letter(original_char, key_letter)
        encrypted_message += new_char
    return encrypted_message


def decrypt(message, message_key):
    decrypted_message = ''
    for i in range(0, len(message)):
        encrypted_char = message[i]
        key_letter = message_key[i % len(message_key)]
        decrypted_message += get_decrypted_letter(encrypted_char, key_letter)
    return decrypted_message


new_text = ''

if mode == 'encrypt':
    new_text = encrypt(text, key)

else:
    new_text = decrypt(text, key)

if mode == 'decrypt':
    print('Decrypted text:\n' + new_text)
else:
    print('Encrypted text:\n' + new_text)
