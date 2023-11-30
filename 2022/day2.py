import enum


class RPS(enum.Enum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3

    def beats(self):
        if self == RPS.ROCK:
            return RPS.SCISSORS
        if self == RPS.PAPER:
            return RPS.ROCK
        if self == RPS.SCISSORS:
            return RPS.PAPER

    def is_beaten_by(self):
        if self == RPS.ROCK:
            return RPS.PAPER
        if self == RPS.PAPER:
            return RPS.SCISSORS
        if self == RPS.SCISSORS:
            return RPS.ROCK


def get_type(letter) -> RPS:
    if letter == "A":
        return RPS.ROCK
    if letter == "B":
        return RPS.PAPER
    if letter == "C":
        return RPS.SCISSORS
    if letter == "X":
        return RPS.ROCK
    if letter == "Y":
        return RPS.PAPER
    if letter == "Z":
        return RPS.SCISSORS


def get_types(me, them):
    # Lose
    if me == "X":
        return them.beats()
    # Draw
    if me == "Y":
        return them

    # Win
    return them.is_beaten_by()


def score(me, them):
    # Draw
    if me == them:
        return me.value + 3
    # Win
    if me == RPS.ROCK and them == RPS.SCISSORS:
        return me.value + 6
    if me == RPS.PAPER and them == RPS.ROCK:
        return me.value + 6
    if me == RPS.SCISSORS and them == RPS.PAPER:
        return me.value + 6
    # Lose
    return me.value


def part1():
    total_score = 0
    with open("input/day2.txt") as f:
        for line in f.readlines():
            total_score += score(get_type(line[2]), get_type(line[0]))

    print(total_score)


def part2():
    total_score = 0
    with open("input/day2.txt") as f:
        for line in f.readlines():
            total_score += score(
                get_types(line[2], get_type(line[0])), get_type(line[0])
            )

    print(total_score)


part1()
part2()
