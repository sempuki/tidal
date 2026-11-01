# Q-0045: Must an anonymous union in an unnamed namespace be declared `static`?

**Status:** provisional
**Affects:** STO-11, UNI-9, `tests/declarations/storage-linkage/anonymous-union-unnamed-namespace.cpp`, `tests/classes/unions/anonymous-union-unnamed-namespace.cpp`

## Context

A namespace-scope anonymous union must be declared `static`. In an
unnamed namespace, the members already have internal linkage. Clang 22
and 18 accept an anonymous union there without `static`. GCC 16 and 14
and EDG 7.0 reject the union. The gap review summary says the unnamed
namespace form is valid.

STO-11 states the rule. UNI-9 lists the other limits on an anonymous
union and refers to STO-11 for this one. Q-0057 asked the same question
for UNI-9 and is a duplicate of this question.

## Evidence

The two tests in **Affects**. Each has an override for Clang.

## Options

1. `static` is necessary at every namespace scope, including an unnamed
   namespace, as GCC and EDG require.
2. An unnamed namespace is enough, as Clang does and the review summary
   says.

## Provisional answer

Option 1. GCC and EDG agree, and the rule is the same for every
namespace. The review summary's reading is a strong reason to revisit
the answer.

## Opinions
