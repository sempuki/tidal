# Q-0168: Is an alias template that passes its parameters through the same template argument as its class template?

**Status:** provisional
**Affects:** TARG-19, TARG-22

## Context

`template <class T> using ListAlias = List<T>;` passes its parameter
to `List` unchanged. Both templates are used as the argument of a
template template parameter, `Use<List>` and `Use<ListAlias>`.

| Question | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| `Use<List>` and `Use<ListAlias>` are one type | no | yes | yes |

## Evidence

`tests/templates/arguments/same-specialization-alias-template.cpp`
asserts that the two types differ. GCC and EDG fail the assertion and
have overrides.

## Options

1. The alias template is a template of its own, and the two types
   differ (Clang).
2. The alias template stands for the class template, and the two types
   are one (GCC, EDG).
3. The result is unspecified, and a program must not rely on either.

## Provisional answer

Option 3. The implementations split, and neither result follows from
another Tidal rule. TARG-22 makes the result unspecified, and TARG-19
covers the other cases.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
