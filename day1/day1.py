import sys

file = open("list.txt", "r")

lines = file.read()

elves = [[int(line) for line in elf.split("\n") if line != ""] for elf in lines.split("\n\n")]

def f(number):
    return ("{:,}".format(number))

def find_largest(arr):
    max = arr[0]
    for i in arr:
        if i > max:
            max = i
    return max

# function to find n largest elements in a list
def find_n_largest(arr, n):
    nlargest = []
    for i in range(0, n):
        largest = find_largest(arr)
        nlargest.append(largest)
        arr.remove(largest)
    return nlargest

def sum(arr):
    sum = 0
    for i in arr:
        sum = sum + i
    return(sum)

arr = [sum(elf) for elf in elves]
lib = [{sum(elf): elves.index(elf)} for elf in elves]
answer = find_largest(arr)

# Part 1 Answer
for i in lib:
    if i.get(answer) != None:
        print(f"Elf #{i.get(answer)} is carrying the most with a total of {f(answer)} kcals")


# Part 2 Answer
top_elves = find_n_largest(arr, 3)
print(f"Top 3 elves: {top_elves}")
print(f"The top 3 Elves are carrying a total of {f(sum(top_elves))} kcals")
