# Q-0047: May `using enum` name an enumeration that is declared but not defined?

**Status:** provisional
**Affects:** ENUM-9, `tests/declarations/enumerations/using-enum-opaque.cpp`

## Context

`enum class E : int; using enum E;` names an enumeration whose enumerators are not yet known. Clang and GCC reject it. EDG 7.0 accepts it.

## Options

1. Rejected: the enumeration must be complete.
2. Valid.

## Provisional answer

Option 1. Clang and GCC, in both versions, reject it.

## Opinions
