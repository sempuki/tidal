# Probe grid tables for Instantiation

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/templates/instantiation.md` says what each grid probes
and how to read its rows and columns. `tools/probe_grid.py GRID.json`
compiles a grid on the baseline compilers and prints its table, so each
table here can be made again from the grid file that its section names.
The tables below come from runs on 2026-11-05.

## class-uses

Grid file `class-uses.json`.

| | valid body | failing body |
|---|---|---|
| pointer variable | accept | accept |
| reference parameter | accept | accept |
| function returning the class, declared | accept | accept |
| alias | accept | accept |
| template argument | accept | accept |
| pointer comparison | accept | accept |
| cast from void pointer | accept | accept |
| object | accept | reject |
| sizeof | accept | reject |
| member access through a pointer | accept | reject |
| pointer arithmetic | accept | reject |
| delete through a pointer | accept | reject |
| conversion to a base pointer | accept | reject |
| call with one void pointer parameter | accept | reject |

## function-uses

Grid file `function-uses.json`.

| | valid body | failing body |
|---|---|---|
| no use | accept | accept |
| decltype of a call | accept | accept |
| sizeof of a call | accept | accept |
| noexcept of a call | accept | accept |
| decltype of a call, deduced return type | accept | reject |
| call | accept | reject |
| address | accept | reject |
| discarded branch | accept | accept |
| explicit instantiation declaration | accept | accept |
| member function, class used | accept | accept |
| member function, call | accept | reject |
| static data member, class used | accept | accept |
| static data member, read | accept | reject |
| variable template, decltype | accept | accept |
| variable template, read | accept | reject |
