from functools import reduce

with open("input.txt") as f:
    content = f.readlines()

content = [x.strip() for x in content]

incs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
counters = []

for i in range(0, len(incs)):
    counter = 0
    x = 0
    y = 0

    for j in range(0, len(content)):
        if y >= len(content):
            break

        if content[y][x] == '#':
            counter = counter + 1

        x = (x + incs[i][0]) % len(content[0])
        y = y + incs[i][1]

    counters.append(counter)

print(counters)
print(reduce(lambda a,b: a * b, counters))