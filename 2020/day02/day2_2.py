import re

with open("input.txt") as f:
    lines = f.readlines()

counter = 0

for line in lines:
    p = re.compile('(\d*)-(\d*) (.): (\w*)')
    m = p.match(line)
    first = int(m.group(1))
    last = int(m.group(2))
    letter = m.group(3)
    pwd = m.group(4)

    count = 0

    if pwd[first-1] == letter:
        count = count + 1

    if pwd[last-1] == letter:
        count = count + 1

    if count == 1:
        counter = counter + 1

print(counter)