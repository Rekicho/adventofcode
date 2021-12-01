with open("input.txt") as f:
    content = f.read()

content = content.split('\n\n')
content = [x.strip() for x in content]
necessary = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

counter = 0

for passport in content:
    if all(x in passport for x in necessary):
        counter = counter + 1

print(counter)