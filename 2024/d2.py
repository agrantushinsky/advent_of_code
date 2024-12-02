import bisect 

def is_safe(levels):
    is_decreasing = levels[1] < levels[0]
    last_level = levels[0]
    for i, level in enumerate(levels[1:]):
        if is_decreasing != last_level < level if is_decreasing else level < last_level:
            return False

        diff = abs(last_level - level)
        if diff < 1 or diff > 3:
            return False

        last_level = level
    return True


def solve(lines):
    safes_p1 = 0
    safes_p2 = 0
    for line in lines:
        levels = [int(level) for level in line.split()]
        if is_safe(levels):
            safes_p1 += 1
            safes_p2 += 1
        else:
            # :)
            for to_remove in range(0, len(levels)):
                new_levels_test = [l for i, l in enumerate(levels) if i != to_remove]
                if is_safe(new_levels_test):
                    safes_p2 += 1
                    break

    return (safes_p1, safes_p2)

if __name__ == "__main__":
    input = open("./d2_input", "r")

    part_1, part_2 = solve([line.rstrip() for line in input])
    print(f"part 1:\n{part_1}\npart 2:\n{part_2}")
