# Access control and friends

Access control limits which code can use the name of a class member. A
member is public, protected, or private. A friend of a class has the same
access as the members of the class.

## Syntax

```{eval-rst}
.. productionlist:: access
   access_label: `access_specifier` ":"
   access_specifier: "public" | "protected" | "private"
   base_specifier: [`access_specifier`] ["virtual"] `class_name`
   friend_decl: "friend" `function_decl`
              : "friend" `function_definition`
              : "friend" ("class" | "struct" | "union") `class_name` ";"
```

An *access label* is an `access_label` in a class body. A *friend
declaration* is a `friend_decl` in a class body.

## Legality Rules

:::{rule} ACC-1
:kind: required
:tests: classes/access/class-default-private.cpp classes/access/struct-default-public.cpp classes/access/union-default-public.cpp classes/access/labels.cpp classes/access/access-label-on-nothing.cpp

The default access of a member depends on the class key:

- `class` gives private access.
- `struct` and `union` give public access.

An access label sets the access of the members after it, up to the next
label. A class can have any number of labels, in any order.
:::

:::{rule} ACC-2
:kind: reject
:tests: classes/access/private-outside.cpp classes/access/static-member-access.cpp classes/access/private-in-member.cpp classes/access/private-type-named.cpp

Only the members and friends of a class can use the name of a private
member. This applies to data members, member functions, static members,
and member types. A member function can use the private members of any
object of its class.
:::

:::{rule} ACC-3
:kind: reject
:tests: classes/access/access-after-overload-resolution.cpp

The implementation checks access after overload resolution
*(referenced)* selects a function. If the selected function is not
accessible, the call is not valid. An accessible overload that is a worse
match does not make the call valid.
:::

:::{rule} ACC-11
:kind: required
:tests: classes/access/private-type-via-auto.cpp classes/access/private-type-out-of-line.cpp classes/access/virtual-access-static-type.cpp

Access control applies to names. It does not apply in these cases:

- A type that comes from deduction, for example with `auto`, and is not
  named.
- The final overrider of a virtual function call. The access of the
  function that the call names applies.
- The return type and declarator of a member definition outside its
  class. They have the access of a member.
:::

:::{rule} ACC-4
:kind: reject
:tests: classes/access/protected-outside.cpp classes/access/protected-through-base.cpp classes/access/protected-through-derived.cpp classes/access/protected-static-through-base.cpp classes/access/protected-member-pointer.cpp classes/access/protected-member-pointer-derived.cpp

A protected member has the access of a private member. Members and
friends of a derived class `D` also have access to it, with these limits:

- For a non-static member used through an object, the class of the object
  must be `D` or a class derived from `D`.
- A pointer to a non-static member must be formed with `D`, as `&D::m`.

A static protected member has no such limit.
:::

:::{rule} ACC-5
:kind: required
:tests: classes/access/friend-function.cpp classes/access/friend-class.cpp classes/access/friend-member-function.cpp classes/access/friend-member-function-only.cpp classes/access/friend-ignores-label.cpp

A friend declaration gives a function or a class the access of a member.
A friend can be:

- a function,
- a member function of another class, and only that function,
- a class, and all its member functions.

An access label has no effect on a friend declaration.
:::

:::{rule} ACC-6
:kind: reject
:tests: classes/access/friend-not-transitive.cpp classes/access/friend-not-inherited.cpp

A friend of a friend does not get access. A class derived from a friend
does not get access.
:::

:::{rule} ACC-7
:kind: required
:tests: classes/access/hidden-friend.cpp classes/access/hidden-friend-not-visible.cpp

A friend function that is first declared in a class is a member of the
enclosing namespace. Ordinary name lookup does not find it. Only
argument-dependent lookup *(referenced)* finds it, unless the namespace
also declares it.
:::

:::{rule} ACC-8
:kind: required
:tests: classes/access/private-base.cpp classes/access/struct-base-default-public.cpp classes/access/protected-base-in-derived.cpp classes/access/private-base-name-in-derived.cpp

The access specifier of a base sets the access of the base's public and
protected members in the derived class:

| Base access | Public member becomes | Protected member becomes |
|---|---|---|
| `public` | public | protected |
| `protected` | protected | protected |
| `private` | private | private |

If the base has no access specifier, a `class` makes the base private and
a `struct` makes the base public. Private members of a base are not
accessible in the derived class.
:::

:::{rule} ACC-12
:kind: reject
:tests: classes/access/private-base-conversion.cpp

A conversion from a derived class to one of its bases needs access to
that base. This applies to pointers, references, and objects. In code
that cannot access a private or protected base, the conversion is not
valid.
:::

:::{rule} ACC-9
:kind: required
:tests: classes/access/using-changes-access.cpp classes/access/using-reduces-access.cpp

A using-declaration *(referenced)* in a derived class gives a base member
the access of the label where the declaration occurs. This can increase
or decrease the access.
:::

:::{rule} ACC-13
:kind: reject
:tests: classes/access/using-private-base-member.cpp

A using-declaration must name a base member that is accessible in the
derived class.
:::

:::{rule} ACC-10
:kind: required
:tests: classes/access/nested-accesses-enclosing.cpp classes/access/enclosing-not-access-nested.cpp

A nested class is a member of its enclosing class. It has access to the
private and protected members of the enclosing class. The enclosing class
has no special access to the members of the nested class.
:::

## Examples

```cpp
class Account {
  long balance = 0;              // private by default
  friend class Auditor;          // Auditor can read balance
public:
  long get() const { return balance; }
};

struct Shape { protected: int sides = 0; };
struct Square : Shape {
  int mine() const { return sides; }               // valid
  int theirs(const Shape& s) const { return s.sides; } // rejected
};

class Stack : private Vector {   // Vector's members become private
public:
  using Vector::size;            // size is public again
};

class Token {
  int id = 0;
  friend bool same(Token a, Token b) { return a.id == b.id; }
};                               // same() is found only through Token arguments
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 37 of
38 tests.

- ACC-9: EDG 7.0 accepts a use of a member that a using-declaration makes
  private, if the member is public in the base ({question}`Q-0052`).

Inputs: clang-sema-access, gcc-cp-class, edg-class-decl.
