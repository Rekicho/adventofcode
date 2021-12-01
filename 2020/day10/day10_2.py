with open("input.txt") as f:
    lines = f.readlines()

numbers = [int(x.strip()) for x in lines]
numbers.append(0)
numbers.sort()
numbers.append(numbers[len(numbers)-1] + 3)

poss = [False for i in range(len(numbers))]

def calcPoss(num):
    if num == len(numbers) - 1:
        return 1

    if poss[num]:
        return poss[num]

    counter = 0

    for i in range(num + 1, len(numbers)):
        if numbers[i] - numbers[num] > 3:
            break

        else:
            counter = counter + calcPoss(i)

    poss[num] = counter
    return counter

print(calcPoss(numbers[0]))