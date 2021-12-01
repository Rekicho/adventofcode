import re

with open("input.txt") as f:
    lines = f.readlines()

mask = ''
memory = {}

for line in lines:
    if line[1] == 'a':
        p = re.compile(r'mask = (\w*)')
        m = p.match(line)
        mask = m.group(1)
    
    else:
        p = re.compile(r'mem\[(\d*)\] = (\d*)')
        m = p.match(line)
        memory[int(m.group(1))] = int(m.group(2)) \
                                & int(mask.replace('X', '1'), 2) \
                                | int(mask.replace('X', '0'), 2)

print(sum(memory.values()))