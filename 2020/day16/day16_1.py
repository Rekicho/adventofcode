import re

with open("input.txt") as f:
    content = f.read()

content = content.split('\n\n')

rules = content[0].strip().split('\n')
my_ticket = content[1].split(':\n')[1].strip().split('\n')[0].split(',')
other_tickets = content[2].split(':\n')[1].strip().split('\n')

ranges = []

for rule in rules:
    p = re.compile(r'(.*): (\d*)-(\d*) or (\d*)-(\d*)')
    m = p.match(rule)

    ranges.append([[int(m.group(2)), int(m.group(3))], [int(m.group(4)), int(m.group(5))]])

my_ticket = [int(field) for field in my_ticket]

for i in range(len(other_tickets)):
    fields = other_tickets[i].split(',')
    other_tickets[i] = [int(field) for field in fields]

error = 0

for ticket in other_tickets:
    for field in ticket:
        valid = False

        for rule in ranges:
            if (rule[0][0] <= field <= rule[0][1]) \
                or (rule[1][0] <= field <= rule[1][1]):
                valid = True
                break
        
        if not valid:
            error += field
            break

print(error) 