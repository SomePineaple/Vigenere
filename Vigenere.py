mode = raw_input('Enter a mode, either encrypt or decrypt: ')

text = ''

if mode == 'encrypt':
    text = raw_input("Enter text to encrypt: ")
elif mode == 'decrypt':
    text = raw_input('Enter some text to decrypt: ')
else:
    print 'Improper mode'
    quit(1)

key = raw_input("Enter a key (ie 'TESTKEY'): ")

print mode + 'ing'

alphabets = ['ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:',
             'BCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:A',
             'CDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:AB',
             'DEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABC',
             'EFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCD',
             'FGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDE',
             'GHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEF',
             'HIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFG',
             'IJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGH',
             'JKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHI',
             'KLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJ',
             'LMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJK',
             'MNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKL',
             'NOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLM',
             'OPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMN',
             'PQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNO',
             'QRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOP',
             'RSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQ',
             'STUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQR',
             'TUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRS',
             'UVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRST',
             'VWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTU',
             'WXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUV',
             'XYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVW',
             'YZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWX',
             'Zabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXY',
             'abcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZ',
             'bcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZa',
             'cdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZab',
             'defghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabc',
             'efghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcd',
             'fghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcde',
             'ghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef',
             'hijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefg',
             'ijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefgh',
             'jklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghi',
             'klmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghij',
             'lmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijk',
             'mnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijkl',
             'nopqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklm',
             'opqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmn',
             'pqrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmno',
             'qrstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnop',
             'rstuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopq',
             'stuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqr',
             'tuvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrs',
             'uvwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrst',
             'vwxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstu',
             'wxyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuv',
             'xyz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvw',
             'yz1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwx',
             'z1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxy',
             '1234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz',
             '234567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1',
             '34567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz12',
             '4567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123',
             '567890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234',
             '67890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz12345',
             '7890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123456',
             '890!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567',
             '90!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz12345678',
             '0!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz123456789',
             '!@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890',
             '@#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!',
             '#$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@',
             '$%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#',
             '%^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$',
             '^&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%',
             '&*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^',
             '*()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&',
             '()\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*',
             ')\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*(',
             '\'" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()',
             '" ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'',
             ' ,.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'"',
             ',.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ',
             '.:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,',
             ':ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()\'" ,.'
             ]


def get_letter_of_alphabet(letter, alphabet):
    for i in range(0, len(alphabet)):
        if str(letter) == alphabet[i]:
            return i
    return 69


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
        if i % 25 == 0 and i > 10:
            encrypted_message += '\n'
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
    print 'Decrypted text:\n' + new_text.lower()
else:
    print 'Encrypted text:\n' + new_text

# h6$%(c)97'9dX9&Ab6*6ZC(AX'
# &75ESYka8)$W"a005#Z (c 6$
# %b!%(Z"(*5Y)85,%%Z@8^X,0C


# ai45yB*IGUxDP)@7Vut
