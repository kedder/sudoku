package main

import "os"
import "strings"
import "strconv"
import "io/ioutil"
import "errors"
import "fmt"

type problem struct {
	data      [9 * 9]byte
	options   [9 * 9][9]byte
	optcounts [9 * 9]byte
	coords    [9 * 3]coord
}

type coord struct {
	x int
	y int
}

type move struct {
	crd   coord
	value byte
}

func NewProblem() *problem {
	// set only specific field value with field key
	p := problem{}
	for i, _ := range p.options {
		p.options[i] = [9]byte{1, 2, 3, 4, 5, 6, 7, 8, 9}
	}
	for i, _ := range p.optcounts {
		p.optcounts[i] = 9
	}
	return &p
}

func (p *problem) Get(x int, y int) byte {
	idx := x*9 + y
	return p.data[idx]
}

func (p *problem) Set(x int, y int, v byte) error {
	// coords := []coord{}
	curcoord := 0
	// verify by column
	for ty := 0; ty < 9; ty++ {
		p.coords[curcoord] = coord{x, ty}
		curcoord++
		if p.Get(x, ty) == v {
			return errors.New(fmt.Sprintf("Value %d is already in the column %d", v, x))
		}
	}

	// verify by row
	for tx := 0; tx < 9; tx++ {
		p.coords[curcoord] = coord{tx, y}
		curcoord++
		if p.Get(tx, y) == v {
			return errors.New(fmt.Sprintf("Value %d is already in the row %d", v, y))
		}
	}
	// verify by sector
	sx := x / 3
	sy := y / 3
	for tx := sx * 3; tx < sx*3+3; tx++ {
		for ty := sy * 3; ty < sy*3+3; ty++ {
			// fmt.Println("Checking %s, %s")
			p.coords[curcoord] = coord{tx, ty}
			curcoord++
			if p.Get(tx, ty) == v {
				return errors.New(fmt.Sprintf("Value %d is already in the sector (%d, %d)", v, sx, sy))
			}
		}
	}

	// Finally we can set the value
	idx := x*9 + y
	p.data[idx] = v

	// remove v from options of any relevant cell
	for _, c := range p.coords {
		p.removeOption(c.x, c.y, v)
	}
	return nil
}

func (p *problem) GetOptions(x int, y int) *[9]byte {
	idx := x*9 + y
	return &p.options[idx]
}

func (p *problem) CountOptions(x int, y int) byte {
	return p.optcounts[x*9+y]
}

func (p *problem) GetMinimumOptionsCoord() coord {
	var mincount byte
	mincount = 9
	mincoord := coord{0, 0}
	for x := 0; x < 9; x++ {
		for y := 0; y < 9; y++ {
			if p.Get(x, y) != 0 {
				continue
			}
			count := p.CountOptions(x, y)
			if count == 2 {
				// We are not going to find anything better than this, shortcut
				return coord{x, y}
			}
			if count < mincount {
				mincount = count
				mincoord = coord{x, y}
			}
		}
	}
	return mincoord
}

func (p *problem) removeOption(x int, y int, v byte) {
	have_opt := p.options[x*9+y][v-1] != 0
	if have_opt {
		p.optcounts[x*9+y]--
		p.options[x*9+y][v-1] = 0
	}
}

// Problem is solved when all the cells are filled
func (p *problem) IsSolved() bool {
	for x := 0; x < 9; x++ {
		for y := 0; y < 9; y++ {
			if p.Get(x, y) == 0 {
				return false
			}
		}
	}
	return true
}

// Problem is solvable if all empty cells have at least one option
func (p *problem) IsSolvable() bool {
	for x := 0; x < 9; x++ {
		for y := 0; y < 9; y++ {
			if p.Get(x, y) == 0 {
				// empty cell
				if p.CountOptions(x, y) == 0 {
					// no options available here, problem is not solvable
					return false
				}
			}
		}
	}
	return true
}

func (p problem) Copy() *problem {
	copied := p
	// // copy all the options
	// for i, opts := range p.options {
	// 	copied.options[i] = append([9]int{}, opts...)
	// }
	return &copied
}

func (p *problem) Format() string {
	out := ""

	for block := 0; block < 3; block++ {
		for line := 0; line < 3; line++ {
			y := block*3 + line

			cell := func(x int) string {
				n := p.Get(x, y)
				if n == 0 {
					return " "
				}
				return strconv.Itoa(int(n))
			}

			out += fmt.Sprintf("%s %s %s | %s %s %s | %s %s %s\n",
				cell(0), cell(1), cell(2),
				cell(3), cell(4), cell(5),
				cell(6), cell(7), cell(8),
			)
		}
		if block != 2 {
			out += "------+-------+------\n"
		}
	}
	return out
}

func ReadProblem(filename string) *problem {
	p := NewProblem()
	dat, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(dat), "\n")
	for y, line := range lines {
		nums := strings.Split(line, " ")
		for x, snum := range nums {
			num, err := strconv.Atoi(snum)
			if err != nil {
				panic(err)
			}
			p.Set(x, y, byte(num))
		}
		if y == 8 {
			break
		}
	}
	return p
}

func Solve(p *problem) (*problem, error) {
	for !p.IsSolved() {
		nexttrivial := genTrivialMoves(p)
		lentrivial := 0
		for {
			mv, havemore := nexttrivial()
			if !havemore {
				break
			}
			lentrivial++
			if err := p.Set(mv.crd.x, mv.crd.y, mv.value); err != nil {
				fmt.Print(p.Format())
				panic(err)
			}
			if !p.IsSolvable() {
				return p, errors.New("Cannot solve")
			}

		}
		if lentrivial == 0 {
			// No trivial moves are left. We have to solve by trials and
			// errors.
			return fork(p)
		}
	}
	return p, nil
}

func fork(p *problem) (*problem, error) {
	first := p.GetMinimumOptionsCoord()
	opts := p.GetOptions(first.x, first.y)

	for _, candidate := range opts {
		if candidate == 0 {
			continue
		}
		attempt := p.Copy()
		attempt.Set(first.x, first.y, candidate)
		solved, err := Solve(attempt)
		if err == nil {
			return solved, nil
		}
	}
	return p, errors.New("Cannot solve")
}

func genTrivialMoves(p *problem) func() (*move, bool) {
	empties := getEmptyCoords(p)
	countempties := len(empties)
	cur := 0
	return func() (*move, bool) {
		for ; cur < countempties; cur++ {
			crd := empties[cur]
			if p.CountOptions(crd.x, crd.y) != 1 {
				continue
			}
			for _, opt := range p.GetOptions(crd.x, crd.y) {
				if opt != 0 {
					return &move{crd, opt}, true
				}
			}
		}
		return nil, false
	}
}

func getEmptyCoords(p *problem) []coord {
	res := make([]coord, 0, 40)
	for x := 0; x < 9; x++ {
		for y := 0; y < 9; y++ {
			if p.Get(x, y) == 0 {
				res = append(res, coord{x, y})
			}
		}
	}
	return res
}

func main() {
	problem := ReadProblem(os.Args[1])

	fmt.Println("Initial problem:")
	fmt.Print(problem.Format())

	copied := problem.Copy()
	solved, err := Solve(copied)
	if err != nil {
		panic(err)
	}

	for i := 0; i < 0; i++ {
		copied := problem.Copy()
		_, err := Solve(copied)
		if err != nil {
			panic("Cannot solve this sudoku")
		}

	}
	fmt.Println("Solved problem:")
	fmt.Print(solved.Format())
}
