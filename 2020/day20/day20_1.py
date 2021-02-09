import re

with open("input.txt") as f:
    content = f.read().strip()

lines = content.split('\n\n')
tiles = {}
p = re.compile(r'Tile (\d*)')

for tile in lines:
    tile = tile.split(':\n')
    m = p.match(tile[0])
    tile[1] = [line.replace('#', '1').replace('.', '0') for line in tile[1].split('\n')]
    tiles[int(m.group(1))] = tile[1]
    
corners = {}

for tile_n in tiles:
    tile = tiles[tile_n]
    corners[tile_n] = [int(tile[0], 2), int(tile[0][::-1], 2),\
                        int(tile[len(tile)-1], 2), int(tile[len(tile)-1][::-1], 2)]

    left = ''.join([line[0] for line in tile])
    right = ''.join([line[len(line)-1] for line in tile])

    corners[tile_n].extend([int(left, 2), int(left[::-1], 2),\
                            int(right, 2), int(right[::-1], 2)])

corner_value = 1

for i in corners:
    unique = 0
    for tile in corners[i]:
        found = False

        for j in corners:
            if i == j:
                continue

            if tile in corners[j]:
                found = True
                break

        if not found:
            unique += 1

    if unique == 4:
        corner_value *= i

print(corner_value)