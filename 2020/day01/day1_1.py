with open("input.txt") as f:
    content = f.readlines()

content = [int(x.strip()) for x in content]

found = False

for x in content:
    for y in content:
        if x == y:
            continue
        if x + y == 2020:
            found = True
            print(x * y)
            break
    if found:
        break