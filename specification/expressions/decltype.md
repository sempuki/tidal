# decltype, unevaluated operands, and discarded values

`decltype` gives the type of an expression without evaluating it. The
operand of `decltype` is one of the *unevaluated operands* of the
language, with those of `sizeof` and `noexcept`. A *discarded-value
expression* is evaluated only for its effects, and its value is not
used. This page covers:

- the type that `decltype` gives,
- what an unevaluated operand allows,
- what happens to a discarded value.

Depends on: primary expressions, value categories, unary operators.

## Principles

:::{principle} DTY-P1
:rules: DTY-1 DTY-2 DTY-4

For a plain name, `decltype` reports the declared type of the entity.
For any other expression, including a name in parentheses, `decltype`
reports the type of the expression marked with its {term}`value category`
({principle}`VC-P1`).

| Operand | Result |
|---|---|
| A plain name | The declared type, with the reference and the qualifiers the declaration wrote |
| Any other lvalue | A reference |
| An xvalue | An rvalue reference |
| A prvalue | The bare type |

A plain name gives the declared type, because the programmer meant that
type by the name. An overloaded function name has no single type, so the
name fits neither case.
:::

:::{principle} DTY-P2
:rules: DTY-3
:exceptions: DTY-5

The result of `decltype` is a type, so the result can stand wherever a
type name can stand, including inside a qualified name. The result
cannot stand in the qualified name of a declaration, where the name must
be written out.
:::

:::{principle} UNEV-P1
:rules: UNEV-1 UNEV-4 UNEV-5

The operands of `decltype`, `sizeof`, and `noexcept` exist only to be
asked about, and are never evaluated. So these operands:

- cause no {term}`side effect`,
- need no definition for what they name,
- can describe an operation that would be undefined if it ran.

A member can be named there without an object. The implementation still
has to understand the expression. So these checks apply as anywhere
else:

- lookup,
- access control,
- deleted functions.

A side effect written in such an operand never happens, and an
implementation should say so.
:::

:::{principle} UNEV-P2
:rules: UNEV-2 UNEV-3 UNEV-6

A discarded value is evaluated in full, unlike an unevaluated operand.
The result of the expression is dropped, and only its effects matter.

- A class prvalue becomes a temporary, which ends when its full
  expression ends, as every temporary does ({rule}`VC-6`).
- A glvalue is not copied.
- A volatile object that the expression refers to by a name, a
  subscript, a member access, or a similar form is read, because the
  read is an effect.

A cast to `void` says the discard is intended. An expression with no
effect in that position is probably a mistake, and an implementation
should warn.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   decltype_spec: `decltype_expr`
                : | "decltype" "(" "auto" ")"
   decltype_expr: "decltype" "(" `expression` ")"
```

`decltype(auto)` is a {term}`placeholder type`. The rules for `decltype(auto)`
are with placeholder types. Only a `decltype_expr` can name a base,
start a `nested_name`, follow `~`, or be the type of a functional cast
({rule}`AUTO-11`). A discarded-value expression is one of
these:

- the expression of an expression statement,
- the iteration expression of a `for` statement ({rule}`LOOP-3`),
- the left operand of the built-in `,` operator ({rule}`COMMA-1`),
- the operand of a cast to `void`.

The left operand of an overloaded `,` is an argument of a call
({rule}`COMMA-7`).

## Constraints

:::{rule} DTY-1
:kind: required
:tests: expressions/decltype/bitfield-declared.cpp expressions/decltype/captured-in-lambda.cpp expressions/decltype/enumerator-name.cpp expressions/decltype/function-name.cpp expressions/decltype/member-declared-type.cpp expressions/decltype/name-declared-type.cpp expressions/decltype/parameter-name.cpp expressions/decltype/structured-binding-name.cpp

If the operand of `decltype` is a name without parentheses, the result
is the type that the entity was declared with. This rule applies to:

- variables, parameters, and references, with their {term}`cv-qualifiers <cv-qualifier>` and
  reference,
- member access `a.m` and `p->m`, which gives the declared type of `m`,
- functions, enumerators, bit-fields, and structured bindings.

A name of a variable that a lambda captures gives the type of the
variable, not of the capture.
:::

:::{rule} DTY-4
:kind: reject
:tests: expressions/decltype/overloaded-name.cpp declarations/auto/decltype-auto-overload-set.cpp

The operand of `decltype` must not be the name of an {term}`overload set` that
{rule}`OVL-20` does not reduce to one function.
:::

:::{rule} DTY-2
:kind: required
:tests: expressions/decltype/arithmetic-common-type.cpp expressions/decltype/conditional.cpp expressions/decltype/paren-lvalue.cpp expressions/decltype/prvalue-const-class.cpp expressions/decltype/prvalue.cpp expressions/decltype/string-literal.cpp expressions/decltype/xvalue.cpp

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
:tests: expressions/decltype/as-type-specifier.cpp expressions/decltype/base-specifier.cpp expressions/decltype/decltype-auto-function.cpp expressions/decltype/nested-name.cpp expressions/decltype/trailing-return.cpp expressions/decltype/destructor-call.cpp expressions/decltype/member-initializer-base.cpp expressions/decltype/member-initializer-base-braces.cpp

`decltype` can occur where a type name can occur. These places include:

- a variable or function type,
- a trailing return type,
- a base list,
- the name of a base in a member initializer list,
- the qualified name of a member, such as `decltype(s)::type`,
- the name after `~` in an explicit destructor call.
:::

:::{rule} DTY-5
:kind: reject
:tests: expressions/decltype/declarative-nested-name.cpp expressions/decltype/declarative-nested-name-static-member.cpp

`decltype` must not occur in the qualified name of a declaration, such
as the name of a member function defined outside its class
({question}`Q-0065`).
:::

:::{rule} UNEV-1
:kind: required
:tests: expressions/decltype/abstract-prvalue-decltype.cpp expressions/decltype/constexpr-not-needed.cpp expressions/decltype/incomplete-return-prvalue.cpp expressions/decltype/incomplete-return-sizeof.cpp expressions/decltype/lambda-in-unevaluated.cpp expressions/decltype/member-without-object.cpp expressions/decltype/nested-class-member-evaluated.cpp expressions/decltype/nested-class-member-unevaluated.cpp expressions/decltype/no-definition-needed.cpp expressions/decltype/no-side-effects.cpp expressions/decltype/odr-use-not-counted.cpp expressions/decltype/template-instantiation-not-needed.cpp expressions/decltype/ub-not-evaluated.cpp expressions/decltype/deleted-destructor-decltype.cpp

The operands of `decltype`, `sizeof`, and `noexcept` are not evaluated.
These rules apply to such operands:

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
destructor. This exception also holds for such a call as the right
operand of a comma operator.
:::

:::{rule} UNEV-5
:kind: reject
:tests: expressions/decltype/deleted-function-in-unevaluated.cpp expressions/decltype/private-access-still-checked.cpp expressions/decltype/deleted-destructor-sizeof.cpp expressions/decltype/incomplete-member-access.cpp

The operands of UNEV-1 still have these requirements:

- `sizeof` needs a complete type ({rule}`UN-14`) and a usable
  destructor.
- A member access on a call that gives a prvalue of class type needs a
  complete class.
- Name lookup, access control, and deleted functions are still checked.
:::

:::{rule} UNEV-6
:kind: reject
:tests: expressions/decltype/discarded-volatile-read.cpp expressions/decltype/discarded-volatile-operand-read.cpp expressions/decltype/discarded-volatile-conditional-read.cpp expressions/decltype/discarded-volatile-comma-read.cpp expressions/decltype/discarded-volatile-parenthesized-member-read.cpp

{term}`Constant evaluation <constant evaluation>` rejects a read of a
volatile object in a discarded-value expression (CE-12). The read can
come from a form that UNEV-2 lists, or from an operator that uses the
value of a volatile operand.
:::

## Runtime Behavior

:::{rule} UNEV-2
:kind: required
:tests: expressions/decltype/discarded-call-runs.cpp expressions/decltype/discarded-constant.cpp expressions/decltype/discarded-prvalue-materialized.cpp expressions/decltype/discarded-reference-no-copy.cpp expressions/decltype/void-cast-no-warning.cpp expressions/decltype/void-cast-temporary-lives-to-full-expression-constant.cpp statements/loops/for-iteration-temporary-constant.cpp expressions/decltype/discarded-volatile-conditional-read.cpp expressions/decltype/discarded-volatile-comma-read.cpp expressions/decltype/discarded-volatile-parenthesized-member-read.cpp

A {term}`discarded-value expression` is evaluated, and its value is not
used. A prvalue of class type creates a {term}`temporary object`, which
is destroyed when its {term}`full expression` ends. A glvalue is not
copied. A cast to `void` makes any
expression a discarded-value expression and stops warnings about an
unused result. Constant evaluation follows these rules.

If the expression has one of these forms and refers to a volatile
object, the program reads that object:

- a name,
- a subscript,
- a member access,
- an indirection,
- a pointer-to-member access,
- one of these forms in parentheses,
- a conditional expression whose second and third operands both have
  one of these forms,
- a comma expression whose right operand has one of these forms.
:::

## Recommended Practice

:::{rule} UNEV-3
:kind: recommended
:tests: expressions/decltype/discarded-warns.cpp expressions/decltype/discarded-sum-warns.cpp statements/loops/for-iteration-name-warns.cpp expressions/comma/unused-left-literal-warns.cpp expressions/comma/unused-left-member-warns.cpp expressions/decltype/discarded-effects-no-warning.cpp

If the step that gives the value of a discarded-value expression has no
effect, an implementation should warn. A step has an effect if the step
does one of these:

- calls a function, a constructor or a literal operator included,
- writes an object,
- reads a volatile object,
- allocates storage.

A cast to `void` should prevent the warning.
:::

:::{rule} UNEV-4
:kind: recommended
:tests: expressions/decltype/side-effect-unevaluated-warns.cpp expressions/decltype/side-effect-decltype-warns.cpp

An implementation should warn about an unevaluated operand with a side
effect, such as `sizeof(i++)`, because the effect does not occur.
:::

## Examples

% tidal-example: reject
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

struct Sealed { ~Sealed() = delete; };
Sealed make();
using A = decltype(make());      // valid: no destructor needed
using B = decltype(0, make());   // valid: also after a comma
auto z = sizeof(make());         // rejected: sizeof needs the destructor

struct Outer {
  int m;
  struct Inner {
    int size() { return sizeof(m); }  // valid: unevaluated
    int get() { return m; }           // rejected: no Outer object
  };
};
```

`decltype` of a built-in arithmetic expression names the common type of
the {term}`usual arithmetic conversions` ({rule}`CONV-19`). A
conditional expression whose operands are lvalues of one type is an
lvalue, so `decltype` of it gives a reference:

```cpp
int i = 0;
short h = 1;
unsigned u = 2;
float x = 1.0f;

decltype(h + h)        g;      // int: both operands promote
decltype(u + -1)       k;      // unsigned
decltype(i + x)        m;      // float
decltype(true ? i : i) r = i;  // int&
decltype(true ? i : h) t;      // int: the operands differ, so a prvalue
```

A function that is declared and never defined can name the type of a
call inside `decltype`. The operand is not evaluated, so the function
is not used ({rule}`ODR-3`) and needs no definition, and a function
template is not instantiated ({rule}`INST-7`):

```cpp
template <class T> T&& value_of();          // declared only
template <class F, class T>
using call_result = decltype(value_of<F&>()(value_of<T&>()));

struct Square { long operator()(int& v) const { return v * v; } };
call_result<Square, int> r = 0;             // long
```

Each line below is a discarded-value expression. The step that gives
the value decides the warning of UNEV-3:

```cpp
int next();
volatile int port = 0;
int run(int a) {
  a == 1;                          // warning: the comparison has no effect
  a + next();                      // warning: the addition has no effect
  next();                          // no warning: a call
  port;                            // no warning: a volatile read
  (void)a;                         // no warning: a cast to void
  for (int k = 0; k < 2; a) ++k;   // warning: the iteration expression
  return (a, next());              // warning: the left operand a
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 59
of 65 tests. The UNEV-4 tests turn the warning into an error, so EDG
skips them.

### Divergences

- {question}`Q-0065` (DTY-5; settled on DTY-P2): whether `decltype` can
  name the class in the qualified name of a member definition. It
  cannot, as Clang and EDG say. GCC accepts it, also for a nested class
  and an enumeration definition.
- {question}`Q-0089` (DTY-3; settled on DTY-P2): whether `decltype` can
  name the class after `~` in an explicit destructor call. It can, as
  Clang and EDG say. GCC rejects the call.
- {question}`Q-0450` (DTY-3; settled on DTY-P2): whether a member initializer that names its base with
  `decltype` can take a braced list. It can, as Clang and EDG say. GCC
  rejects the braced form and accepts the form with parentheses.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `declarative-nested-name-static-member.cpp` | DTY-5 | reject | reject | accept | reject | {question}`Q-0065` |
| `declarative-nested-name.cpp` | DTY-5 | reject | reject | accept | reject | {question}`Q-0065` |
| `destructor-call.cpp` | DTY-3 | accept | accept | reject | accept | {question}`Q-0089` |
| `member-initializer-base-braces.cpp` | DTY-3 | run | run | reject | accept | {question}`Q-0450` |
| `side-effect-decltype-warns.cpp` | UNEV-4 | reject | reject | accept | skips | |
| `side-effect-unevaluated-warns.cpp` | UNEV-4 | reject | reject | accept | skips | |

### Notes

- DTY-3, AUTO-11: every compiler rejects `decltype(auto)` as a base,
  as the name of a base in a member initializer list, after `~` in a
  destructor call, and as the type of a functional cast. The gap review
  of the phase 17 text reads the placeholder as no valid form there
  (RV10-1, RV10-4), so these places take a `decltype_expr`.
- UNEV-2, COMMA-1, LOOP-3: in constant evaluation, every compiler ends
  the temporary of a comma's left operand after the right operand runs,
  also when the operand is cast to `void`
  (`void-cast-temporary-lives-to-full-expression-constant.cpp`). Every
  compiler ends a class prvalue of a `for` iteration expression before
  the condition runs again (`for-iteration-temporary-constant.cpp`).
- UNEV-2, UNEV-6: the gap review of the phase 18 text reads the C++23
  text as reading a volatile object in a discarded value only for a
  closed set of forms (RV12-1). UNEV-2 lists them. Every compiler
  rejects the read in constant evaluation for a conditional expression,
  a comma expression, and a pointer-to-member access in parentheses
  (`discarded-volatile-conditional-read.cpp`,
  `discarded-volatile-comma-read.cpp`,
  `discarded-volatile-parenthesized-member-read.cpp`). An operator that
  uses the value of a volatile operand reads the operand in any
  evaluation (`discarded-volatile-operand-read.cpp`).
- UNEV-3, COMMA-5, STMT-4: the round 15 writer grid
  `reports/clean-room/roadmap-15/writer-probes/x26-warnings.json` tries
  30 forms under `-Wunused-value -Werror`, each as an expression
  statement, a left operand of `,`, and an iteration expression. The
  three places give the same result for each form. EDG skips the grid.
  UNEV-3 is a `recommended` rule, so GCC's silence in the second row is
  a result that the class allows.

  | Forms | Clang 18, 22 | GCC 14, 16 |
  |---|---|---|
  | a name, a literal, `a + 1`, `a + f()`, `a == 1`, `sizeof a`, `s.m`, `p[0]`, `*p`, `(long)a`, `c ? a : b`, `(a, f())` | warns | warns |
  | a lambda expression that is not called, `(f(), a)` | warns | no warning |
  | a volatile name, `P(1)`, `P{1}`, `new int`, `5_k`, `++a`, `a++`, `a = 1`, `a += 1`, `f()`, `pf()`, a called lambda, `(void)a`, `c ? f() : f()` | no warning | no warning |

- UNEV-4: Clang warns by default. GCC gives no warning. UNEV-4 is a
  `recommended` rule. Every test of UNEV-4 has an override, and each
  override records a result that the class allows.

### Sources

Inputs: clang-sema-expr, clang-sema-type, clang-diag-sema, edg-decls.
