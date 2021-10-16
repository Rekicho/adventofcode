import re

with open("input.txt") as f:
    lines = f.readlines()

counter = 0

for line in lines:
    p = re.compile('(\d*)-(\d*) (.): (\w*)')
    m = p.match(line)
    low = int(m.group(1))
    high = int(m.group(2))
    letter = m.group(3)
    pwd = m.group(4)

    count = pwd.count(letter)

    if count >= low and count <= high:
        counter = counter + 1

print(counter)