indice = "avbdewifax"
lettres = "]ykl[|dW_v"
i = 0
titi = ''
while i < len(indice):
    if ord(indice[i]) < 109:
       titi=titi+chr(ord(letres[i])+10)
    else:
        titi=titi+chr(ord(letres[i])-10)
