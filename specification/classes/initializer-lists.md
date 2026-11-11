# List initialization of classes

A braced list can initialize an object of a class that has constructors.
The list selects a constructor. A constructor whose first parameter is
`std::initializer_list<E>` has priority. This page covers:

- how a braced list selects a constructor,
- initializer-list constructors,
- the array behind a `std::initializer_list`,
- narrowing.

List initialization of aggregates and scalars is with initialization.

Depends on: list initialization and aggregates, constructors and
destructors, overload resolution
({doc}`/overloading/overload-resolution`).

## Principles

:::{principle} ILC-P1
:rules: ILC-1 ILC-2 ILC-7

For a class with constructors, a braced list is a constructor call. The
elements are the arguments, and empty braces call the {term}`default constructor`.
:::

:::{principle} ILC-P2
:rules: ILC-3 ILC-4 ILC-8

A class that accepts a `std::initializer_list` means the braces as a
list of values, so the initializer-list constructor is tried first:

```text
braced list
     |
     v
a list constructor fits the whole list? --yes--> it is called
     | no
     v
the elements are tried as separate arguments
```

Braces also promise that no value is silently cut down, so a narrowing
conversion makes the program not valid and is never a reason to try
another constructor.
:::

:::{principle} ILC-P3
:rules: ILC-5 ILC-6 ILC-9 ILC-10

A `std::initializer_list` is a view of a temporary array of `const`
elements that the braces create.

- The array lives as long as a temporary would.
- The elements cannot be changed or moved.
- A function parameter of this type takes a braced list directly.
:::

## Syntax

An *initializer-list constructor* is a constructor whose first parameter
is `std::initializer_list<E>` or a reference to that type, and whose
other parameters have default arguments. `std::initializer_list` comes
from the standard library ({rule}`LSUP-1`).

## Constraints

:::{rule} ILC-1
:kind: required
:tests: classes/initializer-lists/selects-constructor.cpp classes/initializer-lists/selects-constructor-return-and-argument.cpp declarations/initialization/list/braced-constructor-template.cpp

A braced list for a class that is not an aggregate selects a
constructor. The elements of the list are the arguments. This rule
applies to direct list initialization, as in `T x{a, b}`, and to copy
list initialization, as in `T x = {a, b}`. A class with a user-declared
constructor is not an aggregate.
:::

:::{rule} ILC-7
:kind: reject
:tests: classes/initializer-lists/designated-non-aggregate.cpp classes/initializer-lists/not-aggregate-with-ctor.cpp

A braced list for a class that is not an aggregate cannot initialize the
members of the class directly or use designators.
:::

:::{rule} ILC-2
:kind: required
:tests: classes/initializer-lists/empty-braces-default.cpp classes/initializer-lists/empty-braces-no-default.cpp

An empty braced list `{}` calls the default constructor, even if the
class has an initializer-list constructor. If the class has no default
constructor, an initializer-list constructor receives an empty list.
:::

:::{rule} ILC-3
:kind: required
:tests: classes/initializer-lists/constexpr-list-ctor.cpp classes/initializer-lists/copy-from-single-same-class.cpp classes/initializer-lists/list-ctor-copy-list.cpp classes/initializer-lists/list-ctor-extra-default-params.cpp classes/initializer-lists/list-ctor-not-viable-fallback.cpp classes/initializer-lists/list-ctor-preferred-over-better-match.cpp classes/initializer-lists/list-ctor-preferred.cpp classes/initializer-lists/nested-braces-element.cpp classes/initializer-lists/explicit-not-candidate-equals.cpp classes/initializer-lists/braced-copy-list-ctor-other-element-type.cpp classes/initializer-lists/braced-beside-constructor-template-deduction-fails.cpp classes/initializer-lists/braced-beside-constructor-template-no-conversion.cpp

Constructor selection from a braced list has two phases:

1. Only initializer-list constructors are candidates. The list is one
   argument of type `std::initializer_list<E>`.
2. If no initializer-list constructor is viable, all constructors are
   candidates, with each element as an argument.

The first phase wins even if another constructor matches the elements
better. A list with one object of the same class also uses the first
phase ({question}`Q-0081`). Nested braces initialize the elements.

In copy list initialization, `explicit` constructors are candidates too.
Copy initialization with `=` and one value is different, because
`explicit` constructors are not candidates there. {rule}`ICS-19` gives
the conversion sequence that this choice forms for a parameter.
{rule}`OVL-14` gives the candidates of the other forms of
initialization.
:::

:::{rule} ILC-8
:kind: reject
:tests: classes/initializer-lists/list-ctor-explicit-copy-list.cpp classes/initializer-lists/explicit-best-match-braced.cpp overloading/conversion-sequences/chosen-explicit-copy-list.cpp

In copy list initialization, if the best match is `explicit`, the
program is not valid. A worse match is not used. {rule}`ICS-8` gives
the same fault for a function parameter ({question}`Q-0132`).
:::

:::{rule} ILC-4
:kind: reject
:tests: classes/initializer-lists/list-ctor-narrowing-still-chosen.cpp classes/initializer-lists/narrowing-constant-fits.cpp classes/initializer-lists/narrowing-in-ctor-args.cpp classes/initializer-lists/narrowing-in-list-elements.cpp classes/initializer-lists/narrowing-paren-ok.cpp

A {term}`narrowing conversion` of an element to its parameter or element type is
not valid. In the first phase, an initializer-list constructor stays
viable when an element narrows. If that phase selects the constructor,
the narrowing makes the program not valid. A constant that fits the target type does not
narrow. Parentheses do not check narrowing.
:::

:::{rule} ILC-6
:kind: required
:tests: classes/initializer-lists/deduce-list-element-type.cpp classes/initializer-lists/function-param-list.cpp classes/initializer-lists/return-braced-list.cpp

A function parameter of type `std::initializer_list<E>` accepts a braced
list, also an empty one ({rule}`ICS-16`). A {term}`function template` can deduce `E` if all
elements have the same type. A `return` statement with a braced list
initializes a class return type through its constructors.
:::

:::{rule} ILC-10
:kind: reject
:tests: classes/initializer-lists/deduce-list-element-type-mismatch.cpp classes/initializer-lists/return-deduced-from-list.cpp

A function template cannot deduce `E` from a braced list whose elements
have different types. A braced list cannot deduce an `auto` return type.
:::

:::{rule} ILC-9
:kind: reject
:tests: classes/initializer-lists/elements-const.cpp classes/initializer-lists/move-only-element.cpp

A program cannot change or move the elements of the array that ILC-5
describes, so an element type that cannot be copied is not valid.
:::

## Runtime Behavior

:::{rule} ILC-5
:kind: required
:tests: classes/initializer-lists/backing-array-copy-elements.cpp classes/initializer-lists/backing-array-lives-with-list.cpp

A braced list for `std::initializer_list<E>` creates an array of
`const E` and copies each element into the array. The
`std::initializer_list` refers to that array. The array lives as long as
the `std::initializer_list` object that the list initializes, as for a
reference that binds to a temporary.
:::

## Examples

% tidal-example: reject
```cpp
struct Buffer {
  Buffer(int size, char fill);
  Buffer(std::initializer_list<char> bytes);
};
Buffer a(4, 'x');      // four 'x' characters
Buffer b{4, 'x'};      // two bytes: 4 and 'x'
Buffer c{};            // no default constructor: an empty list of bytes

struct Point { Point(int x, int y); };
Point p{1, 2};         // calls Point(int, int)
Point q{1.5, 2};       // rejected: narrowing
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 34
of 36 tests. The tests declare a minimal `std::initializer_list`.

### Divergences

- {question}`Q-0081` (ILC-3; settled on ILC-P2): whether a braced list
  with one object of the same class uses the initializer-list
  constructor. It does, as Clang 22 and GCC say. Clang 18 copies, and no
  newest family differs.
- {question}`Q-0132` (ICS-8, ILC-8; open): whether a braced argument
  whose best constructor is `explicit` has a conversion sequence. It
  has, and GCC then rejects the call. Clang and EDG form no sequence and
  choose another function.

### Split tests

EDG runs no programs, so its cell for a run test gives the result of
the compile.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `copy-from-single-same-class.cpp` | ILC-3 | run | Clang 22: run; Clang 18: run-fail | run | accept | {question}`Q-0081` |
| `overloading/conversion-sequences/chosen-explicit-copy-list.cpp` | ICS-8, ILC-8 | reject | accept | reject | accept | {question}`Q-0132` |

### Sources

Inputs: clang-sema-init, clang-sema-declcxx, clang-diag-sema,
edg-decl-inits, edg-overload.
