import re

with open("input.txt") as f:
    content = f.read()

content = content.split('\n\n')
content = [x.strip() for x in content]
necessary = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

counter = 0

for passport in content:
    if not all(x in passport for x in necessary):
        continue

    p = re.compile('byr:(\d*)')
    m = p.search(passport)

    if not m:
        continue

    byr = int(m.group(1))

    if not (byr >= 1920 and byr <= 2002):
        continue

    p = re.compile('iyr:(\d*)')
    m = p.search(passport)

    if not m:
        continue

    iyr = int(m.group(1))

    if not (iyr >= 2010 and iyr <= 2020):
        continue

    p = re.compile('eyr:(\d*)')
    m = p.search(passport)

    if not m:
        continue

    eyr = int(m.group(1))

    if not (eyr >= 2020 and eyr <= 2030):
        continue

    p = re.compile('hgt:(\d*)(cm|in)')
    m = p.search(passport)

    if not m:
        continue

    hgt = int(m.group(1))
    unit = m.group(2)

    if unit == 'cm':
        if not (hgt >= 150 and hgt <= 193):
            continue

    elif unit == 'in':
        if not (hgt >= 59 and hgt <= 76):
            continue

    else:
        continue

    p = re.compile('hcl:#(\w){6}')
    m = p.search(passport)
    
    if not m:
        continue

    p = re.compile('ecl:(amb|blu|brn|gry|grn|hzl|oth)')
    m = p.search(passport)
    
    if not m:
        continue
    
    p = re.compile('pid:(\d{9})(.*)')
    m = p.search(passport)

    if not m:
        continue

    if m.group(2):
        p = re.compile(' .*')
        m = p.match(m.group(2))

        if not m:
            continue
    
    counter = counter + 1

print(counter)