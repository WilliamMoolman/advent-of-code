def get_common_letter(a, b):
    set_a = set(a)
    for c in b:
        if c in set_a:
            return c


def get_common_letter_three(a, b, c):
    set_a = set(a)
    set_b = set(b)
    set_c = set(c)

    return set_a.intersection(set_b, set_c).pop()


def get_letter_priority(c):
    if ord(c) > 96:
        return ord(c) - 96
    return ord(c) - 64 + 26


def part1():
    with open("input/day3.txt") as f:
        total = 0
        for row in f.read().splitlines():
            length = len(row)
            c = get_common_letter(row[: length // 2], row[length // 2 :])
            total += get_letter_priority(c)

        print(total)


def part2():
    with open("input/day3.txt") as f:
        total = 0
        temp_list = []
        for i, row in enumerate(f.read().splitlines()):
            temp_list.append(row)
            if i % 3 == 2:
                c = get_common_letter_three(*temp_list)
                total += get_letter_priority(c)
                temp_list = []

        print(total)


part1()
part2()
