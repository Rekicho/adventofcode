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
        invalid = numbers[i]

        for j in range(0, len(numbers)):
            sum = 0
            end = False
            for k in range(j, len(numbers)):
                if numbers[k] == invalid:
                    break

                sum += numbers[k]

                if sum == invalid:
                    end = k
                    break
                if sum > invalid:
                    break

            if end:
                print(min(numbers[j:k]) + max(numbers[j:k]))
                break
