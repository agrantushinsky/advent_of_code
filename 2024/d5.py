import numpy as np

def check_valid(all_rules, update):
    for i, page in enumerate(update):
        rules = all_rules.get(page)
        if rules != None:
            for rp in rules:
                rule_i = update.index(rp) if rp in update else None
                if rule_i != None and i > rule_i:
                    return False
    return True

def solve(lines):
    lsplit = lines.index("")
    all_rules = {}
    for line in lines[:lsplit]:
        pair = [int(x) for x in line.split("|")]
        if all_rules.get(pair[0]) == None:
            all_rules[pair[0]] = []

        all_rules[pair[0]].append(pair[1])


    updates = [list(map(int, line.split(","))) for line in lines[lsplit+1:]]

    valid_updates = []
    fixed_updates = []

    for update in updates:
        if check_valid(all_rules, update):
            valid_updates.append(update)
        else:
            fixed_update = update.copy()
            while not check_valid(all_rules, fixed_update):
                for i, page in enumerate(fixed_update):
                    rules = all_rules.get(page)
                    if rules != None:
                        for rp in rules:
                            rule_i = fixed_update.index(rp) if rp in update else None
                            if rule_i != None and i > rule_i:
                                bad_page = fixed_update.pop(rule_i)
                                fixed_update.insert(i, bad_page)

            fixed_updates.append(fixed_update)

    part_1 = sum(map(lambda u: u[len(u)//2], valid_updates))
    part_2 = sum(map(lambda u: u[len(u)//2], fixed_updates))
    return (part_1, part_2)

if __name__ == "__main__":
    input = open("./d5_input", "r")

    part_1, part_2 = solve([l.rstrip() for l in input.readlines()])
    print(f"part 1:\n{part_1}\npart 2:\n{part_2}")
