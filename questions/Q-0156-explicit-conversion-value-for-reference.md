# Q-0156: Can direct initialization of a reference use an explicit conversion function that returns a value?

**Status:** provisional
**Affects:** ICS-45, `tests/foundations/conversion-sequences/user-defined-explicit-reference-value.cpp`

## Context

A class `Source` has only `explicit operator Item()`. A reference is
direct-initialized from a `Source` lvalue, for example
`const Item& r(s);` or `static_cast<Item&&>(s)`. The conversion function
returns a prvalue, so the reference would bind to that prvalue or to a
temporary.

| Compiler | `const Item&` | `Item&&` | `int&&` from `explicit operator int()` |
|---|---|---|---|
| Clang 18, Clang 22 | accepted | accepted | accepted |
| GCC 14, GCC 16 | rejected | rejected | rejected |
| EDG 7.0 | rejected | rejected | rejected |

Every compiler accepts `const Item&` from `explicit operator Item&()`,
where the reference binds directly to an lvalue.

## Evidence

`tests/foundations/conversion-sequences/user-defined-explicit-reference-value.cpp`
and `user-defined-explicit.cpp`.

## Options

1. Only an explicit conversion function that returns an lvalue reference
   serves a reference (GCC, EDG).
2. An explicit conversion function that returns a value also serves a
   reference in direct initialization (Clang).

## Provisional answer

No; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
