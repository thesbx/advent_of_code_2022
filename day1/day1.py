import sys

file = open("list.txt", "r")

lines = file.read()

elves = [[int(line) for line in elf.split("\n") if line != ""] for elf in lines.split("\n\n")]

def f(number):
    return ("{:,}".format(number))

def find_largest(arr, n):
    max = arr[0]
    for i in range(1, n):
        if arr[i] > max:
            max = arr[i]
    return max

def sum(arr):
    sum = 0
    for i in arr:
        sum = sum + i
    return(sum)

arr = [sum(elf) for elf in elves]
lib = [{sum(elf): elves.index(elf)} for elf in elves]
answer = find_largest(arr, len(arr))

# Part 1 Answer
for i in lib:
    if i.get(answer) != None:
        print(f"Elf #{i.get(answer)} is carrying the most with a total of {f(answer)} kcals")


# Part 2 Answer
# function to find n largest elements in a list
def find_n_largest(arr, n):
    collection = []
    for i in range(0, n):
        max = 0
        for j in range(len(arr)):
            if arr[j] > max:
                max = arr[j]
        arr.remove(max)
        collection.append(max)
    return(collection)

answer_two = sum(find_n_largest(arr, 3))
print(f"The top 3 Elves are carrying a total of {f(answer_two)} kcals")
