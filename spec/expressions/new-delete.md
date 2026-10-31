# New and delete

A `new` expression creates an object with dynamic {term}`storage
duration` and gives a pointer to it. A `delete` expression destroys the
object and frees its storage.

## Syntax

```{eval-rst}
.. productionlist:: expr_new
   new_expr: ["::"] "new" [`placement`] `new_type` [`new_init`]
           : ["::"] "new" [`placement`] `type` "[" [`expression`] "]" {"[" `constant_expression` "]"} [`new_init`]
   placement: "(" `expression_list` ")"
   new_type: `type` | "(" `type` ")"
   new_init: "(" [`expression_list`] ")" | `braced_init`
   delete_expr: ["::"] "delete" `expression`
              : ["::"] "delete" "[" "]" `expression`
```

## Constraints

:::{rule} NEW-1
:kind: required
:tests: expressions/new-delete/array-of-auto.cpp expressions/new-delete/forms.cpp expressions/new-delete/new-auto-no-initializer.cpp expressions/new-delete/new-auto-two-values.cpp expressions/new-delete/const-scalar-with-init.cpp

`new T`, `new T(args)`, and `new T{args}` create one object of type `T`.
With `new auto`, the type comes from exactly one initializer. A program
must not use `auto` as the element type of an array in `new`.
:::

:::{rule} NEW-2
:kind: required
:tests: expressions/new-delete/array-forms.cpp expressions/new-delete/array-needs-size.cpp expressions/new-delete/array-negative-constant-size.cpp expressions/new-delete/array-non-integral-size.cpp expressions/new-delete/array-paren-init.cpp expressions/new-delete/array-runtime-size.cpp expressions/new-delete/array-size-from-initializer.cpp expressions/new-delete/array-too-many-initializers.cpp expressions/new-delete/array-zero.cpp expressions/new-delete/zero-length-non-null.cpp expressions/new-delete/multi-dimension-type.cpp

`new T[n]` creates an array of `n` elements. These rules apply:

- `n` can be known only at run time, and it can be zero. With zero, the
  result is a non-null pointer that differs from other live allocations.
- Each bound after the first must be a positive constant. The result
  points to the first element, which is itself an array for a
  multi-dimensional `new`.
- If `n` is absent, the initializer must give the number of elements.
- A constant `n` must not be negative.
- The initializer must not have more elements than `n`.
- `n` is converted to the type of `sizeof` ({question}`Q-0036`).
:::

:::{rule} NEW-6
:kind: reject
:tests: expressions/new-delete/leak-in-constant-evaluation.cpp

During {term}`constant evaluation`, each object that `new` creates must be
deleted before the evaluation ends.
:::

:::{rule} NEW-7
:kind: reject
:tests: expressions/new-delete/abstract-type.cpp expressions/new-delete/function-type.cpp expressions/new-delete/incomplete-type.cpp expressions/new-delete/new-void.cpp expressions/new-delete/reference-type.cpp

The type in a `new` expression must be a complete object type. It must
not be `void`, an incomplete type, an abstract class *(referenced)*, a
reference, or a function type.
:::

:::{rule} NEW-8
:kind: reject
:tests: expressions/new-delete/delete-non-pointer.cpp expressions/new-delete/delete-void-pointer.cpp expressions/new-delete/delete-incomplete-class.cpp expressions/new-delete/delete-pointer-to-const.cpp

`delete` needs a pointer to an object type. A `void*` operand is not
valid ({question}`Q-0035`). A pointer to an incomplete class and a pointer
to a `const` object are valid.
:::

:::{rule} NEW-10
:kind: required
:tests: expressions/new-delete/placement-arguments.cpp expressions/new-delete/parenthesized-type.cpp

A `new` expression can pass placement arguments in parentheses before
the type, as in `new (a) T`. They become extra arguments of the
allocation function. The type can also be in parentheses, as in
`new (int)`.
:::

:::{rule} NEW-11
:kind: reject
:tests: expressions/new-delete/later-dimension-runtime.cpp expressions/new-delete/const-scalar-needs-init.cpp expressions/new-delete/array-no-default-ctor.cpp

A `new` expression must not:

- have a bound other than the first that is not a constant,
- create a `const` object of scalar type without an initializer,
- create an array of a class that has no default constructor, without
  an initializer for each element.
:::

:::{rule} NEW-13
:kind: required
:tests: expressions/new-delete/class-allocation-function.cpp

A class can declare its own `operator new` and `operator delete`. A `new`
or `delete` for that class uses them. `::new` and `::delete` use the
global functions.
:::

:::{rule} NEW-15
:kind: required
:tests: expressions/new-delete/delete-class-converting-to-pointer.cpp

The operand of `delete` can be a class object that converts to a pointer
to an object type by exactly one conversion function that is not
`explicit`.
:::

## Dynamic Semantics

:::{rule} NEW-4
:kind: required
:tests: expressions/new-delete/delete-null-constant.cpp expressions/new-delete/delete-null.cpp

`delete` and `delete[]` of a null pointer do nothing.
:::

:::{rule} NEW-9
:kind: required
:tests: expressions/new-delete/array-destroy-order.cpp expressions/new-delete/destroys-and-constructs.cpp

A `new` expression allocates storage and then initializes the object.
`delete` destroys the object and then frees the storage. `delete[]`
destroys the elements of an array from the last element to the first.
:::

:::{rule} NEW-12
:kind: required
:tests: expressions/new-delete/allocation-before-initializer.cpp

The allocation function is called before the arguments of the
initializer are evaluated.
:::

## Undefined Behavior

:::{rule} NEW-3
:kind: undefined
:tests: expressions/new-delete/uninitialized-read.cpp expressions/new-delete/uninitialized-write-then-read.cpp

An object that `new T` creates without an initializer has no value. If
the program reads it before it writes it, the behavior is undefined.
:::

:::{rule} NEW-5
:kind: undefined
:tests: expressions/new-delete/delete-mismatch-array.cpp expressions/new-delete/delete-mismatch-scalar.cpp expressions/new-delete/delete-not-from-new.cpp expressions/new-delete/double-delete.cpp expressions/new-delete/delete-derived-array-through-base-constant.cpp

The behavior is undefined if the program:

- uses `delete` for an array from `new[]`, or `delete[]` for an object
  from `new`,
- deletes the same object twice,
- deletes a pointer that `new` did not give,
- uses `delete[]` through a pointer to a base for an array of a derived
  class ({question}`Q-0091`). VIRT-12 gives the rule for one object.
:::

## Implementation Requirements

:::{rule} NEW-14
:kind: unspecified
:tests: expressions/new-delete/allocation-may-be-omitted.cpp

The number of calls to the global allocation functions can be smaller
than the number of `new` expressions. An implementation can skip an
allocation or serve two allocations with one call. A program that
replaces these functions and counts calls must accept a lower count.
:::

## Examples

```cpp
int* a = new int(5);            // 5
int* b = new int();             // 0
int* c = new int;               // no value yet
auto* d = new auto(7);          // int*
int* e = new int[3]{1, 2};      // 1, 2, 0
int* f = new int[]{1, 2, 3};    // three elements

delete a;
delete[] e;
delete[] a;                     // undefined: a is not from new[]
int* n = nullptr;
delete n;                       // does nothing
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 43 of
46 tests.

- NEW-3, NEW-5, NEW-6: all compilers reject these cases during constant
  evaluation and name the cause.
- NEW-2: Clang and EDG convert a floating size such as `1.5` to `1`. Clang
  warns. GCC rejects it ({question}`Q-0036`).
- NEW-5: GCC accepts `delete[]` of a derived array through a base pointer
  during constant evaluation. Clang and EDG reject it ({question}`Q-0091`).
- NEW-8: GCC accepts `delete` of a `void*` with a warning that it is
  undefined. Clang (pedantic) and EDG reject it ({question}`Q-0035`).

Inputs: clang-sema-expr, clang-diag-sema.
