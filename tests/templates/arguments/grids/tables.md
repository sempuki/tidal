# Probe grid tables for Template arguments

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/templates/arguments.md` says what each grid probes
and how to read its rows and columns. `tools/probe_grid.py GRID.json`
compiles a grid on the baseline compilers and prints its table, so each
table here can be made again from the grid file that its section names.
The tables below come from runs on 2026-11-05.

## constant-arguments

Grid file `constant-arguments.json`.

| | int | unsigned char | bool | long | double | Plain | Scoped | int* | const int* | int& | const int& | int (*)() | auto |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| 300 | accept | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| 'a' | accept | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| true | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| 2.0 | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept |
| unscoped enumerator | accept | accept | accept | accept | reject | accept | reject | reject | reject | reject | reject | reject | accept |
| scoped enumerator | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | accept |
| enumerator of another scoped enumeration | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| braced Scoped value | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | accept |
| class with conversion to int | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| constexpr int variable | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | accept | reject | accept |
| int variable | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject |
| nullptr | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject | accept | accept |
| 0 | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| &object | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject | reject | accept |
| object | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject |
| const_object | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | accept | reject | accept |
| string literal | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| function | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |
