with open("input.txt") as f:
    cups = f.read().strip()

cups = [int(cup) for cup in cups]

current = 0

for i in range(100):
    cups_copy = cups[:]
    removed = []
    taken_from_front = 0
    for j in range(3):
        if current + 1 >= len(cups_copy):
            removed_cup = cups_copy.pop((current + 1) % len(cups_copy) - taken_from_front)
            taken_from_front += 1
        else:
            removed_cup = cups_copy.pop((current + 1) % len(cups_copy))
        removed.append(removed_cup)

    target = cups[current] - 1

    while target not in cups_copy:
        target -= 1

        if target < min(cups_copy):
            target = max(cups_copy)
            break

    destination = cups_copy.index(target) + 1

    for removed_cup in removed:
        cups_copy.insert(destination, removed_cup)
        destination += 1


    current = (cups_copy.index(cups[current]) + 1) % len(cups_copy)
    cups = cups_copy
    

while cups[0] != 1:
    first = cups.pop(0)
    cups.append(first)

res = ''

for cup in cups[1:]:
    res += str(cup)

print(res)