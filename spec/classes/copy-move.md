# Copy and move

A copy constructor and a copy assignment operator make an object equal to
another object of its class. A move constructor and a move assignment
operator do the same, and they can take resources from an object that a
program no longer needs. This page covers how to declare them, when a
class gets them implicitly, when they are deleted, what the implicit ones
do, and when a copy or move does not occur.

Depends on: constructors and destructors, reference binding, overload
resolution *(referenced)*, deleted and defaulted functions.

## Syntax

```{eval-rst}
.. productionlist:: copy
   copy_ctor: `class_name` "(" [`cv`] `class_name` "&" {`default_param`} ")"
   move_ctor: `class_name` "(" [`cv`] `class_name` "&&" {`default_param`} ")"
   copy_assign: `return_type` "operator" "=" "(" `copy_param` ")"
   copy_param: `class_name` | [`cv`] `class_name` "&"
   move_assign: `return_type` "operator" "=" "(" [`cv`] `class_name` "&&" ")"
   default_param: "," `parameter` "=" `expression`
```

The four functions are the *copy and move functions* of a class. A
*user-declared* function is one that the class declares, including with
`= default` or `= delete`.

## Constraints

:::{rule} CPY-1
:kind: required
:tests: classes/copy-move/copy-ctor-const-ref.cpp classes/copy-move/copy-ctor-default-args-suppresses-implicit.cpp classes/copy-move/copy-ctor-default-args.cpp classes/copy-move/copy-ctor-nonconst-ref-const-source.cpp classes/copy-move/copy-ctor-nonconst-ref.cpp classes/copy-move/copy-ctor-volatile.cpp classes/copy-move/template-not-copy-ctor.cpp

A *copy constructor* of class `T` is a constructor that is not a template.
Its first parameter is `T&`, `const T&`, `volatile T&`, or
`const volatile T&`. Each other parameter has a default argument. A
constructor template is never a copy constructor.
:::

:::{rule} CPY-15
:kind: reject
:tests: classes/copy-move/copy-ctor-by-value.cpp

A constructor of class `T` must not have a first parameter of type `T`
if all other parameters have default arguments.
:::

:::{rule} CPY-2
:kind: required
:tests: classes/copy-move/move-ctor-default-args.cpp classes/copy-move/move-ctor.cpp

A *move constructor* of class `T` is a constructor that is not a
template. Its first parameter is `T&&` with any cv-qualifiers. Each other
parameter has a default argument.
:::

:::{rule} CPY-3
:kind: required
:tests: classes/copy-move/copy-assign-forms.cpp

A *copy assignment operator* of class `T` is a non-static member
`operator=` with one parameter of type `T` or a reference to `T` with any
cv-qualifiers. A *move assignment operator* has one parameter of type
`T&&` with any cv-qualifiers. The return type can be any type.
:::

:::{rule} CPY-16
:kind: reject
:tests: classes/copy-move/copy-assign-nonmember.cpp classes/copy-move/copy-assign-static.cpp

A class declares its assignment operators inside its own definition. An
assignment operator declared at namespace scope or as `static` is not
valid.
:::

:::{rule} CPY-4
:kind: required
:tests: classes/copy-move/defaulted-copy-restores.cpp classes/copy-move/implicit-copy.cpp classes/copy-move/move-assign-deletes-copy-ctor.cpp classes/copy-move/move-ctor-deletes-copy-assign.cpp classes/copy-move/move-ctor-deletes-copy.cpp classes/copy-move/user-dtor-keeps-copy.cpp

If a class has no user-declared copy constructor, it has an implicit one.
If it has no user-declared copy assignment operator, it has an implicit
one. Each implicit copy function is deleted if the class has a
user-declared move constructor or move assignment operator. A
user-declared destructor does not remove them.
:::

:::{rule} CPY-5
:kind: required
:tests: classes/copy-move/implicit-move-assign.cpp classes/copy-move/implicit-move.cpp classes/copy-move/no-implicit-move-assign-user-dtor.cpp classes/copy-move/no-implicit-move-user-copy-assign.cpp classes/copy-move/no-implicit-move-user-copy.cpp classes/copy-move/no-implicit-move-user-dtor.cpp

A class has an implicit move constructor and move assignment operator
only if it has none of these user-declared functions:

- a copy constructor or copy assignment operator,
- a move constructor or move assignment operator,
- a destructor.

If a class has no move function, an rvalue uses the copy function.
:::

:::{rule} CPY-6
:kind: required
:tests: classes/copy-move/assign-result-lvalue.cpp classes/copy-move/implicit-copy-assign-nonconst.cpp classes/copy-move/implicit-copy-nonconst-from-member-ok.cpp classes/copy-move/implicit-copy-nonconst-from-member.cpp

The implicit copy constructor takes `const T&` if each base and member
can be copied from a `const` object. If not, it takes `T&`. The same
applies to the implicit copy assignment operator. Each implicit
assignment operator returns `T&`, which refers to `*this`.
:::

:::{rule} CPY-7
:kind: required
:tests: classes/copy-move/const-member-assign.cpp classes/copy-move/const-member-move-assign.cpp classes/copy-move/defaulted-copy-deleted-used.cpp classes/copy-move/deleted-copy-move-only-member.cpp classes/copy-move/deleted-copy-private-member.cpp classes/copy-move/deleted-move-falls-back-to-copy.cpp classes/copy-move/explicitly-deleted-move-not-ignored.cpp classes/copy-move/move-only-member-moves.cpp classes/copy-move/reference-member-assign.cpp classes/copy-move/reference-member-copy-ctor.cpp

An implicit or defaulted copy or move function is deleted if a base or
member cannot be copied or moved in the same way. A function that is
deleted or not accessible prevents this. Also, the implicit copy and move
assignment operators are deleted if the class has a reference member or a
`const` member. A defaulted move function that is deleted takes no part
in overload resolution, so an rvalue uses the copy function. A move
function that the program deletes explicitly takes part.
:::

:::{rule} CPY-14
:kind: required
:tests: classes/copy-move/defaulted-copy-const-mismatch-unused.cpp classes/copy-move/defaulted-copy-const-mismatch-used.cpp classes/copy-move/defaulted-copy-nonconst-param.cpp classes/copy-move/defaulted-move-const-param-used.cpp classes/copy-move/defaulted-move-const-param.cpp classes/copy-move/defaulted-out-of-line-copy.cpp classes/copy-move/defaulted-volatile-param-used.cpp classes/copy-move/defaulted-volatile-param.cpp

A copy or move constructor defaulted in its class can have a parameter
type different from the implicit one. Then it is deleted
({question}`Q-0055`). A defaulted copy constructor can take `T&`. A
function defaulted after its class definition is not deleted by this
rule.
:::

:::{rule} CPY-17
:kind: reject
:tests: classes/copy-move/defaulted-assign-wrong-return-used.cpp classes/copy-move/defaulted-assign-wrong-return.cpp classes/copy-move/defaulted-copy-assign-by-value.cpp classes/copy-move/defaulted-out-of-line-would-delete.cpp

A defaulted copy or move assignment operator must take a reference and
return `T&`. A function defaulted after its class definition must not be
one that would be deleted.
:::

:::{rule} CPY-9
:kind: required
:tests: classes/copy-move/cast-move-only-accepted.cpp classes/copy-move/const-rvalue-picks-copy.cpp classes/copy-move/explicit-copy-ctor-copy-init.cpp classes/copy-move/lvalue-move-only-rejected.cpp classes/copy-move/rvalue-picks-move.cpp

Overload resolution chooses between the copy and move functions:

- An lvalue uses the copy function.
- A non-`const` rvalue uses the move function, if there is one.
- A `const` rvalue uses the copy function.

An lvalue of a class with no usable copy constructor cannot initialize a
new object. An `explicit` copy constructor is not used in
copy-initialization.
:::

:::{rule} CPY-10
:kind: required
:tests: classes/copy-move/prvalue-argument-no-copy.cpp classes/copy-move/prvalue-no-copy-count.cpp classes/copy-move/prvalue-no-copy.cpp

A prvalue of class type initializes the target object directly. No copy
or move constructor is called, and none needs to be accessible or not
deleted. This applies to nested prvalues, return values, and arguments.
:::

:::{rule} CPY-11
:kind: required
:tests: classes/copy-move/named-return-needs-move.cpp classes/copy-move/return-converting-move.cpp classes/copy-move/return-global-not-moved.cpp classes/copy-move/return-local-as-lvalue-ref.cpp classes/copy-move/return-local-move-only.cpp classes/copy-move/return-local-prefers-move.cpp classes/copy-move/return-lvalue-ref-not-moved.cpp classes/copy-move/return-member-not-moved.cpp classes/copy-move/return-param-move-only.cpp classes/copy-move/return-parenthesized-local.cpp classes/copy-move/return-rvalue-ref-move-only.cpp classes/copy-move/return-rvalue-ref-to-ref.cpp

In a `return` statement, a name of one of these is an xvalue:

- a local variable with automatic storage duration,
- a parameter of the function,
- a variable of rvalue reference type that the function declares.

Parentheses around the name do not change this. The return value is then
initialized by a move, or by a converting constructor that takes an
rvalue. A name of a variable of lvalue reference type, a global variable,
or a member is an lvalue.
:::

## Dynamic Semantics

:::{rule} CPY-8
:kind: required
:tests: classes/copy-move/implicit-copy-constexpr.cpp classes/copy-move/memberwise-array.cpp classes/copy-move/memberwise-assign-order.cpp classes/copy-move/memberwise-order.cpp

An implicit copy or move function copies or moves each part of the
object in this order:

1. direct bases, in the order of the base list,
2. non-static data members, in the order of their declarations.

A member of array type is copied element by element. An implicit copy
function can be used in constant evaluation.
:::

## Implementation Requirements

:::{rule} CPY-12
:kind: unspecified
:tests: classes/copy-move/named-return-elision-either.cpp

If a `return` statement names a local variable of the return type, it is
unspecified if the variable is moved to the return value or is the return
value itself.
:::

## Recommended Practice

:::{rule} CPY-13
:kind: recommended
:tests: classes/copy-move/deprecated-copy-warns.cpp classes/copy-move/move-on-return-no-warning.cpp classes/copy-move/pessimizing-move-warns.cpp

An implementation should warn about these:

- a use of an implicit copy function in a class that has a user-provided
  copy constructor or copy assignment operator,
- a cast to an rvalue in a `return` statement that prevents the variable
  from being the return value.
:::

## Examples

```cpp
struct Handle {
  Handle(int fd);
  Handle(Handle&& other);          // no implicit copy functions now
  Handle& operator=(Handle&& other);
};
Handle open_log();
Handle a = open_log();             // no move: the prvalue initializes a
Handle b = a;                      // rejected: copy constructor deleted

Handle reopen() {
  Handle h(3);
  return h;                        // h is moved, or h is the return value
}

struct Point { int x, y; ~Point() {} };
Point p{1, 2};
Point q = static_cast<Point&&>(p); // copies: the destructor removes the move
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 77 of
79 tests.

- CPY-14: GCC 14 rejects a defaulted constructor whose parameter type
  differs from the implicit one. GCC 16, Clang, and EDG accept it and
  delete the function ({question}`Q-0055`).
- CPY-12: Clang and GCC use the variable as the return value at `-O0`
  and `-O2`. EDG runs no programs. The test accepts a move too.
- CPY-13: Clang and GCC give the first warning under `-Wextra` and the
  second under `-Wall`.

Inputs: clang-sema-declcxx, clang-sema-init, clang-diag-sema,
edg-class-decl.
