with open("input.txt") as f:
    lines = f.readlines()

numbers = [int(x.strip()) for x in lines]

for i in range(25, len(numbers)):
    found = False
    for j in range(i-25,i):
        if found:
            break
        for k in range(i-25,i):
            if j == k:
                continue
            if numbers[j] + numbers[k] == numbers[i]:
                found = True

    if not found:
        print(numbers[i])
