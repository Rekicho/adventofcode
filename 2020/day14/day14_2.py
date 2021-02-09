import re
import itertools

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

        addr = int(m.group(1)) | int(mask.replace('X', '0'), 2)
        addr = bin(int(addr))[2:].zfill(36)

        for i in range(len(mask)):
            if mask[i] == 'X':
                addr = list(addr)
                addr[i] = 'X'
                addr = ''.join(addr)

        for it in itertools.product('10', repeat=addr.count('X')):
            new_addr = addr

            for opt in it:
                new_addr = new_addr.replace('X', opt, 1)

            memory[int(new_addr)] = int(m.group(2))

print(sum(memory.values()))