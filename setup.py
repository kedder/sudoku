
from setuptools import setup, find_packages

setup(
    name = 'kedsudoku',
    version = '1.0.0',
    url = 'https://github.com/kedder/sudoku.git',
    author = 'Author Name',
    author_email = 'andrey.lebedev@gmail.com',
    description = 'Sudoku solver with static typing',
    packages=find_packages('src'),
    package_dir={'': 'src'},
    install_requires = [],
)