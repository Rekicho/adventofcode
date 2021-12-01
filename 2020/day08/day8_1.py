import re

with open("input.txt") as f:
    lines = f.readlines()

lines = [x.strip() for x in lines]
inst = []
p = re.compile('(\w*) ((\+|-)\d*)')

for line in lines:
    m = p.match(line)
    inst.append((m.group(1),int(m.group(2))))

visited = [0 for ele in range(len(inst))]
acc = 0
pc = 0

while True:
    if visited[pc] != 0:
        break

    visited[pc] = visited[pc] + 1

    if inst[pc][0] == 'acc':
        acc = acc + inst[pc][1]
        pc = pc + 1

    elif inst[pc][0] == 'jmp':
        pc = pc + inst[pc][1]

    elif inst[pc][0] == 'nop':
        pc = pc + 1

print(acc)