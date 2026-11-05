# Q-0322: How do tests reach the operations that only the implementation can provide?

**Status:** provisional
**Affects:** `foundations/language-support.md`, `foundations/storage.md` (STG-P4, STG-19), CPY-19, CLS-19, NEW-27, decision 0041
**Waits for:** the maintainer (pending.md row 2)

## Context

Decision 0041 lets a program declare the shape of a language-support
type in `namespace std`. Some operations have no shape that a program
can write:

| Operation | What a self-declared shape does on GCC 16 and Clang 22 |
|---|---|
| the launder operation (STG-19) | compiles, fails to link (W15-2) |
| `typeid` with `std::type_info` and its comparison | compiles, fails to link |
| a bit copy between types of equal size, in constant evaluation | no core construct does it |
| class properties as named queries (trivially copyable, standard-layout) | no core construct names them |
| the default new alignment as a value (NEW-27) | needs a predefined macro |

GCC 16 and Clang 22 accept a builtin form of each operation. EDG 7.0,
as the matrix runs it (`--c++23 --strict`), accepts some of them:

| Builtin form | EDG 7.0, `--c++23 --strict` |
|---|---|
| `__is_trivially_copyable`, `__is_trivially_destructible`, `__is_standard_layout` | accepts, with the same results as GCC and Clang |
| `__STDCPP_DEFAULT_NEW_ALIGNMENT__` | accepts |
| `__builtin_bit_cast` | rejects: the identifier is undefined |
| `__builtin_launder` | rejects: the identifier is undefined |
| a self-declared `std::type_info` | rejects: the declaration conflicts with a reserved class |

The two rejected builtins depend on EDG's emulation mode. Without
`--strict`, and with `--c++26`, EDG still rejects both. With `--g++`,
EDG accepts `__builtin_launder` and rejects `__builtin_bit_cast`. With
`--clang`, EDG accepts both. So option 2 works on the matrix for the
class property queries and the default new alignment, and does not
work for the bit copy, the launder operation, or `typeid`.

## Evidence

The roadmap round 3 reports (`reports/clean-room/roadmap-3/`): the
programmer view (item 3, sketches `rtti.cpp` and `bi.cpp`), the
implementer view (item 9), and the editor view (section 4). W15-2.

The EDG 7.0 results come from the round-4 programmer view
(`reports/clean-room/roadmap-4/programmer.md`, item 8) and from a
clean-room writer's probe on 2026-11-03 (phase 8 item R2), which gave
the same results in strict mode and added the other modes.

## Options

1. Keep the operations referenced. Tests use no standard header, and a
   program keeps the pointer that placement `new` returns (STG-P4).
2. A support header under `tests/` that declares each `std` name and
   defines it with a builtin that GCC, Clang, and EDG share. The rule
   text names only the `std` operation.
3. Allow `#include <new>` as the only standard header, for the launder
   operation and the placement forms. The list of allowed names is
   closed.

## Provisional answer

Keep them referenced; tests use no standard header (decision 0051) (option 1). Options 2 and 3 move the line between the language and the
library that the charter draws, so the maintainer decides. Writers may
use builtin queries in probes as Evidence for class properties
(decision 0051), and no test uses them.

Kept open on 2026-11-03 under decision 0054, because the review is silent, and the question waits on the maintainer (`documentation/decisions/pending.md`, row 2).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as silent. The question is about Tidal's test method, and the reference puts each of these operations in the library.
