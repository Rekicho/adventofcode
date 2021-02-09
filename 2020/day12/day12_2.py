import copy

with open("input.txt") as f:
    lines = f.readlines()

lines = [x.strip() for x in lines]

waypoint_x = 10
waypoint_y = 1

x = 0
y = 0

for line in lines:
    old_waypoint_x = waypoint_x
    old_waypoint_y = waypoint_y

    if line[0] == 'N':
        waypoint_y += int(line[1:])
    if line[0] == 'S':
        waypoint_y -= int(line[1:])
    if line[0] == 'E':
        waypoint_x += int(line[1:])
    if line[0] == 'W':
        waypoint_x -= int(line[1:])
    if line[0] == 'F':
        x += int(line[1:]) * waypoint_x
        y += int(line[1:]) * waypoint_y
    if line[0] == 'L' or line[0] == 'R':
        inc = 1 if line[0] == 'R' else -1
        direction = int((inc * (int(line[1:]) / 90)) % 4)
        if direction == 1:
            waypoint_x = old_waypoint_y
            waypoint_y = -old_waypoint_x
        elif direction == 2:
            waypoint_x = -old_waypoint_x
            waypoint_y = -old_waypoint_y
        elif direction == 3:
            waypoint_x = -old_waypoint_y
            waypoint_y = old_waypoint_x

print(abs(x) + abs(y))