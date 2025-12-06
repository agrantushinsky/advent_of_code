import numpy as np

def solve(lines):
    dial = 50
    zero_count = 0
    zero_count_extra = 0
    for i, line in enumerate(lines):
        rot = int(line[1:])
        last_dial = dial
        if line[0] == 'R':
            print(f"rr: {rot}")
            dial += rot
        else:
            print(f"lr: {rot}")
            dial -= rot

        while dial < 0 or dial > 99:
            if last_dial == 0 and dial < 0 and abs(rot) < 100:
                zero_count_extra = zero_count_extra - 1
                last_dial = dial
                print(f"removing extra 0")

            if dial > 99:
                dial = dial - 100;
                zero_count_extra = zero_count_extra + 1
                print(f"extra 0")
            elif dial < 0:
                dial = 100 + dial;
                zero_count_extra = zero_count_extra + 1
                print(f"extra 0")

            if dial == 0:
                zero_count_extra = zero_count_extra - 1
                print(f"removing normal extra 0")

        if dial == 0:
            zero_count = zero_count + 1
            print(f"normal 0")

        print(f"dial is now at: {dial}\n")
    
    return (zero_count, zero_count + zero_count_extra)

if __name__ == "__main__":
    input = open("./d1_input", "r")

    part_1, part_2 = solve([l.rstrip() for l in input.readlines()])
    print(f"part 1:\n{part_1}\npart 2:\n{part_2}")
