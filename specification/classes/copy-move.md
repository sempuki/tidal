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
resolution ({doc}`/overloading/overload-resolution`), deleted and defaulted
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

A constructor that takes the class by value is not valid. An lvalue
argument would need that constructor itself to copy the argument into
the parameter.
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

Whether a copy or a move happens follows from the {term}`value category` of the
source:

| Source | Result |
|---|---|
| An lvalue | A copy |
| An rvalue | A move if a move is possible, and a copy if not |
| A `const` rvalue | A copy, because a move would change the source |
| A prvalue | Neither. The prvalue initializes its target directly, so a class with no usable copy or move function can still be returned and passed as a prvalue. |

A `return` of a local variable or a parameter treats the name as an
rvalue, because the object is about to end. The rvalue initializes the
return value also when the return type is another class. A
{term}`converting constructor` of the return type that takes an rvalue
then runs ({rule}`CPY-11`).

A program moves from a named object by casting it to an rvalue
reference. `static_cast<T&&>(x)` is an xvalue ({rule}`VC-3`), so the
move function is chosen. The library's `std::move` *(library
referenced)* does the same cast, and a program that includes no header
writes the cast itself.
:::

:::{principle} CPY-P5
:rules: CPY-12 CPY-18 CPY-13

The implementation can:

- make a named local variable the return object instead of moving the
  variable, except for a parameter and except in {term}`constant evaluation`,
- assign a shared {term}`virtual base` more than once.

The implementation should warn when:

- a program relies on the implicit copy of a class that manages its own
  copying,
- a cast in a `return` statement defeats the freedom to elide.
:::

:::{principle} CPY-P6
:rules: CPY-19 CPY-27 CPY-28

A copy or move function that copies part by part, with nothing virtual
and nothing user-provided, is *trivial*. A class whose copying is
trivial all the way down is *trivially copyable*. Copying the bytes of
an object of such a class copies the value of the object.

Only the functions that a copy can use count. These are the
{term}`eligible` ones, so a deleted function, or one whose {term}`constraints <constraint>`
fail, neither helps nor blocks. A class with a `const` or reference
member has no usable assignment, and its objects still copy by bytes as
its constructors do. Access plays no part. A class whose destructor is
deleted, or that has no eligible copy or move function at all, says
that its objects are not to be copied, so it is not trivially copyable.

The property extends from classes to types:

| Type | Trivially copyable |
|---|---|
| A {term}`scalar type` | Always |
| A class type | If CPY-19 holds |
| An array type | If the element type is |
| A cv-qualified type | If the type without the qualifiers is |
| A reference type, `void`, a function type | Never |
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
:tests: classes/copy-move/copy-ctor-by-value.cpp classes/copy-move/template-ctor-by-value-own-class.cpp classes/copy-move/class-template-ctor-by-value.cpp

A constructor of class `T` must not have a first parameter of type `T`
if all other parameters have default arguments. In a {term}`class template`,
the implementation rejects such a constructor at the template's
definition, before any {term}`instantiation`. A constructor template
never gives such a constructor, so a copy of a `T` uses the copy
constructor ({rule}`CPY-1`).
:::

:::{rule} CPY-2
:kind: required
:tests: classes/copy-move/move-ctor-default-args.cpp classes/copy-move/move-ctor.cpp

A *move constructor* of class `T` is a constructor that is not a
template. Its first parameter is `T&&` with any {term}`cv-qualifiers <cv-qualifier>`. Each other
parameter has a default argument.
:::

:::{rule} CPY-3
:kind: required
:tests: classes/copy-move/copy-assign-forms.cpp classes/copy-move/explicit-object-copy-assign.cpp classes/copy-move/move-assign-only-from-prvalue.cpp classes/copy-move/assign-converted-owner.cpp

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
:tests: classes/copy-move/defaulted-copy-restores.cpp classes/copy-move/implicit-copy.cpp classes/copy-move/user-dtor-keeps-copy.cpp classes/copy-move/implicit-copy-of-owner-sanitizer.cpp classes/copy-move/move-assign-only-from-prvalue.cpp classes/copy-move/move-assign-deletes-copy-assign.cpp

A class has an implicit copy constructor if it has no user-declared copy
constructor, and an implicit copy assignment operator if it has no
user-declared copy assignment operator. Each implicit copy function is
deleted if the class has a user-declared move constructor or move
assignment operator. A user-declared destructor does not remove the
implicit copy functions.
:::

:::{rule} CPY-21
:kind: reject
:tests: classes/copy-move/move-assign-deletes-copy-ctor.cpp classes/copy-move/move-ctor-deletes-copy-assign.cpp classes/copy-move/move-ctor-deletes-copy.cpp classes/copy-move/move-assign-deletes-copy-assign.cpp

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
:tests: classes/copy-move/deleted-move-falls-back-to-copy.cpp classes/copy-move/move-only-member-moves.cpp classes/copy-move/reference-member-copy-ctor.cpp classes/copy-move/member-dtor-keeps-assignment.cpp classes/unions/union-deleted-move-uses-copy.cpp classes/copy-move/trivially-copyable-const-member.cpp classes/copy-move/trivially-copyable-deleted-destructor.cpp

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

A part with no move function is moved by its copy function
({rule}`CPY-5`), so that part deletes the move only if its copy is not
usable. A deleted defaulted move function, or a move function that
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
({question}`Q-0055`), for example a move constructor that takes
`const T&&`. A defaulted copy constructor can take `T&`. A function
defaulted after its class definition is not deleted by this rule.
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
:tests: classes/copy-move/cast-move-only-accepted.cpp classes/copy-move/const-rvalue-picks-copy.cpp classes/copy-move/rvalue-picks-move.cpp classes/copy-move/trivially-copyable-every-eligible.cpp

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
{term}`copy-initialization`. Two rules name specific cases of this rule
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
:tests: classes/copy-move/return-converting-move.cpp classes/copy-move/return-local-move-only.cpp classes/copy-move/return-local-prefers-move.cpp classes/copy-move/return-param-move-only.cpp classes/copy-move/return-parenthesized-local.cpp classes/copy-move/return-rvalue-ref-move-only.cpp classes/copy-move/return-rvalue-ref-to-ref.cpp classes/copy-move/parameter-return-moved.cpp classes/constructors/return-owner-converts-to-base-owner.cpp

In a `return` statement, a name of one of these is an xvalue:

- a local variable with {term}`automatic storage` duration,
- a parameter of the function,
- a variable of rvalue reference type that the function declares.

Parentheses around the name do not change the classification. The return
value is then initialized by a move, or by a {term}`converting constructor` that
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
:tests: classes/copy-move/trivially-copyable-member-selected.cpp classes/copy-move/trivial-copy-array-member-nontrivial.cpp classes/copy-move/trivially-copyable-member-template-selected.cpp classes/copy-move/trivially-copyable-defaulted-later.cpp classes/copy-move/trivial-assign-implicit.cpp classes/copy-move/trivial-assign-user-provided.cpp classes/copy-move/trivial-copy-defaulted.cpp classes/copy-move/trivial-copy-implicit.cpp classes/copy-move/trivial-copy-member-initializer.cpp classes/copy-move/trivial-copy-member-nontrivial.cpp classes/copy-move/trivial-copy-user-provided.cpp classes/copy-move/trivial-copy-virtual-base.cpp classes/copy-move/trivial-copy-virtual-function.cpp classes/copy-move/trivial-move-user-provided.cpp classes/copy-move/trivially-copyable-private-copy.cpp classes/copy-move/trivially-copyable-volatile-member.cpp classes/unions/union-copy-two-copy-constructors.cpp

This rule covers the copy and move constructors and the copy and move
assignment operators. Each of these makes such a function non-trivial:

- The program provides its body. A function defaulted after its first
  declaration counts.
- The class has a virtual function or a virtual base.
- A base, or a member of class type or of array of class type, is
  copied or moved by a non-trivial one. The function that the copy or
  move selects for the part decides ({question}`Q-0339`).

With none of these conditions, the function is *trivial*. Default member
initializers play no part, and neither does a `volatile` member
of scalar type.
:::

## Runtime Behavior

:::{rule} CPY-8
:kind: required
:tests: classes/copy-move/implicit-copy-constexpr.cpp classes/copy-move/self-pointer-copy.cpp classes/copy-move/memberwise-array.cpp classes/copy-move/memberwise-assign-order.cpp classes/copy-move/memberwise-order.cpp

An implicit copy or move function copies or moves each part of the
object in this order:

1. direct bases, in the order of the base list,
2. non-static data members, in the order of their declarations.

A member of array type is copied element by element. An implicit copy
function can be used in constant evaluation.
:::

:::{rule} CPY-19
:kind: required
:tests: classes/copy-move/trivially-copyable-bytes.cpp classes/copy-move/trivially-copyable-at-least-one.cpp classes/copy-move/trivially-copyable-const-member.cpp classes/copy-move/trivially-copyable-volatile-member.cpp classes/copy-move/trivially-copyable-deleted-destructor.cpp classes/copy-move/trivially-copyable-defaulted-later.cpp classes/copy-move/trivially-copyable-every-eligible.cpp classes/copy-move/trivially-copyable-member-selected.cpp classes/copy-move/trivially-copyable-constraint-fails.cpp classes/copy-move/trivially-copyable-private-copy.cpp

A class is *trivially copyable* if all of these conditions hold:

- The class has at least one {term}`eligible` copy or move function.
- Each eligible copy or move function is trivial ({rule}`CPY-27`).
- The destructor is trivial ({rule}`DTOR-9`) and not deleted
  ({question}`Q-0343`).

If a program copies the bytes of such an object into another object of
the same type, the second object gets the same value. {rule}`STG-12`
gives the byte types that the copy can go through and the limit on
padding bytes. If the second object is `const` or has a `const`
{term}`subobject`, the copy changes a `const` object, and the behavior is
undefined ({rule}`CV-5`, {question}`Q-0344`).
:::

:::{rule} CPY-28
:kind: required
:tests: classes/copy-move/trivially-copyable-types.cpp

These types are {term}`trivially copyable` types:

- scalar types,
- trivially copyable classes (CPY-19),
- arrays of trivially copyable types,
- the cv-qualified forms of these types.

A reference type, `void`, and a function type are not trivially
copyable.
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
:tests: classes/copy-move/virtual-base-assigned-count.cpp classes/copy-move/virtual-base-move-assigned-count.cpp

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

% tidal-example: reject
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

An owner of an object of a class with virtual functions has one owner
for each object. So the owner deletes its copy functions and writes its
move functions. A member template converts an owner of a derived class
to an owner of a base class. A member template is never a move
constructor (CPY-2), so the class still needs its own move constructor:

```cpp
template <class T> class Owner {
  T* p;
  template <class U> friend class Owner;  // reads Owner<U>::p (ACC-20)
public:
  explicit Owner(T* q) : p(q) {}
  Owner(const Owner&) = delete;          // one owner for each object
  Owner& operator=(const Owner&) = delete;
  Owner(Owner&& o) : p(o.p) { o.p = nullptr; }
  Owner& operator=(Owner&& o) {
    if (this != &o) { delete p; p = o.p; o.p = nullptr; }
    return *this;
  }
  template <class U>                     // U* converts to T* (CONV-14)
  Owner(Owner<U>&& o) : p(o.p) { o.p = nullptr; }
  ~Owner() { delete p; }                 // deletes through the base
  T* operator->() const { return p; }
};

template <class T, class... Args>        // forwarding references (DED-8)
Owner<T> make(Args&&... args) {
  return Owner<T>(new T(static_cast<Args&&>(args)...));
}

struct Shape {
  virtual ~Shape() = default;            // needed for delete (VIRT-12)
  virtual int area() const = 0;
};
struct Square : Shape {
  int side;
  explicit Square(int s) : side(s) {}
  int area() const override { return side * side; }
};

Owner<Shape> unit_square() {
  Owner<Square> s = make<Square>(1);
  return s;                // an rvalue, so the converting constructor runs (CPY-11)
}
```

A member that points into the object's own storage needs copy functions
that the class writes. The implicit copy copies the pointer, so the copy
points into the source object (CPY-8):

```cpp
struct Line {
  char text[16] = {};
  char* cursor = text;             // points into this object's text
  Line() = default;
  Line(const Line& o) : cursor(text + (o.cursor - o.text)) {
    for (int i = 0; i < 16; ++i) text[i] = o.text[i];
  }
  Line& operator=(const Line& o) {
    for (int i = 0; i < 16; ++i) text[i] = o.text[i];
    cursor = text + (o.cursor - o.text);
    return *this;
  }
};
Line a;
Line b = a;                        // b.cursor points into b.text
```

Without the two functions, `b.cursor` would point into `a.text`. A class
that keeps the result of placement `new` into its own bytes has the same
need ({doc}`/foundations/storage`).

The parameter of an assignment operator can refer to `*this`. A read
through the parameter after a change to `*this` then sees the change. A
counted owner saves what it needs from the source before it releases
its own resource:

% tidal-example: fragment
```cpp
struct Block { int count; int value; };
struct Shared {
  Block* block;
  explicit Shared(int v) : block(new Block{1, v}) {}
  Shared(const Shared& o) : block(o.block) { ++block->count; }
  Shared& operator=(const Shared& o) {
    Block* next = o.block;         // saved first: o can be *this
    ++next->count;
    release();                     // can delete the block of *this
    block = next;
    return *this;
  }
  ~Shared() { release(); }
  void release() { if (--block->count == 0) delete block; }
};
Shared a(5);
a = a;                             // count goes up, then down; a keeps its block

struct Unique {
  int* p = nullptr;
  Unique& operator=(Unique&& o) noexcept {
    if (this != &o) { delete p; p = o.p; o.p = nullptr; }  // a self-move keeps p
    return *this;
  }
};
```

A constructor template is never a copy constructor (CPY-1), so the class
keeps its implicit copy constructor. The two can tie, and then the
function that is not a template wins ({rule}`OVL-27`):

```cpp
struct Any {
  Any() {}
  template <class T> Any(const T& value);
};
Any a;
Any b(a);                          // the implicit copy constructor
Any c(1);                          // the template, with T = int
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 118
of 123 tests.

### Divergences

- {question}`Q-0055` (CPY-14; settled on CPY-P3): whether a defaulted
  copy or move constructor whose parameter type differs from the
  implicit one is deleted or not valid. It is deleted, and only a use
  is rejected. No newest family differs. GCC 14 rejects the declaration
  of a `const volatile` copy constructor and a `const` move
  constructor.
- {question}`Q-0339` (CPY-27, CPY-19; open): whether a part with a
  trivial and a non-trivial copy constructor makes the enclosing copy
  constructor non-trivial when the copy selects the trivial one. The
  selected function decides, as Clang says. GCC and EDG delete the copy
  constructor of a union that holds such a class, and EDG copies a
  union whose member's constructor template is selected.
- {question}`Q-0343` (CPY-19; open): whether a class with a deleted
  destructor, or with all four copy and move functions deleted, is
  trivially copyable. It is not. The builtin query of every compiler
  says it is, and no test can show the difference. The question waits
  on the maintainer (`documentation/decisions/pending.md`).
- {question}`Q-0344` (CPY-19; open): whether a byte copy can write over
  a live object that has a `const` member. The behavior is undefined,
  as {rule}`CV-5` gives. No test can show the difference, and the tests
  copy into a buffer or into an object with no `const` member.

### Split tests

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `defaulted-move-const-param.cpp` | CPY-14 | accept | accept | GCC 16: accept; GCC 14: reject | accept | {question}`Q-0055` |
| `defaulted-volatile-param.cpp` | CPY-14 | accept | accept | GCC 16: accept; GCC 14: reject | accept | {question}`Q-0055` |
| `trivially-copyable-member-selected.cpp` | CPY-27, CPY-19, UNI-3 | accept | accept | reject | reject | {question}`Q-0339` |
| `trivially-copyable-member-template-selected.cpp` | CPY-27, UNI-11 | reject | reject | reject | accept | {question}`Q-0339` |
| `classes/unions/union-copy-two-copy-constructors.cpp` | UNI-3, CPY-27 | accept | accept | reject | reject | {question}`Q-0339` |

### Notes

- CPY-15: Clang and GCC reject a constructor `W(W)` of a class
  template at the template's definition, with no instantiation. GCC
  reports the error under `-Wtemplate-body`, a warning group that is an
  error by default.
- CPY-14: every compiler accepts a defaulted `S(const S&)` where the
  implicit parameter is `S&`, and deletes the function. Every compiler
  accepts a defaulted `S(S&)` where the implicit parameter is
  `const S&`. Every compiler rejects a use of a deleted constructor
  (CPY-26).
- CPY-12: Clang and GCC use the variable as the return value at `-O0`
  and `-O2`. EDG runs no programs. The test accepts a move too.
- CPY-13: Clang and GCC give the first warning under `-Wextra` and the
  second under `-Wall`. Both warn for a call of a function named
  `std::move`, also when the function is not a template. Neither warns
  for `static_cast` or for a call of a function with another name.
- CPY-13: Clang 22 warns under `-Wall` (`-Wself-move`) when a program
  assigns `static_cast<T&&>(x)` to `x` itself, for a class and for an
  `int`. GCC 16 gives no warning under `-Wall -Wextra`. The warning is
  outside CPY-13, and a move assignment that the class writes must
  still work for a source that is the object itself.
- CPY-7: all compilers accept an assignment of a class whose member has a
  deleted or private destructor, and reject a copy or move construction.
  All compilers move a union whose move constructor UNI-3 deletes by its
  copy constructor.
- CPY-18: Clang and GCC assign the virtual base two times. The test
  accepts one or two.
- CPY-19, CPY-27, CPY-28: no core-language check gives a yes or no for
  trivially copyable as a whole (inventory item
  OBS-no-core-trivial-whole). The tests observe each condition through
  a union that holds the class ({rule}`UNI-3`), a requires expression,
  or a byte copy. The probe grid `trivially-copyable.json` compares
  these checks with each compiler's builtin query, which no test uses
  (decision 0051). The query of every compiler agrees with CPY-19 on
  the rows that this table leaves out:

  | Class | Clang query | GCC query | EDG query | CPY-19 |
  |---|---|---|---|---|
  | a `const` member, or a reference member | yes | yes | yes | yes |
  | only a trivial move constructor, other copy and move functions deleted | yes | yes | yes | yes |
  | all four copy and move functions deleted | yes | yes | yes | no |
  | a deleted destructor | yes | yes | yes | no |
  | two copy constructors, one user-provided | no | no | no | no |
  | a member with two copy constructors, one user-provided | yes | no | no | yes |
  | a member whose constructor template copies it from a non-`const` source | no | yes | yes | no |
  | a private trivial copy constructor | yes | yes | yes | yes |

  The two rows where every query says yes and CPY-19 says no follow the
  class-properties inventory (items TCC-all-deleted and
  TCC-deleted-destructor).
- CPY-19: a class with a `const` member is trivially copyable
  (`trivially-copyable-const-member.cpp`). The gap review of item O
  (`reports/review/2026-11-03-gap-review-item-o.md`) finds that the
  reference does not say whether the byte copy or the `const` rule
  decides.
- CPY-27: a member of array of class type counts as a part, as the
  gap review of item O describes. Every compiler deletes the copy
  constructor of a union that holds a class whose array member has a
  user-provided copy constructor (`trivial-copy-array-member-nontrivial.cpp`).

### Sources

CPY-19 and CPY-28 come from the grid `trivially-copyable.json`
and the inventory items TCC-at-least-one, TCC-const-member,
TCC-every-eligible, TCC-inaccessible-ok, TCC-reference-member,
TCC-volatile-member, TYPE-array, TYPE-cv, TYPE-non-object, and
TYPE-scalar. The CPY-27 sentence on the selected function comes from
item TCC-member-selected and the same grid.

Inputs: clang-sema-declcxx, clang-sema-init, clang-diag-sema,
edg-class-decl.
