import sys
from copy import deepcopy
from typing import List, Set, Tuple, Iterator

from sudoku import interfaces

# See http://lipas.uwasa.fi/~timan/sudoku/ for sample problems

class UnsolvableSudoku(Exception):
    pass


class Problem(interfaces.IProblem):
    _data: List[List[int]]
    _options: List[List[Set[int]]]

    def __init__(self) -> None:
        self._data = [[0] * 9 for y in range(9)]
        self._options = [[set(range(1, 10)) for x in range(9)]
                         for y in range(9)]

    @classmethod
    def parse(cls, raw: str) -> interfaces.IProblem:
        p = Problem()
        lines = raw.strip().split("\n")
        for y, line in enumerate(lines):
            for x, sn in enumerate(line.strip().split()):
                n = int(sn)
                if not n:
                    continue
                p.set(x, y, n)
        return p

    def get(self, x: int, y: int) -> int:
        return self._data[y][x]

    def get_options(self, x: int, y: int) -> Set[int]:
        return self._options[y][x]

    def set(self, x:int, y:int, value:int) -> None:
        # Verify the value
        row_coords = [(x, y) for x in range(9)]
        if value in set(self.get(x, y) for x, y in row_coords):
            raise ValueError(f"Value {value} is already in the row {y}")

        col_coords = [(x, y) for y in range(9)]
        if value in set(self.get(x, y) for x, y in col_coords):
            raise ValueError(f"Value {value} is already in the col {x}")

        bx = x // 3 * 3
        by = y // 3 * 3
        sec_coords = [(x, y)
                      for x in range(bx, bx+3)
                      for y in range(by, by+3)]
        if value in set(self.get(x, y) for x, y in sec_coords):
            raise ValueError(f"Value {value} is already in the sector {bx, by}")

        # Set the value
        self._data[y][x] = value

        # Finally, remove from options
        self._options[y][x] = set()
        for coords in [row_coords, col_coords, sec_coords]:
            for x, y in coords:
                if value not in self._options[y][x]:
                    continue
                self._options[y][x].remove(value)

    def is_solved(self) -> bool:
        # Problem is solved when all the cells are filled
        return all(self.get(x, y) for x in range(9) for y in range(9))

    def is_solvable(self) -> bool:
        # Problem is solvable if all empty cells have at least one option
        return all(self._options[y][x] for x in range(9) for y in range(9)
                    if not self.get(x, y))

    def copy(self) -> interfaces.IProblem:
        c = Problem()
        c._data = deepcopy(self._data)
        c._options = deepcopy(self._options)
        return c

    def format(self) -> str:
        out = []

        for blockn in range(3):
            for line in self._data[blockn*3 : blockn*3+3]:
                outline = ""
                outline += (" ".join(str(n or " ") for n in line[0:3]))
                outline += " | "
                outline += (" ".join(str(n or " ") for n in line[3:6]))
                outline += " | "
                outline += (" ".join(str(n or " ") for n in line[6:9]))
                out.append(outline)

            if blockn != 2:
                out.append("------+-------+------")

        return "\n".join(out)

    def print(self) -> None:
        print(self.format())


class Solver(interfaces.ISolver):
    def __init__(self, problem: interfaces.IProblem):
        self.problem = problem

    def solve(self) -> interfaces.IProblem:
        while not self.problem.is_solved():
            moves = self._get_trivial_moves()
            if not moves:
                # No trivial moves are left. We have to solve by trials and
                # errors.
                return self._fork()
            for x, y, value in moves:
                self.problem.set(x, y, value)
                if not self.problem.is_solvable():
                    raise UnsolvableSudoku()

        return self.problem

    def _fork(self) -> interfaces.IProblem:
        # Find first cell with options
        x, y = next(self._get_empty_coords())
        opts = self.problem.get_options(x, y)
        # Try all options one by one
        for candidate in opts:
            attempt = self.problem.copy()
            attempt.set(x, y, candidate)
            subsolver = Solver(attempt)
            try:
                return subsolver.solve()
            except UnsolvableSudoku:
                # Didn't work, try another option
                continue

        # All options exhausted, we can't solve this
        raise UnsolvableSudoku()

    def _get_trivial_moves(self) -> List[Tuple[int, int, int]]:
        moves = []
        for x, y in self._get_empty_coords():
            opts = self.problem.get_options(x, y)
            if len(opts) == 1:
                moves.append((x, y, list(opts)[0]))

        return moves

    def _get_empty_coords(self) -> Iterator[Tuple[int, int]]:
        for x in range(9):
            for y in range(9):
                val = self.problem.get(x, y)
                if val:
                    continue
                yield (x, y)


def load_problem(fname: str) -> Problem:
    with open(fname, 'r') as f:
        probstr = f.read()
    return Problem.parse(probstr)


def main() -> None:
    problem = load_problem(sys.argv[1])
    print("Initial problem:")
    problem.print()

    solver = Solver(problem)
    solved = solver.solve()

    print("Solved problem:")
    solved.print()


if __name__ == '__main__':
    main()
