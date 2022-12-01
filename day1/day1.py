import sys

file = open("list.txt", "r")

lines = file.read()

elves = [[int(line) for line in elf.split("\n") if line != ""] for elf in lines.split("\n\n")]

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
top3 = []


for i in lib:
    if i.get(answer) != None:
        print(f"Elf #{i.get(answer)} is carrying {answer}kcals")

def findTop3(arr):
    third = first = second = -sys.maxsize

    for i in range(0, len(arr)):
        if (arr[i] > first):
                   third = second
                   second = first
                   first = arr[i]
        elif (arr[i] > second):
                   third = second
                   second = arr[i]
        elif (arr[i] > third):
                   third = arr[i]
    print(first + second + third)

findTop3(arr)
