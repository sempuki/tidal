# Operator overloading

A class or enumeration can give an operator a meaning for its own values.
An expression that uses the operator then calls an *operator function*.
This page covers:

- how to declare operator functions,
- which operators a program can overload,
- how an expression selects an operator function,
- the increment and member-access operators,
- literal operators.

Defaulted comparison operators have their own page.

Depends on: class definitions and members, conversion functions, copy and
move, sequencing and evaluation order, overload resolution
({doc}`/overloading/overload-resolution`).

## Principles

:::{principle} OPR-P1
:rules: OPR-1 OPR-6 OPR-7 OPR-11 OPR-12 OPR-15

An operator applied to a class or enumeration operand is a function call
in operator syntax. The function is chosen by overload resolution among:

- the member operators of the left operand,
- the non-member operators that lookup finds,
- the built-in operators after conversion.

The syntax does not change. Precedence and associativity stay, and the
operands become ordinary arguments, evaluated in the built-in order only
where the language promises that order.
:::

:::{principle} OPR-P2
:rules: OPR-2 OPR-3 OPR-4 OPR-5 OPR-13 OPR-14

An {term}`operator function` keeps the shape of the operator it stands for. It
has:

- the same number of operands,
- no default arguments,
- no new tokens.

These operators reach into the object, so they must be members:

- assignment,
- call,
- subscript,
- arrow.

The postfix increment takes a dummy `int` to tell the postfix form from
the prefix form, and `operator->` chains until a pointer appears.
:::

:::{principle} OPR-P3
:rules: OPR-8 OPR-9 OPR-10 OPR-16

A literal with a suffix is a call to the literal operator for that
suffix, and the form of the literal picks the parameter list. A
numeric literal can also pass its characters to a template, one
template argument per character. A string literal can pass itself to a
template whose one parameter is a constant of class type. The
literal supplies every argument, so a literal operator has no default
arguments, as for the other operator functions. Literal
operators live at namespace scope, where lookup by suffix finds them.
Suffixes without a leading underscore belong to the language, and
suffixes with two underscores in a row belong to the implementation. A
program that declares a literal operator for such a suffix is not
valid, and no diagnostic is required.

The language uses these suffixes. The `u` part and the `l`, `ll`, or
`z` part each take either case, so `Ul` and `uL` are such suffixes;
the two letters of `ll` keep one case. A suffix that the
language uses keeps its built-in meaning ({principle}`PRIM-P1`), and
the literal calls no literal operator:

| Literal | Suffixes |
|---|---|
| integer | `u`, `l`, `ll`, `z`, and each of `l`, `ll`, and `z` with `u` before or after it |
| floating | `f` and `l` |

The suffixes of the extended floating-point types, such as `f32`, are
outside Tidal's scope. `u8` is not among the suffixes. It is a prefix
of character and string literals, so `1u8` has a suffix that the
language does not use.

No baseline compiler rejects the declaration of a literal operator for
a reserved suffix, also under
`-pedantic-errors`. Every compiler accepts a literal whose suffix has
two underscores in a row. For a suffix that does not start
with `_` and that the language does not use, the compilers warn at the
declaration and differ on a literal with the suffix:

| Compiler | A literal with such a suffix, such as `1km` or `1u8` |
|---|---|
| GCC | calls the literal operator |
| Clang | is rejected as a literal with an invalid suffix |
| EDG | calls the literal operator, unless the suffix starts with a suffix that the language uses, as `u8` starts with `u`. Then EDG reads the rest as extra text and rejects the literal. |
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   operator_function_id: "operator" `overloadable_operator`
   overloadable_operator: "new" | "delete" | "new" "[" "]" | "delete" "[" "]"
                        : | "co_await" | "(" ")" | "[" "]" | "->" | "->*"
                        : | "~" | "!" | "+" | "-" | "*" | "/" | "%" | "^" | "&"
                        : | "|" | "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "^="
                        : | "&=" | "|=" | "==" | "!=" | "<" | ">" | "<=" | ">="
                        : | "<=>" | "&&" | "||" | "<<" | ">>" | "<<=" | ">>="
                        : | "++" | "--" | ","
   literal_operator_id: "operator" '""' `identifier`
```

An *operator function* has an `operator_function_id` as its name. A
*literal operator* has a `literal_operator_id` as its name. This page
does not cover the operator functions for `new` and `delete`
({rule}`NEW-13`), or for `co_await` *(coroutines referenced)*.

## Constraints

:::{rule} OPR-1
:kind: required
:tests: classes/operators/address-of-overload.cpp classes/operators/comma-overload.cpp classes/operators/enum-operator.cpp classes/operators/explicit-call-syntax.cpp classes/operators/member-binary.cpp classes/operators/member-unary.cpp classes/operators/nonmember-binary.cpp classes/operators/nonmember-unary.cpp classes/operators/operator-template-ok.cpp classes/operators/reference-param-nonmember.cpp classes/operators/literal-operator-template-called-by-name.cpp classes/operators/literal-operator-template-called-by-name-value.cpp

An operator function can be a member function or a non-member function.
For a member, the object is the left operand, or the only operand of a
unary operator. A program can call an operator function by its name, for
example `a.operator+(b)` or `operator*(a, b)`. An enumeration can have
non-member operator functions. An operator function can be a template.
:::

:::{rule} OPR-2
:kind: reject
:tests: classes/operators/arrow-nonmember.cpp classes/operators/assign-nonmember.cpp classes/operators/call-default-argument-ok.cpp classes/operators/call-nonmember.cpp classes/operators/default-argument.cpp classes/operators/new-operator-token.cpp classes/operators/nonmember-no-class-param.cpp classes/operators/nonmember-pointer-param.cpp classes/operators/not-overloadable-conditional.cpp classes/operators/not-overloadable-dot-star.cpp classes/operators/not-overloadable-dot.cpp classes/operators/not-overloadable-scope.cpp classes/operators/postfix-wrong-param.cpp classes/operators/static-call-not-other.cpp classes/operators/static-member-operator.cpp classes/operators/subscript-nonmember.cpp classes/operators/unary-or-binary-ok.cpp classes/operators/variadic-operator.cpp classes/operators/wrong-arity-member.cpp classes/operators/wrong-arity-nonmember.cpp

An operator function must obey these limits:

- A non-member must have a parameter of class or enumeration type, or a
  reference to one.
- `.`, `.*`, `::`, and `?:` cannot be overloaded. A program cannot make
  a new operator token.
- A unary operator has one operand and a binary operator has two,
  including the object of a member. `+`, `-`, `*`, and `&` can be either.
- Only `operator()` and `operator[]` can have default arguments or be
  `static`. No operator function can be variadic.
- `=`, `()`, `[]`, and `->` must be member functions.
- The parameter of a postfix `++` or `--` must be `int`.
:::

:::{rule} OPR-11
:kind: required
:tests: classes/operators/associativity-kept.cpp classes/operators/precedence-kept.cpp classes/operators/relational-chain-class.cpp

An overloaded operator keeps the precedence and associativity of the
built-in operator.
:::

:::{rule} OPR-3
:kind: required
:tests: classes/operators/call-operator-overloads.cpp classes/operators/call-operator.cpp classes/operators/static-call-operator.cpp classes/operators/static-subscript.cpp classes/operators/subscript-default-arg.cpp classes/operators/subscript-multi.cpp classes/operators/subscript-operator.cpp classes/operators/subscript-zero-params.cpp classes/operators/subscript-explicit-object-two-args.cpp classes/operators/call-operator-template.cpp classes/operators/static-call-operator-template.cpp

`operator()` and `operator[]` can be `static` member functions. Both can
be overloaded and can have default arguments. `operator[]` can have any
number of parameters, including none, and the call `a[x, y]` passes each
expression as an argument.
:::

:::{rule} OPR-4
:kind: required
:tests: classes/operators/postfix-arg-zero.cpp classes/operators/postfix-nonmember.cpp classes/operators/prefix-postfix.cpp

`operator++()` with no extra parameter is the prefix form.
`operator++(int)` is the postfix form, and the same applies to `--`. A
postfix use passes `0` as the argument.
:::

:::{rule} OPR-13
:kind: reject
:tests: classes/operators/postfix-not-declared.cpp classes/operators/prefix-not-declared.cpp

An expression with `++` or `--` uses only the form that the class
declares.
:::

:::{rule} OPR-5
:kind: required
:tests: classes/operators/arrow-chain.cpp classes/operators/arrow-declaration-ok.cpp

For `x->m`, if `x` has class type, `x.operator->()` is called. If the
result has class type, `->` applies to the result again, until the
result is a pointer. A declaration of `operator->` can have any return
type.
:::

:::{rule} OPR-14
:kind: reject
:tests: classes/operators/arrow-circular.cpp classes/operators/arrow-returns-int.cpp

A use of `->` that OPR-5 describes is not valid if the chain does not
end in a pointer.
:::

:::{rule} OPR-6
:kind: required
:tests: classes/operators/argument-dependent-lookup.cpp classes/operators/assign-not-from-other.cpp classes/operators/builtin-candidate-via-conversion.cpp classes/operators/enum-builtin-still-used.cpp classes/operators/left-operand-conversion-nonmember.cpp classes/operators/member-hides-not-nonmember.cpp foundations/names/operator-lookup-block-hides.cpp foundations/names/operator-lookup-namespace-hides.cpp templates/declarations/member-operator-outside.cpp classes/access/friend-class-then-operator-calls.cpp classes/operators/initializer-list-parameter.cpp expressions/primary/fold-comma-overloaded-class.cpp

An expression with an operand of class or enumeration type selects the
operator function from these candidates:

- member operator functions of the class of the left operand,
- non-member operator functions found by ordinary and argument-dependent
  lookup ({rule}`NAM-5`, {rule}`NAM-27`). As for a call by name, a
  nearer block or namespace that declares the operator function hides
  the outer declarations ({rule}`NAM-38`).
- built-in operators, after conversion of the operands.

Implicit conversions apply to every operand of a non-member and to no
left operand of a member. `a @= b` uses only `operator@=` and is never
rewritten as `a = a @ b`. {rule}`OVL-7` gives the full candidate sets,
and {rule}`OVL-9` lists the {term}`built-in candidates <built-in candidate>`.
:::

:::{rule} OPR-15
:kind: reject
:tests: classes/operators/compound-not-synthesized.cpp classes/operators/left-operand-no-conversion-member.cpp classes/operators/member-and-nonmember-ambiguous.cpp classes/operators/no-candidate.cpp overloading/overload-resolution/enum-operand-no-parameter.cpp overloading/overload-resolution/builtin-enumeration-no-arithmetic.cpp overloading/overload-resolution/builtin-integer-only.cpp overloading/overload-resolution/builtin-bool-increment.cpp overloading/overload-resolution/builtin-member-pointer-order.cpp overloading/overload-resolution/builtin-other-shape-kept.cpp foundations/names/operator-lookup-block-hides.cpp foundations/names/operator-lookup-namespace-hides.cpp classes/comparisons/rewritten-ambiguous-strict.cpp

For an expression that OPR-6 describes, an ambiguity or no viable
candidate makes the expression not valid. The rule is a specific case of
{rule}`OVL-35` and {rule}`OVL-36`.
:::

:::{rule} OPR-8
:kind: required
:tests: classes/operators/literal-block-scope.cpp classes/operators/literal-char.cpp classes/operators/literal-constexpr.cpp classes/operators/literal-float.cpp classes/operators/literal-in-namespace.cpp classes/operators/literal-integer.cpp classes/operators/literal-numeric-over-raw.cpp classes/operators/literal-raw-float.cpp classes/operators/literal-raw.cpp classes/operators/literal-string-concat-suffix.cpp classes/operators/literal-string.cpp classes/operators/literal-builtin-suffix-wins.cpp classes/operators/integer-suffix-mixed-case.cpp classes/operators/literal-raw-long-digits.cpp classes/operators/literal-operator-template-char-pack.cpp classes/operators/literal-operator-template-long-digits.cpp classes/operators/literal-numeric-form-value-too-large.cpp classes/operators/literal-operator-template-numeric-form-wins.cpp classes/operators/literal-operator-template-string-literal.cpp classes/operators/literal-operator-template-class-parameter.cpp classes/operators/literal-operator-template-class-parameter-and-string-form.cpp classes/operators/literal-operator-template-class-parameter-string-form-fallback.cpp classes/operators/literal-operator-template-called-by-name.cpp classes/operators/literal-numeric-form-value.cpp classes/operators/literal-float-form-out-of-range.cpp classes/operators/literal-operator-template-called-by-name-value.cpp

A literal with a suffix calls a literal operator with that suffix. A
literal whose suffix is a form in the syntax of
{doc}`/expressions/primary` keeps its built-in meaning and calls no
literal operator. The form of the literal selects the parameter list:

| Literal | Parameter list |
|---|---|
| An integer literal | `unsigned long long`, or `const char*` for the raw digits, or a template |
| A floating literal | `long double`, or `const char*`, or a template |
| A character literal | The character type |
| A string literal | A pointer to the character type, and the length, or a template |

A *literal operator template* has an empty parameter list and one
template parameter of one of these kinds:

- A pack of `char`. An integer or floating literal calls the template
  with the characters of the literal as the template arguments
  ({question}`Q-0435`). The characters are those of the raw form, with
  a prefix such as `0x`, a point, and digit separators. A call of the
  template by its name ({rule}`OPR-1`) gives the characters as
  explicit template arguments.
- A constant of class type. A string literal calls the template with
  the string literal as the template argument, if the string literal is
  a valid argument for the parameter. Otherwise the string literal calls
  the form with a pointer and a length
  ({question}`Q-0446`).

The numeric form is `unsigned long long` for an integer literal and
`long double` for a floating literal. The numeric form receives the
value that the literal without its suffix has as a literal of that
type. {rule}`PRIM-21` gives the rounding of a floating value, and
{rule}`PRIM-15` a value outside the range of `long double`. If the numeric form of the
literal's kind exists for a suffix, the literal uses it, also when the
`const char*` form or the template exists. Adjacent string
literals with the same suffix, or with a suffix on only one of them, are
joined and then passed to the literal operator. A literal operator can
be `constexpr`.
:::

:::{rule} OPR-16
:kind: reject
:tests: classes/operators/literal-integer-for-float-missing.cpp classes/operators/literal-not-found.cpp classes/operators/literal-operator-template-and-raw-form.cpp classes/operators/literal-operator-template-string-literal.cpp classes/operators/literal-operator-template-and-raw-form-declared.cpp classes/operators/literal-operator-template-class-parameter-not-valid-argument.cpp classes/operators/literal-operator-template-class-parameter-integer.cpp

A literal for which no matching literal operator exists is not valid.
If the `const char*` form and the template both exist for a suffix,
and no numeric form of the literal's kind exists, a numeric or floating
literal with the suffix is not valid. The two declarations alone are
valid.
:::

:::{rule} OPR-10
:kind: reject
:tests: classes/operators/literal-block-scope.cpp classes/operators/literal-bad-params.cpp classes/operators/literal-default-arg.cpp classes/operators/literal-extern-c.cpp classes/operators/literal-friend.cpp classes/operators/literal-member.cpp classes/operators/literal-string-concat-two-suffixes.cpp classes/operators/literal-three-params.cpp classes/operators/literal-operator-template-char-pack.cpp classes/operators/literal-operator-template-long-digits.cpp classes/operators/literal-operator-template-head-not-char-pack.cpp classes/operators/literal-operator-template-class-parameter.cpp classes/operators/literal-operator-template-two-class-parameters.cpp

A literal operator must:

- be a non-member function with C++ linkage,
- have one of the parameter lists that OPR-8 gives, or be a literal
  operator template of one of the two kinds that OPR-8 gives
  ({question}`Q-0435`),
- have no default argument ({question}`Q-0060`).

A literal operator is a function of a namespace. A program can declare
one in a namespace, in a block, or as a friend in a class. A definition
can be in a namespace or in a friend declaration. Adjacent string
literals must not have two different suffixes.
:::

## Runtime Behavior

:::{rule} OPR-7
:kind: required
:tests: classes/operators/arrow-star-order.cpp classes/operators/assign-order-kept.cpp classes/operators/comma-order-kept.cpp classes/operators/compound-assign-order-kept.cpp classes/operators/logical-and-order.cpp classes/operators/no-short-circuit.cpp classes/operators/shift-order-kept.cpp classes/operators/shift-right-order-kept.cpp classes/operators/subscript-order-kept.cpp classes/operators/subscript-multi-order.cpp templates/declarations/member-operator-outside.cpp

An expression that selects an operator function calls the operator
function with the operands as arguments. Both operands of an overloaded
`&&` or `||` are evaluated, the left one first. For these operators, the
operands are evaluated in the order of the built-in operator:

- `<<` and `>>`,
- `=` and compound assignment,
- `[]`,
- `->*`,
- `,`.

In `a[x, y]`, `a` is evaluated first, then the arguments from left to
right. Each argument is evaluated completely before the next one starts.
:::

## Implementation Requirements

:::{rule} OPR-12
:kind: unspecified
:tests: classes/operators/address-of-incomplete-unspecified.cpp classes/operators/address-of-incomplete-const.cpp

If unary `&` applies to an object of a class that is incomplete at that
point, and the complete class overloads `operator&`, it is unspecified
if the built-in operator or the operator function is used.
:::

:::{rule} OPR-9
:kind: no-diagnostic
:tests: classes/operators/literal-no-underscore-warns.cpp classes/operators/literal-no-underscore-floating-warns.cpp classes/operators/literal-double-underscore-suffix.cpp classes/operators/literal-reserved-suffix-used.cpp classes/operators/literal-reserved-suffix-u8.cpp classes/operators/literal-spaced-reserved-identifier.cpp classes/operators/literal-builtin-suffix-wins.cpp

A program must not declare a literal operator whose suffix does not
start with `_`, or whose suffix has two underscores in a row. The first
kind is reserved for later forms of literals, and the second for the
implementation. In the form with a space between `""` and the suffix,
the suffix is an ordinary identifier, so a suffix that starts with `_`
and an uppercase letter is reserved too. Written without the space,
the same suffix is allowed. No diagnostic is required. A literal that uses such a suffix can call
the literal operator or be rejected ({question}`Q-0383`).
:::

## Examples

% tidal-example: reject
```cpp
struct Vec { double x, y; };
Vec operator+(Vec a, Vec b) { return {a.x + b.x, a.y + b.y}; }
Vec operator*(double k, Vec v) { return {k * v.x, k * v.y}; }

Vec p{1, 2}, q{3, 4};
Vec r = p + 2 * q;               // * before +, as for built-in operators
p += q;                          // rejected: no operator+=

struct Matrix {
  double operator[](int row, int col) const;
};

struct Meters { double v; };
constexpr Meters operator""_m(long double v) { return {double(v)}; }
Meters len = 2.5_m;

constexpr long long operator""_cents(long double v) {
  return static_cast<long long>(v * 100 + 0.5L);
}
static_assert(19.99_cents == 1999);    // v is the value of 19.99L

template <char... Cs> constexpr int operator""_digits() { return sizeof...(Cs); }
static_assert(operator""_digits<'4', '2'>() == 42_digits);   // a call by name
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 127
of 135 tests.

### Divergences

- {question}`Q-0435` (OPR-8, OPR-10; open): whether a literal operator
  template whose head is one pack of `char` is valid, and which literals
  call it. It is valid for integer and floating literals, as every
  compiler says. The `const char*` form and the template can both be
  declared for one suffix, as Clang and EDG say, and GCC rejects the
  second declaration.
- {question}`Q-0446` (OPR-8,
  OPR-16; open): which literal operator a string literal calls when a
  template with a class-type parameter exists. The string literal calls
  the template when it is a valid argument for it, as Clang says. GCC and EDG
  call the string form when it exists, and EDG accepts a string literal
  that fits neither.
- {question}`Q-0436` (OPR-8; open): whether a literal whose digits no
  integer type holds can call the raw form or the template. It can, as
  every compiler says.
- {question}`Q-0440` (PRIM-2,
  OPR-8; open): whether a literal whose suffix calls only the
  `unsigned long long` form must have a value that the type holds. It
  must, as Clang and EDG say. GCC accepts the literal with a warning.
- {question}`Q-0063` (PRIM-15, OPR-8; open): whether a floating
  literal outside the range of its type is valid. The value is
  implementation-defined, and an implementation can reject the literal.
  Clang and GCC accept such a literal that calls the `long double`
  form, with a warning, and EDG rejects it. Each result conforms. The
  gap reviews read the literal as not valid, with the warning as the
  diagnostic (`pending.md` row 4).
- {question}`Q-0060` (OPR-10; settled on OPR-P3): whether a literal
  operator can have a default argument. It cannot, as Clang and EDG
  say. GCC accepts the default argument.
- {question}`Q-0383` (OPR-9; settled): whether a literal calls a literal
  operator whose suffix does not start with `_`, such as `km` or `u8`.
  The declaration makes the program not valid with no diagnostic
  required, so each result conforms. GCC calls the operator for both
  suffixes, EDG for `km` only, and Clang rejects every such literal.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `literal-default-arg.cpp` | OPR-10 | reject | reject | accept | reject | {question}`Q-0060` |
| `literal-float-form-out-of-range.cpp` | OPR-8, PRIM-15 | accept | accept | accept | reject | {question}`Q-0063` |
| `literal-numeric-form-value-too-large.cpp` | PRIM-2, OPR-8 | reject | reject | accept | reject | {question}`Q-0440` |
| `literal-operator-template-and-raw-form-declared.cpp` | OPR-16 | accept | accept | reject | accept | {question}`Q-0435` |
| `literal-operator-template-class-parameter-and-string-form.cpp` | OPR-8 | accept | accept | reject | reject | {question}`Q-0446` |
| `literal-operator-template-class-parameter-not-valid-argument.cpp` | OPR-16 | reject | reject | reject | accept | {question}`Q-0446` |
| `literal-reserved-suffix-u8.cpp` | OPR-9 | accept | reject | accept | reject | {question}`Q-0383` |
| `literal-reserved-suffix-used.cpp` | OPR-9 | accept | reject | accept | accept | {question}`Q-0383` |

### Notes

- OPR-9: Clang and GCC warn without extra flags about a suffix that
  does not start with `_`. Clang also warns about a suffix with two
  underscores in a row, and GCC and EDG accept that suffix without a
  warning. Each compiler calls the operator for it.
- OPR-8: every compiler gives a literal with a suffix such as `u`, `z`,
  `ll`, `f`, or `L` its built-in type, even when the program declares a
  literal operator for that suffix. GCC warns that the implementation
  shadows the operator.
- OPR-8, OPR-16: for one suffix, every compiler calls the numeric form
  when a template also exists, and rejects a numeric literal when only
  the `const char*` form and the template exist. GCC rejects the second
  declaration, and Clang and EDG reject the call
  (`literal-operator-template-numeric-form-wins.cpp`,
  `literal-operator-template-and-raw-form.cpp`).
- OPR-8, OPR-10: every compiler accepts a literal operator template
  whose one template parameter is a constant of class type, called by a
  string literal, and rejects an integer literal with only that template
  and a head with two class-type parameters. A string literal that is
  not a valid argument calls the string form on every compiler
  (`literal-operator-template-class-parameter-string-form-fallback.cpp`).
  The gap review of the phase 15 text
  (`reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`,
  RV8-A-3) reads the form as valid.
- OPR-8, OPR-16: the same review (RV8-A-4) reads the numeric form as
  the one of the literal's kind, and places the conflict between the
  `const char*` form and the template on a literal that uses the suffix.
  A floating literal with only the `unsigned long long` form is rejected
  on every compiler (`literal-integer-for-float-missing.cpp`).
- OPR-8: every compiler passes the `long double` form the value of the
  literal without its suffix as a `long double` literal, such as
  `19.99L`, `0.1L`, `0x1p-2L`, and `1'000.5L`, and the
  `unsigned long long` form the value of `18446744073709551615`,
  `0x10`, `0b101`, and `017` (`literal-numeric-form-value.cpp`). The P4
  Port exercise asked for the sentence (P4-7).
- OPR-8, PRIM-15: a floating literal whose value is outside the range
  of `long double` and that calls the `long double` form follows
  PRIM-15. Clang and GCC accept `1e5000_ld` with a warning, and EDG
  rejects it as it rejects `1e5000L`
  (`literal-float-form-out-of-range.cpp`). The raw form and the
  template accept the same literal on every compiler. Under `-Werror`,
  every Clang and GCC rejects `1e5000_ld`
  (`reports/clean-room/roadmap-13/writer-probes/rv9-warnings.json`).
  The gap review of the phase 16 text (RV9-6) reads the call as one
  whose argument is the plain `long double` literal, so the program is
  not valid, and a warning is its diagnostic. OPR-8 and PRIM-15 follow
  the compilers
  (`documentation/decisions/pending.md`, row 4, {question}`Q-0063`).
- OPR-1, OPR-8: every compiler accepts a call of a literal operator
  template by its name, with the characters as explicit template
  arguments or with an empty list, and gives the value of the literal
  with the same characters (`literal-operator-template-called-by-name-value.cpp`).
- OPR-7: for `a[x, y]`, every compiler evaluates `a` first and then the
  arguments from left to right, at run time and in constant evaluation.
  At run time, GCC 14 and GCC 16 evaluate the arguments of the call
  `a.operator[](x, y)` from right to left, and those of `a[x, y]` from
  left to right.

### Sources

Inputs: clang-sema-declcxx, clang-sema-expr, clang-diag-sema,
edg-overload, edg-class-decl.
