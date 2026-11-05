# Probe grid tables for Packs

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/templates/packs.md` says what each grid probes
and how to read its rows and columns. `tools/probe_grid.py GRID.json`
compiles a grid on the baseline compilers and prints its table, so each
table here can be made again from the grid file that its section names.
The tables below come from runs on 2026-11-05.

## sites

Grid file `sites.json`.

| | no element | one element | three elements |
|---|---|---|---|
| call arguments | accept | accept | accept |
| braced list | accept | accept | accept |
| template arguments | accept | accept | accept |
| function parameters | accept | accept | accept |
| base list | accept | accept | accept |
| member initializers | accept | accept | accept |
| alignment specifier | accept | accept | accept |
| capture | accept | accept | accept |
| init capture | accept | accept | accept |
| using-declaration | accept | accept | accept |
| pack-size operator | accept | accept | accept |
| fold | accept | accept | accept |
