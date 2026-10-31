# Access control and friends

Access control limits which code can use the name of a class member. A
member is public, protected, or private. A friend of a class has the same
access as the members of the class.

## Principles

:::{principle} ACC-P1
:rules: ACC-1 ACC-2 ACC-3 ACC-11 ACC-16

Access control is about names, not objects. Each member name carries an
access, set by the class key or by the last label before it. A use of
the name is checked where the use occurs, after lookup and overload
resolution have chosen the declaration, so an accessible overload that
loses never rescues a call. Only names are checked: a deduced type, the
final overrider of a virtual call, and the declarator of a member defined
outside its class carry no name to check. A default argument is checked
where the function is declared, because that is where its names are
written.
:::

:::{principle} ACC-P2
:rules: ACC-4

A protected member is part of the interface that a class offers to its
derived classes, and to nothing else. A derived class can use it only
through its own type, so that one derived class cannot reach into the
protected parts of another.
:::

:::{principle} ACC-P3
:rules: ACC-5 ACC-6 ACC-7 ACC-15

Friendship is granted by the class and belongs to the one it names. It
does not pass on to the friends of a friend or to the classes derived
from a friend. A class can befriend a function, one member function of
another class, or a whole class, and needs access to a member function
before it can name it. A friend function first declared in the class
lives in the enclosing namespace but is found only through its
arguments, so it stays with the class it serves.
:::

:::{principle} ACC-P4
:rules: ACC-8 ACC-9 ACC-10 ACC-12 ACC-13 ACC-14

A derived class sees its base's public and protected members, and the
access of the base caps what the derived class passes on. A conversion
to a base needs access to that base. A using-declaration can raise or
lower the access of a base member it can reach. The injected name of a
private base is private too. A nested class is a member, with a member's
access to the enclosing class, while the enclosing class gets no special
access in return.
:::

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

## Constraints

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
:tests: classes/access/access-after-overload-resolution.cpp classes/access/private-hides-public-base.cpp

The implementation checks access after overload resolution
*(referenced)* selects a function. If the selected function is not
accessible, the call is not valid. An accessible overload that is a worse
match does not make the call valid. Access does not change name lookup,
so a private member of a derived class hides a public member of a base
with the same name.
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
:tests: classes/access/friend-function.cpp classes/access/friend-class.cpp classes/access/friend-member-function.cpp classes/access/friend-member-function-only.cpp classes/access/friend-ignores-label.cpp classes/access/friend-plain-type-name.cpp

A friend declaration gives a function or a class the access of a member.
A friend can be:

- a function,
- a member function of another class, and only that function,
- a class, and all its member functions.

A friend class can be named with its class key, or by a type name or type
alias of a class that is already declared. A friend declaration that
names a type that is not a class is ignored. An access label has no
effect on a friend declaration.
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
:tests: classes/access/private-base.cpp classes/access/struct-base-default-public.cpp classes/access/protected-base-in-derived.cpp classes/access/private-base-name-in-derived.cpp classes/access/access-best-path.cpp

The access specifier of a base sets the access of the base's public and
protected members in the derived class:

| Base access | Public member becomes | Protected member becomes |
|---|---|---|
| `public` | public | protected |
| `protected` | protected | protected |
| `private` | private | private |

If the base has no access specifier, a `class` makes the base private and
a `struct` makes the base public. Private members of a base are not
accessible in the derived class. If a member can be reached through
several paths, the path that gives the most access applies.
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
:tests: classes/access/nested-accesses-enclosing.cpp classes/access/enclosing-not-access-nested.cpp classes/access/local-class-member-access.cpp

A nested class is a member of its enclosing class. It has access to the
private and protected members of the enclosing class. The enclosing class
has no special access to the members of the nested class. A local class
in a member function has the access of that member function.
:::

:::{rule} ACC-14
:kind: reject
:tests: classes/access/injected-name-private-base.cpp classes/access/injected-name-qualified.cpp

The name of a class is also a member of the class. In a class derived
from a class with a private base, the unqualified name of that base
refers to that member, which is private. A qualified name, such as
`::A`, names the class through its namespace and is accessible.
:::

:::{rule} ACC-15
:kind: reject
:tests: classes/access/friend-private-member-of-other.cpp

A friend declaration that names a member function of another class needs
access to that member function.
:::

:::{rule} ACC-16
:kind: required
:tests: classes/access/default-argument-private.cpp

Access for names in a default argument is checked at the declaration of
the function, not at the call. A caller outside the class can use a
default argument that names a private member.
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 45 of 46 tests.

- ACC-9: EDG 7.0 accepts a use of a member that a using-declaration makes
  private, if the member is public in the base ({question}`Q-0052`).

Inputs: clang-sema-access, edg-class-decl.
