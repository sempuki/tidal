# Probe grid tables for Copy and move

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/classes/copy-move.md` says what each grid probes
and how to read its rows and columns. `tools/probe_grid.py GRID.json`
compiles a grid on the baseline compilers and prints its table, so each
table here can be made again from the grid file that its section names.
The tables below come from runs on 2026-11-05.

## trivially-copyable

Grid file `trivially-copyable.json`.

| | builtin trivially copyable | builtin trivial | X copies | U default | U copy | U move | U copy assign | U move assign | U destroy |
|---|---|---|---|---|---|---|---|---|---|
| int members | accept | accept | accept | accept | accept | accept | accept | accept | accept |
| const member | accept | accept: clang-18, clang-22, clang-host, gcc-14; reject: edg-7.0, edg-7.0-ilp32, gcc-16, gcc-host | accept | reject | accept | accept | reject | reject | accept |
| reference member | accept | accept: clang-18, clang-22, clang-host, gcc-14; reject: edg-7.0, edg-7.0-ilp32, gcc-16, gcc-host | accept | reject | accept | accept | reject | reject | accept |
| volatile member | accept | accept | accept | accept | accept | accept | accept | accept | accept |
| user-provided destructor | reject | reject | accept | accept | accept | accept | accept | accept | reject |
| deleted destructor | accept | accept | accept | reject | reject | reject | accept | accept | reject |
| virtual function | reject | reject | accept | reject | reject | reject | reject | reject | accept |
| virtual destructor | reject | reject | accept | reject | reject | reject | reject | reject | reject |
| virtual base | reject | reject | accept | reject | reject | reject | reject | reject | accept |
| four copy and move deleted | accept | accept | reject | accept | reject | reject | reject | reject | accept |
| only move constructor | accept | accept | reject | accept | reject | accept | reject | reject | accept |
| copy deleted, move defaulted | accept | accept | reject | accept | reject | accept | reject | accept | accept |
| copy defaulted after declaration | reject | reject | accept | accept | reject | reject | accept | accept | accept |
| user-provided copy | reject | reject | accept | accept | reject | reject | accept | accept | accept |
| two copy constructors | reject | reject | accept | accept | accept: clang-18, clang-22, clang-host; reject: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host | accept: clang-18, clang-22, clang-host, edg-7.0, edg-7.0-ilp32; reject: gcc-14, gcc-16, gcc-host | accept | accept | accept |
| member with two copy constructors | accept: clang-18, clang-22, clang-host; reject: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host | accept: clang-18, clang-22, clang-host; reject: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host | accept | accept | accept: clang-18, clang-22, clang-host; reject: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host | accept: clang-18, clang-22, clang-host, edg-7.0, edg-7.0-ilp32; reject: gcc-14, gcc-16, gcc-host | accept | accept | accept |
| member template wins | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host | accept: edg-7.0, edg-7.0-ilp32, gcc-14; reject: clang-18, clang-22, clang-host, gcc-16, gcc-host | reject | accept | reject | reject | accept | accept | accept |
| constrained, holds | accept | accept | accept | accept | accept: clang-18, clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: edg-7.0, edg-7.0-ilp32 | accept | accept | accept | accept |
| constrained, fails | reject | reject | accept | accept | reject | reject | accept | accept | accept |
| private trivial copy | accept | accept | reject | accept | reject | reject | accept | accept | accept |
| member with user copy | reject | reject | accept | accept | reject | reject | accept | accept | accept |
| default member initializer | accept | reject | accept | reject | accept | accept | accept | accept | accept |
| user-provided default constructor | accept | reject | accept | reject | accept | accept | accept | accept | accept |
| virtual function, defaulted destructor | reject | reject | accept | reject | reject | reject | reject | reject | accept |
| lambda, no capture | accept | accept | accept | accept | accept | accept | accept | accept | accept |
| lambda, copy capture | accept | reject | accept | reject | accept | accept | reject | reject | accept |
| lambda, reference capture | accept | reject | accept | reject | accept | accept | reject | reject | accept |
| array of class | accept | accept | reject | accept | accept | accept | accept | accept | accept |
| const class | accept | accept | accept | reject | accept | accept | reject | reject | accept |
| volatile class | accept | accept | reject | accept | reject | reject | reject | accept: edg-7.0, edg-7.0-ilp32; reject: clang-18, clang-22, clang-host, gcc-14, gcc-16, gcc-host | accept |
| volatile int | accept | accept | accept | accept | accept | accept | accept | accept | accept |
| enumeration | accept | accept | accept | accept | accept | accept | accept | accept | accept |
| pointer to member | accept | accept | accept | accept | accept | accept | accept | accept | accept |
| nullptr type | accept | accept | accept | accept | accept | accept | accept | accept | accept |
| reference | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| union, trivial members | accept | accept | accept | accept | accept | accept | accept | accept | accept |
| union, member with user copy, own copy | reject | reject | accept | reject | reject | reject | accept | accept | accept |
