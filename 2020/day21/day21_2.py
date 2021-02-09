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

aller_mapping = {}

for i in range(len(all_aller)):
    for aller in all_aller:
        if aller in aller_mapping.values():
            continue

        possible = []

        for ingr in all_ingr:
            if ingr in aller_mapping:
                continue

            maybe = True

            for i in range(len(lines)):
                if (aller in aller_list[i]) and (not ingr in ingr_list[i]):
                    maybe = False
                    break

            if maybe:
                possible.append(ingr)

        if len(possible) == 1:
            aller_mapping[possible[0]] = aller
    
aller_mapping = {v: k for k, v in aller_mapping.items()}
keys = [key for key in aller_mapping.keys()]
keys.sort()

canonical = ''

for key in keys:
    canonical += aller_mapping[key] + ','

print(canonical[:-1])