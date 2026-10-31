# Q-0057: Must an anonymous union in an unnamed namespace be static?

**Status:** provisional
**Affects:** UNI-7, `tests/classes/unions/anonymous-union-unnamed-namespace.cpp`

## Context

An anonymous union at namespace scope must be declared `static`. In an
unnamed namespace, the members already have internal linkage. Clang
accepts the union there without `static`. GCC and EDG reject it.

## Options

1. `static` is necessary at every namespace scope, including an unnamed
   namespace.
2. An unnamed namespace is enough.

## Provisional answer

Option 1. GCC and EDG agree, and the rule is the same for every
namespace.

## Opinions
