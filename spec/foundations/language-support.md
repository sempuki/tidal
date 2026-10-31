# Language-support types

Some core rules name types that the standard library defines. This page
gives what the language requires of each such type, and nothing else
about the library (decision 0041):

| Type | Core rules that use it |
|---|---|
| `std::initializer_list` | {rule}`ILC-1`, {rule}`AUTO-7`, {rule}`RFOR-1` |
| `std::strong_ordering`, `std::weak_ordering`, `std::partial_ordering` | {rule}`CMP-17`, {rule}`DCMP-4` |
| `std::tuple_size`, `std::tuple_element` | {rule}`SB-5` |
| `std::size_t`, `std::ptrdiff_t`, `std::nullptr_t` | {rule}`TYP-1`, {rule}`ADD-12`, {rule}`CONV-8` |

The exception that a failed allocation throws stays with the library
*(library referenced)*.

Depends on: fundamental types; list initialization of classes;
comparison operators; structured bindings.

## Principles

:::{principle} LSUP-P1
:rules: LSUP-1 LSUP-2 LSUP-3 LSUP-4

A program normally gets these types from the library. The language
needs only their shape, so a program that includes no header can
declare each type itself, as Tidal's tests do. The implementation
checks the shape where it uses the type, because it builds objects of
the type itself: an `std::initializer_list` for a braced list, and an
ordering value for `<=>` on built-in operands.
:::

## Constraints

:::{rule} LSUP-1
:kind: reject
:tests: foundations/language-support/initializer-list-pointer-and-size.cpp foundations/language-support/initializer-list-no-members.cpp foundations/language-support/initializer-list-not-template.cpp foundations/language-support/initializer-list-missing.cpp

A program that needs an `std::initializer_list<E>` must declare
`std::initializer_list` first, with this shape:

- a class template with one type parameter `E`,
- two non-static data members, a pointer to `const E` and a count of
  type `std::size_t`,
- a constructor that takes the pointer and the count.
:::

:::{rule} LSUP-2
:kind: reject
:tests: foundations/language-support/strong-ordering-minimal.cpp foundations/language-support/strong-ordering-missing-member.cpp foundations/language-support/strong-ordering-missing.cpp

A program that uses `<=>` on built-in operands, or a defaulted `<=>`,
must declare the three ordering classes in namespace `std` first. Each
class has static data members of its own type, defined as constants:

| Class | Members |
|---|---|
| `strong_ordering` | `less`, `equal`, `equivalent`, `greater` |
| `weak_ordering` | `less`, `equivalent`, `greater` |
| `partial_ordering` | `less`, `equivalent`, `greater`, `unordered` |
:::

:::{rule} LSUP-3
:kind: required
:tests: foundations/language-support/tuple-protocol-minimal.cpp

A class uses the tuple protocol of {rule}`SB-5` when the program
declares these templates in namespace `std` and specializes them for
the class:

| Template | Specialization gives |
|---|---|
| `tuple_size<E>` | a constant member `value`, the number of elements |
| `tuple_element<I, E>` | a member type `type`, the type of element `I` |
:::

:::{rule} LSUP-4
:kind: required
:tests: foundations/language-support/size-and-null-types.cpp

The library names three core types. A program can declare each name as
an alias of the core type:

| Name | Core type |
|---|---|
| `std::size_t` | the type of `sizeof` ({rule}`TYP-1`) |
| `std::ptrdiff_t` | the type of a pointer difference ({rule}`ADD-12`) |
| `std::nullptr_t` | the type of `nullptr`, `decltype(nullptr)` |
:::

## Examples

```cpp
namespace std {
template <class E> class initializer_list {
  const E* first;
  decltype(sizeof 0) count;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n)
      : first(f), count(n) {}
public:
  constexpr const E* begin() const { return first; }
  constexpr const E* end() const { return first + count; }
};
}
auto values = {1, 2, 3};           // std::initializer_list<int>
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on all
9 tests.

- LSUP-1: each compiler checks the shape in its own way. Clang checks
  the layout of the two members, and also accepts a second pointer in
  place of the count. GCC compares the class with the one in its own
  header and rejects the two-pointer form. EDG looks for the
  constructor. The rule gives the shape that all three accept.
- LSUP-2: EDG needs all three classes declared before it accepts `<=>`
  on any built-in operand. Clang and GCC need only the class that the
  operation yields.
