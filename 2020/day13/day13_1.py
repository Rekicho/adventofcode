import math

with open("input.txt") as f:
    depart = int(f.readline())
    words = f.readline().strip().split(',')

buses = []
wait = []

for word in words:
    try:
        bus = int(word)
    except ValueError:
        continue

    buses.append(bus)
    wait.append((math.ceil(depart / bus) * bus) - depart)

min_wait = min(wait)
min_index = wait.index(min_wait)
print(buses[min_index] * min_wait)