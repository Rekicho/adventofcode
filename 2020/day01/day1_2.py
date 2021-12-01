with open("input.txt") as f:
    content = f.readlines()

content = [int(x.strip()) for x in content]

found = False

for x in content:
    for y in content:
        for z in content:
            if x == y or x == z or y == z:
                continue
            if x + y + z == 2020:
                found = True
                print(x * y * z)
                break
        if found:
            break
    if found:
        break