from string import ascii_lowercase

with open("input.txt") as f:
    content = f.read()

content = content.split('\n\n')
content = [x.strip().split('\n') for x in content]

counter = 0

for group in content:
    for c in ascii_lowercase:
        if all(map(lambda x: c in x, group)):
            counter = counter + 1

print(counter)