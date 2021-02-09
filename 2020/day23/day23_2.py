with open("input.txt") as f:
    cups_list = f.read().strip()

cups_list = [int(cup) for cup in cups_list]

cups = [0] * 1000001

for i, cup in enumerate(cups_list):
    cups[cup] = cups_list[i + 1] if i + 1 < len(cups_list) else 10

for i in range(max(cups_list) + 1, 1000000):
    cups[i] = i + 1

cups[1000000] = cups_list[0]
current = cups_list[0]

for i in range(10000000):
    dest = current
    next1 = cups[current]
    next2 = cups[next1]
    next3 = cups[next2]

    dest -= 1
    if dest == 0:
        dest = 1000000

    while dest == next1 or dest == next2 or dest == next3:
        dest -= 1
        if dest == 0:
            dest = 1000000

    cups[current] = cups[next3]
    cups[next3] = cups[dest]
    cups[dest] = next1

    current = cups[current]

value = cups[1] * cups[cups[1]]
print(value)