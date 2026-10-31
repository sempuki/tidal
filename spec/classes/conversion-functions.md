# Conversion functions

A conversion function is a member function that converts an object of its
class to another type. An implicit conversion can use it, unless it is
`explicit`. This page covers:

- how to declare conversion functions,
- where implicit and explicit conversions use them,
- how they combine with converting constructors and standard
  conversions.

Depends on: constructors and destructors, standard conversions, overload
resolution *(overload resolution referenced)*.

## Principles

:::{principle} CVF-P1
:rules: CVF-1 CVF-5 CVF-7 CVF-9

A conversion function is a member whose name is a type, and it says how
an object of the class becomes a value of that type. It takes no
parameters and declares no return type, because the name already says
both. A derived class inherits them, and a conversion function with the
same target type hides the base's, as any member with the same name
would.
:::

:::{principle} CVF-P2
:rules: CVF-2 CVF-3 CVF-4 CVF-6 CVF-8 CVF-10 CVF-11 CVF-12

An implicit conversion may call one conversion function, as one of its
user-defined steps, wherever a value of the target type is needed.
`explicit` withdraws the function from implicit use and leaves it for:

- direct initialization,
- casts,
- the conditions where `bool` is asked for.

Overload resolution chooses among conversion functions as among
other functions, and a context that needs one integral value cannot pick
between two. A conversion to the class itself, a base, or `void` can
never be chosen, so an implementation should warn about one.
:::

## Syntax

```{eval-rst}
.. productionlist:: cvf
   conversion_decl: {`cvf_specifier`} "operator" `conversion_type` "(" ")" [`cv`] [`ref_qualifier`]
   cvf_specifier: "constexpr" | "virtual" | "inline" | `explicit_spec`
   conversion_type: `type_specifiers` {"*" | "&" | "&&"}
```

The *target type* of a conversion function is its `conversion_type`. The
name of the function is `operator` followed by the target type.

## Constraints

:::{rule} CVF-1
:kind: required
:tests: classes/conversion-functions/basic.cpp classes/conversion-functions/call-by-name.cpp classes/conversion-functions/constexpr-conversion.cpp classes/conversion-functions/deduced-auto.cpp classes/conversion-functions/explicit-object-conversion.cpp classes/conversion-functions/out-of-line.cpp classes/conversion-functions/pointer-target.cpp classes/conversion-functions/to-function-pointer.cpp classes/conversion-functions/to-reference.cpp classes/conversion-functions/virtual-conversion.cpp

A conversion function is a non-static member function. Its target type
can be:

- an arithmetic, enumeration, class, or pointer type,
- a reference type, which makes the result an lvalue or xvalue,
- `auto`, deduced from the `return` statements ({question}`Q-0059`).

It can:

- be `const`, `virtual`, or `constexpr`,
- have an explicit object parameter,
- be defined outside its class.

A call by name,
such as `s.operator int()`, is valid.
:::

:::{rule} CVF-9
:kind: reject
:tests: classes/conversion-functions/const-qualified-needed.cpp

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
:tests: classes/conversion-functions/array-bound-new.cpp classes/conversion-functions/builtin-operator.cpp classes/conversion-functions/call-through-function-reference.cpp classes/conversion-functions/delete-class-operand.cpp classes/conversion-functions/function-argument.cpp classes/conversion-functions/return-statement.cpp classes/conversion-functions/switch-condition.cpp classes/conversion-functions/then-standard-conversion.cpp

An implicit conversion can call a conversion function that is not
`explicit`. This applies in:

- initialization,
- function arguments,
- `return` statements,
- operands of built-in operators,
- `switch` conditions,
- array bounds in `new`,
- the operand of `delete`.

A call `s(args)` can use a
conversion to a pointer or reference to function. A standard conversion
can follow the conversion function.
:::

:::{rule} CVF-10
:kind: reject
:tests: classes/conversion-functions/two-converting-ctors.cpp classes/conversion-functions/two-user-conversions.cpp

An implicit conversion uses at most one conversion function or
converting constructor.
:::

:::{rule} CVF-3
:kind: required
:tests: classes/conversion-functions/explicit-bool-if.cpp classes/conversion-functions/explicit-bool-noexcept.cpp classes/conversion-functions/explicit-bool-static-assert.cpp classes/conversion-functions/explicit-brace-init.cpp classes/conversion-functions/explicit-condition-constant.cpp classes/conversion-functions/explicit-direct-init.cpp

An `explicit` conversion function can be used by these:

- direct initialization, including `T(e)`, `T{e}`, and `static_cast`,
- {term}`contextual conversion` to `bool`, if the target type is `bool`.

Contextual conversion occurs in:

- conditions,
- the operands of `!`, `&&`, and `||`,
- the first operand of `?:`,
- `static_assert`,
- `noexcept`.

`explicit(c)` makes the function `explicit` if `c` is `true`.
:::

:::{rule} CVF-11
:kind: reject
:tests: classes/conversion-functions/explicit-argument.cpp classes/conversion-functions/explicit-arithmetic.cpp classes/conversion-functions/explicit-bool-comparison.cpp classes/conversion-functions/explicit-bool-copy-init.cpp classes/conversion-functions/explicit-bool-return.cpp classes/conversion-functions/explicit-copy-init.cpp classes/conversion-functions/explicit-int-in-if.cpp

The uses that CVF-3 lists are the only uses of an `explicit` conversion
function. Other conversions must not use it.
:::

:::{rule} CVF-4
:kind: required
:tests: classes/conversion-functions/best-match-exact.cpp classes/conversion-functions/const-overloads.cpp classes/conversion-functions/conversion-to-base-not-used.cpp classes/conversion-functions/conversion-to-self-not-used.cpp classes/conversion-functions/conversion-to-void-explicit-call.cpp classes/conversion-functions/conversion-to-void.cpp

Overload resolution chooses between conversion functions as it does for
other functions. The object is the implied argument, so cv-qualifiers
and ref-qualifiers select between overloads. A conversion function to
its own class, a base of it, or `void` is never used by a conversion. A
call by name still calls it.
:::

:::{rule} CVF-12
:kind: reject
:tests: classes/conversion-functions/ambiguous-ctor-and-conversion.cpp classes/conversion-functions/ambiguous-two-functions.cpp

An ambiguous conversion is not valid. If a converting constructor and a
conversion function are equally good, the conversion is ambiguous.
:::

:::{rule} CVF-5
:kind: required
:tests: classes/conversion-functions/alias-same-type-hides.cpp classes/conversion-functions/different-type-not-hidden.cpp classes/conversion-functions/hidden-same-type.cpp classes/conversion-functions/inherited.cpp

Conversion functions of a base are members of the derived class too.
Two conversion functions with the same target type have the same name, so
the one in the derived class hides the one in the base. A type alias for
the target type counts as the same type. Other targets are not hidden.
:::

:::{rule} CVF-8
:kind: reject
:tests: classes/conversion-functions/new-size-two-integral-conversions.cpp classes/conversion-functions/switch-two-integral-conversions.cpp

A `switch` condition and an array bound in `new` need one conversion to
an integral or enumeration type. A class with two conversion functions
to different integral types makes these uses ambiguous.
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 59 of
60 tests.

- CVF-1: GCC 16 rejects `operator auto()` under `-pedantic-errors`. GCC
  14, Clang, and EDG accept it ({question}`Q-0059`).
- CVF-6: Clang and GCC give these warnings by default.

Inputs: clang-sema-declcxx, clang-sema-init, clang-diag-sema,
edg-class-decl, edg-overload.
