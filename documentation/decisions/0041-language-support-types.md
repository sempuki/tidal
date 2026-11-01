# 0041. Language-support types and headers

Date: 2026-10-31
Status: Accepted. Changes the charter.

## Context

Some core rules name library types:

| Type | Used by |
|---|---|
| `std::initializer_list` | braced lists for classes, `auto` with braces, range-based `for` |
| `std::strong_ordering`, `std::weak_ordering`, `std::partial_ordering` | `<=>` and defaulted comparisons |
| `std::tuple_size`, `std::tuple_element`, `get` | structured bindings |
| `std::nullptr_t`, `std::size_t`, `std::ptrdiff_t` | `nullptr`, `sizeof`, pointer subtraction |
| the exception that `new` throws | `new` expressions |

The charter puts the library out of scope, so these types are marked
*(library referenced)*. 72 tests declare minimal versions of them in
`namespace std` and include no header. A reader of Tidal cannot tell
which parts of these types the language needs.

## Decision

- A new Foundations page, "Language-support types", states what the
  language requires of each type in the table: its kind, the members
  the language uses, and their values. It specifies nothing else about
  the library.
- Tests and clean-room exercises include no headers. A program that
  needs one of these types declares the shape that the page gives, in
  `namespace std`. The page's tests check that every matrix compiler
  accepts those declarations and uses them as the core rules say.
- The charter lists the page as the one part of the library in scope.

## Consequences

- *(library referenced)* markers that name these types become links to
  the page. Markers for other library facilities stay.
- Each compiler's own checks on user-declared `std` types, such as
  Clang's checks on the comparison categories, are evidence for the
  page.
