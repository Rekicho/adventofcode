with open("input.txt") as f:
    content = f.read()

content = content.split('\n\n')
content = [x.strip().replace('\n','') for x in content]

counter = 0

for group in content:
    counter = counter + len(set(group))

print(counter)