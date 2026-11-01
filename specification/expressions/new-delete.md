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
:rules: NEW-10 NEW-13 NEW-14 NEW-19 NEW-20 NEW-21 NEW-22 NEW-23 NEW-24 NEW-25 NEW-26

The storage comes from an allocation function. The global allocation and
deallocation functions exist in every program without a declaration, so
a program can also call them by name. A form with placement arguments
exists only where a declaration gives it. A program can supply its own
allocation functions for one class, through the member `operator new`
and `operator delete` of the class, or with extra placement arguments.
`::new` reaches past a class's own functions to the global ones. The
deallocation functions never throw. The implementation chooses how often
the global functions are called, because a program only ever sees the
objects. So two `new` expressions may share one allocation.

If the initialization throws, the storage goes back through the
deallocation function that pairs with the allocation function. The
extra parameters make the pair, so a placement form needs an
`operator delete` with the same extra parameters. Only that function
knows how to free storage that the placement form obtained. Without the
function, the storage stays allocated.

| Allocation | Deallocation function after a throw | Rules |
|---|---|---|
| no placement arguments | a usual form, with a `void*` parameter and an optional size | NEW-22 |
| placement arguments | the form with the same parameters after the first | NEW-23, {rule}`DED-39` |
| either, when no single function matches | none, and the storage is not freed | NEW-24 |
:::

:::{principle} NEW-P4
:rules: NEW-6 NEW-37 NEW-39

Constant evaluation can use `new`, since constant evaluation runs the
same program ({principle}`TR-P4`). Nothing can outlive the evaluation,
so each object made during the evaluation must be deleted before the
evaluation ends. The evaluation manages the storage itself, so it can
use only the global allocation functions of the implementation. A
function that the program writes, or a placement form, has effects that
the evaluation cannot model.
:::

:::{principle} NEW-P5
:rules: NEW-27 NEW-28 NEW-29 NEW-30

Every global allocation function without an alignment parameter gives
storage aligned to the {term}`default new alignment`. A type that needs
more alignment gets it from an aligned form: the `new` expression
passes the alignment to the allocation function, and `delete` passes
it to the deallocation function. A class that declares only the forms
without alignment still works, because the expression drops the
alignment when no function accepts it. The expression never adds an
alignment that the first call did not pass.

| Alignment of the type | First call | If no function accepts the first call |
|---|---|---|
| at most the default new alignment | `operator new(size, placement...)` | none; the program is rejected |
| more than the default new alignment | `operator new(size, std::align_val_t(alignof(T)), placement...)` | `operator new(size, placement...)` |
:::

:::{principle} NEW-P6
:rules: NEW-31 NEW-35 NEW-36 NEW-38
:exceptions: NEW-32

An allocation function reports failure in the way that its exception
specification gives. A potentially throwing function throws, so the
`new` expression trusts a result that it gets. A non-throwing function
returns a null pointer, so the expression tests the result and stops
at null. An erroneous run-time array bound (NEW-35) follows the same
split, and no allocation happens.

| Allocation function | It fails | The array bound is erroneous |
|---|---|---|
| potentially throwing | throws `std::bad_alloc` | the expression throws `std::bad_array_new_length` |
| non-throwing | returns null, and the expression gives null | the expression gives null |

A potentially throwing function that returns null breaks the model,
and the behavior is undefined.
:::

:::{principle} NEW-P7
:rules: NEW-33 NEW-34

An array `new` can ask the allocation function for more storage than
the elements need. The implementation keeps its own data in the extra
storage, such as the number of elements that `delete[]` destroys. The
{term}`non-allocating placement form` builds the array at the address
that it receives. So an array `new` that uses the form asks for no
extra storage, and the array starts at that address.
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
:tests: expressions/new-delete/array-needs-size.cpp expressions/new-delete/array-negative-constant-size.cpp expressions/new-delete/array-too-many-initializers.cpp expressions/new-delete/array-constant-size-too-large.cpp

These conditions apply to `new T[n]` (NEW-2):

- Each bound after the first must be a positive constant.
- If `n` is absent, the initializer must give the number of elements.
- A constant `n` must not be negative.
- A constant `n` must not make the size of the array too large for the
  type of `sizeof`.
- The initializer must not have more elements than `n`.

NEW-35 gives the result of such an `n` known only at run time.
:::

:::{rule} NEW-6
:kind: reject
:tests: expressions/new-delete/leak-in-constant-evaluation.cpp

During {term}`constant evaluation`, each object that `new` creates must
be deleted before the evaluation ends.
:::

:::{rule} NEW-37
:kind: reject
:tests: expressions/new-delete/constant-evaluation-class-allocation.cpp expressions/new-delete/constant-evaluation-placement.cpp expressions/new-delete/constant-evaluation-nothrow.cpp expressions/new-delete/aligned-new-constant.cpp

During constant evaluation, a `new` expression must call a global
allocation function of the implementation. These functions are the
forms of NEW-19 and NEW-26, and the nothrow forms ({rule}`LSUP-6`,
{question}`Q-0303`). A class's own `operator new` and a placement form
that the program defines are not valid there.
:::

:::{rule} NEW-39
:kind: reject
:tests: expressions/new-delete/bad-bound-constant-negative.cpp expressions/new-delete/bad-bound-constant-too-few.cpp

During constant evaluation, the first bound of `new T[n]` must not be
erroneous (NEW-35). The throw that an erroneous bound causes is not
valid there ({rule}`CE-8`).
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

:::{rule} NEW-26
:kind: required
:tests: expressions/new-delete/aligned-forms-by-name.cpp expressions/new-delete/aligned-forms-specifiers.cpp foundations/language-support/alignment-enumeration-with-enumerator.cpp

Every translation unit also has the aligned forms of the functions of
NEW-19. Each aligned form adds a last parameter of type
`std::align_val_t` ({rule}`LSUP-5`). An aligned `operator new` gives
storage aligned to the value of that argument.
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

:::{rule} NEW-30
:kind: reject
:tests: expressions/new-delete/aligned-form-only-for-ordinary-type.cpp

If the alignment of the type is at most the {term}`default new
alignment`, a `new` expression must find an allocation function that
accepts the size and the placement arguments. An aligned form is not
tried in its place ({question}`Q-0300`).
:::

:::{rule} NEW-25
:kind: reject
:tests: expressions/new-delete/deallocation-private.cpp expressions/new-delete/deallocation-private-no-throw.cpp expressions/new-delete/deallocation-deleted.cpp expressions/new-delete/deallocation-deleted-no-throw.cpp expressions/new-delete/deallocation-placement-deleted.cpp expressions/new-delete/placement-matches-usual-form.cpp

The deallocation function that NEW-21 to NEW-23 choose must be
accessible and must not be deleted ({question}`Q-0221`). The
requirement holds also when the initialization cannot throw. With
placement arguments, the chosen function must not be a usual form
(NEW-22).
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
:tests: expressions/new-delete/allocation-before-initializer.cpp expressions/sequencing/new-placement-and-bound-order.cpp expressions/new-delete/throw-in-initializer-argument.cpp

The allocation function is called after the placement arguments and
the array bound are evaluated, and before the arguments of the
initializer are evaluated.
:::

:::{rule} NEW-21
:kind: required
:tests: expressions/new-delete/throw-in-initializer-argument.cpp expressions/new-delete/throw-class-both-usual-forms.cpp expressions/new-delete/throw-class-sized-form-only.cpp expressions/new-delete/throw-class-delete-hides-global.cpp expressions/new-delete/throw-global-scope-new.cpp expressions/new-delete/throw-placement-in-base-class.cpp expressions/new-delete/throw-placement-global.cpp expressions/new-delete/throw-placement-class-new-global-delete.cpp expressions/new-delete/throw-placement-array.cpp expressions/new-delete/aligned-delete-passes-alignment.cpp

If the initialization in a `new` expression throws, a deallocation
function frees the storage before the exception leaves the expression
({rule}`UNW-3`). The initialization includes the evaluation of the
initializer's arguments. The function is an `operator delete`, or an
`operator delete[]` for an array, that this search finds:

- If the type is a class and the expression does not start with `::`,
  the search looks in the class and its bases.
- If that search finds no function with the name, the search uses the
  global functions.
:::

:::{rule} NEW-22
:kind: required
:tests: expressions/new-delete/throw-class-both-usual-forms.cpp expressions/new-delete/throw-class-sized-form-only.cpp expressions/new-delete/throw-global-scope-new.cpp expressions/new-delete/aligned-global-delete.cpp

Without placement arguments, the deallocation function is a usual form.
A usual form has a `void*` parameter, and can have a second parameter
of the type of `sizeof 0`. The forms that the search finds decide the
call:

- If a class declares both usual forms, the form with one parameter is
  called.
- If a class declares one usual form, that form is called.
- For the global functions, either form can be called ({rule}`UNW-3`).
:::

:::{rule} NEW-23
:kind: required
:tests: expressions/new-delete/throw-placement-in-base-class.cpp expressions/new-delete/throw-placement-global.cpp expressions/new-delete/throw-placement-class-new-global-delete.cpp expressions/new-delete/throw-placement-array.cpp expressions/new-delete/throw-placement-two-parameters.cpp expressions/new-delete/throw-placement-default-argument.cpp expressions/new-delete/throw-placement-function-and-template.cpp expressions/new-delete/throw-placement-function-before-template-instantiation.cpp

With placement arguments, the deallocation function is the one that
matches the allocation function. The two match if their parameters after the first
have the same types in the same order. These conditions also apply:

- A parameter with a default argument counts, also when the `new`
  expression gives no argument for the parameter.
- A function template matches as {rule}`DED-39` says.
- If a function and a function template both match, the function is
  called ({question}`Q-0220`).
:::

:::{rule} NEW-24
:kind: required
:tests: expressions/new-delete/throw-placement-converting-type-no-match.cpp expressions/new-delete/throw-placement-reference-type-no-match.cpp expressions/new-delete/throw-placement-ellipsis-no-match.cpp expressions/new-delete/throw-class-delete-hides-global.cpp expressions/new-delete/throw-placement-templates-unordered.cpp

The implementation calls no deallocation function, and the storage is
not freed, in these cases:

- No function that the search finds matches. A function with an
  ellipsis does not match, and a usual form or a global function is
  not called in place of the match.
- Two or more functions match, and neither NEW-23 nor {rule}`ORD-9`
  chooses one ({question}`Q-0222`).
:::

:::{rule} NEW-28
:kind: required
:tests: expressions/new-delete/aligned-new-passes-alignment.cpp expressions/new-delete/aligned-delete-passes-alignment.cpp expressions/new-delete/aligned-global-delete.cpp expressions/new-delete/aligned-new-constant.cpp foundations/language-support/alignment-enumeration-self-declared.cpp foundations/language-support/alignment-enumeration-not-declared.cpp

If the alignment of `T` is more than the {term}`default new
alignment`, a `new` expression for `T` or an array of `T` passes
`alignof(T)` as an `std::align_val_t` argument. The argument comes
after the size and before the placement arguments. The deallocation
function gets the alignment too, after `delete` and after a throw
(NEW-21).
:::

:::{rule} NEW-29
:kind: required
:tests: expressions/new-delete/aligned-fallback-without-alignment.cpp

If no allocation function accepts the arguments of NEW-28, the `new`
expression tries again without the alignment argument. A `delete`
expression for the object finds its deallocation function in the same
way.
:::

:::{rule} NEW-31
:kind: required
:tests: expressions/new-delete/null-result-nothrow-tag.cpp expressions/new-delete/null-result-non-throwing-placement.cpp foundations/language-support/nothrow-tag-self-declared.cpp

If the allocation function is non-throwing ({rule}`EXS-4`) and returns
a null pointer, the `new` expression gives a null pointer. The
expression then evaluates no argument of the initializer, initializes
nothing, and calls no deallocation function.
:::

:::{rule} NEW-33
:kind: required
:tests: expressions/new-delete/placement-array-no-overhead.cpp foundations/language-support/placement-forms-self-declared.cpp

A `new` expression that uses a {term}`non-allocating placement form`
({rule}`LSUP-7`) creates the object at the address of the placement
argument. For an array, the result is that address. The expression asks
for no extra storage (NEW-34).
:::

:::{rule} NEW-35
:kind: required
:tests: expressions/new-delete/bad-bound-negative.cpp expressions/new-delete/bad-bound-too-large.cpp expressions/new-delete/bad-bound-too-few.cpp foundations/language-support/exception-classes-array-length.cpp

The first bound `n` of `new T[n]` is *erroneous* in these cases:

- `n` is negative.
- The size of the array is too large for the type of `sizeof`.
- The initializer has more elements than `n`.

The expression then calls no allocation function and throws
`std::bad_array_new_length` ({rule}`LSUP-8`, {question}`Q-0301`).
:::

:::{rule} NEW-36
:kind: required
:tests: expressions/new-delete/bad-bound-nothrow.cpp

If the bound is erroneous (NEW-35) and the allocation function is
non-throwing, the `new` expression gives a null pointer and throws
nothing ({question}`Q-0302`).
:::

:::{rule} NEW-38
:kind: required
:tests: foundations/language-support/exception-classes-allocation.cpp

A global allocation function of NEW-19 or NEW-26 that cannot allocate
the storage throws `std::bad_alloc` ({rule}`LSUP-8`). The function never
returns a null pointer.
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

:::{rule} NEW-32
:kind: undefined
:tests: expressions/new-delete/null-result-throwing-form-sanitizer.cpp

If an allocation function that is not non-throwing returns a null
pointer, the behavior is undefined.
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

:::{rule} NEW-27
:kind: impl-defined
:tests: expressions/new-delete/default-new-alignment.cpp expressions/new-delete/aligned-new-passes-alignment.cpp

The {term}`default new alignment` is implementation-defined. The
predefined macro `__STDCPP_DEFAULT_NEW_ALIGNMENT__` gives its value
*(preprocessor referenced)*. The matrix targets choose these values:

| Target | Default new alignment |
|---|---|
| x86-64, Clang, GCC, and EDG | 16 |
| EDG with 32-bit `int`, `long`, and pointers | 16 |
:::

:::{rule} NEW-34
:kind: unspecified
:tests: expressions/new-delete/array-overhead-other-forms.cpp

An array `new` that does not use a non-allocating placement form can ask
for more storage than its elements need. The amount is unspecified and
can differ between two expressions. The array then starts at an
unspecified place in the storage.
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

```cpp
struct alignas(64) Block { int v; };
Block* b = new Block;           // operator new(sizeof(Block), std::align_val_t(64))
delete b;                       // operator delete(b, std::align_val_t(64)), or the sized form

long count = -1;
int* g = new int[count];        // throws std::bad_array_new_length
int* h = new (std::nothrow) int[count];  // null
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 95
of 111 tests.

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
- NEW-21 to NEW-24: Clang, GCC, and EDG agree on the search in the
  class and its bases, on the choice between the usual forms of a class,
  and on the placement forms with no match. EDG runs no programs, so
  its results say only that these programs are valid.
- NEW-23: GCC calls a function in preference to a function template
  when both match. EDG chooses the same function. A probe with a failing
  `static_assert` in the template shows that EDG does not instantiate
  the template. Clang warns that two functions match and calls neither
  ({question}`Q-0220`).
- NEW-24: Clang and GCC call no deallocation function when two
  templates match and partial ordering prefers neither. Clang warns.
  EDG rejects the `new` expression as ambiguous ({question}`Q-0222`).
- NEW-26, NEW-28: Clang, GCC, and EDG declare the aligned forms with
  no declaration in the program, once the program declares
  `std::align_val_t`. Clang 18 does not declare the sized forms, as
  for NEW-19 ({question}`Q-0125`). Clang and GCC pass `alignof(T)` to a
  program's aligned `operator new` and `operator delete`, for one
  object, an array, a placement form, and a deallocation after a throw.
  A type whose alignment equals the default new alignment gets no
  alignment argument.
- NEW-27: the targets in the matrix give 16. The host Clang with
  `-m32` gives 8, and the host GCC with `-m32` gives 16. Neither is a
  matrix target, so the table leaves them out. The value is the
  implementation's choice, so the rule is `impl-defined`.
- NEW-29, NEW-30: every compiler drops the alignment when a class
  declares only the forms without alignment. Every compiler rejects
  the reverse case, a class of ordinary alignment whose only
  `operator new` takes an alignment ({question}`Q-0300`).
- NEW-31: Clang and GCC test the result of a `noexcept` allocation
  function, with the nothrow tag or with another placement argument.
  On null they evaluate no initializer argument and run no
  constructor.
- NEW-32: the undefined behavior sanitizer of Clang and GCC reports a
  constructor call on a null pointer when a potentially throwing
  placement form returns null.
- NEW-33, NEW-34: Clang and GCC place an array at the address that the
  non-allocating placement form receives, also for a class with a
  destructor. With another placement form, both ask for extra storage
  for an array of a class with a destructor and put the array after
  it, and ask for none for an array of `int`. The amount is the
  implementation's choice and need not be documented, so NEW-34 is
  `unspecified`.
- NEW-35: GCC throws `std::bad_array_new_length` for a negative bound
  and for a size too large, without a call of the allocation function.
  For a bound smaller than the initializer, GCC calls the allocation
  function, frees the storage, and then throws. Clang passes the
  allocation function a size that it cannot meet, so the function
  throws `std::bad_alloc` ({question}`Q-0301`).
- NEW-39: constant evaluation rejects each erroneous bound on every
  compiler. Clang and EDG name the bound in a note, and GCC names the
  call that would throw.
- NEW-36: Clang gives a null pointer for an erroneous bound with the
  nothrow form. GCC throws `std::bad_array_new_length`
  ({question}`Q-0302`).
- NEW-37: every compiler rejects a class's own `operator new` and a
  placement form during constant evaluation, and accepts an
  over-aligned type. Clang 18 and 22 and GCC 16 accept the nothrow
  forms. GCC 14 and EDG reject them ({question}`Q-0303`).
- NEW-38: Clang and GCC throw `std::bad_alloc` from the global
  allocation function for a request that no system can meet.
- NEW-25: Clang, GCC, and EDG reject a private deallocation function,
  also when the initialization cannot throw. Clang and EDG reject a
  deleted one in the same cases. GCC accepts the deleted function
  ({question}`Q-0221`). All three reject a placement form whose match
  is a usual form.

Inputs: clang-sema-expr, clang-diag-sema.
