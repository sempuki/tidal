# Probe grid tables for partial ordering

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/templates/ordering.md` names them in
its Evidence. `tools/probe_grid.py GRID.json` compiles a grid on the
baseline compilers and prints its table, so each table here can be made
again from the grid file that its section names. The compilers agree on
every cell of both grids.

## Function templates

Grid file `function-templates.json`. Rules: ORD-1, ORD-2, ORD-4, ORD-10,
ORD-11.

Each column declares two templates `f` in two namespaces that a
using-declaration merges. The first returns `One` and the second `Two`.
The row "X → first" checks that the call `f(X)` chooses the first
template. A `reject` in both rows means that the call is ambiguous or
has no viable function.

| | T / T pointer | T / const T pointer | T / T lref | T / const T lref | T / T rref | T / Box of T | T / pack | T pointer / const T pointer | T pointer / T lref | T pointer / const T lref | T pointer / T rref | T pointer / Box of T | T pointer / pack | const T pointer / T lref | const T pointer / const T lref | const T pointer / T rref | const T pointer / Box of T | const T pointer / pack | T lref / const T lref | T lref / T rref | T lref / Box of T | T lref / pack | const T lref / T rref | const T lref / Box of T | const T lref / pack | T rref / Box of T | T rref / pack | Box of T / pack |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int lvalue → first | accept | accept | reject | reject | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | accept | accept | accept | accept | reject |
| int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | reject | accept | reject | reject | reject | reject | accept | reject | reject | reject | reject | accept |
| const int lvalue → first | accept | accept | reject | reject | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject |
| const int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| int prvalue → first | accept | accept | accept | reject | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject |
| int prvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | accept | reject | accept | accept | reject | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject | accept |
| int pointer → first | reject | accept | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | accept | reject | accept | accept | accept | accept | reject | accept | accept | accept | accept | reject |
| int pointer → second | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | accept | reject | reject | reject | reject | accept | reject | reject | reject | reject | accept |
| const int pointer → first | reject | reject | reject | reject | reject | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | reject |
| const int pointer → second | accept | accept | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | accept |
| Box lvalue → first | accept | accept | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | accept | reject | reject | accept | reject | accept | accept |
| Box lvalue → second | reject | reject | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | accept | reject | accept | accept | reject | accept | reject | reject |

Many cells are decided before partial ordering, by viability or by the
sequences (ICS-28, ICS-30). The columns `T / T lref`, `T / const T
lref`, and `T / T rref` reject in every row, because the sequences tie,
and after ORD-2 both types are `T`.

## Partial specializations

Grid file `partial-specializations.json`. Rules: ORD-14, TSP-15, TSP-16.

Each column declares two partial specializations of `Pair`. The first
gives `value` 1 and the second 2. The row "X → first" checks that
`Pair<X>` selects the first. A `reject` in both rows means that the
selection is ambiguous or that the primary template is selected.

| | T T / T pointer U | T T / T U pointer | T T / T pointer T pointer | T T / T int | T T / const T U | T pointer U / T U pointer | T pointer U / T pointer T pointer | T pointer U / T int | T pointer U / const T U | T U pointer / T pointer T pointer | T U pointer / T int | T U pointer / const T U | T pointer T pointer / T int | T pointer T pointer / const T U | T int / const T U |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int int → first | accept | accept | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| int int → second | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject |
| int pointer int pointer → first | reject | reject | reject | accept | accept | reject | reject | accept | accept | reject | accept | accept | accept | accept | reject |
| int pointer int pointer → second | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject |
| int pointer char → first | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| int pointer char → second | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int int pointer → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| int int pointer → second | reject | accept | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| const int int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| const int int → second | reject | reject | reject | accept | accept | reject | reject | accept | accept | reject | accept | accept | accept | accept | reject |
| char int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| char int → second | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject |
