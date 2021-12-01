with open("input.txt") as f:
    line = f.readline()
    numbers = line.split(',')
    numbers = [int(number) for number in numbers]

previous = {}

for i in range(len(numbers) - 1):
    previous[numbers[i]] = i + 1
    
last = numbers[len(numbers) - 1]
new_last = None

for i in range(len(numbers) - 1, 30000000 - 1):
    if last not in previous:
        new_last = 0
    else:
        new_last = i - previous[last] + 1
    
    previous[last] = i + 1
    last = new_last

print(last)