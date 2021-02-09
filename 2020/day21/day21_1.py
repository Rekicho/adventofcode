with open("input.txt") as f:
    lines = f.readlines()

ingr_list = []
aller_list = []
all_ingr = []
all_aller = []

for line in lines:
    parts = line.strip().split(' (contains ')
    ingr = parts[0].split(' ')
    aller = parts[1][:-1].split(', ')

    ingr_list.append(ingr)
    aller_list.append(aller)

    all_ingr += ingr
    all_aller += aller

all_aller = set(all_aller)
all_ingr = set(all_ingr)

not_aller = []

for ingr in all_ingr:
    found = False

    for aller in all_aller:
        maybe = True

        for i in range(len(lines)):
            if (aller in aller_list[i]) and (not ingr in ingr_list[i]):
                maybe = False

        if maybe:
            found = True
            break

    if not found:
        not_aller.append(ingr)

count = 0

for ingr in not_aller:
    count += str(ingr_list).count("'" + ingr + "'")

print(count)