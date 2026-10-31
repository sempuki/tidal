# decltype, unevaluated operands, and discarded values

`decltype` gives the type of an expression without evaluating it. Its
operand is one of the *unevaluated operands* of the language, with those
of `sizeof` and `noexcept`. A *discarded-value expression* is evaluated
only for its effects, and its value is not used. This page covers the
type that `decltype` gives, what an unevaluated operand allows, and what
happens to a discarded value.

Depends on: primary expressions, value categories, unary operators.

## Principles

:::{principle} DTY-P1
:rules: DTY-1 DTY-2 DTY-4

`decltype` answers one of two questions, and the parentheses choose
which. For a plain name it reports the declared type of the entity, with
the reference and the qualifiers the declaration wrote, because that is
what the programmer meant by the name. For any other expression,
including a name in parentheses, it reports the type of the expression
marked with its value category ({principle}`VC-P1`): a reference for an
lvalue, an rvalue reference for an xvalue, and the bare type for a
prvalue. An overloaded function name has no single type, so it fits
neither question.
:::

:::{principle} DTY-P2
:rules: DTY-3
:exceptions: DTY-5

The result of `decltype` is a type, so it can stand wherever a type name
can stand, including inside a qualified name. The one place it cannot
stand is the qualified name of a declaration, where the name must be
written out.
:::

:::{principle} UNEV-P1
:rules: UNEV-1 UNEV-4

Some operands exist only to be asked about. The operands of `decltype`,
`sizeof`, and `noexcept` are never evaluated, so they cause no side
effect, need no definition for what they name, and may describe an
operation that would be undefined if it ran. A member can be named there
without an object. The implementation still has to understand the
expression, so lookup, access control, and deleted functions are checked
as anywhere else. A side effect written in such an operand never
happens, and an implementation should say so.
:::

:::{principle} UNEV-P2
:rules: UNEV-2 UNEV-3

A discarded value is the opposite case. The expression is evaluated in
full and its result is dropped, so its effects are the point. A class
prvalue becomes a temporary and is destroyed at once, a glvalue is not
copied, and a volatile object named by the expression is read, because
the read is an effect. A cast to `void` says the discard is intended. An
expression with no effect in that position is probably a mistake, and an
implementation should warn.
:::

## Syntax

```{eval-rst}
.. productionlist:: dty
   decltype_specifier: "decltype" "(" `expression` ")"
                     : "decltype" "(" "auto" ")"
```

`decltype(auto)` is a placeholder type. Its rules are with placeholder
types. A discarded-value expression is the expression of an expression
statement, the left operand of `,`, or the operand of a cast to `void`.

## Constraints

:::{rule} DTY-1
:kind: required
:tests: expressions/decltype/bitfield-declared.cpp expressions/decltype/captured-in-lambda.cpp expressions/decltype/enumerator-name.cpp expressions/decltype/function-name.cpp expressions/decltype/member-declared-type.cpp expressions/decltype/name-declared-type.cpp expressions/decltype/parameter-name.cpp expressions/decltype/structured-binding-name.cpp

If the operand of `decltype` is a name without parentheses, the result
is the type that the entity was declared with. This applies to:

- variables, parameters, and references, with their cv-qualifiers and
  reference,
- member access `a.m` and `p->m`, which gives the declared type of `m`,
- functions, enumerators, bit-fields, and structured bindings.

A name of a variable that a lambda captures gives the type of the
variable, not of the capture.
:::

:::{rule} DTY-4
:kind: reject
:tests: expressions/decltype/overloaded-name.cpp

The operand of `decltype` must not be the name of an overloaded function.
:::

:::{rule} DTY-2
:kind: required
:tests: expressions/decltype/conditional.cpp expressions/decltype/paren-lvalue.cpp expressions/decltype/prvalue-const-class.cpp expressions/decltype/prvalue.cpp expressions/decltype/string-literal.cpp expressions/decltype/xvalue.cpp

For any other operand of type `T`, the value category sets the result:

| Operand | Result |
|---|---|
| lvalue | `T&` |
| xvalue | `T&&` |
| prvalue | `T` |

A name in parentheses, such as `(x)`, is an lvalue, so the result is a
reference. A prvalue of a non-class type has no cv-qualifiers.
:::

:::{rule} DTY-3
:kind: required
:tests: expressions/decltype/as-type-specifier.cpp expressions/decltype/base-specifier.cpp expressions/decltype/decltype-auto-function.cpp expressions/decltype/nested-name.cpp expressions/decltype/trailing-return.cpp expressions/decltype/destructor-call.cpp

`decltype` can occur where a type name can occur. This includes a
variable or function type, a trailing return type, a base list, and the
qualified name of a member, such as `decltype(s)::type`, and the name
after `~` in an explicit destructor call.
:::

:::{rule} DTY-5
:kind: reject
:tests: expressions/decltype/declarative-nested-name.cpp

`decltype` must not occur in the qualified name of a declaration, such as
the name of a member function defined outside its class
({question}`Q-0065`).
:::

:::{rule} UNEV-1
:kind: required
:tests: expressions/decltype/abstract-prvalue-decltype.cpp expressions/decltype/constexpr-not-needed.cpp expressions/decltype/deleted-function-in-unevaluated.cpp expressions/decltype/incomplete-return-prvalue.cpp expressions/decltype/incomplete-return-sizeof.cpp expressions/decltype/lambda-in-unevaluated.cpp expressions/decltype/member-without-object.cpp expressions/decltype/nested-class-member-evaluated.cpp expressions/decltype/nested-class-member-unevaluated.cpp expressions/decltype/no-definition-needed.cpp expressions/decltype/no-side-effects.cpp expressions/decltype/odr-use-not-counted.cpp expressions/decltype/private-access-still-checked.cpp expressions/decltype/template-instantiation-not-needed.cpp expressions/decltype/ub-not-evaluated.cpp expressions/decltype/deleted-destructor-decltype.cpp expressions/decltype/deleted-destructor-sizeof.cpp expressions/decltype/incomplete-member-access.cpp

The operands of `decltype`, `sizeof`, and `noexcept` are not evaluated.
These rules apply to them:

- No side effect occurs.
- A function or variable that only such an operand uses needs no
  definition.
- An expression that would be undefined if evaluated is valid.
- A non-static member can be named without an object, also from a nested
  class.
- A lambda expression can occur.
- A function call does not need the definition of a template that only
  its return type names.

For `decltype`, a function call that gives a prvalue of class type can
have an incomplete or abstract type, and the class needs no usable
destructor. `sizeof` needs a usable destructor. A member access on such
a call needs a complete class. Name lookup, access control, and deleted functions are
still checked.
:::

## Dynamic Semantics

:::{rule} UNEV-2
:kind: required
:tests: expressions/decltype/discarded-call-runs.cpp expressions/decltype/discarded-constant.cpp expressions/decltype/discarded-prvalue-materialized.cpp expressions/decltype/discarded-reference-no-copy.cpp expressions/decltype/void-cast-no-warning.cpp expressions/decltype/discarded-volatile-read.cpp expressions/decltype/discarded-volatile-operand-read.cpp

A discarded-value expression is evaluated, and its value is not used. A
prvalue of class type creates a temporary object, which is then
destroyed. A glvalue is not copied. A cast to `void` makes any
expression a discarded-value expression and stops warnings about an
unused result. Constant evaluation follows these rules.

If the expression is a name, a member access, a subscript, or an
indirection that refers to a volatile object, the program reads that
object. A volatile operand of an operator is read too. Constant
evaluation rejects both.
:::

## Recommended Practice

:::{rule} UNEV-3
:kind: recommended
:tests: expressions/decltype/discarded-warns.cpp

An implementation should warn about a discarded-value expression that
has no effect, such as `x == 1;`.
:::

:::{rule} UNEV-4
:kind: recommended
:tests: expressions/decltype/side-effect-unevaluated-warns.cpp

An implementation should warn about an unevaluated operand with a side
effect, such as `sizeof(i++)`, because the effect does not occur.
:::

## Examples

```cpp
int i = 0;
const int& cr = i;
struct S { double d; } s;

decltype(i)      a;          // int
decltype((i))    b = i;      // int&
decltype(cr)     c = i;      // const int&
decltype(s.d)    d;          // double
decltype((s.d))  e = s.d;    // double&
decltype(i + 1)  f;          // int

int log_once();              // no definition needed below
auto n = sizeof(log_once()); // log_once is not called
(void)log_once();            // called; result discarded
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 46 of 49 tests.

- DTY-5: GCC accepts `decltype` in the qualified name of a member
  definition. Clang and EDG reject it ({question}`Q-0065`).
- UNEV-4: Clang warns by default. GCC gives no warning.
- DTY-3: GCC rejects `decltype` after `~` in an explicit destructor call.
  Clang and EDG accept it ({question}`Q-0089`).

Inputs: clang-sema-expr, clang-sema-type, clang-diag-sema, edg-decls.
