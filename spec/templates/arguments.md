# Template arguments

A {term}`template-id` is the name of a template followed by a list of
{term}`template arguments <template argument>` in angle brackets. The
template-id names one {term}`specialization` of the template. This page
covers:

- when `<` after a name starts a template argument list, and where the
  list ends,
- how the arguments match the parameters,
- what each kind of parameter accepts as an argument,
- when two template-ids name the same specialization.

Depends on: template declarations, name lookup, constant evaluation,
standard conversions.

## Principles

:::{principle} TARG-P1
:rules: TARG-1 TARG-2 TARG-3
:exceptions: TARG-4

Lookup of the name before `<` decides how the `<` reads:

```text
name <            lookup finds a template?
                    yes ──► "<" opens a template argument list
                    no  ──► "<" is the less-than operator
```

In an argument list, the first `>` that is not inside parentheses,
brackets, or braces closes the list. A `>>` token closes two lists. A
program puts a `>` or `>>` operator in parentheses to keep it inside
the list. The same rule makes `Box<Box<int>>` close two lists with no
space between the brackets.
:::

:::{principle} TARG-P2
:rules: TARG-5 TARG-8 TARG-9 TARG-11 TARG-17 TARG-20 TARG-21
:exceptions: TARG-6 TARG-7 TARG-10 TARG-18

The arguments match the parameters by position. Each argument must be
of the kind that its parameter takes:

| Parameter | Argument | Rules |
|---|---|---|
| type parameter | a type | TARG-5, TARG-7 |
| constant parameter | a constant expression that converts to the parameter's type | TARG-12 to TARG-16 |
| template template parameter | the name of a class template or alias template | TARG-17, TARG-18 |
| a pack | the rest of the arguments, zero or more | TARG-21 |

A parameter with no argument takes its default (TARG-9). Outside a call,
a template name needs an argument list, even `<>`, to name a
specialization. Inside a class template, the template's own name alone
names the current specialization (TARG-11).
:::

:::{principle} TARG-P3
:rules: TARG-12 TARG-14 TARG-16
:exceptions: TARG-13 TARG-15

A constant argument must have a value at compile time that two
template-ids can compare. So the argument is a constant expression, and
the conversion to the parameter's type is one that keeps the value
({rule}`CE-19`). A pointer or reference argument names a function or an
object with static storage duration, because only such an entity has
one address for the whole program.

The grid in Evidence gives the result for each kind of argument and
each parameter type.
:::

:::{principle} TARG-P4
:rules: TARG-19 TARG-22

Two template-ids name the same specialization when they name the same
template and their argument lists are the same after defaults are
added. Arguments compare by kind:

| Kind | Two arguments are the same when |
|---|---|
| type | they are the same type, after aliases are replaced |
| integral, enumeration, or `bool` value | the values are equal |
| pointer, reference, pointer to member | they refer to the same entity, or both are null |
| floating-point value | the values have the same representation, so `0.0` and `-0.0` differ |
| class object | each base and member is the same, by these rules |
| template | they name the same template |
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   template_id: `template_name` "<" [`template_args`] ">"
   template_name: `identifier`
   template_args: `template_arg` {"," `template_arg`}
   template_arg: `type_id` | `constant_expr` | [`nested_name`] `template_name`
```

A `template_id` is an `unqualified_id` ({doc}`../expressions/primary`),
and it can name a class in a `simple_type`. A `template_name` is an
`identifier` that lookup finds to be a template (TARG-1). A
`constant_expr` in a `template_arg` does not contain a `>` outside
parentheses, brackets, or braces (TARG-3).

## Constraints

### Reading a template-id

:::{rule} TARG-1
:kind: required
:tests: templates/arguments/less-after-template-name.cpp templates/arguments/less-after-qualified-name.cpp

If lookup of a name finds a template, a `<` right after the name opens
a template argument list. The name and the list form a
{term}`template-id`. The rule applies to qualified and unqualified
names.
:::

:::{rule} TARG-2
:kind: required
:tests: templates/arguments/less-after-variable.cpp

If lookup of the name before `<` finds something other than a template,
the `<` is the less-than operator.
:::

:::{rule} TARG-3
:kind: required
:tests: templates/arguments/closing-angles.cpp

A template argument list ends at the first `>` token that is not inside
parentheses, brackets, or braces. A `>>` token there ends the list and
the list that encloses it.
:::

:::{rule} TARG-4
:kind: reject
:tests: templates/arguments/greater-ends-list.cpp templates/arguments/shift-ends-list.cpp

A `>` or `>>` operator in a template argument must be in parentheses.
Without them, the operator ends the list (TARG-3), and the rest of the
expression does not fit after the template-id.
:::

### Matching arguments to parameters

:::{rule} TARG-5
:kind: required
:tests: templates/arguments/match-in-order.cpp

The first template argument goes to the first parameter, the second to
the second, and so on. The table in {principle}`TARG-P2` gives the kind
of argument that each kind of parameter takes.
:::

:::{rule} TARG-6
:kind: reject
:tests: templates/arguments/too-many.cpp templates/arguments/too-few.cpp templates/arguments/function-explicit-too-many.cpp

A template-id must not have more arguments than the template has
parameters, unless the template has a pack. Each parameter without an
argument must have a default, except in the cases that TARG-20 lists
for function templates.
:::

:::{rule} TARG-7
:kind: reject
:tests: templates/arguments/expression-for-type.cpp templates/arguments/type-for-constant.cpp templates/arguments/class-for-template.cpp templates/arguments/variable-template-for-class-template.cpp templates/arguments/type-or-expression-constant.cpp

An argument must be of the kind that its parameter takes. These
arguments are not valid:

- an expression for a type parameter,
- a type for a constant parameter,
- a class, or a variable template, for a template template parameter
  that takes a class template.
:::

:::{rule} TARG-8
:kind: required
:tests: templates/arguments/type-or-expression.cpp templates/arguments/type-or-expression-constant.cpp

If an argument can be read as a type or as an expression, the argument
is a type. For example, `int()` is the type of a function with no
parameters that returns `int`, also where the parameter is a constant
parameter.
:::

:::{rule} TARG-9
:kind: required
:tests: templates/arguments/defaults-fill.cpp

A parameter with no argument takes its {term}`default template
argument`. A default that names an earlier parameter uses that
parameter's argument. An empty list `<>` takes every default.
:::

:::{rule} TARG-10
:kind: reject
:tests: templates/arguments/name-without-arguments.cpp templates/arguments/variable-template-without-arguments.cpp templates/arguments/empty-list-needs-angles.cpp

Outside its own class, a template name must be followed by an argument
list where the program needs a type or a value. The rule holds also
when every parameter has a default. Two cases are exceptions, and both
belong with deduction *(templates referenced)*:

- the name of a function template, whose arguments a call or a target
  type can supply,
- a class template as the type of a variable with an initializer.
:::

:::{rule} TARG-11
:kind: required
:tests: templates/arguments/injected-name.cpp templates/arguments/injected-name-as-template.cpp

In a class template, the {term}`injected class name` with no argument
list names the current specialization ({rule}`NAM-33`). Where a template
template argument is needed, the injected class name names the
template.
:::

:::{rule} TARG-20
:kind: required
:tests: templates/arguments/function-explicit-arguments.cpp templates/arguments/function-explicit-too-many.cpp

A template-id of a function template can give arguments for the leading
parameters only. Each later parameter gets its argument from deduction
*(templates referenced)* or from its default.
:::

:::{rule} TARG-21
:kind: required
:tests: templates/arguments/pack-takes-rest.cpp

A {term}`template parameter pack` takes all the arguments after the
parameters before it. The pack can take no arguments.
:::

### Constant arguments

:::{rule} TARG-12
:kind: required
:tests: templates/arguments/constant-converted.cpp

The argument for a constant parameter is a constant expression
({principle}`CE-P1`). The argument is converted to the parameter's type. The
conversion can be a promotion, an integral conversion that keeps the
value, or a user-defined conversion that is a constant expression.
:::

:::{rule} TARG-13
:kind: reject
:tests: templates/arguments/constant-not-constant.cpp templates/arguments/constant-narrowing.cpp templates/arguments/constant-floating-to-integer.cpp templates/arguments/constant-integer-to-floating.cpp templates/arguments/constant-int-to-bool.cpp templates/arguments/constant-scoped-enum.cpp

The argument for a constant parameter must be a constant expression. Its
conversion to the parameter's type must not:

- narrow, as `300` to `unsigned char` or `2` to `bool` does,
- convert between an integer type and a floating-point type,
- need an explicit conversion, as a scoped enumeration to `int` does.

The narrowing case is the case of {rule}`CE-19` for template arguments,
and a test cites this rule as the specific one (decision 0042).
:::

:::{rule} TARG-14
:kind: required
:tests: templates/arguments/pointer-arguments.cpp templates/arguments/reference-arguments.cpp templates/arguments/pointer-subobject.cpp

A pointer argument is a null pointer, such as `nullptr`, or the address
of a function or of an object with static storage duration. A reference
argument names such a function or object. The object can be a member or
an array element of a larger object, and the entity can have internal
linkage.
:::

:::{rule} TARG-15
:kind: reject
:tests: templates/arguments/pointer-string-literal.cpp templates/arguments/pointer-local.cpp templates/arguments/reference-temporary.cpp templates/arguments/pointer-zero.cpp

These arguments are not valid for a pointer or reference parameter:

- a string literal, or the address of one,
- the address of an object with automatic storage duration,
- a value that would bind a reference to a temporary object,
- the integer `0`, because an integer does not convert to a pointer
  here.
:::

:::{rule} TARG-16
:kind: required
:tests: templates/arguments/floating-arguments.cpp templates/arguments/class-arguments.cpp

The argument for a floating-point parameter is a constant of a
floating-point type. The argument for a parameter of class type
initializes the parameter's object by {term}`copy-initialization`,
also from a braced list or a value of another type.
:::

### Template template arguments

:::{rule} TARG-17
:kind: required
:tests: templates/arguments/template-template-arguments.cpp templates/arguments/template-template-defaults.cpp templates/arguments/template-template-pack.cpp

A template template parameter takes, as its argument, a class template
or an alias template, named without arguments. The argument is valid
when each
argument list that the parameter's head accepts is also valid for the
argument template. So the argument can have more parameters, if the
extra parameters have defaults, and a parameter with a pack accepts a
template with any number of parameters of that kind.
:::

:::{rule} TARG-18
:kind: reject
:tests: templates/arguments/template-template-mismatch.cpp templates/arguments/template-template-too-many.cpp

A template template argument must accept the argument lists that the
parameter's head allows (TARG-17). A parameter kind that differs, or a
parameter of the argument with no default and no match, is not valid.
:::

### The same specialization

:::{rule} TARG-19
:kind: required
:tests: templates/arguments/same-specialization.cpp templates/arguments/same-specialization-pointers.cpp templates/arguments/same-specialization-floating.cpp templates/arguments/same-specialization-class.cpp

Two template-ids name the same specialization if they name the same
template and, after defaults are added, each pair of arguments is the
same by the table in {principle}`TARG-P4`. Otherwise the
specializations are different entities, and two class specializations
are different types.
:::

## Implementation Requirements

:::{rule} TARG-22
:kind: unspecified
:tests: templates/arguments/same-specialization-alias-template.cpp

An alias template can pass its parameters, unchanged and in order, to a
class template. It is unspecified whether such an alias template and
the class template are the same template argument
({question}`Q-0168`).
:::

## Examples

```cpp
template <int N> struct Bits { };
template <class T> struct Box { };

Bits<(8 > 4)> a;               // the > is in parentheses
Bits<8 > 4> b;                 // rejected: the list ends at the >
Box<Box<int>> c;               // >> closes both lists

template <unsigned char C> struct Byte { };
Byte<'x'> d;                   // char to unsigned char, value kept
Byte<300> e;                   // rejected: narrows
```

Arguments of a pointer type need an object with one address:

```cpp
template <const int* P> struct At { };
int counter;
At<&counter> f;                // static storage duration
At<nullptr> g;                 // a null pointer
void run() {
  int local;
  At<&local> h;                // rejected: automatic storage
}
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 47
of 50 tests.

The clean-room writer probed template-ids, argument kinds, and constant
arguments with Clang 22, GCC 16, and EDG 7.0, and checked the tests on
the eight baseline compilers.

- TARG-22: Clang treats an alias template that passes its parameters
  through as a template of its own. GCC and EDG treat it as the class
  template, so `Use<List>` and `Use<ListAlias>` are one type
  ({question}`Q-0168`).
- TARG-17: Clang 18 rejects a template template argument with more
  parameters than the parameter, even when the extra parameters have
  defaults. Clang 22, GCC, and EDG accept it ({question}`Q-0169`).
- TARG-8: EDG reads `int()` as the value `0` when the parameter is a
  constant parameter. Clang and GCC read the type and reject the
  argument ({question}`Q-0170`).
- TARG-4: all compilers end the list at the first `>`, and report the
  rest of the expression as a syntax error.

The probe grid is in `tests/templates/arguments/grids/`, and
`tools/probe_grid.py` prints the table from it. Each column declares
`template <type P> struct Take`, and each row gives one argument. An
`accept` means that the argument is valid for the parameter. The
compilers agree on every cell:

| | int | unsigned char | bool | long | double | Plain | int* | const int* | int& | const int& | int (*)() | auto |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | accept |
| 300 | accept | reject | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept |
| 'a' | accept | accept | reject | accept | reject | reject | reject | reject | reject | reject | reject | accept |
| true | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | accept |
| 2.0 | reject | reject | reject | reject | accept | reject | reject | reject | reject | reject | reject | accept |
| unscoped enumerator | accept | accept | accept | accept | reject | accept | reject | reject | reject | reject | reject | accept |
| scoped enumerator | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept |
| class with conversion to int | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | accept |
| constexpr int variable | accept | accept | accept | accept | reject | reject | reject | reject | reject | accept | reject | accept |
| int variable | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject |
| nullptr | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject | accept | accept |
| 0 | accept | accept | accept | accept | reject | reject | reject | reject | reject | reject | reject | accept |
| &object | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject | reject | accept |
| object | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept | reject | reject |
| const_object | accept | accept | accept | accept | reject | reject | reject | reject | reject | accept | reject | accept |
| string literal | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject |
| function | reject | reject | reject | reject | reject | reject | reject | reject | reject | reject | accept | accept |

`Plain` is an unscoped enumeration with the enumerator of value 1.
`object` is an `int` variable, and `const_object` is a `const int`
variable initialized with a constant. The `auto` column follows {rule}`TPL-11`:
the argument gives the type, so only an argument that is not a constant
fails.

Inputs: clang-diag-sema, clang-diag-parse.
