# Probe grid tables for Explicit and partial specialization

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/templates/specialization.md` says what each grid probes
and how to read its rows and columns. `tools/probe_grid.py GRID.json`
compiles a grid on the baseline compilers and prints its table, so each
table here can be made again from the grid file that its section names.
The tables below come from runs on 2026-11-05.

## matching

Grid file `matching.json`.

| | T* | const T | const T* | T& | T&& | T[N] | T[] | R(A) | R(*)(A) | T C::* | Box<T> | Box<T*> | C<T> | Pair<T, T> |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| const int | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int* | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| const int* | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int* const | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int& | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int&& | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int[3] | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject |
| int[] | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject |
| int() | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int(char) | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject |
| int(*)(char) | accept | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject |
| int Owner::* | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject |
| Box<int> | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | accept | reject |
| Box<int*> | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject |
| Pair<int, int> | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| Pair<int, char> | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
