import numpy as np

def solve(matrix):
    all_moves = np.array([
        [1, 0], [-1, 0],
        [0, 1], [0, -1],
        [1, 1], [-1, -1],
        [-1, 1], [1, -1]
    ])
    diagonal_moves = np.array([
        [[1, 1], [-1, -1]],
        [[-1, 1], [1, -1]]
    ])
    xmas = list("XMAS")

    sum_p1 = 0
    sum_p2 = 0
    rows = len(matrix)
    columns = len(matrix[0])
    for r in range(0, rows):
        for c in range(0, columns):
            cur = [r, c]
            if matrix[r][c] == 'X':
                for move in all_moves:
                    found = True
                    for i, letter in enumerate(xmas):
                        m = np.array(move) * i + cur
                        if m[0] < 0 or m[1] < 0 or m[0] >= columns or m[1] >= rows:
                            found = False
                            break

                        if matrix[m[0]][m[1]] != letter:
                            found = False
                            break
                    if found:
                        sum_p1 += 1

            if matrix[r][c] == 'A':
                diags = 0
                for moves in diagonal_moves:
                    letters = []

                    for imove in moves:
                        m = np.array(imove) + cur
                        if m[0] < 0 or m[1] < 0 or m[0] >= columns or m[1] >= rows:
                            break

                        letters.append(matrix[m[0]][m[1]])
                    if "".join(sorted(letters)) == "MS":
                        diags += 1

                if diags == 2:
                    sum_p2 += 1



    return (sum_p1, sum_p2)

if __name__ == "__main__":
    input = open("./d4_input", "r")

    part_1, part_2 = solve(np.array([list(x.rstrip()) for x in input.readlines()]))
    print(f"part 1:\n{part_1}\npart 2:\n{part_2}")
