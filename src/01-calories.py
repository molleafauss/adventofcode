"""
https://adventofcode.com/2022/day/1#part2
Q1) Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

Q2) Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
"""
if __name__ == '__main__':
    elf = 0
    elf_calories = 0
    calories = []
    with open("01-calories.txt") as f:
        for l in f:
            l = l.strip()
            if not l:
                print("Elf n. {} holding {} calories".format(elf, elf_calories))
                calories.append([elf, elf_calories])
                elf_calories = 0
                elf += 1
                continue
            elf_calories += int(l)
    calories.append([elf, elf_calories])
    calories.sort(key=lambda x: x[1], reverse=True)
    print("Saw {} elves: maximum: {}".format(elf, calories[0]))
    top3 = [x[1] for x in calories[0:3]]
    print("First 3 elves: {}".format(sum(top3)))