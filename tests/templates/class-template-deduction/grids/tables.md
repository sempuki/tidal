# Probe grid tables for Class template argument deduction

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/templates/class-template-deduction.md` says what each grid probes
and how to read its rows and columns. `tools/probe_grid.py GRID.json`
compiles a grid on the baseline compilers and prints its table, so each
table here can be made again from the grid file that its section names.
The tables below come from runs on 2026-11-05.

## forms

Grid file `forms.json`.

| | constructor | constructor template | explicit constructor | initializer-list constructor | aggregate | user guide | explicit user guide | alias template | inherited constructor |
|---|---|---|---|---|---|---|---|---|---|
| parentheses | accept | accept | accept | reject | accept | accept | accept | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| braces | accept | accept | accept | accept | accept | accept | accept | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| copy | accept | accept | reject | reject | reject | accept | reject | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| copy-list | accept | accept | reject | accept | accept | accept | reject | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| functional cast | accept | accept | accept | reject | accept | accept | accept | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| braced cast | accept | accept | accept | accept | accept | accept | accept | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| new | accept | accept | accept | reject | accept: clang-18, clang-22, clang-host, edg-7.0, edg-7.0-ilp32; reject: gcc-14, gcc-16, gcc-host | accept | accept | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18, clang-22, clang-host |
| copy, parentheses | accept | accept | accept | accept | accept | accept | accept | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept |
| copy, braces | accept | accept | accept | accept | accept | accept | accept | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept |
| copy, equals | accept | accept | accept | accept | accept | accept | accept | accept: clang-22, clang-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16, gcc-host; reject: clang-18 | accept |
