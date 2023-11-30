def line_input(day, test=False):
    filename = f"input/day{day}.txt" if not test else "test.txt"
    with open(filename) as f:
        for line in f.readlines():
            if line[-1] == "\n":
                yield line[:-1]
            else:
                yield line
