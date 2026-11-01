# New and delete

A `new` expression creates an object with dynamic {term}`storage
duration` and gives a pointer to it. A `delete` expression destroys the
object and frees its storage.

## Principles

:::{principle} NEW-P1
:rules: NEW-1 NEW-2 NEW-7 NEW-11 NEW-9 NEW-12 NEW-3 NEW-16 NEW-17 NEW-18

`new` creates an object whose life does not depend on any scope
({principle}`OBJ-P2`). `new` first obtains storage and then initializes
the object in the storage, so the allocation happens before the
initializer's arguments are evaluated. `delete` reverses the two steps:

```text
   new      obtain storage --> evaluate the initializer's arguments
                           --> initialize the object

   delete   destroy the object --> free the storage
```

The object must have a complete object type, because its size and its
constructor are needed, and an array can have a size known only at run
time. An object that `new` leaves without an initializer has no value,
as any object does before its first write ({principle}`OBJ-P4`).
:::

:::{principle} NEW-P2
:rules: NEW-8 NEW-4 NEW-5 NEW-15

A `delete` must match its `new` in these ways:

- The single form matches a single object.
- The array form matches an array.
- Each object is deleted once.
- Only a pointer that `new` gave can be deleted, because the
  implementation keeps its records by that pointer.

A null pointer has no object, so deleting it does nothing. The operand
needs a pointer to an object type, or a class object that converts to
such a pointer, so that the implementation knows which destructor to
run.
:::

:::{principle} NEW-P3
:rules: NEW-10 NEW-13 NEW-14 NEW-19 NEW-20

The storage comes from an allocation function. The global allocation
and deallocation functions exist in every program without a
declaration, so a program can also call them by name. A form with
placement arguments exists only where a declaration gives it. A program can supply its
own allocation functions for one class, through the member
`operator new` and `operator delete` of the class, or with extra
placement arguments. `::new` reaches past a class's own functions to the
global ones. A global allocation function that fails throws an
exception of a type that is *(library referenced)*, and the deallocation
functions never throw. The implementation chooses how often the global functions
are called, because a program only ever sees the objects. So two `new`
expressions may share one allocation.
:::

:::{principle} NEW-P4
:rules: NEW-6

Constant evaluation can use `new`, since constant evaluation runs the
same program ({principle}`TR-P4`). Nothing can outlive the evaluation,
so each object made during the evaluation must be deleted before the
evaluation ends.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   new_expr: ["::"] "new" [`placement`] (`new_type` | "(" `type_id` ")") [`new_init`]
   placement: "(" `expression_list` ")"
   new_type: `type_specifier` {`type_specifier`} {"*" {`cv`}} [`new_bounds`]
   new_bounds: "[" [`expression`] "]" {"[" `constant_expr` "]"}
   new_init: "(" [`expression_list`] ")" | `braced_init`
   delete_expr: ["::"] "delete" ["[" "]"] `cast_expr`
```

## Constraints

:::{rule} NEW-1
:kind: required
:tests: expressions/new-delete/forms.cpp expressions/new-delete/const-scalar-with-init.cpp

`new T`, `new T(args)`, and `new T{args}` create one object of type `T`.
With `new auto`, the type comes from the initializer.
:::

:::{rule} NEW-16
:kind: reject
:tests: expressions/new-delete/array-of-auto.cpp expressions/new-delete/new-auto-no-initializer.cpp expressions/new-delete/new-auto-two-values.cpp

With `new auto` (NEW-1), the type must come from exactly one
initializer. A program must not use `auto` as the element type of an
array in `new`.
:::

:::{rule} NEW-2
:kind: required
:tests: expressions/new-delete/array-forms.cpp expressions/new-delete/array-non-integral-size.cpp expressions/new-delete/array-paren-init.cpp expressions/new-delete/array-runtime-size.cpp expressions/new-delete/array-size-from-initializer.cpp expressions/new-delete/array-zero.cpp expressions/new-delete/zero-length-non-null.cpp expressions/new-delete/multi-dimension-type.cpp

`new T[n]` creates an array of `n` elements. These rules apply:

- `n` can be known only at run time, and it can be zero. With zero, the
  result is a non-null pointer that differs from other live allocations.
- The result points to the first element, which is itself an array for
  a multi-dimensional `new`.
- If `n` is absent, the initializer gives the number of elements.
- `n` is converted to the type of `sizeof` ({question}`Q-0036`).
:::

:::{rule} NEW-17
:kind: reject
:tests: expressions/new-delete/array-needs-size.cpp expressions/new-delete/array-negative-constant-size.cpp expressions/new-delete/array-too-many-initializers.cpp

These conditions apply to `new T[n]` (NEW-2):

- Each bound after the first must be a positive constant.
- If `n` is absent, the initializer must give the number of elements.
- A constant `n` must not be negative.
- The initializer must not have more elements than `n`.
:::

:::{rule} NEW-6
:kind: reject
:tests: expressions/new-delete/leak-in-constant-evaluation.cpp

During {term}`constant evaluation`, each object that `new` creates must
be deleted before the evaluation ends.
:::

:::{rule} NEW-7
:kind: reject
:tests: expressions/new-delete/abstract-type.cpp expressions/new-delete/function-type.cpp expressions/new-delete/incomplete-type.cpp expressions/new-delete/new-void.cpp expressions/new-delete/reference-type.cpp

The type in a `new` expression must be a complete object type. The type
must not be one of these:

- `void`,
- an incomplete type,
- an abstract class ({rule}`VIRT-8`),
- a reference,
- a function type.
:::

:::{rule} NEW-8
:kind: reject
:tests: expressions/new-delete/delete-non-pointer.cpp expressions/new-delete/delete-void-pointer.cpp expressions/new-delete/delete-incomplete-class.cpp expressions/new-delete/delete-pointer-to-const.cpp

`delete` needs a pointer to an object type. A `void*` operand is not
valid ({question}`Q-0035`). A pointer to an incomplete class and a
pointer to a `const` object are valid.
:::

:::{rule} NEW-10
:kind: required
:tests: expressions/new-delete/placement-arguments.cpp expressions/new-delete/parenthesized-type.cpp expressions/new-delete/placement-argument-converts.cpp

A `new` expression can pass placement arguments in parentheses before
the type, as in `new (a) T`. The placement arguments become extra
arguments of the allocation function. The type can also be in
parentheses, as in `new (int)`.
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

A class can declare its own `operator new` and `operator delete`. A
`new` or `delete` for that class uses those functions. `::new` and
`::delete` use the global functions.
:::

:::{rule} NEW-19
:kind: required
:tests: expressions/new-delete/global-functions-by-name.cpp expressions/new-delete/global-functions-with-own-forms.cpp expressions/new-delete/global-functions-specifiers.cpp

Every translation unit has these global functions without a
declaration in the program:

- `operator new` and `operator new[]`, with one parameter of the type
  of `sizeof 0`,
- `operator delete` and `operator delete[]`, with a `void*` parameter,
- `operator delete` and `operator delete[]`, with a `void*` parameter
  and a parameter of the type of `sizeof 0`.

A program can call these functions by name, also when it declares its
own forms of `operator new` or `operator delete`. The `operator new`
forms are potentially throwing, and the `operator delete` forms are
non-throwing ({rule}`EXS-4`).
:::

:::{rule} NEW-20
:kind: reject
:tests: expressions/new-delete/placement-form-undeclared.cpp expressions/new-delete/placement-form-wrong-type.cpp expressions/new-delete/placement-form-wrong-count.cpp expressions/new-delete/placement-argument-converts.cpp

A `new` expression with placement arguments must find a declared
allocation function that accepts them. The function accepts them if
their number fits its parameters after the size, and each placement
argument converts implicitly to its parameter type, as in a function
call ({rule}`PFX-6`). The functions of NEW-19 accept no placement
arguments.
:::

:::{rule} NEW-15
:kind: required
:tests: expressions/new-delete/delete-class-converting-to-pointer.cpp

The operand of `delete` can be a class object that converts to a pointer
to an object type by exactly one conversion function that is not
`explicit`.
:::

## Runtime Behavior

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
:tests: expressions/new-delete/allocation-before-initializer.cpp expressions/sequencing/new-placement-and-bound-order.cpp

The allocation function is called after the placement arguments and
the array bound are evaluated, and before the arguments of the
initializer are evaluated.
:::

## Undefined Behavior

:::{rule} NEW-3
:kind: undefined
:tests: expressions/new-delete/uninitialized-read.cpp expressions/new-delete/uninitialized-write-then-read.cpp

An object that `new T` creates without an initializer has no value. If
the program reads the object before writing the object, the behavior is
undefined.
:::

:::{rule} NEW-5
:kind: undefined
:tests: expressions/new-delete/delete-mismatch-array.cpp expressions/new-delete/delete-mismatch-scalar.cpp expressions/new-delete/delete-not-from-new.cpp expressions/new-delete/double-delete.cpp expressions/new-delete/delete-derived-array-through-base-constant.cpp classes/copy-move/implicit-copy-of-owner-sanitizer.cpp expressions/new-delete/delete-array-mismatch-sanitizer.cpp expressions/new-delete/delete-twice-sanitizer.cpp

The behavior is undefined if the program:

- uses `delete` for an array from `new[]`, or `delete[]` for an object
  from `new`,
- deletes the same object twice,
- deletes a pointer that `new` did not give,
- uses `delete[]` through a pointer to a base for an array of a derived
  class ({question}`Q-0091`). VIRT-12 gives the rule for one object.

A read or write of a dynamic object after `delete` breaks {rule}`OBJ-2`.
:::

## Implementation Requirements

:::{rule} NEW-18
:kind: unspecified
:tests: expressions/sequencing/new-placement-and-bound-order.cpp

The placement arguments and the array bound of a `new` expression are
evaluated in an unspecified order.
:::

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

void* raw = ::operator new(16); // storage only, no object
::operator delete(raw);
int x;
new (&x) int(3);                // rejected without a declared placement form
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 53
of 57 tests.

- NEW-18: Clang 22 and GCC 16 both evaluate the array bound before the
  placement arguments. No compiler in the matrix varies the order, so
  NEW-18 is unspecified by the provisional answer of {question}`Q-0005`.
- NEW-3, NEW-5, NEW-6: all compilers reject these cases during constant
  evaluation and name the cause.
- NEW-2: Clang and EDG convert a floating size such as `1.5` to `1`.
  Clang warns. GCC rejects such a size ({question}`Q-0036`).
- NEW-5: GCC accepts `delete[]` of a derived array through a base
  pointer during constant evaluation. Clang and EDG reject it
  ({question}`Q-0091`).
- NEW-19, NEW-20: Clang, GCC, and EDG accept calls by name to the
  global functions with no declaration in the program, also next to a
  global placement form and a class's own functions. All three reject
  `new (&x) int` when no placement form is declared.
- NEW-20: all compilers reject a placement argument that does not
  convert to the parameter type of the only declared placement form.
  All accept an argument that converts through a constructor.
- NEW-8: GCC accepts `delete` of a `void*` with a warning that the
  deletion is undefined. Clang (pedantic) and EDG reject it
  ({question}`Q-0035`).
- NEW-19: all compilers give `noexcept` the value `false` for a call of
  the global `operator new` forms and `true` for the `operator delete`
  forms.

Inputs: clang-sema-expr, clang-diag-sema.
