# Probe grid tables: overload resolution

These tables belong to the page `specification/overloading/overload-resolution.md`. Each table comes from the grid file of the same name in this directory. `tools/probe_grid.py GRID.json` compiles the grid on the baseline compilers and prints the table. A cell shows `accept` or `reject` when every compiler agrees, and lists the compilers on each side when they split. In the labels, lref stands for `&` and rref for `&&`.

## Viability (`viability.json`)

Viability (OVL-22). Each column declares one `f`, and each row calls it.
An `accept` means that the function is viable for the call:

| | none | one | one default | two | one then default | ellipsis | one then ellipsis | pack | one then pack |
|---|---|---|---|---|---|---|---|---|---|
| no argument | accept | reject | accept | reject | reject | accept | reject | accept | reject |
| one argument | reject | accept | accept | reject | accept | accept | accept | accept | accept |
| two arguments | reject | reject | reject | accept | accept | accept | accept | accept | accept |
| three arguments | reject | reject | reject | reject | reject | accept | accept | accept | accept |

## Call contexts (`contexts.json`)

Call contexts (OVL-1 to OVL-5). Each column declares one kind of
candidate, and each row calls it from one context. An `accept` means
that the context takes that kind of candidate:

| | free function | member | static member | explicit object member | call operator | surrogate |
|---|---|---|---|---|---|---|
| name at namespace scope | accept | reject | reject | reject | reject | reject |
| name in a member function | accept | accept | accept | accept | reject | reject |
| name in a static member function | accept | reject | accept | reject | reject | reject |
| qualified name | reject | reject | accept | reject | reject | reject |
| dot call | reject | accept | accept | accept | reject | reject |
| arrow call | reject | accept | accept | accept | reject | reject |
| call on the object | reject | reject | reject | reject | accept | accept |

## Calls on a class object (`object-call.json`)

Calls on a class object (OVL-5, OVL-6, OVL-36). Each column declares two
members of `S`. The first returns `One` and the second `Two`. The row
"X → first" checks that the call `X(1)` chooses the first member, and
"X → second" checks the second. A `reject` in both rows means that the
call is ambiguous or that no candidate is viable:

| | lref call / const lref call | lref call / rref call | lref call / long call | lref call / surrogate | lref call / const surrogate | lref call / reference surrogate | const lref call / rref call | const lref call / long call | const lref call / surrogate | const lref call / const surrogate | const lref call / reference surrogate | rref call / long call | rref call / surrogate | rref call / const surrogate | rref call / reference surrogate | long call / surrogate | long call / const surrogate | long call / reference surrogate | surrogate / const surrogate | surrogate / reference surrogate | const surrogate / reference surrogate |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| S lvalue → first | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| S lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| const S lvalue → first | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | reject | reject | reject | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | reject | reject | reject | reject | reject |
| const S lvalue → second | accept | reject | accept | reject | accept | accept | reject | reject | reject | reject | reject | accept | reject | accept | accept | reject | reject | reject | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-host, gcc-14, gcc-16 | reject |
| S prvalue → first | reject | reject | reject | reject | reject | reject | reject | accept | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| S prvalue → second | accept | accept | accept | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |

## Operators (`operators.json`)

Operators (OVL-7, OVL-9, OVL-32, OVL-33). Each column gives `X` two kinds
of candidate for `x < 1`. Each kind returns its own type, so each row
checks that one kind wins. "rewritten" is a member `operator<=>`, and
"swapped" is a non-member `operator<=>(int, const X&)`:

| | member / non-member | member / built-in | member / rewritten | member / swapped | non-member / built-in | non-member / rewritten | non-member / swapped | built-in / rewritten | built-in / swapped | rewritten / swapped |
|---|---|---|---|---|---|---|---|---|---|---|
| x < 1 → member | reject | accept | accept | accept | reject | reject | reject | reject | reject | reject |
| x < 1 → non-member | reject | reject | reject | reject | accept | accept | accept | reject | reject | reject |
| x < 1 → built-in | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| x < 1 → rewritten | reject | reject | reject | reject | reject | reject | reject | accept | reject | accept |
| x < 1 → swapped | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject |

## Initialization (`initialization.json`)

Initialization (OVL-14). Each column declares how `Src` converts to
`T`, and each row initializes a `T` from the lvalue `s` in one form:

| | converting constructor | explicit constructor | conversion function | explicit conversion function | conversion to derived | constructor and conversion function | explicit constructor and conversion function |
|---|---|---|---|---|---|---|---|
| copy init | accept | reject | accept | reject | accept | reject | accept |
| direct init | accept | accept | accept | accept | accept | accept | accept |
| direct list init | accept | accept | accept | accept | accept | accept | accept |
| copy list init | accept | reject | accept | accept: clang-host, clang-18, clang-22; reject: edg-7.0-ilp32, edg-7.0, gcc-host, gcc-14, gcc-16 | accept | accept | reject |
| functional cast | accept | accept | accept | accept | accept | accept | accept |
| static cast | accept | accept | accept | accept | accept | accept | accept |
| argument | accept | reject | accept | reject | accept | reject | accept |
| return | accept | reject | accept | reject | accept | reject | accept |

## The address of an overload set (`address.json`)

The address of an overload set (OVL-18 to OVL-20). Each column declares
an overload set `f`, and each row takes its address for one target:

| | one plain | two plain | noexcept plain | plain and template | two templates | one template |
|---|---|---|---|---|---|---|
| pointer to One(int) | accept | accept | accept | accept | accept | accept |
| noexcept pointer to One(int) | reject | reject | accept | reject | reject | reject |
| reference to One(int) | accept | accept | accept | accept | accept | accept |
| pointer to Two(long) | reject | accept | accept | accept | reject | reject |
| pointer to Two(int pointer) | reject | reject | reject | accept | accept | reject |
| pointer to Two(double) | reject | reject | reject | accept | reject | reject |
| cast to pointer to One(int) | accept | accept | accept | accept | accept | accept |
| no target | accept | reject | reject | reject | reject | reject |

## Explicit template arguments with no target type (`address-explicit.json`)

Explicit template arguments with no target type (OVL-20). Each column
declares an overload set `f`, and each row takes the address of
`f<int>`:

| | one template | two templates | plain and template | template of two parameters |
|---|---|---|---|---|
| pointer to One(int) | accept | accept | reject | reject |
| no target | accept | reject | accept | reject |
| decltype | accept | reject | accept | reject |

## Surrogates (`surrogates.json`)

Surrogates (OVL-6). Each column declares one conversion function to the
pointer type `F`, and each row calls an object of the class with one
argument. An `accept` means that the conversion function gives a
surrogate for that object:

| | plain | const | lref qualified | const lref qualified | rref qualified | explicit | template |
|---|---|---|---|---|---|---|---|
| S lvalue | accept | accept | accept | accept | reject | reject | reject |
| const S lvalue | reject | accept | reject | accept | reject | reject | reject |
| S prvalue | accept | accept | reject | accept | accept | reject | reject |

## Function references (`reference-kind.json`)

Function references (OVL-15, OVL-27 to OVL-29). Each column declares two
conversion functions of `Source`. The first returns a function that
gives 1, and the second one that gives 2. Each row binds a reference to
function to a `Source` prvalue and checks which function it refers to.
A `reject` in both rows of a target means that the binding is
ambiguous or has no candidate:

| | plain lref / plain rref | plain lref / template lref | plain lref / template rref | plain rref / template lref | plain rref / template rref | template lref / template rref |
|---|---|---|---|---|---|---|
| lref target → first | accept | accept | accept | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-16, gcc-14, clang-18, gcc-host, clang-host, clang-22 | accept: gcc-16, gcc-14, edg-7.0-ilp32, edg-7.0, gcc-host; reject: clang-host, clang-18, clang-22 | accept |
| lref target → second | reject | reject | reject | accept: gcc-host, gcc-16, gcc-14, clang-18, clang-host, clang-22; reject: edg-7.0, edg-7.0-ilp32 | reject | reject |
| rref target → first | reject | accept | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-16, gcc-host, gcc-14 | accept | accept | accept: edg-7.0, edg-7.0-ilp32, clang-host, clang-18, clang-22; reject: gcc-14, gcc-host, gcc-16 |
| rref target → second | accept: edg-7.0, edg-7.0-ilp32, gcc-host, gcc-16, gcc-14; reject: clang-host, clang-18, clang-22 | reject | accept: gcc-16, gcc-host, gcc-14; reject: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22 | reject | reject | accept: gcc-14, gcc-host, gcc-16; reject: edg-7.0-ilp32, edg-7.0, clang-host, clang-18, clang-22 |

## Compound assignment (`compound-integral.json`)

Compound assignment (OVL-9). Each row declares `x` of one type, and each
column applies one operator to `x` and a class that converts to `int`:

| | plus assign | remainder assign | and assign | or assign | shift assign |
|---|---|---|---|---|---|
| bool | accept | accept | accept | accept | accept |
| char | accept | accept | accept | accept | accept |
| signed char | accept | accept | accept | accept | accept |
| unsigned char | accept | accept | accept | accept | accept |
| wchar_t | accept | accept | accept | accept | accept |
| char8_t | accept | accept | accept | accept | accept |
| char16_t | accept | accept | accept | accept | accept |
| short | accept | accept | accept | accept | accept |
| int | accept | accept | accept | accept | accept |
| unsigned long | accept | accept | accept | accept | accept |

## Three-way comparison (`three-way.json`)

Three-way comparison (OVL-9, OVL-11). The row's class is the left
operand and the column's class the right operand of `<=>`. Each class
converts to the type in its label:

| | To bool | To char | To short | To int | To unsigned | To long | To double |
|---|---|---|---|---|---|---|---|
| To bool | accept | reject | reject | reject | reject | reject | reject |
| To char | reject | accept | accept | accept | reject | accept | accept |
| To short | reject | accept | accept | accept | reject | accept | accept |
| To int | reject | accept | accept | accept | reject | accept | accept |
| To unsigned | reject | reject | reject | reject | accept | accept: edg-7.0, gcc-14, gcc-host, gcc-16, clang-host, clang-18, clang-22; reject: edg-7.0-ilp32 | accept |
| To long | reject | accept | accept | accept | accept: edg-7.0, gcc-host, gcc-14, clang-host, gcc-16, clang-18, clang-22; reject: edg-7.0-ilp32 | accept | accept |
| To double | reject | accept | accept | accept | accept | accept | accept |

## Plain functions and templates (`templates.json`)

Plain functions and templates (OVL-25, OVL-27, OVL-28). Each column
declares two functions `f`, in two namespaces that a using-declaration
merges. The first returns `One` and the second `Two`:

| | plain int / plain const int lref | plain int / plain long | plain int / template value | plain int / template const lref | plain int / template pointer | plain int / template forwarding | plain int / constrained template | plain int / ellipsis | plain const int lref / plain long | plain const int lref / template value | plain const int lref / template const lref | plain const int lref / template pointer | plain const int lref / template forwarding | plain const int lref / constrained template | plain const int lref / ellipsis | plain long / template value | plain long / template const lref | plain long / template pointer | plain long / template forwarding | plain long / constrained template | plain long / ellipsis | template value / template const lref | template value / template pointer | template value / template forwarding | template value / constrained template | template value / ellipsis | template const lref / template pointer | template const lref / template forwarding | template const lref / constrained template | template const lref / ellipsis | template pointer / template forwarding | template pointer / constrained template | template pointer / ellipsis | template forwarding / constrained template | template forwarding / ellipsis | constrained template / ellipsis |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| int lvalue → first | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject | accept | accept | reject | reject | accept | reject | reject | reject | reject | accept | accept |
| int lvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | accept | reject | accept | accept | reject | reject | reject | reject | accept | reject | reject | accept | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-14, gcc-16, clang-host, clang-18, clang-22 | reject | reject |
| int prvalue → first | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | accept | reject | reject | accept | reject | reject | accept | reject | accept | reject | reject | accept | accept | reject | reject | accept | reject | reject | reject | reject | accept | accept |
| int prvalue → second | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | accept | reject | accept | accept | reject | reject | reject | reject | accept | reject | reject | accept | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-host, gcc-14, clang-host, gcc-16, clang-18, clang-22 | reject | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-14, gcc-host, gcc-16, clang-host, clang-18, clang-22 | reject | reject |
| long → first | reject | reject | reject | reject | accept | reject | reject | accept | reject | reject | reject | accept | reject | reject | accept | accept | accept | accept | accept | accept | accept | reject | accept | reject | reject | accept | accept | reject | reject | accept | reject | reject | reject | reject | accept | accept |
| long → second | reject | accept | accept | accept | reject | accept | accept | reject | accept | accept | accept | reject | accept | accept | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | accept | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-host, gcc-14, clang-host, gcc-16, clang-22, clang-18 | reject | accept | accept | accept | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-host, gcc-16, clang-host, gcc-14, clang-18, clang-22 | reject | reject |
| int pointer → first | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | reject | reject | reject | accept | accept | accept | accept | reject | accept | accept |
| int pointer → second | reject | reject | accept | accept | accept | accept | accept | accept | reject | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | accept | reject | accept | reject | accept | reject | accept | accept | accept: edg-7.0-ilp32, edg-7.0; reject: gcc-14, gcc-host, gcc-16, clang-22, clang-18, clang-host | reject | reject | reject | reject | accept: edg-7.0, edg-7.0-ilp32; reject: gcc-14, gcc-host, gcc-16, clang-22, clang-18, clang-host | reject | reject |
