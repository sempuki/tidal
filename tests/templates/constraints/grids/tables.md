# Probe grid tables for constraints

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/templates/constraints.md` names them
in its Evidence. `tools/probe_grid.py GRID.json` compiles a grid on the
baseline compilers and prints its table, so each table here can be made
again from the grid file that its section names. A cell where the
compilers split lists the compilers on each side.

## Subsumption

Grid file `subsumption.json`. Rules: CNS-4, CNS-22 to CNS-24.

Each column declares two static members `f` of `Box<T>`, with the two
requires clauses. The first returns 1 and the second 2. `int` satisfies
every clause, so the row "int → first" checks that the first is more
constrained. A `reject` in both rows means that neither is more
constrained. `A2` has the same text as `A`, `Z` is never satisfied for
`int`, and `NotZ` is the concept `!Z<T>`.

| | none / A | none / B | none / A and B | none / A or B | none / AB | none / A2 | none / not Z | none / A and not Z | none / NotZ | none / A and NotZ | A / B | A / A and B | A / A or B | A / AB | A / A2 | A / not Z | A / A and not Z | A / NotZ | A / A and NotZ | B / A and B | B / A or B | B / AB | B / A2 | B / not Z | B / A and not Z | B / NotZ | B / A and NotZ | A and B / A or B | A and B / AB | A and B / A2 | A and B / not Z | A and B / A and not Z | A and B / NotZ | A and B / A and NotZ | A or B / AB | A or B / A2 | A or B / not Z | A or B / A and not Z | A or B / NotZ | A or B / A and NotZ | AB / A2 | AB / not Z | AB / A and not Z | AB / NotZ | AB / A and NotZ | A2 / not Z | A2 / A and not Z | A2 / NotZ | A2 / A and NotZ | not Z / A and not Z | not Z / NotZ | not Z / A and NotZ | A and not Z / NotZ | A and not Z / A and NotZ | NotZ / A and NotZ |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int → second | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | reject | accept | reject | reject | accept | reject | accept | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |

## Spellings

Grid file `spellings.json`. Rules: CNS-1, CNS-23, CNS-26, CNS-27,
ORD-13.

Each column declares two function templates `f` with the two forms, and
the row checks the call `f(1)`. A `reject` in both rows means an
ambiguous call, or for `A T / A auto` and `AB T / AB auto` a
redefinition on Clang (Q-0269).

| | none / A T | none / A leading | none / A trailing | none / A auto | none / AB T | none / AB trailing | none / AB auto | A T / A leading | A T / A trailing | A T / A auto | A T / AB T | A T / AB trailing | A T / AB auto | A leading / A trailing | A leading / A auto | A leading / AB T | A leading / AB trailing | A leading / AB auto | A trailing / A auto | A trailing / AB T | A trailing / AB trailing | A trailing / AB auto | A auto / AB T | A auto / AB trailing | A auto / AB auto | AB T / AB trailing | AB T / AB auto | AB trailing / AB auto |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int → second | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
