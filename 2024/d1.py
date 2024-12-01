import bisect 

def make_lists(input):
    left = []
    right = []
    for line in input:
        spl = line.split()
        bisect.insort(left, int(spl[0]))
        bisect.insort(right, int(spl[1]))

    return (left, right)

def solve(lists):
    left, right = lists

    pairs = zip(left, right)
    part_1 = sum([abs(r - l) for l, r in pairs])

    part_2 = sum([l * right.count(l) for l in left])

    return (part_1, part_2)

if __name__ == "__main__":
    input = open("./d1_input", "r")
    lists = make_lists([line.rstrip() for line in input])

    part_1, part_2 = solve(lists)
    print(f"part 1:\n{part_1}\npart 2:\n{part_2}")
