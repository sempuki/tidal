# Copy and move

A copy constructor and a copy assignment operator make an object equal to
another object of its class. A move constructor and a move assignment
operator do the same and can take resources from an object that a
program no longer needs. This page covers:

- how to declare the copy and move functions,
- when a class gets them implicitly,
- when they are deleted,
- what the implicit ones do,
- when a copy or move does not occur.

Depends on: constructors and destructors, reference binding, overload
resolution *(overload resolution referenced)*, deleted and defaulted
functions.

## Principles

:::{principle} CPY-P1
:rules: CPY-1 CPY-2 CPY-3 CPY-15 CPY-16 CPY-8 CPY-20

A class object is copied or moved part by part: the bases in the order
of the base list, then the members in the order of their declarations,
and an array member element by element. The copy and move constructors
and the copy and move assignment operators do this copying. The language
recognizes these functions by their signatures:

| Function | Parameter |
|---|---|
| Copy constructor or copy assignment operator | A reference to the class |
| Move constructor or move assignment operator | An rvalue reference |

The assignment operators are members of the class.

A constructor that takes the class by value cannot exist, because such
a constructor would need itself to copy its argument.
:::

:::{principle} CPY-P2
:rules: CPY-4 CPY-5 CPY-6 CPY-21 CPY-22

The implementation writes these four functions for a class unless the
class writes them, because part-by-part copying is right for most
classes. The parameter types of the implicit functions follow from what
the parts permit. The implicit copy constructor takes a `const`
reference if every part can be copied from a `const` object.

A class that declares one of these functions says that part-by-part
copying is wrong for the class. So one declaration removes the others:

| User-declared function | Removes |
|---|---|
| A move function | The implicit copy functions |
| A copy function or a destructor | The implicit move functions. Rvalues of that class fall back to copying. |

The implicit assignment operator of a derived class hides those of its
bases for the same reason. A derived class decides for its own parts.

A user-declared destructor removes only the move functions and keeps the
implicit copy functions. So a class that releases a resource in its
destructor must write or delete its copy functions. Otherwise a copy
shares the resource, and both objects release it (NEW-5).
:::

:::{principle} CPY-P3
:rules: CPY-7 CPY-14 CPY-17 CPY-23 CPY-26

When a part cannot be copied or moved part by part, the implicit
function is deleted, so the problem appears only at a use.
Each of these makes the containing function deleted:

- a reference member,
- a `const` member,
- a part whose own function is deleted or hidden.

A part whose destructor is deleted or hidden makes only the copy and
move constructors deleted. A constructor makes a new object that must
end some day, and an assignment makes no new object.

The same holds for a function that the class defaults in its definition,
also when the signature of that function differs from the implicit one.
A deleted move function that the implementation produced steps aside, so
an rvalue copies instead. A move function that the class deletes by name
does not step aside. A function defaulted after the class definition
must be one that works, because by then the class has promised the
function.
:::

:::{principle} CPY-P4
:rules: CPY-9 CPY-10 CPY-11 CPY-24 CPY-25

Whether a copy or a move happens follows from the value category of the
source:

| Source | Result |
|---|---|
| An lvalue | A copy |
| An rvalue | A move if a move is possible, and a copy if not |
| A `const` rvalue | A copy, because a move would change the source |
| A prvalue | Neither. The prvalue initializes its target directly, so a class with no usable copy or move function can still be returned and passed as a prvalue. |

A `return` of a local variable or a parameter treats the name as an
rvalue, because the object is about to end.
:::

:::{principle} CPY-P5
:rules: CPY-12 CPY-18 CPY-13

The implementation can:

- make a named local variable the return object instead of moving the
  variable, except for a parameter and except in constant evaluation,
- assign a shared virtual base more than once.

The implementation should warn when:

- a program relies on the implicit copy of a class that manages its own
  copying,
- a cast in a `return` statement defeats the freedom to elide.
:::

:::{principle} CPY-P6
:rules: CPY-19 CPY-27

A copy or move function that copies part by part, with nothing virtual
and nothing user-provided, is *trivial*. A class whose copying is
trivial all the way down is *trivially copyable*. Copying the bytes of
an object of such a class copies the value of the object.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   copy_ctor: `class_name` "(" [`cv`] `class_name` "&" {`default_param`} ")"
   move_ctor: `class_name` "(" [`cv`] `class_name` "&&" {`default_param`} ")"
   copy_assign: `type_id` "operator" "=" "(" `copy_param` ")"
   copy_param: `class_name` | [`cv`] `class_name` "&"
   move_assign: `type_id` "operator" "=" "(" [`cv`] `class_name` "&&" ")"
   default_param: "," `parameter`
```

The four functions are the *copy and move functions* of a class. A
*user-declared* function is one that the class declares, including with
`= default` or `= delete`.

## Constraints

:::{rule} CPY-1
:kind: required
:tests: classes/copy-move/copy-ctor-const-ref.cpp classes/copy-move/copy-ctor-default-args.cpp classes/copy-move/copy-ctor-nonconst-ref.cpp classes/copy-move/copy-ctor-volatile.cpp classes/copy-move/template-not-copy-ctor.cpp

A *copy constructor* of class `T` is a constructor that is not a template.
Its first parameter is one of these:

- `T&`,
- `const T&`,
- `volatile T&`,
- `const volatile T&`.

Each other parameter has a default argument. A
constructor template is never a copy constructor.
:::

:::{rule} CPY-20
:kind: reject
:tests: classes/copy-move/copy-ctor-default-args-suppresses-implicit.cpp classes/copy-move/copy-ctor-nonconst-ref-const-source.cpp

A copy constructor whose first parameter is `T&` or `volatile T&` cannot
copy a `const` object.
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
:tests: classes/copy-move/copy-assign-forms.cpp classes/copy-move/explicit-object-copy-assign.cpp

A *copy assignment operator* of class `T` is a non-static member
`operator=` with one parameter of type `T` or a reference to `T` with any
cv-qualifiers. A *move assignment operator* has one parameter of type
`T&&` with any cv-qualifiers. The return type can be any type. An
assignment operator with an explicit object parameter and one more
parameter of reference to `T` is also a copy or move assignment operator.
Such an operator is user-declared, so the class gets no implicit one.
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
:tests: classes/copy-move/defaulted-copy-restores.cpp classes/copy-move/implicit-copy.cpp classes/copy-move/user-dtor-keeps-copy.cpp classes/copy-move/implicit-copy-of-owner-sanitizer.cpp

A class has an implicit copy constructor if it has no user-declared copy
constructor, and an implicit copy assignment operator if it has no
user-declared copy assignment operator. Each implicit copy function is
deleted if the class has a user-declared move constructor or move
assignment operator. A user-declared destructor does not remove the
implicit copy functions.
:::

:::{rule} CPY-21
:kind: reject
:tests: classes/copy-move/move-assign-deletes-copy-ctor.cpp classes/copy-move/move-ctor-deletes-copy-assign.cpp classes/copy-move/move-ctor-deletes-copy.cpp

A program must not use an implicit copy function that CPY-4 deletes.
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
:tests: classes/copy-move/assign-result-lvalue.cpp classes/copy-move/implicit-copy-nonconst-from-member-ok.cpp

The implicit copy constructor takes `const T&` if each base and member
can be copied from a `const` object. Otherwise, the implicit copy
constructor takes `T&`. The same choice applies to the implicit copy
assignment operator. Each implicit assignment operator returns `T&`,
which refers to `*this`. The implicit copy assignment operator of a
derived class hides all assignment operators of its bases.
:::

:::{rule} CPY-22
:kind: reject
:tests: classes/copy-move/implicit-copy-assign-nonconst.cpp classes/copy-move/implicit-copy-nonconst-from-member.cpp classes/copy-move/derived-assign-hides-base.cpp

An implicit copy function that takes `T&` cannot copy a `const` object.
An assignment to an object of a derived class does not use a base
assignment operator that CPY-6 hides.
:::

:::{rule} CPY-7
:kind: required
:tests: classes/copy-move/deleted-move-falls-back-to-copy.cpp classes/copy-move/move-only-member-moves.cpp classes/copy-move/reference-member-copy-ctor.cpp classes/copy-move/member-dtor-keeps-assignment.cpp classes/unions/union-deleted-move-uses-copy.cpp

An implicit or defaulted copy or move function is deleted if one of
these conditions is true:

- A base or member cannot be copied or moved in the same way, for
  example because the function for that base or member is deleted or
  not accessible.
- The function is a copy or move constructor, and a base or member has
  a destructor that is deleted or not accessible.
- The function is the copy constructor, and the class has an rvalue
  reference member.
- The function is a copy or move assignment operator, and the class has
  a reference member or a `const` member.

A deleted defaulted move function, or a move function that
{rule}`UNI-3` deletes, takes no part in overload resolution, so an
rvalue uses the copy function. A move function that the program deletes
explicitly takes part.
:::

:::{rule} CPY-23
:kind: reject
:tests: classes/copy-move/const-member-assign.cpp classes/copy-move/const-member-move-assign.cpp classes/copy-move/defaulted-copy-deleted-used.cpp classes/copy-move/deleted-copy-move-only-member.cpp classes/copy-move/deleted-copy-private-member.cpp classes/copy-move/explicitly-deleted-move-not-ignored.cpp classes/copy-move/reference-member-assign.cpp classes/copy-move/rvalue-ref-member-copy-deleted.cpp classes/copy-move/copy-deleted-by-member-dtor.cpp

A program must not use a copy or move function that CPY-7 deletes, or a
move function that the program deletes explicitly.
:::

:::{rule} CPY-14
:kind: required
:tests: classes/copy-move/defaulted-copy-const-mismatch-unused.cpp classes/copy-move/defaulted-copy-nonconst-param.cpp classes/copy-move/defaulted-move-const-param.cpp classes/copy-move/defaulted-out-of-line-copy.cpp classes/copy-move/defaulted-volatile-param.cpp

A copy or move constructor defaulted in its class can have a parameter
type different from the implicit one. Such a constructor is deleted
({question}`Q-0055`). A defaulted copy constructor can take `T&`. A
function defaulted after its class definition is not deleted by this
rule.
:::

:::{rule} CPY-26
:kind: reject
:tests: classes/copy-move/defaulted-copy-const-mismatch-used.cpp classes/copy-move/defaulted-move-const-param-used.cpp classes/copy-move/defaulted-volatile-param-used.cpp

A program must not use a copy or move constructor that CPY-14 deletes.
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
:tests: classes/copy-move/cast-move-only-accepted.cpp classes/copy-move/const-rvalue-picks-copy.cpp classes/copy-move/rvalue-picks-move.cpp

Overload resolution chooses between the copy and move functions:

- An lvalue uses the copy function.
- A non-`const` rvalue uses the move function, if there is one.
- A `const` rvalue uses the copy function.
:::

:::{rule} CPY-24
:kind: reject
:tests: classes/copy-move/explicit-copy-ctor-copy-init.cpp classes/copy-move/lvalue-move-only-rejected.cpp

An lvalue of a class with no usable copy constructor cannot initialize a
new object. An `explicit` copy constructor is not used in
copy-initialization. Two rules name specific cases of this rule
(decision 0042):

- {rule}`CPY-21`, a class whose move function deletes its copy,
- {rule}`JMP-15`, a `return` of a `static` local variable of a move-only
  type.
:::

:::{rule} CPY-10
:kind: required
:tests: classes/copy-move/prvalue-argument-no-copy.cpp classes/copy-move/prvalue-no-copy-count.cpp classes/copy-move/prvalue-no-copy.cpp

A prvalue of class type initializes the target object directly. No copy
or move constructor is called, and none needs to be accessible or not
deleted. The rule applies to:

- nested prvalues,
- return values,
- arguments.
:::

:::{rule} CPY-11
:kind: required
:tests: classes/copy-move/return-converting-move.cpp classes/copy-move/return-local-move-only.cpp classes/copy-move/return-local-prefers-move.cpp classes/copy-move/return-param-move-only.cpp classes/copy-move/return-parenthesized-local.cpp classes/copy-move/return-rvalue-ref-move-only.cpp classes/copy-move/return-rvalue-ref-to-ref.cpp classes/copy-move/parameter-return-moved.cpp

In a `return` statement, a name of one of these is an xvalue:

- a local variable with automatic storage duration,
- a parameter of the function,
- a variable of rvalue reference type that the function declares.

Parentheses around the name do not change the classification. The return
value is then initialized by a move, or by a converting constructor that
takes an rvalue. A name of one of these is an lvalue:

- a variable of lvalue reference type,
- a global variable,
- a member.

A parameter is always moved to the return value, once.
:::

:::{rule} CPY-25
:kind: reject
:tests: classes/copy-move/named-return-needs-move.cpp classes/copy-move/return-global-not-moved.cpp classes/copy-move/return-local-as-lvalue-ref.cpp classes/copy-move/return-lvalue-ref-not-moved.cpp classes/copy-move/return-member-not-moved.cpp

The return value must be valid to initialize from the name, as CPY-11
classifies it. An xvalue name cannot bind to an lvalue reference return
type. An lvalue name needs a usable copy constructor.
:::

:::{rule} CPY-27
:kind: required
:tests: classes/copy-move/trivial-assign-implicit.cpp classes/copy-move/trivial-assign-user-provided.cpp classes/copy-move/trivial-copy-defaulted.cpp classes/copy-move/trivial-copy-implicit.cpp classes/copy-move/trivial-copy-member-initializer.cpp classes/copy-move/trivial-copy-member-nontrivial.cpp classes/copy-move/trivial-copy-user-provided.cpp classes/copy-move/trivial-copy-virtual-base.cpp classes/copy-move/trivial-copy-virtual-function.cpp classes/copy-move/trivial-move-user-provided.cpp

This rule covers the copy and move constructors and the copy and move
assignment operators. Each of these makes such a function non-trivial:

- The program provides its body. A function defaulted after its first
  declaration counts.
- The class has a virtual function or a virtual base.
- A base or a member of class type is copied or moved by a non-trivial
  one.

With none of these conditions, the function is *trivial*. Default member
initializers play no part.
:::

## Runtime Behavior

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

:::{rule} CPY-19
:kind: required
:tests: classes/copy-move/trivially-copyable-bytes.cpp

A *trivially copyable* class meets these conditions, and others that
this page does not list yet *(class properties referenced)*:

- Its copy and move functions and its destructor are implicit or
  defaulted, and at least one copy or move function is not deleted.
- The class has no virtual function and no virtual base.
- Each base and each member is also trivially copyable.

If a program copies the bytes of such an object into another object of
the same type, the second object gets the same value.
:::

## Implementation Requirements

:::{rule} CPY-12
:kind: unspecified
:tests: classes/copy-move/named-return-elision-either.cpp classes/copy-move/constant-evaluation-no-nrvo.cpp

If a `return` statement names a local variable of the return type, it is
unspecified if the variable is moved to the return value or is the return
value itself. A parameter is not such a variable. Constant evaluation
always moves the variable.
:::

:::{rule} CPY-18
:kind: unspecified
:tests: classes/copy-move/virtual-base-assigned-count.cpp

If a virtual base can be reached through more than one path, it is
unspecified how many times the implicit copy or move assignment operator
assigns the virtual base subobject.
:::

## Recommended Practice

:::{rule} CPY-13
:kind: recommended
:tests: classes/copy-move/deprecated-copy-warns.cpp classes/copy-move/move-on-return-no-warning.cpp classes/copy-move/pessimizing-move-warns.cpp classes/copy-move/pessimizing-move-nontemplate.cpp classes/copy-move/cast-on-return-no-warning.cpp

An implementation should warn about these:

- a use of an implicit copy function in a class that has a user-provided
  copy constructor or copy assignment operator,
- a call of `std::move` *(library referenced)* on a local variable in a
  `return` statement, because the call prevents the variable from being
  the return value.

An implementation need not warn about a cast that the program writes,
such as `static_cast`.
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

struct Owner {                     // owns an array, so it writes its copies
  int n;
  int* items;
  explicit Owner(int count) : n(count), items(new int[count]()) {}
  Owner(const Owner& o) : n(o.n), items(new int[o.n]) {
    for (int i = 0; i < n; ++i) items[i] = o.items[i];
  }
  Owner& operator=(const Owner& o) {
    if (this != &o) {              // without the test, o = o copies from
      delete[] items;              // the new array, whose elements have
                                   // no value (NEW-3)
      n = o.n;
      items = new int[n];
      for (int i = 0; i < n; ++i) items[i] = o.items[i];
    }
    return *this;
  }
  ~Owner() { delete[] items; }
};
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 100
of 102 tests.

- CPY-14: GCC 14 rejects a defaulted constructor whose parameter type
  differs from the implicit one. GCC 16, Clang, and EDG accept it and
  delete the function ({question}`Q-0055`).
- CPY-12: Clang and GCC use the variable as the return value at `-O0`
  and `-O2`. EDG runs no programs. The test accepts a move too.
- CPY-13: Clang and GCC give the first warning under `-Wextra` and the
  second under `-Wall`. Both warn for a call of a function named
  `std::move`, also when the function is not a template. Neither warns
  for `static_cast` or for a call of a function with another name.
- CPY-7: all compilers accept an assignment of a class whose member has a
  deleted or private destructor, and reject a copy or move construction.
  All compilers move a union whose move constructor UNI-3 deletes by its
  copy constructor.
- CPY-18: Clang and GCC assign the virtual base two times. The test
  accepts one or two.
- CPY-19: the test checks the consequence for a simple class and does
  not check each condition of the definition.

Inputs: clang-sema-declcxx, clang-sema-init, clang-diag-sema,
edg-class-decl.
