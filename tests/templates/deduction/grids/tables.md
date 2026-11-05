# Probe grid tables for Template argument deduction

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/templates/deduction.md` says what each grid probes
and how to read its rows and columns. `tools/probe_grid.py GRID.json`
compiles a grid on the baseline compilers and prints its table, so each
table here can be made again from the grid file that its section names.
The tables below come from runs on 2026-11-05.

## braced

Grid file `braced.json`.

| | T | initializer_list of T | const initializer_list of T lref | T array of N rref | const T array of N lref | const T array of 2 lref | const T array of M by N lref | Box of T | initializer_list of int and T |
|---|---|---|---|---|---|---|---|---|---|
| empty | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| one int | reject | accept | accept | accept | accept | accept | reject | reject | reject |
| two ints | reject | accept | accept | accept | accept | accept | reject | reject | reject |
| three ints | reject | accept | accept | accept | accept | reject | reject | reject | reject |
| int and long | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| two chars | reject | accept | accept | accept | accept | accept | reject | reject | reject |
| nested pairs | reject | reject | reject | reject | reject | reject | accept | reject | reject |

## call-types

Grid file `call-types.json`.

| | T | const T | T lref | const T lref | T rref | T pointer | const T pointer |
|---|---|---|---|---|---|---|---|
| int lvalue gives int | accept | accept | accept | accept | reject | reject | reject |
| int lvalue gives int lref | reject | reject | reject | reject | accept | reject | reject |
| const int lvalue gives int | accept | accept | reject | accept | reject | reject | reject |
| const int lvalue gives const int | reject | reject | accept | reject | reject | reject | reject |
| const int lvalue gives const int lref | reject | reject | reject | reject | accept | reject | reject |
| int prvalue gives int | accept | accept | reject | accept | accept | reject | reject |
| int xvalue gives int | accept | accept | reject | accept | accept | reject | reject |
| array gives int | reject | reject | reject | reject | reject | accept | accept |
| array gives int pointer | accept | accept | reject | reject | reject | reject | reject |
| array gives int array | reject | reject | accept | accept | reject | reject | reject |
| array gives int array lref | reject | reject | reject | reject | accept | reject | reject |
| function gives function | reject | reject | accept | accept | reject | accept | reject |
| function gives function pointer | accept | accept | reject | reject | reject | reject | reject |
| function gives function lref | reject | reject | reject | reject | accept | reject | reject |
| const int pointer gives const int pointer | accept | accept | accept | accept | reject | reject | reject |
| const int pointer gives const int | reject | reject | reject | reject | reject | accept | reject |
| const int pointer gives int | reject | reject | reject | reject | reject | reject | accept |
| const int pointer gives const int pointer lref | reject | reject | reject | reject | accept | reject | reject |
