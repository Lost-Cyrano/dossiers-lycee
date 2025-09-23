mots ='FORGES POIGNET CLAPET COFFRET CREPES ASTRILD FLORES COFFRET HANSES POULAIN'
a = 0
b = len(mots)
tutu =''
for i in range(0, int(len(mots)/2)):
    if (mots[i]==' '):
        tutu = tutu + mots[abs(a-b)]
    a = a + 2
    b = b -1