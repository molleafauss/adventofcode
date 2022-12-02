

# 1st: A for Rock, B for Paper, and C for Scissors
# 2nd: X for Rock, Y for Paper, and Z for Scissors
# score:
#   shape:  1 for Rock, 2 for Paper, and 3 for Scissors
#   result: 0 if you lost, 3 if the round was a draw, and 6 if you won

SCORE_CHOICE = {
    "R": 1,
    "P": 2,
    "S": 3,
}

SCORE_RESULT = {
    "RR": 3,
    "RP": 6,
    "RS": 0,
    "PR": 0,
    "PP": 3,
    "PS": 6,
    "SR": 6,
    "SP": 0,
    "SS": 3,
}

OPPONENT_PLAY = {
    "A": "R",
    "B": "P",
    "C": "S",
}

YOUR_PLAY = {
    "X": "R",
    "Y": "P",
    "Z": "S",
}

if __name__ == '__main__':
    score = 0
    with open("../files/day02-strategy.txt") as f:
        for l in f:
            l = l.strip()
            if not l:
                break
            (a, b) = l.split(" ")
            opponent = OPPONENT_PLAY[a]
            you = YOUR_PLAY[b]
            score += SCORE_CHOICE[you] + SCORE_RESULT[opponent + you]
    print(f"Resulting score: {score}")