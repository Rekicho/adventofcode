with open("input.txt") as f:
    lines = f.readlines()

numbers = [int(x.strip()) for x in lines]
numbers.sort()

diffs = [0, 0, 0]
last = 0

for number in numbers:
    if number - last > 3:
        break

    diffs[number-last-1] = diffs[number-last-1] + 1
    last = number

diffs[2] = diffs[2] + 1 #device has +3

print(diffs[0] * diffs[2])
