import re

def do_operations(operations):
    pairs = map(lambda op: list(map(int, op[4:-1].split(","))), operations)
    return sum([p[0] * p[1] for p in pairs])

def solve(input):
    reg = r"mul\(\d{1,3},\d{1,3}\)"
    p1_ops = re.findall(reg, input)
    part_1 = do_operations(p1_ops)

    dos_donts = re.finditer(r"do\(\)|don't\(\)", input)

    p2_input = list(input)
    dont = -1
    for toggle in dos_donts:
        do = toggle.group()
        if do == "do()":
            if dont != -1:
                for i in range(dont, toggle.start()):
                    p2_input[i] = '-'
                    dont = -1
        else:
            if dont == -1:
                dont = toggle.end()

    # :)
    if dont != -1:
        for i in range(dont, len(p2_input)):
            p2_input[i] = '-'
            dont = -1

    print(input)
    print("".join(p2_input))
    p2_ops = re.findall(reg, "".join(p2_input))
    print(p2_ops)
    part_2 = do_operations(p2_ops)

    return (part_1, part_2)

if __name__ == "__main__":
    input = open("./d3_input", "r")

    part_1, part_2 = solve(input.read())
    print(f"part 1:\n{part_1}\npart 2:\n{part_2}")
