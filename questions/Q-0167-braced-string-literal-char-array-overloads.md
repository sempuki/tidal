# Q-0167: Does one braced string literal prefer a reference to a `char` array over one to an `unsigned char` array?

**Status:** provisional
**Affects:** ICS-38, LIST-6
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

The call is `pick({"ab"})`. The candidates are
`pick(const char (&)[3])` and `pick(const unsigned char (&)[3])`.
The literal binds the first reference directly (REF-6). For the second,
the literal initializes a temporary array (LIST-6, REF-14). ICS-38 gives
both the identity.

| Candidates | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| `const char (&)[3]`, `const unsigned char (&)[3]` | ambiguous | `char` | ambiguous |
| `const signed char (&)[3]`, `const unsigned char (&)[3]` | ambiguous | no viable function | ambiguous |

GCC gives the literal no sequence to the `signed char` or
`unsigned char` array (Q-0166), so GCC's choice of `char` follows from
that split.

## Evidence

Probes on the eight baseline compilers. No test yet. A test for ICS-38
would assert that the first call is ambiguous, with a GCC override.

## Options

1. Both sequences are the identity, and nothing tells them apart, so the
   call is ambiguous (Clang, EDG, and ICS-38 as written).
2. The direct binding is better (GCC's result, though GCC reaches it by
   giving the other candidate no sequence).

## Provisional answer

No, ambiguous; Clang and EDG agree, GCC picks `char` (option 1). Two of three implementations agree, and ICS-38 gives both
bindings the identity.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
