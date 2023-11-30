def part1():
    datafile = "input/day1.txt"
    max_cal = 0
    current_cal = 0
    with open(datafile) as f:
        for line in f.readlines():
            if line == "\n":
                if current_cal > max_cal:
                    max_cal = current_cal
                current_cal = 0
            else:
                current_cal += int(line[:-1])

    print(max_cal)


def add_cal(current_cal, max_cal):
    for i, cal in enumerate(max_cal):
        if current_cal > cal:
            max_cal.insert(i, current_cal)
            max_cal.pop()
            break


def part2():
    datafile = "input/day1.txt"
    max_cal = [0, 0, 0]
    current_cal = 0
    with open(datafile) as f:
        for line in f.readlines():
            if line == "\n":
                add_cal(current_cal, max_cal)

                current_cal = 0
            else:
                current_cal += int(line[:-1])

    print(sum(max_cal))


part1()
part2()
