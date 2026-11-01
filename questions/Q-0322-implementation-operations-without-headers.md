# Q-0322: How do tests reach the operations that only the implementation can provide?

**Status:** provisional
**Affects:** `foundations/language-support.md`, `foundations/storage.md` (STG-P4, STG-19), CPY-19, CLS-19, NEW-27, decision 0041

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

GCC and Clang both accept builtin forms of these operations. EDG has not
been probed.

## Evidence

The roadmap round 3 reports (`reports/clean-room/roadmap-3/`): the
programmer view (item 3, sketches `rtti.cpp` and `bi.cpp`), the
implementer view (item 9), and the editor view (section 4). W15-2.

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
