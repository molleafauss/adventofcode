from utils import Parser, loader

# https://adventofcode.com/2022/day/2
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

PLAY_ROUND2 = {k[0] + str(v): k[1] for k,v in SCORE_RESULT.items()}

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

SCORE_ROUND2 = {
    "X": 0,
    "Y": 3,
    "Z": 6,
}


class Day02(Parser):
    def __init__(self):
        self.score1 = 0
        self.score2 = 0

    def parse(self, line: str):
        (a, b) = line.split(" ")
        opponent = OPPONENT_PLAY[a]
        you = YOUR_PLAY[b]
        round2 = PLAY_ROUND2[opponent + str(SCORE_ROUND2[b])]
        self.score1 += SCORE_CHOICE[you] + SCORE_RESULT[opponent + you]
        self.score2 += SCORE_CHOICE[round2] + SCORE_ROUND2[b]

    def solve(self):
        print(f"Resulting score (part 1): {self.score1}")
        print(f"Resulting score (part 2): {self.score2}")


if __name__ == '__main__':
    loader("../files/day02-strategy.txt", Day02())
