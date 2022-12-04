
file = open("list.txt", "r")

lines = file.read()

items = [[[int(elf) for elf in item.split("-")] for item in line.split(",")] for line in lines.split("\n")]

def arr_contains(needle, haystack, strict):
    if strict:
        return all(n in haystack for n in needle)
    else:
        return any(n in haystack for n in needle)

def check_range(arr, strict):
    ranges = [] 
    for sets in arr:
        contains = []
        left = list(range(sets[0][0], sets[0][1] + 1))
        right = list(range(sets[1][0], sets[1][1] + 1))
        l = arr_contains(left, right, strict)
        r = arr_contains(right, left, strict)
        if l == 1:
            contains.append(l)
        if r == 1:
            contains.append(r)
        if len(contains) >= 1:
            ranges.append(contains)
    print(len(ranges))

check_range(items, True)
check_range(items, False)

