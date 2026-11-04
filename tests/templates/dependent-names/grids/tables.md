# Probe grid tables for dependent names

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/templates/dependent-names.md` names
them in its Evidence. `tools/probe_grid.py GRID.json` compiles a grid on
the baseline compilers and prints its table, so each table here can be
made again from the grid file that its section names. A cell where the
compilers split lists the compilers on each side.

## Positions of a dependent qualified name

Grid file `typename-positions.json`. Rules: DEP-19, DEP-20.

Each row writes `T::type` in one position and instantiates the template
with a class whose `type` is `int`. The first column writes the name
alone, the second with `typename`.

| | T::type | typename T::type |
|---|---|---|
| alias target | accept | accept |
| data member | accept | accept |
| member function return, in class | accept | accept |
| member function parameter, in class | accept | accept |
| member function return, outside class | accept | accept |
| member function parameter, outside class | accept | accept |
| function template return | accept | accept |
| function template parameter | reject | accept |
| trailing return type | accept | accept |
| default type argument | accept | accept |
| constant parameter type | accept | accept |
| static_cast type | accept | accept |
| new type | accept | accept |
| variable in a block | reject | accept |
| sizeof operand | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, clang-host, gcc-14, gcc-16, clang-18, clang-22 | accept |
| functional cast | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, clang-host, gcc-14, gcc-16, clang-18, clang-22 | accept |
| template argument | reject | accept |
| lambda parameter | accept | accept |
| function declared in a block | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | accept |
| variable template type | accept | accept |
| friend function parameter | accept | accept |

## The keyword template

Grid file `template-keyword.json`. Rules: DEP-24 to DEP-26.

Each row names a member of a class `Has` and instantiates the template
where there is one.

| | no keyword | template |
|---|---|---|
| after T:: in a call | reject | accept |
| after a dot | reject | accept |
| after an arrow | reject | accept |
| after typename T:: | accept: gcc-host, gcc-14, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept |
| alias target | accept: gcc-host, gcc-14, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept |
| template template argument, no list | reject | accept |
| independent qualifier | accept | accept |
| independent object | accept | accept |
| member that is not a template | accept | reject |
| data member that is not a template | accept | accept: gcc-14, gcc-host, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 |

## The current instantiation

Grid file `current-instantiation.json`. Rules: DEP-15 to DEP-18.

Each row is a way to name `Box<T>` inside `Box`. The first column reads
a member type without `typename`. The second reads a missing member in
a class with no dependent base, and the template is not used. The third
reads a member that only a dependent base declares. `Same<U>` is an
alias for `Box<U>`, and `Pointer<U>` is an alias for `Box<U*>`.

| | member type without typename | missing member, no dependent base | missing member, dependent base |
|---|---|---|---|
| Box | accept | accept: gcc-14; reject: edg-7.0-ilp32, edg-7.0, gcc-host, gcc-16, clang-host, clang-18, clang-22 | accept |
| Box<T> | accept | accept: gcc-14; reject: edg-7.0, edg-7.0-ilp32, gcc-16, gcc-host, clang-host, clang-18, clang-22 | accept |
| ::Box<T> | accept | accept: gcc-14; reject: edg-7.0, edg-7.0-ilp32, gcc-host, clang-host, gcc-16, clang-18, clang-22 | accept |
| Same<T> | accept: gcc-host, gcc-14, clang-host, gcc-16, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | accept: edg-7.0, edg-7.0-ilp32, gcc-14; reject: gcc-host, clang-host, gcc-16, clang-18, clang-22 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, clang-host, clang-18, clang-22; reject: gcc-host, gcc-16 |
| Box<T*> | reject | accept | accept |
| Pointer<T> | reject | accept | accept |
| Box<const T> | reject | accept | accept |
