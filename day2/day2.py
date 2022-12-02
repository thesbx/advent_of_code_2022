guide = open("list.txt", "r")

lines = guide.read()

rounds = [[line for line in round.split(" ")] for round in lines.split("\n")]

rock_paper_scissors = {
    "A": 1,
    "B": 2,
    "C": 3,
}

our_scores = {
    "X": 1,
    "Y": 2,
    "Z": 3,
}

def get_winner(round):
    openent_score = rock_paper_scissors[round[0]]
    our_score = our_scores[round[1]]

    if openent_score == 1 and our_score == 2 or openent_score == 2 and our_score == 3 or openent_score == 3 and our_score == 1:
        return 6 + our_score
    elif openent_score == 1 and our_score == 3 or openent_score == 2 and our_score == 1 or openent_score == 3 and our_score == 2:
        return 0 + our_score
    else:
        return 3 + our_score

def sum(arr):
    """
    Sums all the numbers in a list.
    """
    sum = 0
    for i in arr:
        sum = sum + i
    return(sum)


scores = []
for round in rounds:
    scores.append(get_winner(round))

print(f"Answer to round #1: {sum(scores)}")

# Round 2

guide = {
    "X": 0,
    "Y": 3,
    "Z": 6,
}

def determine_score(round):
    result = guide[round[1]]
    opponent_score = rock_paper_scissors[round[0]]
    if opponent_score == 1 and result == 6:
        return 2 + result
    elif opponent_score == 2 and result == 6:
        return 3 + result
    elif opponent_score == 3 and result == 6:
        return 1 + result
    elif opponent_score == 1 and result == 3:
        return 1 + result
    elif opponent_score == 2 and result == 3:
        return 2 + result
    elif opponent_score == 3 and result == 3:
        return 3 + result
    elif opponent_score == 1 and result == 0:
        return 3 + result
    elif opponent_score == 2 and result == 0:
        return 1 + result
    elif opponent_score == 3 and result == 0:
        return 2 + result

new_scores = []
for round in rounds:
    new_scores.append(determine_score(round))

print(sum(new_scores))