import numpy as np

def solve(lines):
    dial = 50
    zero_count = 0
    zero_count_extra = 0
    for i, line in enumerate(lines):
        rot = int(line[1:])
        step = 1 if line[0] == 'R' else -1
        if line[0] == 'R':
            print(f"rr: {rot}")
        else:
            print(f"lr: {rot}")

        while rot > 0:
            dial = dial + step
            rot = rot - 1

            if dial < 0:
                dial = 99
            elif dial > 99:
                dial = 0

            if dial == 0:
                zero_count_extra = zero_count_extra + 1

        if dial == 0:
            zero_count = zero_count + 1



        print(f"dial is now at: {dial}\n")
    
    return (zero_count, zero_count_extra)

if __name__ == "__main__":
    input = open("./d1_input", "r")

    part_1, part_2 = solve([l.rstrip() for l in input.readlines()])
    print(f"part 1:\n{part_1}\npart 2:\n{part_2}")
