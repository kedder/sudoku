from flask import Flask

from sudoku import solver

app = Flask("sudoku")

@app.route("/")
def hello() -> str:
    problem = solver.Problem.parse(solver.PROBLEM_EVIL)
    s = solver.Solver(problem)
    solved = s.solve()
    return "<pre>%s</pre>" % solved.format()


class One:
    pass

class Two(One):
    pass