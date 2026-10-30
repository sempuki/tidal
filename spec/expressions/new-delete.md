# New and delete

A `new` expression creates an object with dynamic {term}`storage
duration` and gives a pointer to it. A `delete` expression destroys the
object and frees its storage.

## Syntax

```{eval-rst}
.. productionlist:: expr_new
   new_expr: ["::"] "new" `type` [`new_init`]
           : ["::"] "new" `type` "[" [`expression`] "]" [`new_init`]
   new_init: "(" [`expression_list`] ")" | `braced_init`
   delete_expr: ["::"] "delete" `expression`
              : ["::"] "delete" "[" "]" `expression`
```

## Legality Rules

:::{rule} NEW-1
:kind: required
:tests: expressions/new-delete/array-of-auto.cpp expressions/new-delete/forms.cpp expressions/new-delete/new-auto-no-initializer.cpp expressions/new-delete/new-auto-two-values.cpp

`new T`, `new T(args)`, and `new T{args}` create one object of type `T`.
With `new auto`, the type comes from exactly one initializer. A program
must not use `auto` as the element type of an array in `new`.
:::

:::{rule} NEW-2
:kind: required
:tests: expressions/new-delete/array-forms.cpp expressions/new-delete/array-needs-size.cpp expressions/new-delete/array-negative-constant-size.cpp expressions/new-delete/array-non-integral-size.cpp expressions/new-delete/array-paren-init.cpp expressions/new-delete/array-runtime-size.cpp expressions/new-delete/array-size-from-initializer.cpp expressions/new-delete/array-too-many-initializers.cpp expressions/new-delete/array-zero.cpp

`new T[n]` creates an array of `n` elements. These rules apply:

- `n` can be known only at run time, and it can be zero.
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
:tests: expressions/new-delete/delete-non-pointer.cpp expressions/new-delete/delete-void-pointer.cpp

`delete` needs a pointer to an object whose type is complete. A `void*`
operand is not valid ({question}`Q-0035`).
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

## Undefined Behavior

:::{rule} NEW-3
:kind: undefined
:tests: expressions/new-delete/uninitialized-read.cpp expressions/new-delete/uninitialized-write-then-read.cpp

An object that `new T` creates without an initializer has no value. If
the program reads it before it writes it, the behavior is undefined.
:::

:::{rule} NEW-5
:kind: undefined
:tests: expressions/new-delete/delete-mismatch-array.cpp expressions/new-delete/delete-mismatch-scalar.cpp expressions/new-delete/delete-not-from-new.cpp expressions/new-delete/double-delete.cpp

The behavior is undefined if the program:

- uses `delete` for an array from `new[]`, or `delete[]` for an object
  from `new`,
- deletes the same object twice,
- deletes a pointer that `new` did not give.
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

The results are in `tests/RESULTS.md`.

- NEW-3, NEW-5, NEW-6: all compilers reject these cases during constant
  evaluation and name the cause.
- NEW-2: Clang and EDG convert a floating size such as `1.5` to `1`. Clang
  warns. GCC rejects it ({question}`Q-0036`).
- NEW-8: GCC accepts `delete` of a `void*` with a warning that it is
  undefined. Clang (pedantic) and EDG reject it ({question}`Q-0035`).

Inputs: clang-sema-expr, clang-diag-sema.
