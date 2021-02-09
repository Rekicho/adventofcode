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

black_tiles = {}

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

    if black_tiles.get((x, y, z)):
        black_tiles[(x, y, z)] = not black_tiles[(x, y, z)]

    elif black_tiles.get((x, y, z)) == False:
        black_tiles[(x, y, z)] = not black_tiles[(x, y, z)]

    else:
        black_tiles[(x, y, z)] = True

print(sum([1 if black == True else 0 for black in black_tiles.values()]))