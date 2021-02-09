import copy

with open("input.txt") as f:
    lines = f.readlines()

lines = [x.strip() for x in lines]

x = 0
y = 0

directions = [[1, 0], [0, -1], [-1, 0], [0, 1]]
curr_dir = 0

for line in lines:
    if line[0] == 'N':
        y += int(line[1:])
    if line[0] == 'S':
        y -= int(line[1:])
    if line[0] == 'E':
        x += int(line[1:])
    if line[0] == 'W':
        x -= int(line[1:])
    if line[0] == 'F':
        x += int(line[1:]) * directions[curr_dir][0]
        y += int(line[1:]) * directions[curr_dir][1]
    if line[0] == 'L':
        curr_dir = int((curr_dir - (int(line[1:]) / 90)) % len(directions))
    if line[0] == 'R':
        curr_dir = int((curr_dir + (int(line[1:]) / 90)) % len(directions))

print(abs(x) + abs(y))