with open("input.txt") as f:
    lines = f.readlines()

instr = []

for line in lines:
    line = line.strip()
    i = 0

    new_instr = []

    while i < len(line):
        if line[i] == 'e' or line[i] == 'w':
            new_instr.append(line[i])

        else:
            new_instr.append(line[i] + line[i + 1])
            i += 1

        i += 1

    instr.append(new_instr)

black_tiles = set()

for line in instr:
    x = 0
    y = 0
    z = 0

    for coord in line:
        if coord == 'e':
            x += 1
            y -= 1

        elif coord == 'se':
            y -= 1
            z += 1

        elif coord == 'sw':
            x -= 1
            z += 1

        elif coord == 'w':
            x -= 1
            y += 1

        elif coord == 'nw':
            y += 1
            z -= 1

        elif coord == 'ne':
            x += 1
            z -= 1

    if (x, y, z) in black_tiles:
        black_tiles.remove((x, y, z))

    else:
        black_tiles.add((x, y, z))

def count_neighbours(x, y, z):
    n = 0

    for (xx, yy, zz) in [(x+1,y-1,z), (x,y-1,z+1), (x-1,y,z+1), (x-1,y+1,z), (x,y+1,z-1), (x+1,y,z-1)]:
        if (xx, yy, zz) in black_tiles:
            n += 1

    return n

for i in range(100):
    new_black = black_tiles.copy()
    possible = set()

    for (x, y, z) in black_tiles:
        possible.add((x, y, z))

        for (xx, yy, zz) in [(x+1,y-1,z), (x,y-1,z+1), (x-1,y,z+1), (x-1,y+1,z), (x,y+1,z-1), (x+1,y,z-1)]:
            possible.add((xx,yy,zz))

    for (x, y, z) in possible:
        n = count_neighbours(x, y, z)

        if (x, y, z) in black_tiles and (n == 0 or n > 2):
            new_black.remove((x, y, z))

        elif (x, y, z) not in black_tiles and n == 2:
            new_black.add((x, y, z))

    black_tiles = new_black.copy()

print(len(black_tiles))