file = open("list.txt", "r")
lines = file.read()

elves = [[int(line) for line in elf.split("\n") if line != ""] for elf in lines.split("\n\n")]

def f(number):
    """
    Formats a number with comma separation.
    """
    return ("{:,}".format(number))

def find_largest(arr):
    """
    Finds the largest number in a list.
    """
    max = arr[0]
    for i in arr:
        if i > max:
            max = i
    return max

def find_n_largest(arr, n):
    """
    Finds the n largest numbers in a list.
    """
    nlargest = []
    for i in range(0, n):
        largest = find_largest(arr)
        nlargest.append(largest)
        arr.remove(largest)
    return nlargest

def sum(arr):
    """
    Sums all the numbers in a list.
    """
    sum = 0
    for i in arr:
        sum = sum + i
    return(sum)

# Array of the largest numbers from the sub arrays.
arr = [sum(elf) for elf in elves]

# Dictionary of the largest numbers from the sub arrays as keys and the index of the sub array as values.
lib = [{sum(elf): elves.index(elf)} for elf in elves]

# Part 1 Answer
answer = find_largest(arr)
for i in lib:
    if i.get(answer) != None:
        print(f"Elf #{i.get(answer)} is carrying the most with a total of {f(answer)} kcals")


# Part 2 Answer
top_elves = find_n_largest(arr, 3)
print(f"Top 3 elves: {top_elves}")
print(f"The top 3 Elves are carrying a total of {f(sum(top_elves))} kcals")
