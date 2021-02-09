with open("input.txt") as f:
    line = f.readline()
    numbers = line.split(',')
    numbers = [int(number) for number in numbers]

i = len(numbers) - 1

while i < 2020:
    if numbers.index(numbers[i]) == i:
        numbers.append(0)

    else:
        rev_numbers = numbers[:]
        rev_numbers.pop()
        rev_numbers.reverse()

        numbers.append(rev_numbers.index(numbers[i]) + 1)

    i += 1

print(numbers[2019])