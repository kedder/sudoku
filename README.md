# Sudoku solver in different languages

This contains an efficient sudoku solver implemented in different languages,
namely:

* Python 3
* Rust
* Go

The goal is just a personal exercise for me to get sense of new programming
languages out there and compare their performance.

## How to run it

### Python

```sh
cd python
python3 sudoku.py ../problems/diabolical.txt
```

### Rust
```sh
cd rust
cargo run --release ../problems/diabolical.txt
```

### Go
```sh
cd go
go run sudoku.go  ../problems/diabolical.txt
```

## Problems

The `problems` directory contains a set of problems of different complexity.
The `diabolical.txt` and `hardest.txt` are not really the hardest of them. The
hardest so far happens to be `hardest3.txt`. Problems are in a simple text format, e.g. `diabolical.txt` looks like this:

```
0 7 0 2 5 0 4 0 0
8 0 0 0 0 0 9 0 3
0 0 0 0 0 3 0 7 0
7 0 0 0 0 4 0 2 0
1 0 0 0 0 0 0 0 7
0 4 0 5 0 0 0 0 8
0 9 0 6 0 0 0 0 0
4 0 1 0 0 0 0 0 5
0 0 7 0 8 2 0 3 0
```

## Algorithm

The idea is to keep a list of all possible options for each sudoku cell and
remove an option from all dependent cells when a particular cell is filled with
a number.

Having that, we can iterate over all cells that have a single option and fill
those, thus reducing number of options for other cells. If there is no cells
with single option, but the problem isn't solved yet, we find first empty cell,
and try solving the problem if each of the options would be placed into the
cell.
