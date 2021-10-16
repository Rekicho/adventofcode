import re

with open("input.txt") as f:
    lines = f.readlines()

lines = [x.strip() for x in lines]

bag_dict = dict()

for line in lines:
    bag = re.search(r'^(\w* \w*) bag', line).group(1)
    bags = re.findall(r'(\d*) (\w* \w*) bag', line)

    if bags[0][1] != 'no other':
        bag_dict[bag] = bags

    else:
        bag_dict[bag] = []

counter = -1 #Initial shiny gold bag does not count
to_visit = [(1, 'shiny gold')]

while len(to_visit) != 0:
    node = to_visit[0]
    to_visit = to_visit[1:]

    counter = counter + node[0]

    for bag in bag_dict[node[1]]:
        to_visit.append((node[0] * int(bag[0]),bag[1]))

print(counter)