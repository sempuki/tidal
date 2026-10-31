# Static assertions

A static assertion checks a condition during translation. If the
condition is `false`, the implementation rejects the program and can show
a message. This page covers where a static assertion can occur, its
condition, and its message.

Depends on: constant expressions, standard conversions.

## Syntax

```{eval-rst}
.. productionlist:: sa
   static_assert_decl: "static_assert" "(" `constant_expression` ["," `string_literal`] ")" ";"
```

## Constraints

:::{rule} SA-1
:kind: reject
:tests: declarations/static-assert/false-rejected.cpp declarations/static-assert/false-with-message.cpp declarations/static-assert/true-accepted.cpp

The condition of a static assertion must be `true`. If it is `false`, the
implementation rejects the program.
:::

:::{rule} SA-2
:kind: required
:tests: declarations/static-assert/in-block-not-statement-position.cpp declarations/static-assert/in-block.cpp declarations/static-assert/in-class-incomplete-self.cpp declarations/static-assert/in-class.cpp declarations/static-assert/in-extern-c.cpp declarations/static-assert/in-for-init.cpp declarations/static-assert/in-namespace.cpp

A static assertion is a declaration. It can occur at namespace scope, in
a class, in a block, as the body of a statement such as `if`, and in a
linkage specification. It cannot be the initializer of a `for`
statement. In a class, the class is still incomplete.
:::

:::{rule} SA-3
:kind: required
:tests: declarations/static-assert/condition-contextual-bool.cpp declarations/static-assert/condition-narrowing.cpp declarations/static-assert/condition-not-constant.cpp declarations/static-assert/condition-pointer.cpp declarations/static-assert/condition-ub.cpp declarations/static-assert/constexpr-function-call.cpp

The condition is a constant expression. It is converted to `bool` by
{term}`contextual conversion`, and a conversion that loses information
is permitted. A condition that is not a constant expression, or whose
evaluation is undefined, makes the program not valid.
:::

:::{rule} SA-4
:kind: required
:tests: declarations/static-assert/message-concatenated.cpp declarations/static-assert/message-not-literal.cpp declarations/static-assert/message-object.cpp declarations/static-assert/message-optional.cpp declarations/static-assert/message-prefixed-string.cpp declarations/static-assert/message-wide-string.cpp

The message is optional. It is a string literal, with any encoding
prefix, and can be several adjacent string literals. A variable or an
object is not a valid message.
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

The results are in `tests/RESULTS.md`. All seven compilers agree on all
27 tests.

Inputs: clang-sema-declcxx, clang-diag-sema, clang-diag-parse, edg-decls.
