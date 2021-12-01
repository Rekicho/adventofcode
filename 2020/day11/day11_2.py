import copy

with open("input.txt") as f:
    lines = f.readlines()

lines = [list(x.strip()) for x in lines]

def checkOccupied(seats, j, i, jinc, iinc):
    if j < 0 or j >= len(seats) or i < 0 or i >= len(seats[j]):
        return 0

    if seats[j][i] == '#':
        return 1

    if seats[j][i] == 'L':
        return 0

    return checkOccupied(seats, j + jinc, i + iinc, jinc, iinc)

def countOccupiedAround(seats, j, i):
    count = 0

    for jj in range(j - 1, j + 2):
        for ii in range(i - 1, i + 2):
            if jj == j and ii == i:
                continue

            count = count + checkOccupied(seats, jj, ii, jj-j, ii-i)

    return count

while True:
    last = copy.deepcopy(lines)

    for j in range(len(lines)):
        for i in range(len(lines[j])):
            if last[j][i] == 'L' and countOccupiedAround(last, j, i) == 0:
                lines[j][i] = '#'

            if last[j][i] == '#' and countOccupiedAround(last, j, i) >= 5:
                lines[j][i] = 'L'

    if str(lines) == str(last):
        break

print(str(lines).count('#'))