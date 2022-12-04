
file = open("list.txt", "r")

lines = file.read()

items = [[[int(elf) for elf in item.split("-")] for item in line.split(",")] for line in lines.split("\n")]

def check_range(arr):
    ranges = [] 
    for sets in arr:
        contains = []
        left = list(range(sets[0][0], sets[0][1] + 1))
        right = list(range(sets[1][0], sets[1][1] + 1))
        l = any(b in right for b in left)
        r = any(b in left for b in right)
        if l == 1:
            contains.append(l)
        if r == 1:
            contains.append(r)
        if len(contains) >= 1:
            ranges.append(contains)
    print(len(ranges))

check_range(items)

