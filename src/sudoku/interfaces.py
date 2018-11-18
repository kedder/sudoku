from abc import ABC, abstractmethod
from typing import Set

class IProblem(ABC):

    @abstractmethod
    def get(self, x: int, y: int) -> int:
        """Get value at coordinates"""

    @abstractmethod
    def get_options(self, x: int, y: int) -> Set[int]:
        """Return allowed options for the cell at (x, y)"""

    @abstractmethod
    def set(self, x:int, y:int, value:int) -> None:
        """Fill the cell at (x, y) with number value"""

    @abstractmethod
    def print(self) -> None:
        """Print the problem as to stdout"""

    @abstractmethod
    def is_solved(self) -> bool:
        """Is this problem solved?"""

    @abstractmethod
    def is_solvable(self) -> bool:
        """Can this problem be solved?"""

    @abstractmethod
    def copy(self) -> "IProblem":
        """Return a complete copy of self"""

    @abstractmethod
    def format(self) -> str:
        """Format prolem to human-readable representation"""


class ISolver(ABC):
    @abstractmethod
    def solve(self) -> IProblem:
        """Solve sudoku"""