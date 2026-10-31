# Pointers to members

A pointer to member refers to a non-static member of a class, without an
object. The operators `.*` and `->*` apply a pointer to member to an
object. This page covers:

- pointer-to-member declarators,
- the expressions that form them,
- the two operators,
- conversions between pointers to members,
- comparisons.

Depends on: primary expressions, unary operators, derived classes,
virtual functions.

## Principles

:::{principle} MPTR-P1
:rules: MPTR-1 MPTR-8 MPTR-2 MPTR-9

A pointer to member names a member without naming an object. The type
of a pointer to member therefore records the class and the type of the
member, and the class can be incomplete because nothing is accessed
through the type alone. Only a non-static member can be named this way.
A static member is an ordinary object or function, so `&` on a static
member gives an ordinary pointer, and so does a member function with an
explicit object parameter. The form `&C::m` with a qualified name is the
only way to make a pointer to member, because only that form says which
class the member belongs to without supplying an object.
:::

:::{principle} MPTR-P2
:rules: MPTR-3 MPTR-10

`.*` and `->*` supply the missing object. The result is the member as if
the member had been accessed on that object. The result takes its value
category and its cv-qualifiers from the object, and a virtual function
dispatches on the object's dynamic type. The object must be of the
member's class or a class derived from the member's class, because only
those objects have the member. A pointer to member function names no
object by itself, so the result of applying the pointer exists only to
be called.
:::

:::{principle} MPTR-P3
:rules: MPTR-4 MPTR-11

Conversions between pointers to members run the opposite way to
conversions between pointers to objects.

| Conversion | Allowed by | Why |
|---|---|---|
| Base to derived | Implicit conversion | A pointer to a member of a base works with every derived object. |
| Derived to base | `static_cast` only | A base object may lack the member. |
| Through a virtual base | Nothing | The offset of the member is not known from the type. |
| To an ordinary pointer, or between data members and member functions | Nothing | A pointer to member is not an address. |

The qualifier rules follow those of ordinary pointers.
:::

:::{principle} MPTR-P4
:rules: MPTR-5 MPTR-7 MPTR-6 MPTR-12

Pointers to members have no order, so only equality is defined. Pointers
to virtual functions compare by implementation detail, so their
equality is unspecified. A null pointer to member names nothing, and a
pointer to a member that the object lacks names nothing in that object,
so applying either is undefined.
:::

## Syntax

```{eval-rst}
.. productionlist:: mptr
   member_pointer_declarator: `nested_name` "*" [`cv`] `declarator`
   member_pointer_expr: "&" `nested_name` `identifier`
   pm_expression: `cast_expression`
                : `pm_expression` ".*" `cast_expression`
                : `pm_expression` "->*" `cast_expression`
```

`int S::* p` declares `p` as a pointer to a data member of `S` with type
`int`. `int (S::* f)(int)` declares a pointer to a member function. `.*`
and `->*` bind more tightly than `*` and less tightly than the unary
operators.

## Constraints

:::{rule} MPTR-1
:kind: required
:tests: expressions/member-pointers/declare-alias.cpp expressions/member-pointers/declare-cv.cpp expressions/member-pointers/declare-data.cpp expressions/member-pointers/declare-function.cpp expressions/member-pointers/declare-incomplete.cpp

A pointer to member has a class and a member type. The class can be
incomplete at the declaration. cv-qualifiers can apply to the member
type and to the pointer itself. A type alias can name a
pointer-to-member type.
:::

:::{rule} MPTR-8
:kind: reject
:tests: expressions/member-pointers/declare-non-class.cpp expressions/member-pointers/declare-pointer-to-reference.cpp expressions/member-pointers/declare-reference-member.cpp expressions/member-pointers/declare-void-member.cpp

The class of a pointer to member must be a class type. The member type
must not be a reference or `void`.
:::

:::{rule} MPTR-2
:kind: required
:tests: expressions/member-pointers/form-overloaded-target.cpp expressions/member-pointers/form-parenthesized-not-member-pointer.cpp expressions/member-pointers/form-static-ordinary-pointer.cpp expressions/member-pointers/form-type.cpp expressions/member-pointers/base-member-type.cpp expressions/member-pointers/explicit-object-ordinary-pointer.cpp

`&C::m` gives a pointer to member if `m` is a non-static member of `C`.
The type of the pointer is `T C::*` for a data member of type `T`, or a
pointer to member function with the parameters and qualifiers of `m`.
These cases have their own rules:

- For a static member, `&C::m` gives an ordinary pointer.
- With parentheses, `&(C::m)` gives an ordinary pointer to the member
  of `*this`.
- If `m` is overloaded, the target type selects the function.
- If `m` is declared in a base `B` of `C`, the type names `B`, as in
  `T B::*`.
- For a member function with an explicit object parameter, `&C::f`
  gives an ordinary pointer to function.
:::

:::{rule} MPTR-9
:kind: reject
:tests: expressions/member-pointers/address-of-bound-member.cpp expressions/member-pointers/form-bitfield.cpp expressions/member-pointers/form-constructor.cpp expressions/member-pointers/form-function-without-amp.cpp expressions/member-pointers/form-needs-qualified.cpp expressions/member-pointers/form-private-access.cpp expressions/member-pointers/explicit-object-not-member-pointer.cpp

A pointer to member must be formed with `&` and a qualified name. These
are not valid:

- `&m` or `C::f` without `&`,
- `&obj.f` for a member function,
- a bit-field, a constructor, or a member that is not accessible,
- a pointer to member for a function with an explicit object parameter.
:::

:::{rule} MPTR-3
:kind: required
:tests: expressions/member-pointers/arrow-star.cpp expressions/member-pointers/call-const-on-nonconst.cpp expressions/member-pointers/constexpr-use.cpp expressions/member-pointers/derived-object.cpp expressions/member-pointers/incomplete-object.cpp expressions/member-pointers/precedence.cpp expressions/member-pointers/ref-qualified-const-lvalue-on-rvalue.cpp expressions/member-pointers/result-const.cpp expressions/member-pointers/result-lvalue.cpp expressions/member-pointers/result-xvalue.cpp expressions/member-pointers/virtual-call.cpp expressions/member-pointers/mutable-direct-ok.cpp

`o.*p` applies the pointer to member `p` to the object `o`, and `q->*p`
is `(*q).*p`. These rules apply:

- The class of `o` must be the class of `p` or derived from the class of
  `p`. The class of `o` can be incomplete if the two classes are the
  same.
- For a data member, the result has the member type with the
  cv-qualifiers of `o`, also for a `mutable` member. The result is an
  lvalue if `o` is an lvalue and an xvalue if not.
- For a member function, the result can only be called, as
  `(o.*p)(args)`. The call uses the final overrider of a virtual
  function.
- Constant evaluation can use both operators.
:::

:::{rule} MPTR-10
:kind: reject
:tests: expressions/member-pointers/call-drops-const.cpp expressions/member-pointers/call-needs-parens.cpp expressions/member-pointers/call-result-only.cpp expressions/member-pointers/ref-qualified-lvalue-only.cpp expressions/member-pointers/rhs-not-member-pointer.cpp expressions/member-pointers/rvalue-qualified-on-lvalue.cpp expressions/member-pointers/unrelated-object.cpp expressions/member-pointers/mutable-through-pointer.cpp

These uses of `.*` and `->*` are not valid:

- a right operand that is not a pointer to member,
- an object of an unrelated class,
- a call written as `o.*p(args)`, which applies `.*` to the result of
  `p(args)`,
- a result of a member function pointer that is not called,
- a call that removes a cv-qualifier of the object,
- a write through `.*` to a `mutable` member of a `const` object,
- a call on an rvalue with a function qualified `&`, or on an lvalue
  with a function qualified `&&`.

A function qualified `const&` can be called on an rvalue.
:::

:::{rule} MPTR-4
:kind: required
:tests: expressions/member-pointers/base-to-derived-implicit.cpp expressions/member-pointers/derived-to-base-static-cast.cpp expressions/member-pointers/null-converts.cpp expressions/member-pointers/reinterpret-between.cpp expressions/member-pointers/to-bool.cpp expressions/member-pointers/qualification-conversion.cpp expressions/member-pointers/reinterpret-function-round-trip.cpp expressions/member-pointers/zero-literal-null.cpp

These conversions are valid:

- A pointer to member of a base `B` converts implicitly to a pointer to
  member of a derived class `D`. `static_cast` converts in the other
  direction.
- A null pointer to member converts to a null pointer to member.
- Every pointer to member converts to `bool`.
- `reinterpret_cast` can convert between two pointer-to-data-member
  types, or between two pointer-to-member-function types. Converting
  back gives the original value.
- A conversion can add cv-qualifiers to the member type, at each level,
  where the same conversion of ordinary pointers is valid.
- The literal `0` converts to a null pointer to member.

A pointer to member with static storage and no initializer is null.
:::

:::{rule} MPTR-11
:kind: reject
:tests: expressions/member-pointers/ambiguous-base-conversion.cpp expressions/member-pointers/derived-to-base-implicit.cpp expressions/member-pointers/no-conversion-to-ordinary-pointer.cpp expressions/member-pointers/reinterpret-data-to-function.cpp expressions/member-pointers/static-cast-from-int-pointer.cpp expressions/member-pointers/virtual-base-conversion.cpp expressions/member-pointers/private-base-conversion.cpp expressions/member-pointers/private-base-static-cast.cpp expressions/member-pointers/qualification-conversion-unsafe.cpp

These conversions are not valid:

- from derived to base without `static_cast`,
- through a virtual base, an ambiguous base, or a base that is not
  accessible,
- a change of cv-qualifiers that is not valid for ordinary pointers,
- between a pointer to member and an ordinary pointer,
- between a pointer to data member and a pointer to member function.
:::

:::{rule} MPTR-5
:kind: required
:tests: expressions/member-pointers/compare-equal.cpp expressions/member-pointers/virtual-vs-null.cpp expressions/member-pointers/union-members-equal.cpp

Two pointers to members can be compared with `==` and `!=`, also with
`nullptr`. A pointer to a virtual member function compares unequal to a
null pointer to member. Pointers to two members of one union with the
same type compare equal.
:::

:::{rule} MPTR-12
:kind: reject
:tests: expressions/member-pointers/compare-relational.cpp expressions/member-pointers/compare-three-way.cpp

Members have no order, so a program must not compare two member
pointers with `<`, `>`, `<=`, `>=`, or `<=>`. MPTR-5 gives the
comparisons that are valid.
:::

## Undefined Behavior

:::{rule} MPTR-6
:kind: undefined
:tests: expressions/member-pointers/dynamic-type-lacks-member-constant.cpp expressions/member-pointers/null-use-constant.cpp

The behavior is undefined in these cases:

- `.*` or `->*` with a null pointer to member,
- `.*` or `->*` with an object that does not have the member, for
  example after a `static_cast` from a member of a derived class.
:::

## Implementation Requirements

:::{rule} MPTR-7
:kind: unspecified
:tests: expressions/member-pointers/compare-virtual-function-unspecified.cpp

If both operands point to member functions and either points to a
virtual function, the result of `==` or `!=` is unspecified. A program
must not rely on the result.
:::

## Examples

```cpp
struct Point { int x, y; int norm() const; };

int Point::* axis = &Point::y;
Point p{3, 4};
p.*axis = 5;                       // p.y is 5
int (Point::* m)() const = &Point::norm;
int n = (p.*m)();                  // calls p.norm()
int bad = p.*m();                  // rejected: needs parentheses

struct Point3 : Point { int z; };
int Point3::* any = &Point::x;     // base to derived: implicit
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on all
66 tests.

Inputs: clang-sema-expr, clang-sema-type, clang-diag-sema, edg-decls.
