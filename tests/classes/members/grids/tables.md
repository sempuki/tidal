# Probe grid tables for Class definitions and members

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/classes/members.md` says what each grid probes
and how to read its rows and columns. `tools/probe_grid.py GRID.json`
compiles a grid on the baseline compilers and prints its table, so each
table here can be made again from the grid file that its section names.
The tables below come from runs on 2026-11-05.

## layout-compatible

Grid file `layout-compatible.json`.

| | builtin layout-compatible |
|---|---|
| same members | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| cv on one member | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| signed and unsigned | reject |
| extra member | reject |
| different alignment | reject |
| bit-fields same width | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| bit-fields different width | reject |
| bit-field and plain member | reject |
| zero-width bit-field in one | reject |
| enumerations, same underlying type | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| enumeration and its underlying type | reject |
| nested layout-compatible structs | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| arrays of layout-compatible structs | reject |
| members from a base | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| overlap mark on one | reject |
| overlap mark on both | accept: gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host, edg-7.0, edg-7.0-ilp32 |
| unions, members reordered | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| unions, different count | reject |
| struct and union | reject |
| enum types direct | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| int and const int | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| int arrays | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |
| struct arrays | reject |
| different access | accept: clang-22, clang-host, gcc-14, gcc-16, gcc-host; reject: clang-18, edg-7.0, edg-7.0-ilp32 |

## standard-layout

Grid file `standard-layout.json`.

| | builtin standard-layout |
|---|---|
| one access | accept |
| two accesses | reject |
| static member in other access | accept |
| members in base and derived | reject |
| members in base only | accept |
| members two bases down | accept |
| empty bases, members in derived | accept |
| unnamed bit-field in derived | reject |
| zero-width bit-field in derived | reject |
| members in two bases | reject |
| same empty base twice | reject |
| same empty base twice, no members | reject |
| base type is first member type | reject |
| base type is second member type | accept |
| base type first inside first member | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| base type second inside first member | accept |
| first member array of base type | accept: gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host, edg-7.0, edg-7.0-ilp32 |
| first member union holding base type | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| overlap member of base type, not first | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| overlap member of other empty type | accept |
| reference member | reject |
| member not standard-layout | reject |
| base not standard-layout | reject |
| virtual function | reject |
| virtual base | reject |
| array member | accept |
| union, two accesses | reject |
| union, member not standard-layout | reject |
| bit-field first | accept |
| empty class | accept |
| user-provided destructor | accept |
| lambda, no capture | accept |
| lambda, copy capture | accept |
| lambda, reference capture | reject |
| int | accept |
| array of int | accept |
| const class | accept |
| array of non-standard-layout | reject |
