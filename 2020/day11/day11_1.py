import copy

with open("input.txt") as f:
    lines = f.readlines()

lines = [list(x.strip()) for x in lines]

def countOccupiedAround(seats, j, i):
    count = 0

    if not(j == 0 or i == 0 or seats[j-1][i-1] != '#'):
        count = count + 1
    if not(j == 0 or seats[j-1][i] != '#'):
        count = count + 1
    if not(j == 0 or i == len(seats[j]) - 1 or seats[j-1][i+1] != '#'):
        count = count + 1
    if not(i == 0 or seats[j][i-1] != '#'):
        count = count + 1
    if not(i == len(seats[j]) - 1 or seats[j][i+1] != '#'):
        count = count + 1
    if not(j == len(seats) - 1 or i == 0 or seats[j+1][i-1] != '#'):
        count = count + 1
    if not(j == len(seats) - 1 or seats[j+1][i] != '#'):
        count = count + 1
    if not(j == len(seats) - 1 or i == len(seats[j]) - 1 or seats[j+1][i+1] != '#'):
        count = count + 1

    return count

while True:
    last = copy.deepcopy(lines)

    for j in range(len(lines)):
        for i in range(len(lines[j])):
            if last[j][i] == 'L' and countOccupiedAround(last, j, i) == 0:
                lines[j][i] = '#'

            if last[j][i] == '#' and countOccupiedAround(last, j, i) >= 4:
                lines[j][i] = 'L'

    if str(lines) == str(last):
        break

print(str(lines).count('#'))