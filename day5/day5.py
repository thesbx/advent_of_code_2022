file1 = open("map.txt", "r")
file2 = open("list.txt", "r")
map = file1.read()
commands = file2.read()

def merge_strings(arr, n):
    count = 0
    res = []
    for item in arr:
        if item == "":
            count += 1
            if count == n:
                res.append("")
                count = 0

        else:
            res.append(item)

    return res

list1 = [[item for item in merge_strings(lines.split(" "), 4)] for lines in map.split("\n")]

def sort_list(arr):
    res = [[],[],[],[],[],[],[],[],[]]
    for a in arr:
        for index, item in enumerate(a):
            res[index].append(item)
    return res

def remove_strings(arr):
    for a in arr:
        while "" in a:
            a.remove("")
        a.reverse()
    return arr

sorted_list = sort_list(list1)
stripped = remove_strings(sorted_list)

command = [[int(c) for c in command.split() if c.isdigit()] for command in commands.split("\n")]

def parse_commands(commands, arr):
    for command in commands:
        for i in range(command[0]):
            item = arr[command[1] - 1].pop()
            arr[command[2] - 1].append(item)
                

    return arr
new = parse_commands(command, stripped)
answer = []
for index, a in enumerate(new):
    answer.append(a[-1])

print(answer)



