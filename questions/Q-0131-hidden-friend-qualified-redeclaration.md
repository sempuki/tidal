# Q-0131: Can a declaration with a qualified name outside the namespace redeclare a friend function that only a class has declared?

**Status:** provisional
**Affects:** ACC-7, ACC-17, `tests/classes/access/hidden-friend-qualified-redeclaration.cpp`
**Waits for:** the maintainer (pending.md row 4)

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

Kept on 2026-11-03. The review reads option 2, which only Clang follows. GCC 16 and EDG 7.0 reject the declaration, and ACC-P3 gives the answer: the friend function is found only through its arguments until the namespace declares it. Tidal keeps option 1 until the maintainer decides. The question is in row 4 of `documentation/decisions/pending.md`, and it stays open under decision 0054, because the review reads against the answer.

Kept open on 2026-11-04 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) reads option 2. A declaration with a qualified name does not look up its last name. It must match an earlier declaration that is nominable in the named namespace, and a friend function first declared in a class targets the enclosing namespace and counts as nominable there. So the declaration at global scope matches the friend and is valid. The lookup rule of ACC-7 does not decide the case, because the declarator is matched without lookup. Clang follows this reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
