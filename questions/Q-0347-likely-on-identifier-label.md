# Q-0347: Can `[[likely]]` apply to a label that is an identifier?

**Status:** provisional
**Affects:** ATTR-8, `tests/declarations/attributes/likely-identifier-label.cpp`

## Context

ATTR-8 says that `[[likely]]` and `[[unlikely]]` can apply to a
statement or a label. A label can be a `case` label, a `default` label,
or an identifier that a `goto` names.

GCC 14 and 16 and EDG 7.0 accept `[[likely]]` before an identifier
label and run the program as without the attribute. Clang 18 and 22
reject the program with "'likely' attribute cannot be applied to a
declaration". Clang accepts the same attribute before a `case` label
and a `default` label (`tests/declarations/attributes/likely-unlikely.cpp`).

## Evidence

`tests/declarations/attributes/likely-identifier-label.cpp`, with an
override for Clang.

## Options

1. Valid. The attribute applies to every kind of label, and Clang has a
   defect.
2. Valid only on `case` and `default` labels. ATTR-8 names the label
   kinds, and a program with the attribute on an identifier label is
   rejected.

## Provisional answer

Valid (option 1). Two of three implementations accept the attribute,
and ATTR-8 makes no difference between the kinds of label.

## Opinions
