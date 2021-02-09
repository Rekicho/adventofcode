import re

with open("input.txt") as f:
    lines = f.readlines()

lines = [line.strip().replace(' ', '') for line in lines]

def calc_expr(expr):
    result = int(expr[0])
    i = 1

    while i < len(expr) and expr[i].isdigit():
        result = result * 10 + int(expr[i])
        i += 1

    while i < len(expr):
        number = int(expr[i + 1])
        j = i + 2

        while j < len(expr) and expr[j].isdigit():
            number = number * 10 + int(expr[j])
            j += 1

        if expr[i] == '*':
            result *= number

        elif expr[i] == '+':
            result += number

        i = j

    return str(result)

def calc(expr):
    p = re.compile(r'(.*)\*(.*)')
    m = p.search(expr)

    if m:
        return str(int(calc(m.group(1))) * int(calc(m.group(2))))

    return calc_expr(expr)

homework = 0

for line in lines:
    while True:
        p = re.compile(r'\(([^\(\)]*)\)')
        m = p.search(line)

        if m:
            line = line.replace('(' + m.group(1) + ')', calc(m.group(1)))

        else:
            homework += int(calc(line))
            break

print(homework)