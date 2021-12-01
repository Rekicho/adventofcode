from math import ceil

with open("input.txt") as f:
    content = f.readlines()

content = [x.strip() for x in content]

seats = []

for line in content:
    min_row = 0
    max_row = 127
    min_col = 0
    max_col = 7

    for char in line:
        if char == 'F':
            max_row = max_row - ceil((max_row - min_row) / 2)
        elif char == 'B':
            min_row = min_row + ceil((max_row - min_row) / 2)
        elif char == 'L':
            max_col = max_col - ceil((max_col - min_col) / 2)
        elif char == 'R':
            min_col = min_col + ceil((max_col - min_col) / 2)
    
    seat_id = min_row * 8 + min_col
    seats.append(seat_id)

seats.sort()

for i in range(1, len(seats)):
    if seats[i] == seats[i-1] + 2:
        print(seats[i] - 1)
        break