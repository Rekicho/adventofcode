import re

with open("input.txt") as f:
    lines = f.readlines()

lines = [x.strip() for x in lines]

bag_dict = dict()

for line in lines:
    bags = re.findall(r'(\w* \w*) bag', line)

    if bags[1] != 'no other':
        bag_dict[bags[0]] = bags[1:]

    else:
        bag_dict[bags[0]] = []

counter = 0

for bag in bag_dict:
    found = False
    to_visit = set({bag})

    while len(to_visit) != 0:
        node = to_visit.pop()

        if 'shiny gold' in bag_dict[node]:
            found = True
            break

        to_visit.update(bag_dict[node])

    if found:
        counter = counter + 1


print(counter)