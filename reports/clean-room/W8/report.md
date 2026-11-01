# W8 Write exercise report: ownership

- Date: 2026-11-01
- Role: clean-room reader
- Persona: Java programmer
- Suite: W8, ownership
- Programs: `unique.cpp`, `shared.cpp`, `tagged.cpp`, `vector.cpp` in this directory

The reader read AGENTS.md, section 3 and the "Write and Port" prompt of documentation/exercises.md, and these Tidal pages: index, glossary, foundations/translation, foundations/language-support, classes/copy-move, classes/constructors, classes/unions, expressions/new-delete, exceptions/unwinding, foundations/storage, and parts of exceptions/handlers, exceptions/throw, classes/operators and declarations/exception-specifications. The reader read nothing outside specification/ and documentation/, and ran no git command.

## Summary

All four programs build without warnings on g++ 16.2.1 and clang++ 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`. All four exit 0, including under `-fsanitize=undefined,address -O0`. The three biggest problems:

1. **Raw storage for a class with a constructor (W8-3, W8-4).** The spec does not say whether `static_cast<T*>(::operator new(n * sizeof(T))) + i` is valid when `T` is not an implicit-lifetime type. STG-8 and STG-P8 suggest the result points to an implicit array and not to its element 0. Every growable container relies on this pattern. Tidal's interpreter stops on the pattern with the wrong rule: it reports STG-17 (misalignment) for an address that is aligned.
2. **Moving a named object without a header (W8-1).** `std::move` is marked library referenced. No rule tells a header-free program to write `static_cast<T&&>(x)`. I found that form only in examples on other pages.
3. **The placement form is hard to find (W8-5).** STG-5 says placement `new` uses "the library's" non-allocating form, marked library referenced. LSUP-7 says a header-free program defines the form itself. STG-5, NEW-10 and UNI-14 do not link to LSUP-7.

## Rule and principle IDs noted before writing

- **Unique owner:** CPY-P2 (one user-declared function removes the others), CPY-1, CPY-2, CPY-3, CPY-4, CPY-5, CPY-21 (deleted copy), CPY-9 (lvalue copies, rvalue moves), CPY-10 (prvalue, no move), CPY-11 and CPY-12 (return of a local), NEW-1, NEW-4 (delete of null), NEW-5 (double delete), NEW-9, DTOR-5, operators page (`operator->`), conversion-functions example (`explicit operator bool`), packs page (`static_cast<Args&&>`).
- **Shared owner:** CPY-P1, CPY-P2, CPY-1 to CPY-3, CPY-9, CPY-11 (a parameter is moved to the return value), NEW-1, NEW-4, NEW-9, DTOR-5.
- **Tagged union:** UNI-P1, UNI-P2, UNI-3 (deleted special members, also for a class with an anonymous union), UNI-4, UNI-5, UNI-6, UNI-7, UNI-11, UNI-14 (placement `new` makes a member active, an explicit destructor call empties the union), LSUP-4, LSUP-7, STG-5, NEW-10, NEW-20, DTOR-6.
- **Exception-safe container:** NEW-19 (global functions callable by name), NEW-38, STG-P2, STG-P3, STG-5, STG-8, STG-17, LSUP-4, LSUP-7, DTOR-6 (a destructor call keeps the storage; scalars through a type name), UNW-P2, UNW-1, UNW-2, UNW-3, TRY-P4, TRY-8 (`throw;`), CPY-9.

## Findings

### W8-1
- **Pages and rules:** classes/copy-move (CPY-P4, CPY-9, CPY-13), glossary "value category", foundations/language-support.
- **Kind:** hard to find, needs example.
- **What happened:** CPY-P4 says an rvalue source is moved. It does not say how to turn a named object into an rvalue. CPY-13 marks `std::move` as library referenced, and language-support does not list it. I found `static_cast<T&&>(x)` only in examples: the copy-move example (`static_cast<Point&&>(p)`), the packs page, references.md and values.md. A Java programmer looks for a "move" operation and finds only the library name.
- **Suggestion:** Add a sentence to CPY-P4 or to the language-support page. It should say that a program with no header writes `static_cast<T&&>(x)` to get an xvalue that selects the move function, and link to the xvalue rule on values.md.

### W8-2
- **Pages and rules:** classes/copy-move (CPY-13), foundations/translation (the warning table).
- **Kind:** missing.
- **What happened:** In unique.cpp, `c = static_cast<Unique<Widget>&&>(c);` makes clang++ 22 warn under `-Wall` with `-Wself-move`. g++ 16 gives no warning with `-Wall -Wextra`. CPY-13 says an implementation need not warn about a cast that the program writes, such as `static_cast`. I read that as "a `static_cast` gives no warnings", so the warning surprised me. I fixed the program by moving through a reference (`Unique<Widget>& same = c;`).
- **Suggestion:** In CPY-13's Evidence, record that Clang warns about a self-move through `static_cast` under `-Wall` (`-Wself-move`) and that GCC does not. State that a move assignment the class writes must handle a source that is the object itself.

### W8-3
- **Pages and rules:** foundations/storage (STG-P3, STG-8, STG-P8, STG-5), expressions/additive (ADD-10), expressions/new-delete (NEW-19).
- **Kind:** unclear.
- **What happened:** vector.cpp gets storage with `::operator new(sizeof(T) * n)`, casts the result to `T*`, and builds elements with `new (data + i) T(...)`. `Item` has a user-provided constructor, so it is not an implicit-lifetime type.
  - STG-8 says the allocation creates implicit-lifetime objects (an `Item[n]` array qualifies) and that "its subobjects of other types are not created". It also says the result "points to one of them", that is, to the array.
  - STG-P8 says an array and its element 0 are not pointer-interconvertible.
  - So the spec does not say whether `static_cast<Item*>(raw)` points to element 0 of an array. That decides whether `+ i` is valid under ADD-10.
  - Both compilers and both sanitizers accept the program.
- **Suggestion:** Add a storage-page example with raw storage for `n` objects of a class that has a constructor, with placement `new` at `p + i`. State which rule makes `p + i` valid, or state that it is not valid and give the form that is.

### W8-4
- **Pages and rules:** foundations/storage (STG-17). This is a front-end result.
- **Kind:** wrong (front-end stop).
- **What happened:** `tidal-fe run vector.cpp` stops with `undefined [STG-17] at line 67: placement new at an address that is not aligned for the type`. The address is `data + count` inside storage from `::operator new`. NEW-P5 and NEW-27 give that storage an alignment of 16, and `Item` needs 4, so the address is aligned. This minimal program reproduces the stop at line 7, the second placement:

  ```cpp
  namespace std { using size_t = decltype(sizeof 0); }
  inline void* operator new(std::size_t, void* at) noexcept { return at; }
  struct Item { int v; explicit Item(int x) : v(x) {} };
  int main() {
    Item* p = static_cast<Item*>(::operator new(sizeof(Item) * 2));
    new (p) Item(1);
    new (p + 1) Item(2);
    int r = p[1].v == 2 ? 0 : 1;
    p[0].~Item(); p[1].~Item();
    ::operator delete(p);
    return r;
  }
  ```

  Both compilers run the program and exit 0.
- **Suggestion:** The front end should not report misalignment for an aligned address. If the front end judges `p + 1` invalid, the stop should name the pointer-arithmetic or implicit-creation rule (W8-3). If the arithmetic is valid, the program should run. Settle W8-3 first.

### W8-5
- **Pages and rules:** foundations/storage (STG-5), expressions/new-delete (NEW-10), classes/unions (UNI-14), foundations/language-support (LSUP-7).
- **Kind:** hard to find.
- **What happened:** For the tagged union, I first looked for placement `new` on the unions page. UNI-14 names placement `new` and STG-5. STG-5 says "with the library's non-allocating placement form *(library referenced)*", which reads as "needs a header". The unions example uses `std::construct_at` instead. I found the form to define (`inline void* operator new(std::size_t, void*) noexcept`) only on the language-support page (LSUP-7).
- **Suggestion:** In STG-5, NEW-10 and UNI-14, link to LSUP-7 and say that a program with no header defines the form as LSUP-7 gives. Remove the library-referenced marker from STG-5.

### W8-6
- **Pages and rules:** expressions/new-delete (NEW-4, NEW-19).
- **Kind:** missing, outside knowledge.
- **What happened:** The Vector destructor calls `::operator delete(data)`, and `data` is null for a vector that never allocated. NEW-4 covers only `delete` expressions. NEW-19 says a program can call the global functions by name, but it does not say what `operator delete` does with a null pointer. I relied on outside knowledge that the call does nothing.
- **Suggestion:** Add to NEW-19, or as a separate rule, what a direct call of the global deallocation functions does with a null pointer.

### W8-7
- **Pages and rules:** exceptions/unwinding (UNW-P2, UNW-2, UNW-3), exceptions/handlers (TRY-8).
- **Kind:** needs example, outside knowledge.
- **What happened:** UNW-2 says what the language destroys when an array that the language builds throws partway. The page does not say that objects a program builds one at a time with placement `new` get no such cleanup. The program's own handler must destroy them, free the storage and rethrow. My first draft forgot to destroy the new element when a later copy threw. I caught that in review before building. Two design choices came from general practice and not from the spec: "build into new storage, then commit" for growth, and "construct the new element first, because the argument can be an element of the same container".
- **Suggestion:** Add an example to the unwinding page: a loop of placement `new` into raw storage, inside `try { } catch (...) { destroy built ones; free; throw; }`. Contrast it with UNW-2's automatic cleanup for `new T[n]`.

### W8-8
- **Pages and rules:** classes/copy-move (CPY-P2, the Owner example).
- **Kind:** needs example.
- **What happened:** In shared.cpp, my copy assignment incremented `other.block->count`, then called `drop()` (which sets `block = nullptr`), then assigned `block = other.block`. For a self-assignment, `other` is `*this`, so the object lost its block. Both compilers gave exit 6, and the Clang sanitized build also reported a leak. The Owner example on copy-move shows the `this != &o` guard for "delete then copy" only.
- **Suggestion:** Next to the Owner example, add one sentence: the parameter can refer to `*this`, so a read through it after a change to `*this` sees the changed value. Show the "save the pointer first" form for a counted owner.

### W8-9
- **Pages and rules:** declarations/exception-specifications (its example), classes/copy-move.
- **Kind:** outside knowledge.
- **What happened:** I marked every move constructor and move assignment `noexcept` because I know that library containers prefer moves that cannot throw. The example on the exception-specifications page shows `Buffer(Buffer&&) noexcept; // moves without throwing`, but no page says why a move should be `noexcept`. My Vector copies elements during growth to stay out of that question.
- **Suggestion:** None needed for the core spec, since the preference belongs to the library. A note on copy-move could say that the core language does not care whether a move is `noexcept`, and that only code which asks with the `noexcept` operator does.

## Attempt log

1. **unique.cpp, attempt 1:**
   - g++ and clang++ exit 16, also when sanitized.
   - Cause: my own count error. The program makes three Widgets, and I expected 4 destroyed.
   - clang++ also warned `-Wself-move` on the self-move line (W8-2).
   - Fix: expect 3, and do the self-move through a reference.
2. **unique.cpp, attempt 2:** all four builds compile with no warning and exit 0.
3. **shared.cpp, attempt 1:**
   - g++ and clang++ exit 6, also when sanitized.
   - The clang++ sanitized build reports a leak of 20 bytes (Block and Node).
   - Cause: self copy-assignment, where `drop()` cleared the pointer I then read through `other` (W8-8).
   - Fix: copy `other.block` into a local before changing `*this`.
4. **shared.cpp, attempt 2:** all four builds compile with no warning and exit 0.
5. **tagged.cpp, attempt 1:** all four builds compile with no warning and exit 0.
6. **vector.cpp, before the first build:** review found two bugs, and I fixed both before compiling.
   - The handler did not destroy the new element when a later copy threw.
   - One test threw in the constructor of a local before `push_back` ran.
7. **vector.cpp, attempt 1:** all four builds compile with no warning and exit 0.

## Interpreter results (`tidal-fe run`)

| Program | Result | Classification |
|---|---|---|
| unique.cpp | `unsupported at line 63: overloaded operator->`, exit 127 | front-end item |
| unique.cpp with `->` replaced by `(*x).` (probe) | `unsupported at line 69: class to bool`, exit 127 | front-end item |
| shared.cpp | `unsupported at line 80: class assignment`, exit 127 | front-end item |
| tagged.cpp | `unsupported at line 12: class layout the front end does not model`, exit 127 | front-end item |
| vector.cpp | `undefined [STG-17] at line 67: placement new at an address that is not aligned for the type`, exit 126 | wrong stop, finding W8-4 |
| probe/align2.cpp (one placement at `p`) | `unsupported at line 7: bytes of storage that holds an object of another type`, exit 127 | front-end item |

In tagged.cpp, line 12 is `int texts_alive = 0;`, a global `int`. The construct the stop refers to is probably the class with the anonymous union further down, so the line number looks wrong. I record this as a note and not as a finding.

## What worked well

- The CPY-P2 table showed which functions a resource-owning class must write. Its last paragraph warns that a user-declared destructor keeps the implicit copy and so causes a double delete. Unique and Shared compiled the first time.
- The CPY-P4 table and CPY-11 answered how a return of a local and a return of a parameter behave. `pass_through` in shared.cpp gave the expected counts.
- UNI-P2, UNI-3 (including the sentence on a class with an anonymous union member), UNI-5 and UNI-14 explain why the tagged union must write its own special members. They also explain how placement `new` and an explicit destructor call switch the active member. tagged.cpp worked on the first build.
- The FixedStack example on the unions page and the Maybe example on the storage page were the closest models for my code. The note after Maybe, about writing its own copy functions, was useful.
- DTOR-6's sentence that a destructor call works on a scalar through a type name told me `p[n].~T()` is fine in a template.
- LSUP-7's shape for the placement form compiled on both compilers with no header.
- UNW-P2's table made it clear which cleanup the language does and which the program must do.
