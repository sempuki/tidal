# Probe grid tables for implicit conversion sequences

These tables give the outcome of every cell of the probe grids in this
directory. The page `specification/overloading/conversion-sequences.md`
names them in its Evidence. `tools/probe_grid.py GRID.json` compiles a
grid on the baseline compilers and prints its table, so each table here
can be made again from the grid file that its section names.

Each column of a grid declares two overloads, `One f(a)` and `Two f(b)`.
The row "X → first" checks that the call with X chooses `f(a)`, and the
row "X → second" checks that the call chooses `f(b)`. An `accept` in one
of the two rows names the better sequence. A `reject` in both rows means
that the sequences tie or that no function is viable. A cell where the
compilers split lists the compilers on each side. In the labels, lref
stands for `&` and rref for `&&`. In the object-parameter grid, each
column gives the qualifiers of a member `operator~` and the parameter of
a non-member `operator~`. In the order grid, each column gives the
parameters of `f(a)` and `f(b)`, and each row gives the argument.

## Arithmetic arguments

Grid file `arithmetic.json`. Rules: ICS-2, ICS-27, ICS-32, ICS-34.

| | bool / short | bool / int | bool / long | bool / float | bool / double | short / int | short / long | short / float | short / double | int / long | int / float | int / double | long / float | long / double | float / double |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| bool → first | accept | accept | accept | accept | accept | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| bool → second | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| char → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| char → second | reject | accept | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| short → first | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| short → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| int → second | reject | accept | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| unsigned → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| unsigned → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| long → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject |
| long → second | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject |
| float → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| float → second | reject | reject | reject | accept | accept | reject | reject | accept | accept | reject | accept | accept | accept | accept | reject |
| double → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| double → second | reject | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | accept | accept |
| long double → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| long double → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| enum → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| enum → second | reject | accept | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| enum : short → first | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| enum : short → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| enum class → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| enum class → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| long long : 3 bit-field → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept: clang-host, clang-22, clang-18; reject: gcc-host, edg-7.0, gcc-14, gcc-16, edg-7.0-ilp32 | accept: clang-host, clang-22, clang-18; reject: edg-7.0, gcc-16, edg-7.0-ilp32, gcc-host, gcc-14 | accept: clang-22, clang-18, clang-host; reject: edg-7.0, gcc-host, edg-7.0-ilp32, gcc-16, gcc-14 | reject | reject | reject |
| long long : 3 bit-field → second | reject | accept: clang-host, clang-22, clang-18; reject: edg-7.0, edg-7.0-ilp32, gcc-14, gcc-host, gcc-16 | reject | reject | reject | accept: clang-host, clang-18, clang-22; reject: edg-7.0, gcc-16, gcc-host, edg-7.0-ilp32, gcc-14 | reject | reject | reject | reject | reject | reject | reject | reject | reject |

## Pointer arguments

Grid file `pointers.json`. Rules: ICS-26, ICS-31 to ICS-33, ICS-36.

| | void* / const void* | void* / Base* | void* / Middle* | void* / const Middle* | void* / bool | const void* / Base* | const void* / Middle* | const void* / const Middle* | const void* / bool | Base* / Middle* | Base* / const Middle* | Base* / bool | Middle* / const Middle* | Middle* / bool | const Middle* / bool |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Derived* → first | accept | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept |
| Derived* → second | reject | accept | accept | accept | reject | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject |
| const Derived* → first | reject | reject | reject | reject | reject | accept | accept | reject | accept | reject | reject | reject | reject | reject | accept |
| const Derived* → second | accept | reject | reject | accept | accept | reject | reject | accept | reject | reject | accept | accept | accept | accept | reject |
| Middle* → first | accept | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept |
| Middle* → second | reject | accept | accept | accept | reject | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject |
| Derived[2] → first | accept | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept |
| Derived[2] → second | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-22, clang-18; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22; reject: gcc-14, gcc-host, gcc-16 | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-14, gcc-host, gcc-16 | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | reject | reject | reject | reject |
| 0 → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| 0 → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| nullptr → first | reject | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | reject | accept | accept |
| nullptr → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |

## Member pointer arguments

Grid file `member-pointers.json`. Rules: ICS-32, ICS-36.

| | int Base::* / int Middle::* | int Base::* / int Derived::* | int Base::* / bool | int Middle::* / int Derived::* | int Middle::* / bool | int Derived::* / bool |
|---|---|---|---|---|---|---|
| &Base::x → first | accept | accept | accept | accept | accept | accept |
| &Base::x → second | reject | reject | reject | reject | reject | reject |
| &Middle::y → first | reject | reject | reject | accept | accept | accept |
| &Middle::y → second | accept | accept | accept | reject | reject | reject |
| 0 → first | reject | reject | reject | reject | reject | reject |
| 0 → second | reject | reject | reject | reject | reject | reject |
| nullptr → first | reject | reject | accept | reject | accept | accept |
| nullptr → second | reject | reject | reject | reject | reject | reject |

## Function arguments

Grid file `functions.json`. Rules: ICS-26, ICS-29, ICS-32.

| | pointer / noexcept pointer | pointer / lref | pointer / rref | pointer / bool | noexcept pointer / lref | noexcept pointer / rref | noexcept pointer / bool | lref / rref | lref / bool | rref / bool |
|---|---|---|---|---|---|---|---|---|---|---|
| run → first | accept | reject | reject | accept | reject | reject | reject | accept | accept | accept |
| run → second | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| stop (noexcept) → first | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept |
| stop (noexcept) → second | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject |

## Reference parameters

Grid file `references.json`. Rules: ICS-11, ICS-12, ICS-28, ICS-30.

| | int / int lref | int / const int lref | int / const volatile int lref | int / int rref | int / const int rref | int / const long lref | int lref / const int lref | int lref / const volatile int lref | int lref / int rref | int lref / const int rref | int lref / const long lref | const int lref / const volatile int lref | const int lref / int rref | const int lref / const int rref | const int lref / const long lref | const volatile int lref / int rref | const volatile int lref / const int rref | const volatile int lref / const long lref | int rref / const int rref | int rref / const long lref | const int rref / const long lref |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int lvalue → first | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| const int lvalue → first | accept | reject | reject | accept | accept | accept | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| const int lvalue → second | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| int xvalue → first | accept | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | accept | accept | accept |
| int xvalue → second | reject | reject | reject | reject | reject | reject | accept | reject | accept | accept | accept | reject | accept | accept | reject | accept | accept | accept | reject | reject | reject |
| int prvalue → first | accept | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | accept | accept | accept |
| int prvalue → second | reject | reject | reject | reject | reject | reject | accept | reject | accept | accept | accept | reject | accept | accept | reject | accept | accept | accept | reject | reject | reject |
| short lvalue → first | accept | reject | accept | reject | reject | accept | reject | reject | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | accept | accept | accept |
| short lvalue → second | reject | reject | reject | reject | reject | reject | accept | reject | accept | accept | accept | reject | accept | accept | reject | accept | accept | accept | reject | reject | reject |
| int : 3 bit-field → first | reject | reject | reject | accept | accept | accept | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| int : 3 bit-field → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |

## Class arguments and parameters

Grid file `classes.json`. Rules: ICS-2, ICS-11, ICS-18, ICS-21, ICS-36.

| | Base / Middle | Base / const Middle lref | Base / Middle rref | Base / long | Base / Box | Base / ellipsis | Middle / const Middle lref | Middle / Middle rref | Middle / long | Middle / Box | Middle / ellipsis | const Middle lref / Middle rref | const Middle lref / long | const Middle lref / Box | const Middle lref / ellipsis | Middle rref / long | Middle rref / Box | Middle rref / ellipsis | long / Box | long / ellipsis | Box / ellipsis |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Derived lvalue → first | reject | reject | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| Derived lvalue → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | accept | accept |
| Derived prvalue → first | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| Derived prvalue → second | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| Middle prvalue → first | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| Middle prvalue → second | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| int → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept |
| int → second | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| ToMiddle → first | reject | reject | reject | accept | accept | accept | reject | reject | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| ToMiddle → second | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| ToInt → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject |
| ToInt → second | reject | reject | reject | accept | reject | accept | reject | reject | accept | reject | accept | reject | accept | reject | accept | accept | reject | accept | reject | reject | accept |
| braced {d} → first | reject | reject | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| braced {d} → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |

## User-defined sequences

Grid file `user-defined.json`. Rules: ICS-3, ICS-7, ICS-21, ICS-25.

| | int / long | int / double | int / Box | int / Wide | int / ellipsis | long / double | long / Box | long / Wide | long / ellipsis | double / Box | double / Wide | double / ellipsis | Box / Wide | Box / ellipsis | Wide / ellipsis |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int → first | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | accept | accept |
| int → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| ToInt → first | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| ToInt → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| ToShort → first | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | reject | reject | reject |
| ToShort → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |
| ToBoth → first | reject | reject | accept | accept | accept | reject | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| ToBoth → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |

## Braced-list arguments

Grid file `lists.json`. Rules: ICS-15 to ICS-19, ICS-22 to ICS-24.

| | int / initializer_list<int> | int / initializer_list<long> | int / int[2] rref | int / int[] rref | int / Point | int / Pair | initializer_list<int> / initializer_list<long> | initializer_list<int> / int[2] rref | initializer_list<int> / int[] rref | initializer_list<int> / Point | initializer_list<int> / Pair | initializer_list<long> / int[2] rref | initializer_list<long> / int[] rref | initializer_list<long> / Point | initializer_list<long> / Pair | int[2] rref / int[] rref | int[2] rref / Point | int[2] rref / Pair | int[] rref / Point | int[] rref / Pair | Point / Pair |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| empty {} → first | reject | reject | accept: gcc-14, gcc-16, gcc-host; reject: edg-7.0-ilp32, clang-18, clang-host, edg-7.0, clang-22 | accept: clang-host, clang-18, gcc-16, gcc-14, gcc-host, clang-22; reject: edg-7.0-ilp32, edg-7.0 | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept: clang-host, clang-18, clang-22; reject: edg-7.0-ilp32, gcc-16, gcc-14, edg-7.0, gcc-host | accept | accept | accept: gcc-host, gcc-14, gcc-16; reject: clang-host, edg-7.0-ilp32, edg-7.0, clang-18, clang-22 | accept: gcc-16, gcc-host, gcc-14; reject: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22 | accept |
| empty {} → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept: gcc-14, gcc-host, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | reject | reject | accept: clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-14, gcc-16 | reject | reject |
| one {1} → first | reject | reject | accept: gcc-host, gcc-14, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept: gcc-host, gcc-14, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept |
| one {1} → second | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject |
| two {1, 2} → first | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject |
| two {1, 2} → second | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| three {1, 2, 3} → first | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | accept | accept | reject |
| three {1, 2, 3} → second | accept | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject |
| nested {{1}} → first | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept |
| nested {{1}} → second | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject |
| designated {.x = 1} → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| designated {.x = 1} → second | reject | reject | reject | reject | accept | reject | reject | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept: clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, edg-7.0-ilp32, edg-7.0, gcc-16 | reject | accept: clang-host, clang-18, clang-22; reject: gcc-host, edg-7.0, edg-7.0-ilp32, gcc-14, gcc-16 | reject | reject |

## Object parameters

Grid file `object-parameter.json`. Rules: ICS-13, ICS-28.

| | no qualifier / Item lref | no qualifier / Item rref | const / const Item lref | const / const Item rref | const lref / const Item rref | lref / Item rref | rref / Item lref | rref / const Item lref |
|---|---|---|---|---|---|---|---|---|
| Item lvalue → member | reject | accept | reject | accept | accept | accept | reject | reject |
| Item lvalue → non-member | reject | reject | reject | reject | reject | reject | accept | accept |
| const Item lvalue → member | reject | reject | reject | accept | accept | reject | reject | reject |
| const Item lvalue → non-member | reject | reject | reject | reject | reject | reject | reject | accept |
| Item prvalue → member | accept | reject | reject | reject | reject | reject | accept | accept |
| Item prvalue → non-member | reject | reject | reject | reject | accept | accept | reject | reject |

## Comparison order

Grid file `order.json`. Rules: ICS-28, ICS-31 to ICS-36.

| | const short lref / int rref | void* const lref / bool rref | const Middle lref / Base rref | Base* const lref / void* rref | Middle* const lref / Base* rref | int Middle::* const lref / int Derived::* rref | const double lref / long double rref | const int* const lref / const volatile int* rref |
|---|---|---|---|---|---|---|---|---|
| low (enum : short) → first | accept: clang-18, clang-host, clang-22; reject: gcc-host, gcc-16, gcc-14, edg-7.0-ilp32, edg-7.0 | reject | reject | reject | reject | reject | reject | reject |
| low (enum : short) → second | accept: gcc-14, edg-7.0-ilp32, edg-7.0, gcc-host, gcc-16; reject: clang-18, clang-host, clang-22 | accept | reject | reject | reject | reject | accept | reject |
| &n (int*) → first | reject | accept: gcc-host, gcc-16, clang-host, gcc-14, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0 | reject | reject | reject | reject | reject | accept: gcc-14, gcc-16, gcc-host; reject: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22 |
| &n (int*) → second | reject | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | accept | reject | reject | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 |
| Derived{} → first | reject | reject | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | reject | reject | reject | reject | reject |
| Derived{} → second | reject | reject | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | reject | reject | reject | reject |
| &d (Derived*) → first | reject | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | reject | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0 | reject | reject | reject |
| &d (Derived*) → second | reject | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | reject | reject |
| &Base::x → first | reject | reject | reject | reject | reject | accept: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0, edg-7.0-ilp32 | reject | reject |
| &Base::x → second | reject | accept | reject | reject | reject | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-14, gcc-host, gcc-16, clang-host, clang-18, clang-22 | reject | reject |
| 1.0f64 → first | reject | reject | reject | reject | reject | reject | accept: gcc-14, gcc-host, gcc-16; reject: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22 | reject |
| 1.0f64 → second | accept: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-14, gcc-16; reject: clang-host, clang-18, clang-22 | accept: edg-7.0-ilp32, edg-7.0, gcc-14, gcc-host, gcc-16; reject: clang-host, clang-18, clang-22 | reject | reject | reject | reject | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject |

## Braced lists for references

Grid file `list-references.json`. Rules: ICS-12, ICS-20, ICS-30.

| | int lref / const int lref | int lref / int rref | int lref / const int rref | int lref / const long lref | int lref / long | const int lref / int rref | const int lref / const int rref | const int lref / const long lref | const int lref / long | int rref / const int rref | int rref / const long lref | int rref / long | const int rref / const long lref | const int rref / long | const long lref / long |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| {i} int lvalue → first | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| {i} int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject |
| {ci} const int lvalue → first | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| {ci} const int lvalue → second | accept | reject | reject | accept | accept | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject |
| {1} prvalue → first | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject |
| {1} prvalue → second | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject |
| {s} short lvalue → first | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject |
| {s} short lvalue → second | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject |
