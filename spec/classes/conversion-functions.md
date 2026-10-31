# Conversion functions

A conversion function is a member function that converts an object of its
class to another type. An implicit conversion can use it, unless it is
`explicit`. This page covers how to declare conversion functions, where
implicit and explicit conversions use them, and how they combine with
converting constructors and standard conversions.

Depends on: constructors and destructors, standard conversions, overload
resolution *(referenced)*.

## Syntax

```{eval-rst}
.. productionlist:: cvf
   conversion_decl: {`cvf_specifier`} "operator" `conversion_type` "(" ")" [`cv`] [`ref_qualifier`]
   cvf_specifier: "constexpr" | "virtual" | "inline" | `explicit_spec`
   conversion_type: `type_specifiers` {"*" | "&" | "&&"}
```

The *target type* of a conversion function is its `conversion_type`. The
name of the function is `operator` followed by the target type.

## Legality Rules

:::{rule} CVF-1
:kind: required
:tests: classes/conversion-functions/basic.cpp classes/conversion-functions/call-by-name.cpp classes/conversion-functions/const-qualified-needed.cpp classes/conversion-functions/constexpr-conversion.cpp classes/conversion-functions/deduced-auto.cpp classes/conversion-functions/out-of-line.cpp classes/conversion-functions/pointer-target.cpp classes/conversion-functions/to-function-pointer.cpp classes/conversion-functions/to-reference.cpp classes/conversion-functions/virtual-conversion.cpp

A conversion function is a non-static member function. Its target type
can be:

- an arithmetic, enumeration, class, or pointer type,
- a reference type, which makes the result an lvalue or xvalue,
- `auto`, deduced from the `return` statements ({question}`Q-0059`).

It can be `const`, `virtual`, or `constexpr`, and it can be defined
outside its class. A call by name, such as `s.operator int()`, is valid.
A conversion function that is not `const` cannot convert a `const`
object.
:::

:::{rule} CVF-7
:kind: reject
:tests: classes/conversion-functions/function-pointer-without-alias.cpp classes/conversion-functions/nonmember.cpp classes/conversion-functions/parameters.cpp classes/conversion-functions/redeclared.cpp classes/conversion-functions/return-type.cpp classes/conversion-functions/static.cpp classes/conversion-functions/to-array.cpp classes/conversion-functions/to-function.cpp classes/conversion-functions/variadic.cpp

A conversion function must not:

- have a return type before `operator`,
- have parameters or `...`,
- be `static` or a non-member,
- convert to an array or a function type,
- be declared two times in its class.

The target type is a type name with `*` and `&` only. A pointer to a
function needs a type alias.
:::

:::{rule} CVF-2
:kind: required
:tests: classes/conversion-functions/array-bound-new.cpp classes/conversion-functions/builtin-operator.cpp classes/conversion-functions/function-argument.cpp classes/conversion-functions/return-statement.cpp classes/conversion-functions/switch-condition.cpp classes/conversion-functions/then-standard-conversion.cpp classes/conversion-functions/two-converting-ctors.cpp classes/conversion-functions/two-user-conversions.cpp

An implicit conversion can call a conversion function that is not
`explicit`. This applies in initialization, function arguments, `return`
statements, operands of built-in operators, `switch` conditions, and
array bounds in `new`. A standard conversion can follow the conversion
function. An implicit conversion uses at most one conversion function or
converting constructor.
:::

:::{rule} CVF-3
:kind: required
:tests: classes/conversion-functions/explicit-argument.cpp classes/conversion-functions/explicit-arithmetic.cpp classes/conversion-functions/explicit-bool-comparison.cpp classes/conversion-functions/explicit-bool-copy-init.cpp classes/conversion-functions/explicit-bool-if.cpp classes/conversion-functions/explicit-bool-noexcept.cpp classes/conversion-functions/explicit-bool-return.cpp classes/conversion-functions/explicit-bool-static-assert.cpp classes/conversion-functions/explicit-brace-init.cpp classes/conversion-functions/explicit-condition-constant.cpp classes/conversion-functions/explicit-copy-init.cpp classes/conversion-functions/explicit-direct-init.cpp classes/conversion-functions/explicit-int-in-if.cpp

An `explicit` conversion function is used only by these:

- direct initialization, including `T(e)`, `T{e}`, and `static_cast`,
- {term}`contextual conversion` to `bool`, if the target type is `bool`.

Contextual conversion occurs in conditions, the operands of `!`, `&&`,
and `||`, the first operand of `?:`, `static_assert`, and `noexcept`.
`explicit(c)` makes the function `explicit` if `c` is `true`.
:::

:::{rule} CVF-4
:kind: required
:tests: classes/conversion-functions/ambiguous-ctor-and-conversion.cpp classes/conversion-functions/ambiguous-two-functions.cpp classes/conversion-functions/best-match-exact.cpp classes/conversion-functions/const-overloads.cpp classes/conversion-functions/conversion-to-base-not-used.cpp classes/conversion-functions/conversion-to-self-not-used.cpp classes/conversion-functions/conversion-to-void-explicit-call.cpp classes/conversion-functions/conversion-to-void.cpp

Overload resolution chooses between conversion functions as it does for
other functions. The object is the implied argument, so cv-qualifiers and
ref-qualifiers select between overloads. If a converting constructor and
a conversion function are equally good, the conversion is ambiguous.
A conversion function to its own class, a base of it, or `void` is never
used by a conversion. A call by name still calls it.
:::

:::{rule} CVF-5
:kind: required
:tests: classes/conversion-functions/alias-same-type-hides.cpp classes/conversion-functions/different-type-not-hidden.cpp classes/conversion-functions/hidden-same-type.cpp classes/conversion-functions/inherited.cpp

Conversion functions of a base are members of the derived class too.
Two conversion functions with the same target type have the same name, so
the one in the derived class hides the one in the base. A type alias for
the target type counts as the same type. Other targets are not hidden.
:::

## Recommended Practice

:::{rule} CVF-6
:kind: recommended
:tests: classes/conversion-functions/conversion-to-base-warns.cpp classes/conversion-functions/conversion-to-self-warns.cpp classes/conversion-functions/conversion-to-void-warns.cpp

An implementation should warn about a conversion function that a
conversion never uses. This is a conversion to its own class, a base, or
`void`.
:::

## Examples

```cpp
struct Path {
  const char* text;
  operator const char*() const { return text; }  // implicit
};
Path p{"/tmp"};
const char* s = p;       // valid
const char* rest = p + 1;  // valid: p converts, then pointer arithmetic

struct File {
  int fd;
  explicit operator bool() const { return fd >= 0; }
};
File f{3};
if (f) {}                // valid: contextual conversion
bool open = f;           // rejected: explicit
bool ok(f);              // valid: direct initialization

struct Celsius { double v; operator double() const { return v; } };
struct Kelvin { Kelvin(double) {} };
void store(Kelvin);
store(Celsius{});        // rejected: two user-defined conversions
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 54 of
55 tests.

- CVF-1: GCC 16 rejects `operator auto()` under `-pedantic-errors`. GCC
  14, Clang, and EDG accept it ({question}`Q-0059`).
- CVF-6: Clang and GCC give these warnings by default.

Inputs: clang-sema-declcxx, clang-sema-init, clang-diag-sema,
edg-class-decl, edg-overload.
