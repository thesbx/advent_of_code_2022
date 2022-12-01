import sys

file = open("list.txt", "r")

lines = file.read()

elves = [[int(line) for line in elf.split("\n") if line != ""] for elf in lines.split("\n\n")]

def f(number):
    return ("{:,}".format(number))

def largest(arr, n):
    max = arr[0]
    for i in range(1, n):
        if arr[i] > max:
            max = arr[i]
    return max

def sum(arr):
    sum = 0
    for i in arr:
        sum += i
    return(sum)

arr = [sum(elf) for elf in elves]
lib = [{sum(elf): elves.index(elf)} for elf in elves]
answer = largest(arr, len(arr))

# Part 1 Answer
for i in lib:
    if i.get(answer) != None:
        print(f"Elf #{i.get(answer)} is carrying the most with a total of {f(answer)} kcals")


# Part 2 Answer
# function to find n largest elements in a list
def nlargest(arr, n):
    nlargest = []
    for i in range(0, n):
        largest = largest(arr, len(arr))
        nlargest.append(largest)
        arr.remove(largest)
    return nlargest

answer_two = sum(nlargest(arr, 3))
top_elves = nlargest(arr, 3)
print(f"Top 3 elves: {top_elves}")
print(f"The top 3 Elves are carrying a total of {f(answer_two)} kcals")
