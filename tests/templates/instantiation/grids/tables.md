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

## static-member-point

Grid file `static-member-point.json`. Run on 2026-11-11 with `tools/probe_grid.py --errors`.

| | no early use, read after | object before probe, read after | sizeof before probe, read after | object before probe, constant read after | read before probe |
|---|---|---|---|---|---|
| static constexpr int value = probe(T{}) | accept | accept: gcc-host, gcc-14, clang-18, clang-host, gcc-16, clang-22; reject: edg-7.0-ilp32, edg-7.0 | accept: gcc-16, clang-18, gcc-14, gcc-host, clang-host, clang-22; reject: edg-7.0, edg-7.0-ilp32 | accept: clang-18, gcc-14, gcc-16, clang-host, gcc-host, clang-22; reject: edg-7.0, edg-7.0-ilp32 | reject |
| static const int value = probe(T{}) | accept | accept: gcc-14, gcc-host, gcc-16; reject: edg-7.0, clang-host, edg-7.0-ilp32, clang-18, clang-22 | accept: gcc-16, gcc-host, gcc-14; reject: edg-7.0-ilp32, edg-7.0, clang-18, clang-host, clang-22 | accept: gcc-16, gcc-14, gcc-host; reject: edg-7.0-ilp32, clang-host, edg-7.0, clang-18, clang-22 | reject |
| static inline const int value = probe(T{}) | accept | accept: gcc-host, clang-host, gcc-14, gcc-16, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | accept: gcc-host, gcc-16, clang-host, gcc-14, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0 | accept: gcc-host, gcc-14, clang-18, gcc-16, clang-host, clang-22; reject: edg-7.0-ilp32, edg-7.0 | reject |
| static inline int value = probe(T{}) | accept | accept: gcc-host, gcc-14, clang-host, gcc-16, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0 | reject | accept: clang-18, clang-host, clang-22; reject: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-host, gcc-16 |
| member decltype(probe(T{})) d; value = 4 | accept | reject | reject | reject | reject |
| static_assert(probe(T{}) == 4) in class; value = 4 | accept | reject | reject | reject | reject |

## static-const-member

Grid file `static-const-member.json`. Run on 2026-11-11 with `tools/probe_grid.py --errors`.

| | no use | an object | sizeof | a pointer only |
|---|---|---|---|---|
| static const int v = B<N>::missing | accept | accept: gcc-host, gcc-16, gcc-14; reject: edg-7.0, clang-host, clang-18, edg-7.0-ilp32, clang-22 | accept: gcc-16, gcc-14, gcc-host; reject: clang-host, edg-7.0-ilp32, clang-18, edg-7.0, clang-22 | accept |
| static const long v = B<N>::missing | accept | accept: gcc-16, gcc-14, gcc-host; reject: edg-7.0-ilp32, clang-host, clang-18, edg-7.0, clang-22 | accept: gcc-16, gcc-host, gcc-14; reject: edg-7.0, clang-18, edg-7.0-ilp32, clang-host, clang-22 | accept |
| static const bool v = B<N>::missing | accept | accept: gcc-14, gcc-16, gcc-host; reject: clang-host, clang-18, edg-7.0-ilp32, edg-7.0, clang-22 | accept: gcc-14, gcc-host, gcc-16; reject: clang-host, edg-7.0, edg-7.0-ilp32, clang-18, clang-22 | accept |
| static const Color v = B<N>::missing | accept | accept: gcc-14, gcc-host, gcc-16; reject: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22 | accept: gcc-host, gcc-16, gcc-14; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept |
| static inline const int v = B<N>::missing | accept | accept: gcc-host, gcc-14, clang-host, gcc-16, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | accept: gcc-14, gcc-host, gcc-16, clang-18, clang-host, clang-22; reject: edg-7.0, edg-7.0-ilp32 | accept |
| static constexpr int v = B<N>::missing | accept | accept: gcc-14, gcc-host, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0 | accept: clang-host, gcc-14, gcc-16, gcc-host, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0 | accept |
| static const int v = N / 0 + 1 | accept | accept: gcc-14, gcc-16, gcc-host; reject: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22 | accept: gcc-host, gcc-14, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept |
