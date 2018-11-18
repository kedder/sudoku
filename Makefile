VEBIN = .ve/bin

all: mypy

.PHONY: mypy
mypy:
	$(VEBIN)/mypy src/sudoku/ \
		--no-incremental \
		--junit-xml mypy-report/mypy-junit.xml \
		--html-report mypy-report

.PHONY: run
run:
	$(VEBIN)/python src/sudoku/solver.py

.PHONY: virtualenv
virtualenv:
	virtualenv -p python3 .ve
	$(VEBIN)/pip install -r requirements.txt

.PHONY: serve
serve:
	FLASK_APP=src/sudoku/webapp.py $(VEBIN)/flask run