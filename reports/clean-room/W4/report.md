# W4 report: pointers and memory

- Date: 2026-11-01
- Role: clean-room reader
- Persona: Java programmer
- Programs: `list.cpp`, `grid.cpp`, `arena.cpp`, `vector.cpp` in this directory

All four programs build with g++ 16.2.1 and clang++ 22.1.8 under `-std=c++23 -pedantic-errors -Wall -Wextra`. All four exit 0, with no warnings, both plain and under `-fsanitize=undefined,address -O0`. Two compile errors on the way were my own mistakes, and the specification covered both. The interpreter gives 0 for two programs and stops with "unsupported" on the other two. It gave no wrong result.

## Summary

The three biggest problems:

1. **Growable array of class objects (W4-1).** The specification does not say clearly how to point into allocated storage for elements that are not built yet. The only example uses `int`, whose elements exist with the array.
2. **Reusing arena storage for plain types (W4-4).** A byte array creates implicit objects only when its lifetime starts. No page says what happens when an arena gives out reused bytes as `int` storage. I changed my design because of a guess.
3. **Freeing storage by name (W4-2).** No rule says a call of `operator delete` by name must match the `operator new` form that gave the storage. AddressSanitizer reports a mismatch that no rule names.

## Findings

**W4-1**
- Pages and rules: storage (STG-7, STG-8, STG-10, STG-16, the `static_cast<int*>(raw)` example), additive (ADD-6, ADD-14).
- Kind: unclear.
- What happened: the vector gets storage from `::operator new` and uses `static_cast<T*>(raw)`. Then it computes `fresh + i` before any element exists. I chose this from the `int[4]` example in the storage page. That example works for `int` because scalar elements exist with the array. My `Item` class has a user-provided destructor, so it is not an implicit-lifetime type. STG-7 makes the array of `Item` an implicit-lifetime type, but STG-8 says its elements are not created. STG-16 says a cast between an array and its first element keeps the original object. ADD-14 says arithmetic through a pointer of another type treats the object as an array of one. So a strict reading makes `fresh + 1` undefined. STG-10 does not list pointer arithmetic among the uses allowed before a lifetime begins. Both compilers and both sanitizers accept the program.
- Suggestion: state where the result of an allocation function points when the implicit object is an array, and whether arithmetic over element positions is valid before the elements are built. Add a small growable array of a class type as an example. That is the most common reason to use raw storage.

**W4-2**
- Pages and rules: new and delete (NEW-19, NEW-26, NEW-5), language support (LSUP-5).
- Kind: missing.
- What happened: the vector allocates with `::operator new(size, std::align_val_t(a))` and frees with the aligned `::operator delete`. I chose the pair because it looked symmetric. No rule told me. A probe that frees aligned storage with the plain `::operator delete(p)` gets "new-delete-type-mismatch" from AddressSanitizer on both compilers. NEW-5 covers only `delete` expressions.
- Suggestion: add an undefined rule for calls by name. The deallocation function must be the partner of the allocation function: aligned with aligned, with the same alignment value, and with the same size if it is a sized form.

**W4-3**
- Pages and rules: language support (LSUP-7, examples), new and delete (NEW-20, NEW-33), storage (STG-5).
- Kind: needs example.
- What happened: I declared only `operator new(std::size_t, void*)`, as the language-support example does. Then `new (place) T[n]()` failed on both compilers ("no matching function for call to operator new[](sizetype, void*&)"). The interpreter rejected it under NEW-20. The LSUP-7 table lists `operator new[]`, but none of the three examples shows it.
- Suggestion: show all four non-allocating forms (`new`, `new[]`, `delete`, `delete[]`) in one example. Add a note that the array form of placement `new` needs `operator new[]`.

**W4-4**
- Pages and rules: storage (STG-P3, STG-6, STG-8, STG-18), objects (OBJ-2).
- Kind: unclear.
- What happened: my first arena had `raw_array<T>(n)`. It returned `static_cast<T*>(bytes)` and let the caller write `int` values, relying on objects created implicitly in the byte array. STG-8 creates such objects only when the byte array's lifetime starts. After `reset()`, a `Point` and its cleanup record had taken those bytes (STG-6), so no `int` objects remain there. I could not find which rule makes a later write through `int*` into those bytes undefined. OBJ-2 talks about objects outside their lifetime, and here no object exists at all. I rewrote the function as `make_array<T>(n)`, which uses `new (place) T[n]()`.
- Suggestion: say in STG-8 or STG-P3 that bytes reused by an explicit object do not get implicit objects back. Name the rule for an access to storage that holds no object of the type. Show an arena that resets as an example.

**W4-5**
- Pages and rules: casts (CAST-6, CAST-7, CAST-16), language support (LSUP-4), unary (UN-15).
- Kind: missing.
- What happened: to check alignment I convert a pointer to an integer. The specification names no integer type that is large enough to hold a pointer. I used `std::size_t`, because UN-15 says `sizeof(void*)` and `sizeof(sizeof 0)` are both 8 on x86-64. The idea that a pointer-sized integer type exists in the library is outside knowledge.
- Suggestion: state in CAST-16 which built-in integer types hold a pointer on the matrix targets. Or list the library's pointer-sized integer types on the language-support page as library referenced.

**W4-6**
- Pages and rules: new and delete (NEW-1, NEW-2, examples), initialization forms, copy and move (the `Owner` example).
- Kind: needs example.
- What happened: the grid uses `new int[r * c]()` to get zeroed cells. NEW-2 and the new-delete examples show `new int()`, which gives 0, and `new int[3]{1, 2}`, which gives 1, 2, 0. They do not show `()` after an array bound. I found the `()` form only in the copy-move page's `Owner` example, with no rule cited. The program behaves as expected.
- Suggestion: add a line to NEW-2 that says `new T[n]()` value-initializes each element. Add the form to the examples.

**W4-7**
- Pages and rules: new and delete (NEW-P2, NEW-5), copy and move (CPY-8, the `Owner` example), storage (the note after the `Maybe` example).
- Kind: hard to find.
- What happened: a Java programmer does not expect that copying a `List` or a `Grid` copies the raw pointer, so that both copies delete the same nodes. The warning sits in the copy-move page and in a note on the storage page. The new-delete page, which is where I looked, does not link to either. I made the copy functions `= delete` from outside knowledge.
- Suggestion: add a sentence to NEW-P2 that says a class that owns a `new` result must write or delete its copy functions. Link it to CPY-8 and the `Owner` example.

**W4-8**
- Pages and rules: new and delete (NEW-11), storage (STG-5).
- Kind: hard to find.
- What happened: `new (place) T[n]()` with `T = Tracked` failed on both compilers, because `Tracked` had only an `explicit Tracked(int)` constructor. NEW-11 states the rule clearly. STG-5, which gives the array form of placement `new`, does not point to it. I added a default constructor.
- Suggestion: add a cross-reference from STG-5's array form to NEW-11.

**W4-9**
- Pages and rules: constructors (DTOR-6, DTOR-8).
- Kind: needs example.
- What happened: the arena's `destroy_all<T>` calls `first[n].~T()` with `T = char` and `T = int`. DTOR-6 says a destructor call on a scalar works "through a type name". It does not say whether a template parameter counts. Both compilers accept the call.
- Suggestion: add an example of `p->~T()` in a template that is instantiated with a scalar type.

**W4-10**
- Pages and rules: functions (DEL-1, DEL-2), range-for (the member `begin` and `end` rule), alignment (ALN-P1).
- Kind: outside knowledge.
- What happened: I used three things that the W4 target pages did not give me. These are `= delete` for copy functions, range-for over a class with member `begin()` and `end()`, and the round-up formula `(x + a - 1) & ~(a - 1)`. The first two exist on pages outside W4, and I checked them afterwards. ALN-P1 says alignments are powers of two, which makes the formula valid, but no page shows how to round an offset up.
- Suggestion: add a short note under ALN-P1 that a power-of-two alignment allows rounding with a mask, with a one-line example.

## Attempt log

1. Read AGENTS.md, exercises.md (section 3 and "Write and Port"), then the specification pages: index, glossary, translation, language support, objects, alignment, storage, new and delete, declarators, postfix, additive, casts, plus parts of unary, constructors, copy and move, and range-for.
2. Noted the rule IDs for each program (they are in each file's header comment) and wrote all four programs.
3. First build: all four built and exited 0 on g++ and clang++, plain and with sanitizers.
4. Reviewed `Arena::raw_array` against STG-8 and judged it unsafe after `reset()` (W4-4). Replaced it with `make_array`, which uses `new (place) T[n]()`, and added cleanup records for arrays.
5. Both compilers rejected `new (place) T[n]()` because no `operator new[]` placement form was declared. The interpreter rejected it under NEW-20. Cause: I copied the LSUP-7 example, which shows only the single-object form (W4-3). Fix: declared `operator new[]` and `operator delete[]`.
6. Both compilers rejected `make_array<Tracked>` because `Tracked` had no default constructor. Cause: NEW-11, which I had not connected to placement arrays (W4-8). Fix: added `Tracked()`.
7. Rebuilt the arena. Exit 0 on all four builds.
8. Probes:
   - probe1: placement `new` into a standalone byte array.
   - probe2: placement `new` into a byte array that is a class member.
   - probe3: aligned allocation freed with the unaligned form. AddressSanitizer reports new-delete-type-mismatch on both compilers (W4-2).
   - probe4: read through a pointer to a deleted node.
   - probe5: pointer arithmetic on a `new[]` array.
   - arena-external: the arena with its buffer outside the class, for the interpreter only.

## Interpreter results

`tidal-fe run FILE`:

| File | Result | Assessment |
|---|---|---|
| list.cpp | `result: 0` | correct |
| grid.cpp | `unspecified: result 0 with fixed choices: SEQ-8 at line 62 (the left operand of '+' first); SEQ-8 at line 66 (... '!=' ...); SEQ-8 at line 66 (... '-' ...); SEQ-8 at line 67 (... '!=' ...)` | correct. The noted operand orders are function calls with no side effects, so the result does not depend on them |
| arena.cpp | `unsupported at line 55: bytes of storage that holds an object of another type` | front-end item |
| vector.cpp | `unsupported at line 21: call of an allocation function` | front-end item |
| arena-external | runs up to `unsupported at line 108: CE-4: reinterpret_cast` | front-end item |
| probe1 | `result: 0` | correct |
| probe2 | `unsupported at line 8: bytes of storage that holds an object of another type` | front-end item |
| probe4 | `undefined [OBJ-2] at line 6: indirection through a pointer to an object whose lifetime has ended` | correct stop |
| probe5 | `result: 0` | correct |
| arena.cpp, after step 5 only | `rejected [NEW-20] at line 86: no declared 'operator new[]' takes 1 placement argument` | correct, matches both compilers |

Notes for the front end:
- Line 55 of arena.cpp reads the member `used`, which does not overlap the buffer. The stop happens once the class's member byte array hosts an object. probe2 reproduces it in eight lines. A standalone byte array works (probe1).
- The `reinterpret_cast` stop cites CE-4, a constant-evaluation rule, during a run.

## What worked well

- The storage page's `Maybe` example gave me the whole raw-storage pattern: keep the pointer that placement `new` returns, end the object with `p->~T()`, and forward arguments with `static_cast<Args&&>`. With no `std::move`, the LSUP-10 shape showed the core form of a move.
- LSUP-7 and LSUP-5 let me write programs with no `#include`. The aligned `::operator new` (NEW-26) worked on both compilers, with a 64-byte aligned element.
- STG-P2's diagram (storage lives on, objects come and go) matched the arena's reset model exactly.
- NEW-P2's list of how `delete` must match `new`, ADD-P6's diagram of valid positions, and the ADD-10 note that each row of a 2D array is its own array made the list and the grid straightforward.
- NEW-20's example (placement `new` rejected without a declared form) predicted the compile error that I hit in step 5.
- The undefined rules name the sanitizer behavior in their Evidence. That told me which mistakes a sanitizer run would catch.
