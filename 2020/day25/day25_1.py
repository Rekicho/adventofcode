with open("input.txt") as f:
    content = f.read().strip().split('\n')

public_keys = [int(key) for key in content]

i = 0

def transform(sub_number, loop_size):
    return pow(sub_number, loop_size, 20201227)

while True:
    res = transform(7, i)

    if res == public_keys[0]:
        private = i
        public = public_keys[1]
        break
        
    if res == public_keys[1]:
        private = i
        public = public_keys[0]
        break

    i = i+1

key = transform(public, private)

print(key)