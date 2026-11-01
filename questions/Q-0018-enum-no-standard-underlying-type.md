# Q-0018: What happens when no standard integer type holds all of an unscoped enumeration's values?

**Status:** provisional
**Affects:** ENUM-5, `tests/declarations/enumerations/unscoped-value-too-large.cpp`

## Context

`enum Huge { A = 1ULL << 63, B = -1 };` needs a signed type with at least
65 bits. Clang and EDG reject it. GCC accepts it with a 16-byte underlying
type, even under `-pedantic-errors`. Clang's rejection is a pedantic
diagnostic (`-Wenum-too-large`), so Clang also treats such an enumeration
as an extension outside strict mode. With `-fshort-enums`, Clang and GCC
give a small enumeration a 1-byte underlying type. This behavior shows
that the choice of type is the implementation's.

## Evidence

`tests/RESULTS.md`, row `unscoped-value-too-large.cpp`. GCC 16 reports
`sizeof(Huge) == 16`.

## Options

1. `impl-defined`: an implementation may use a wider integer type of its
   own, or reject the enumeration.
2. `reject`: only standard integer types may be used.

## Provisional answer

Impl-defined (option 1). Both behaviors exist, and the choice of underlying type is
already the implementation's (ENUM-5).

## Opinions
