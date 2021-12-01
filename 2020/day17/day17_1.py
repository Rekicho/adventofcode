with open("input.txt") as f:
    lines = f.readlines()

lines = [line.strip() for line in lines]

active_cubes = {}

for x in range(len(lines)):
    for y in range(len(lines[x])):
        if lines[x][y] == '#':
            active_cubes[(x, y, 0)] = True

for i in range(6):
    old = active_cubes.copy()

    for x in range(min(key[0] for key in old.keys()) - 1, max(key[0] for key in old.keys()) + 2):
        for y in range(min(key[1] for key in old.keys()) - 1, max(key[1] for key in old.keys()) + 2):
            for z in range(min(key[2] for key in old.keys()) - 1, max(key[2] for key in old.keys()) + 2):
                neighbours = 0

                for xx in range(x - 1, x + 2):
                    for yy in range(y - 1, y + 2):
                        for zz in range(z - 1, z + 2):
                            if xx == x and yy == y and zz == z:
                                continue

                            if old.get((xx, yy, zz)):
                                neighbours += 1

                active = old.get((x, y, z))

                if active and (not (neighbours == 2 or neighbours == 3)):
                    active_cubes[x, y, z] = False

                if (not active) and (neighbours == 3):
                    active_cubes[x, y, z] = True

print(sum([1 if active else 0 for active in active_cubes.values()]))