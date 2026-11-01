# Q-0131: Can a declaration with a qualified name outside the namespace redeclare a friend function that only a class has declared?

**Status:** provisional
**Affects:** ACC-7, ACC-17, `tests/classes/access/hidden-friend-qualified-redeclaration.cpp`

## Context

A friend function first declared in a class is a member of the
enclosing namespace. Qualified lookup does not find the function until
the namespace declares the function (ACC-7). A declaration such as
`int geo::make_seven();` outside `geo` names the function with a
qualified name. The compilers decide this case in different ways:

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | accepts |
| GCC 14, GCC 16 | rejects ("has not been declared within"), with a note that the function is declared only as a friend |
| EDG 7.0 | rejects ("may not be redeclared in the current scope") |

All the compilers accept the same declaration inside `geo`, and all
find the function with `geo::make_seven` after that declaration.

## Evidence

`tests/classes/access/hidden-friend-qualified-redeclaration.cpp`. The
probe compiled the test with `-std=c++23 -pedantic-errors` (Clang,
GCC) and `--c++23 --strict` (EDG).

## Options

1. The declaration is rejected. A qualified declaration outside the
   namespace must refer to a member that qualified lookup finds, and
   qualified lookup does not find the friend. ACC-7 and ACC-17 need no
   extra case.
2. The declaration is valid. A qualified declaration also finds a
   friend that only a class declares. ACC-7 then needs a third place
   where the function is visible.

## Provisional answer

No; GCC and EDG reject, Clang accepts (option 1). GCC and EDG reject the declaration, and the answer follows
from the lookup rule of ACC-7 with no exception.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
