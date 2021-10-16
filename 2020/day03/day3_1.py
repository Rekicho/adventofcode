with open("input.txt") as f:
    content = f.readlines()

content = [x.strip() for x in content]

counter = 0
x = 0
y = 0

for y in range(0, len(content)):
    if content[y][x] == '#':
        counter = counter + 1

    x = (x + 3) % len(content[0])

print(counter)