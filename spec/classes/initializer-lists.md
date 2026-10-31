# List initialization of classes

A braced list can initialize an object of a class that has constructors.
The list selects a constructor. A constructor whose first parameter is
`std::initializer_list<E>` has priority. This page covers how a braced
list selects a constructor, initializer-list constructors, the array
behind a `std::initializer_list`, and narrowing.

List initialization of aggregates and scalars is with initialization.

Depends on: list initialization and aggregates, constructors and
destructors, overload resolution *(referenced)*.

## Principles

:::{principle} ILC-P1
:rules: ILC-1 ILC-2

For a class with constructors, a braced list is a constructor call. The
elements are the arguments, and empty braces call the default
constructor.
:::

:::{principle} ILC-P2
:rules: ILC-3 ILC-4

A class that accepts a `std::initializer_list` means the braces as a
list of values, so that constructor is tried first with the whole list,
and the elements are tried as separate arguments only if no list
constructor fits. Braces also promise that no value is silently cut
down, so a narrowing conversion is an error and never a reason to try
another constructor.
:::

:::{principle} ILC-P3
:rules: ILC-5 ILC-6

A `std::initializer_list` is a view of a temporary array of `const`
elements that the braces create. The array lives as long as a temporary
would, the elements cannot be changed or moved, and a function
parameter of this type takes a braced list directly.
:::

## Syntax

An *initializer-list constructor* is a constructor whose first parameter
is `std::initializer_list<E>`, or a reference to it, and whose other
parameters have default arguments. `std::initializer_list` comes from the
standard library *(referenced)*.

## Constraints

:::{rule} ILC-1
:kind: required
:tests: classes/initializer-lists/designated-non-aggregate.cpp classes/initializer-lists/not-aggregate-with-ctor.cpp classes/initializer-lists/selects-constructor.cpp

A braced list for a class that is not an aggregate selects a constructor.
The elements of the list are the arguments. This applies to direct list
initialization, as in `T x{a, b}`, and to copy list initialization, as in
`T x = {a, b}`. A class with a user-declared constructor is not an
aggregate, so the list cannot initialize its members directly or use
designators.
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
:tests: classes/initializer-lists/constexpr-list-ctor.cpp classes/initializer-lists/copy-from-single-same-class.cpp classes/initializer-lists/list-ctor-copy-list.cpp classes/initializer-lists/list-ctor-explicit-copy-list.cpp classes/initializer-lists/list-ctor-extra-default-params.cpp classes/initializer-lists/list-ctor-not-viable-fallback.cpp classes/initializer-lists/list-ctor-preferred-over-better-match.cpp classes/initializer-lists/list-ctor-preferred.cpp classes/initializer-lists/nested-braces-element.cpp classes/initializer-lists/explicit-best-match-braced.cpp classes/initializer-lists/explicit-not-candidate-equals.cpp

Constructor selection from a braced list has two phases:

1. Only initializer-list constructors are candidates. The list is one
   argument of type `std::initializer_list<E>`.
2. If no initializer-list constructor is viable, all constructors are
   candidates, with each element as an argument.

The first phase wins even if another constructor matches the elements
better. A list with one object of the same class also uses the first
phase ({question}`Q-0081`). Nested braces initialize the elements.

In copy list initialization, `explicit` constructors are candidates too.
If the best match is `explicit`, the program is not valid, and a worse
match is not used. Copy initialization with `=` and one value is
different, because `explicit` constructors are not candidates there.
:::

:::{rule} ILC-4
:kind: reject
:tests: classes/initializer-lists/list-ctor-narrowing-still-chosen.cpp classes/initializer-lists/narrowing-constant-fits.cpp classes/initializer-lists/narrowing-in-ctor-args.cpp classes/initializer-lists/narrowing-in-list-elements.cpp classes/initializer-lists/narrowing-paren-ok.cpp

A narrowing conversion of an element to its parameter or element type is
not valid. If the first phase selects an initializer-list constructor,
the narrowing makes the program not valid. It does not select another
constructor. A constant that fits the target type does not narrow.
Parentheses do not check narrowing.
:::

:::{rule} ILC-6
:kind: required
:tests: classes/initializer-lists/deduce-list-element-type-mismatch.cpp classes/initializer-lists/deduce-list-element-type.cpp classes/initializer-lists/function-param-list.cpp classes/initializer-lists/return-braced-list.cpp classes/initializer-lists/return-deduced-from-list.cpp

A function parameter of type `std::initializer_list<E>` accepts a braced
list, also an empty one. A function template can deduce `E` if all
elements have the same type. A `return` statement with a braced list
initializes a class return type through its constructors. A braced list
cannot deduce an `auto` return type.
:::

## Runtime Behavior

:::{rule} ILC-5
:kind: required
:tests: classes/initializer-lists/backing-array-copy-elements.cpp classes/initializer-lists/backing-array-lives-with-list.cpp classes/initializer-lists/elements-const.cpp classes/initializer-lists/move-only-element.cpp

A braced list for `std::initializer_list<E>` creates an array of `const E`
and copies each element into it. The `std::initializer_list` refers to
that array. The array lives as long as the `std::initializer_list`
object that the list initializes, as for a reference that binds to a
temporary. A program cannot change or move the elements, so an element
type that cannot be copied is not valid.
:::

## Examples

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

The results are in `tests/RESULTS.md`. All seven compilers agree on 29 of 30 tests. The tests declare a minimal `std::initializer_list`.

- ILC-3: Clang 18 copies for `S b{a}` when `S` also has an
  `initializer_list<S>` constructor. Clang 22 and GCC use that
  constructor ({question}`Q-0081`).

Inputs: clang-sema-init, clang-sema-declcxx, clang-diag-sema,
edg-decl-inits, edg-overload.
