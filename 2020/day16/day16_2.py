import re

with open("input.txt") as f:
    content = f.read()

content = content.split('\n\n')

rules = content[0].strip().split('\n')
my_ticket = content[1].split(':\n')[1].strip().split('\n')[0].split(',')
other_tickets = content[2].split(':\n')[1].strip().split('\n')

rule_names = []
ranges = []

for rule in rules:
    p = re.compile(r'(.*): (\d*)-(\d*) or (\d*)-(\d*)')
    m = p.match(rule)

    rule_names.append(m.group(1))
    ranges.append([[int(m.group(2)), int(m.group(3))], [int(m.group(4)), int(m.group(5))]])

my_ticket = [int(field) for field in my_ticket]

for i in range(len(other_tickets)):
    fields = other_tickets[i].split(',')
    other_tickets[i] = [int(field) for field in fields]

valid_tickets = [my_ticket]

for ticket in other_tickets:
    valid_ticket = True

    for field in ticket:
        valid = False

        for rule in ranges:
            if (rule[0][0] <= field <= rule[0][1]) \
                or (rule[1][0] <= field <= rule[1][1]):
                valid = True
                break
        
        if not valid:
            valid_ticket = False
            break

    if valid_ticket:
        valid_tickets.append(ticket)

fields_rules = {}

while len(fields_rules) != len(ranges):
    for j in range(len(ranges)):
        if j in fields_rules:
                continue

        valid_index = []
        for i in range(len(valid_tickets[0])):
            if i in fields_rules.values():
                continue

            valid = True

            for ticket in valid_tickets:
                if not ((ranges[j][0][0] <= ticket[i] <= ranges[j][0][1]) \
                    or (ranges[j][1][0] <= ticket[i] <= ranges[j][1][1])):    
                    valid = False
                    break
            
            if valid:
                valid_index.append(i)

        if len(valid_index) == 1:
            fields_rules[j] = valid_index[0]
            break
            
value = 1

for i in range(len(rule_names)):
    if rule_names[i].startswith('departure'):
        value *= my_ticket[fields_rules[i]]

print(value)