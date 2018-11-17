all: mypy

.PHONY: mypy
mypy:
	.ve/bin/mypy src/sudoku/sudoku.py \
		--junit-xml mypy-report/mypy-junit.xml \
		--html-report mypy-report

.PHONY: run
run:
	.ve/bin/python src/sudoku/sudoku.py

.PHONY: virtualenv
virtualenv:
	virtualenv -p python3 .ve
	.ve/bin/pip install mypy lxml
