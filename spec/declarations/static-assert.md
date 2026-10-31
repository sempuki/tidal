# Static assertions

A static assertion checks a condition during translation. If the
condition is `false`, the implementation rejects the program and can
show a message. This page covers:

- where a static assertion can occur,
- its condition,
- its message.

Depends on: constant expressions, standard conversions.

## Principles

:::{principle} SA-P1
:rules: SA-1 SA-3 SA-7 SA-4 SA-8

A static assertion is a check that constant evaluation performs on the
program's behalf. The condition is a constant expression, converted as
a condition is, and it must be `true`. A `false` condition makes the
program invalid, and the message, a string literal for the reader, goes
into the diagnostic.
:::

:::{principle} SA-P2
:rules: SA-2 SA-6 SA-5

A static assertion is a declaration, so it can stand wherever a
declaration can, including in a class body and in a block. Like other
code in a template, a static assertion is checked for each instantiation
and not before, and a template that is never instantiated is never
checked.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   static_assert_decl: "static_assert" "(" `constant_expr` ["," `string_literal`] ")" ";"
```

## Constraints

:::{rule} SA-1
:kind: reject
:tests: declarations/static-assert/false-rejected.cpp declarations/static-assert/false-with-message.cpp declarations/static-assert/true-accepted.cpp declarations/static-assert/message-shown.cpp declarations/static-assert/discarded-branch-nontemplate.cpp declarations/static-assert/template-instantiated-false.cpp

The condition of a static assertion must be `true`. If the condition is
`false`, the implementation rejects the program. The diagnostic
includes the message, if there is one.
:::

:::{rule} SA-2
:kind: required
:tests: declarations/static-assert/in-block-not-statement-position.cpp declarations/static-assert/in-block.cpp declarations/static-assert/in-class.cpp declarations/static-assert/in-extern-c.cpp declarations/static-assert/in-namespace.cpp

A static assertion is a declaration. It can occur:

- at namespace scope,
- in a class,
- in a block,
- as the body of a statement such as `if`,
- in a linkage specification.

SA-6 lists what is not valid.
:::

:::{rule} SA-6
:kind: reject
:tests: declarations/static-assert/in-class-incomplete-self.cpp declarations/static-assert/in-for-init.cpp

A static assertion (SA-2) cannot be the initializer of a `for`
statement. In a class, the class is still incomplete, so the condition
must not need a complete class.
:::

:::{rule} SA-3
:kind: required
:tests: declarations/static-assert/condition-contextual-bool.cpp declarations/static-assert/condition-narrowing.cpp declarations/static-assert/condition-pointer.cpp declarations/static-assert/constexpr-function-call.cpp

The condition is a constant expression. It is converted to `bool` by
{term}`contextual conversion`, and a conversion that loses information
is permitted. SA-7 lists what is not valid.
:::

:::{rule} SA-7
:kind: reject
:tests: declarations/static-assert/condition-not-constant.cpp declarations/static-assert/condition-ub.cpp

A condition (SA-3) that is not a constant expression, or whose
evaluation is undefined, makes the program not valid.
:::

:::{rule} SA-4
:kind: required
:tests: declarations/static-assert/message-concatenated.cpp declarations/static-assert/message-optional.cpp declarations/static-assert/message-prefixed-string.cpp declarations/static-assert/message-wide-string.cpp

The message is optional. It is a string literal, with any encoding
prefix, and can be several adjacent string literals. SA-8 lists what is
not valid.
:::

:::{rule} SA-8
:kind: reject
:tests: declarations/static-assert/message-not-literal.cpp declarations/static-assert/message-object.cpp

A variable or an object is not a valid message (SA-4).
:::

:::{rule} SA-5
:kind: required
:tests: declarations/static-assert/discarded-branch-nontemplate.cpp declarations/static-assert/discarded-branch.cpp declarations/static-assert/template-dependent-false.cpp declarations/static-assert/template-instantiated-false.cpp declarations/static-assert/template-nondependent-false.cpp

In a template, a condition is checked when the template is instantiated.
A static assertion in a template that is never instantiated is not
checked, even if its condition does not depend on a template parameter.
A static assertion in a {term}`discarded statement` of a template is not
checked. Outside a template, a discarded statement is checked.
:::

## Examples

```cpp
static_assert(sizeof(void*) == 8, "needs a 64-bit target");

struct Header {
  unsigned char kind;
  unsigned char flags;
  static_assert(sizeof(int) >= 4);   // valid in a class
};

template <class T> T half(T v) {
  static_assert(sizeof(T) > 1, "type too small");
  return v / 2;
}
int h = half(10);                     // checked for T = int: valid
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on all
28 tests. Clang, GCC, and EDG each show the message of a failed
assertion.

Inputs: clang-sema-declcxx, clang-diag-sema, clang-diag-parse,
edg-decls.
