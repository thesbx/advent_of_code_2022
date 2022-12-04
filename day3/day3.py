from itertools import islice 
file = open("list.txt", "r")
lines = file.read()

items = [[char for char in line] for line in lines.split("\n")]

a_z = {
    "a": 1,
    "b": 2,
    "c": 3,
    "d": 4,
    "e": 5,
    "f": 6,
    "g": 7,
    "h": 8,
    "i": 9,
    "j": 10,
    "k": 11,
    "l": 12,
    "m": 13,
    "n": 14,
    "o": 15,
    "p": 16,
    "q": 17,
    "r": 18,
    "s": 19,
    "t": 20,
    "u": 21,
    "v": 22,
    "w": 23,
    "x": 24,
    "y": 25,
    "z": 26
}

A_Z = {
    "A": 27,
    "B": 28,
    "C": 29,
    "D": 30,
    "E": 31,
    "F": 32,
    "G": 33,
    "H": 34,
    "I": 35,
    "J": 36,
    "K": 37,
    "L": 38,
    "M": 39,
    "N": 40,
    "O": 41,
    "P": 42,
    "Q": 43,
    "R": 44,
    "S": 45,
    "T": 46,
    "U": 47,
    "V": 48,
    "W": 49,
    "X": 50,
    "Y": 51,
    "Z": 52
}

def split_list(arr):
    half = len(arr)//2
    return arr[:half], arr[half:]

def common_char_in_list(array):
    list1, list2 = split_list(array)
    for char1 in list1:
        for char2 in list2:
            if char1 == char2:
                return char1

def get_index(char):
    if char in a_z:
        return a_z[char]
    elif char in A_Z:
        return A_Z[char]
    else:
        return 0

def sum(arr):
    """
    Sums all the numbers in a list.
    """
    sum = 0
    for i in arr:
        sum = sum + i
    return(sum)

def create_batches(arr,n):
 if n < 1:
     raise ValueError("n must be at least one")
 it = iter(arr)
 while (batch := list(islice(it, n))):
     yield batch 

group = create_batches(items, 3)
print(list(group))
answer = []
for item in items:
    common_char = common_char_in_list(item)
    index = get_index(common_char)
    answer.append(index)

print(sum(answer))


