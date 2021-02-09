import re

with open("input.txt") as f:
    content = f.read().strip()

content = content.split('\n\n')
messages = content[1].split('\n')

rules = {}

for rule in content[0].split('\n'):
    p = re.compile(r'(\d*): (.*)')
    m = p.match(rule)
    rules[int(m.group(1))] = m.group(2).replace('"', '')

def defineRule(number):
    if 'a' in rules[number] or 'b' in rules[number]:
        return rules[number]

    if '|' in rules[number]:
        parts = rules[number].split('|')
        parts = [part.strip() for part in parts]
        rule = ''

        for part in parts:
            groups = [int(s) for s in part.split(' ') if s.isdigit()]

            for group in groups:
                rule += defineRule(int(group))
                
            rule += '|'

        rule = '(' + rule[:-1] + ')'
        rules[number] = rule
        return rule

    groups = [int(s) for s in rules[number].split(' ') if s.isdigit()]
    rule = ''

    for group in groups:
        rule += defineRule(int(group))

    rules[number] = rule

    return rule

valid_rule = defineRule(0)
valid = 0

for message in messages:
    p = re.compile(valid_rule)
    
    if p.fullmatch(message):
        valid += 1

print(valid)