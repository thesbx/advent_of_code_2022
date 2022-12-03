file = open("list.txt", "r")
lines = file.read()

items = [[char for char in line] for line in lines.split("\n")]

def split_list(arr):
    half = len(arr)//2
    return [arr[:half], arr[half:]]


for item in items:
    print(split_list(item))

