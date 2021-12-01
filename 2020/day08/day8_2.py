import re

with open("input.txt") as f:
    lines = f.readlines()

lines = [x.strip() for x in lines]
inst = []
p = re.compile('(\w*) ((\+|-)\d*)')

for line in lines:
    m = p.match(line)
    inst.append((m.group(1),int(m.group(2))))

acc = 0

for i in range(len(lines)):
    inst_temp = inst[:]

    if inst_temp[i][0] == 'acc':
        continue

    elif inst_temp[i][0] == 'jmp':
        inst_temp[i] = ('nop', inst_temp[i][1])

    elif inst_temp[i][0] == 'nop':
        inst_temp[i] = ('jmp', inst_temp[i][1])

    visited = [0 for ele in range(len(inst_temp))]
    acc = 0
    pc = 0
    finish = False

    while True:
        if pc == len(inst_temp):
            finish = True
            break

        if visited[pc] != 0:
            break

        visited[pc] = visited[pc] + 1

        if inst_temp[pc][0] == 'acc':
            acc = acc + inst_temp[pc][1]
            pc = pc + 1

        elif inst_temp[pc][0] == 'jmp':
            pc = pc + inst_temp[pc][1]

        elif inst_temp[pc][0] == 'nop':
            pc = pc + 1

    if finish:
        break

print(acc)