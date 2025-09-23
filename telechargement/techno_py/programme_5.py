from PIL import Image
title = "pomme.jpg"
img = Image.open(title)
lx, ly = img.size

for y in range(ly):
    for x in range(lx):
        r,v,b = img.getpixel((x, y))
        r -= 2*r
        v -= 2*v
        b -= 2*b
        img.putpixel((x, y), (r, v, b))

_title = "negatatif_" + title
img.save(_title)
img.show()
