# P2: Port suite, Java persona

Date: 2026-11-11

Commit read: bafaf6da549b4f9db8956375e7fb0bb7cec79fe7

Role: clean-room reader. The persona is an experienced Java programmer who has never written C++. The reader read only files under `specification/` and `reports/clean-room/P2/`.

The reader returned this report as a message, because the write of `report.md` was refused. The integrator saved it here with the reader's programs and the Java originals, which a clean-room writer wrote for the suite.

## Pages read

- `specification/index.md`
- `specification/foundations/translation.md` (whole page)
- `specification/foundations/language-support.md` (principles, rules, examples)
- `specification/statements/range-for.md` (whole page)
- `specification/classes/operators.md` (whole page)
- `specification/expressions/lambdas.md` (whole page)
- `specification/classes/comparisons.md` (principles, DCMP-1, examples)
- `specification/classes/copy-move.md` (principles, CPY-8)
- `specification/expressions/new-delete.md` (principles, NEW-4, NEW-5)
- `specification/classes/constructors.md` (principles)
- `specification/templates/declarations.md` (principles)
- `specification/templates/dependent-names.md` (principles)
- `specification/classes/virtual.md` (principles)
- `specification/classes/initializer-lists.md` (ILC-1, ILC-3, ILC-5, ILC-6)
- `specification/expressions/multiplicative.md` (MUL-6 to MUL-10)
- `specification/expressions/additive.md` (ADD-9, ADD-10)
- `specification/foundations/conversions.md` (CONV-2)
- `specification/expressions/primary.md` (PRIM-P2, PRIM-7, PRIM-9)
- `specification/expressions/comparison.md` (CMP-6, CMP-15)
- `specification/foundations/types.md` (TYP-3)
- `specification/foundations/objects.md` (OBJ-2)
- `specification/foundations/constant-evaluation.md` (CE-3)
- `specification/classes/members.md` (CLS-16)
- `specification/glossary.md` (entry "usable in constant expressions")

## Rules and principles noted before writing

| Construct | Java form | IDs that told the reader how to write it |
|---|---|---|
| generic classes | `class ChainMap<K, V>`, `LinkedSeq<T>` | TPL-P1, TPL-1, TPL-P5, DEP-P4 |
| hasher | `interface Hasher<K>` with a lambda | LAM-P4, LAM-10, LAM-1 |
| predicate | `Predicate<T>` with a lambda | LAM-15, TPL-P5 (member function template) |
| iterator for a for-each loop | `Iterable<T>`, `Iterator<T>` | RFOR-P1, RFOR-2, RFOR-7, OPR-4, OPR-13, DCMP-P2 |
| records with equality | `record Point`, `record Segment` | DCMP-P1, DCMP-1 |
| records with hashing | `hashCode()` | none; written by hand (P2-1) |
| `HashSet<>(List.of(a, b, c))` | a braced list | LSUP-1, LSUP-4, ILC-3, ILC-5 |
| shared references | two `Book` objects share one `Account` | CPY-P2, CPY-P4, NEW-P2, NEW-5, OPR-5 |
| `ArrayList`, `HashMap` storage | library classes | NEW-P1, NEW-P2, NEW-5, CPY-P2, CTOR-P4 |
| `null` for a missing value | `return null` | CMP-6, CMP-15, NEW-4 |
| `String.equals` | string keys | CMP-15, PRIM-9, PRIM-7 |
| `h * 31 + c` with wrapping | `int` arithmetic | ADD-9, MUL-10, MUL-8, CONV-2 |
| `Math.floorMod` | remainder | MUL-6, MUL-7 |
| `System.exit(failures)` | exit status | TR-2, TR-11 |
| `static final int` as an array bound | constant | CE-3 |

## Summary

All three ports built on the first try with no warnings on GCC 16 and Clang 22 in every configuration. They ran clean under the sanitizers, exited 0, and ran to a result of 0 in `tidal-fe`. Each final version is the same as its first try. The three biggest problems:

1. **The specification gives no guidance for a program without the library (P2-1).** Each Java program uses `ArrayList`, `HashMap`, `HashSet`, `String`, and object references. The specification gives the language-support types and nothing else. So the reader wrote a growable list, a hash set, string equality, and a reference-counted handle by hand. Only LAM-P4 says how a program without the library does a common job.
2. **Java habits that compile and are wrong are stated only as separate facts (P2-3, P2-4, P2-5).** String `==` compares addresses (CMP-15, PRIM-9). Integer overflow is undefined (ADD-9, MUL-10). A member read after `delete` is undefined (OBJ-2). Each fact is in the specification. The reader found each one only because the reader already suspected it.
3. **The shared reference needed outside knowledge (P2-2).** The specification explains why an owning class must write or delete its copy functions (CPY-P2, NEW-P2). The reference-counted handle that gives Java's sharing came from outside knowledge.

## Findings

### P2-1: No guidance for a program that has no library

- Pages: `foundations/language-support.md`, `expressions/lambdas.md`, `expressions/new-delete.md`, `classes/copy-move.md`.
- Rule IDs: LSUP-P1, LAM-P4, NEW-P2, CPY-P2.
- Kind: needs example.
- What happened: the task forbids `#include`, and each Java program uses library collections and strings. The language-support page lists the types that the core rules need and says that it gives nothing else about the library. LAM-P4 is the one place that says how a program without the library does a common job (closures in one table). The reader built a growable array (`List<T>`), a chained hash map, a hash set, a string comparison, a number-to-text conversion, and a shared handle. Each piece worked on the first try, but the reader designed the shapes. Java records also give `hashCode`. C++ has no defaulted hash, so the reader wrote `hash()` members by hand. The specification says nothing about hashing. That is correct for a core specification, but a Java reader looks for it next to DCMP-1.
- Suggestion: add an informative example of a small owning array class with its copy functions deleted, on the new and delete page or on the copy and move page, and cite NEW-P2 and CPY-P2 from it. A sentence on the comparisons page could say that the language defaults equality and ordering and has no defaulted hash.

### P2-2: The shared reference came from outside knowledge

- Pages: `classes/copy-move.md`, `expressions/new-delete.md`, `classes/operators.md`.
- Rule IDs: CPY-P2, CPY-P4, NEW-P2, OPR-5.
- Kind: outside knowledge.
- What happened: `Ledger.java` shares one `Account` between two `Book` objects, and one `Account` is made in the call to `add`. In C++ some code must delete each `Account`. The reader used outside knowledge of reference counting and wrote `Ref<T>` with these parts:
  - a control block,
  - a copy constructor,
  - a copy assignment that increments before it releases, so that self-assignment is safe,
  - `operator->`.

  CPY-P2 and NEW-P2 told the reader why the copy functions must be written. OPR-5 told the reader how `->` chains. The counting design and the order inside the assignment are outside knowledge.
- Suggestion: NEW-P2 says that an owner writes or deletes its copy functions. A short example of the "write" branch, where copies share the object and count it, would show a Java reader the usual answer.

### P2-3: String equality with `==` is a silent trap for a Java reader

- Pages: `expressions/comparison.md`, `expressions/primary.md`.
- Rule IDs: CMP-15, PRIM-9, PRIM-7.
- Kind: needs example.
- What happened: Java compares strings with `equals`. A Java programmer knows that `==` on strings is wrong in Java too. In C++ a key of type `const char*` compares by address (CMP-15), and two equal literals can be one object or two (PRIM-9). So a map keyed on `const char*` compiles and can pass or fail by chance. The reader wrapped the key in a `Str` class whose `operator==` compares characters. A probe compared two pointers that both held "pear". In `tidal-fe` the probe gave "unspecified: ... PRIM-9 ... other outcomes: result 1", which shows the trap well. GCC and Clang both gave 0.
- Suggestion: add a line to the PRIM-9 or CMP-15 examples that compares two `const char*` holding equal text and notes that `==` compares their addresses.

### P2-4: Java's wrapping arithmetic is spread over three pages

- Pages: `expressions/additive.md`, `expressions/multiplicative.md`, `foundations/conversions.md`.
- Rule IDs: ADD-9, MUL-10, MUL-8, CONV-2.
- Kind: hard to find.
- What happened: the Java hash `h = h * 31 + c` relies on `int` wrapping. ADD-9 and MUL-10 make the same code undefined in C++ when it overflows. MUL-8 says that unsigned arithmetic wraps. CONV-2 says that the conversion back to `int` keeps the low bits. Together the three rules give Java's result: compute in `unsigned` and cast at the end. The reader found all three, but went looking only because of outside knowledge that signed overflow is undefined in C. The test words are short, so the original `int` code would not have overflowed, and no tool would have flagged it.
- Suggestion: an example on the multiplicative page that computes a wrapping hash in `unsigned` and converts it with CONV-2 would tie the three rules together.

### P2-5: A literal port of `removeIf` reads a deleted node

- Pages: `expressions/new-delete.md`, `foundations/objects.md`.
- Rule IDs: NEW-5, OBJ-2, CTOR-P4.
- Kind: front end. The finding comes from a probe, and the front end behaved as the specification says.
- What happened: `Chain.java` unlinks a node inside a `for` loop whose step reads `n.next`. In Java the garbage collector keeps the node. A C++ port that deletes the node and then runs the step reads freed memory. The reader knew this from CTOR-P4 and NEW-5 (which points to OBJ-2), so the final program saves `next` before the `delete`. A probe of the literal form stops in `tidal-fe` with "undefined [OBJ-2] at line 5: indirection through a pointer to an object whose lifetime has ended". Address sanitizer reports a heap use after free. Without sanitizers, GCC's build exits 0 and hides the error. The reduction is `reductions/removeif-literal-port.cpp`. It reduces the literal port. The final program does not stop.
- Suggestion: no change to the rules is needed. The last sentence of NEW-5 pointed the reader to OBJ-2, and the front end message is clear.

### P2-6: The shape of `std::initializer_list` and its constructor's access

- Pages: `foundations/language-support.md`.
- Rule IDs: LSUP-1, LSUP-4.
- Kind: unclear.
- What happened: LSUP-1 lists "a constructor that takes the pointer and the count" and a count "of type `std::size_t`". The example on the same page makes the constructor private and spells the count type `decltype(sizeof 0)`. The reader copied the example, and it worked on both compilers and in `tidal-fe`. The rule does not say whether the constructor can be private. It also does not say whether the program must declare `std::size_t` as well. The reader declared it, as LSUP-4 allows.
- Suggestion: say in LSUP-1 that the constructor's access does not matter, because the implementation calls the constructor. Also say that the count type can be spelled `decltype(sizeof 0)` with no declaration of `std::size_t`.

### P2-7: "Null" for a missing value has no stated pattern

- Pages: `expressions/comparison.md`, `expressions/new-delete.md`.
- Rule IDs: CMP-6, CMP-15, NEW-4.
- Kind: outside knowledge.
- What happened: `ChainMap.get` and `LinkedSeq.lastMatching` return `null` in Java. The reader returned a pointer to the stored value, and `nullptr` when nothing matches. Using a pointer for "maybe a value" came from outside knowledge. So did the knowledge that a pointer into a container is valid only until the container changes. The specification has the pieces (CMP-6 for a comparison with `nullptr`, OBJ-2 for a dangling pointer) but no example.
- Suggestion: no change is required. If P2-1 adds an example container, a lookup that returns a pointer or `nullptr` would also cover this case.

### P2-8: A `static const int` member as an array bound

- Pages: `foundations/constant-evaluation.md`, `glossary.md`.
- Rule IDs: CE-3.
- Kind: hard to find.
- What happened: the reader wrote `static const int bucketCount = 16;` in a class template and used it as an array bound, as Java code would use a `static final int`. The reader knew from outside knowledge that this works. A search for "usable in constant expressions" found CE-3 and the glossary entry, which confirm it. No page on class members or arrays points to CE-3.
- Suggestion: cite CE-3 from the rule on array bounds in the declarators page.

### P2-9: The front end reports an unspecified order that cannot change the result

- Pages: none read for SEQ-1.
- Rule IDs: SEQ-1.
- Kind: front end.
- What happened: `tidal-fe run ledger.cpp` printed "unspecified: result 0 with fixed choices: SEQ-1 at line 86 (arguments evaluated from left to right)" and "explored: 8 runs; other outcomes: result 0". Line 86 calls `combine((unsigned)from.hash(), to.hash())`, whose two arguments have no side effects. The output is correct and shows that every order gives 0. A Java reader can still read the word "unspecified" as a warning.
- Suggestion: when every explored run gives the same result, the front end could say so on the first line, for example "result 0 in every explored order".

## Attempt log

### buckets.cpp

1. Read range-for, lambdas, new and delete, copy and move, and the arithmetic rules. The reader made these choices:
   - a function pointer from a capture-free lambda for the hasher (LAM-10, LAM-P4),
   - a `Str` key whose `==` compares characters (CMP-15),
   - an owning `List<T>` with deleted copy functions (NEW-P2, CPY-P2),
   - hashing in `unsigned` (MUL-8, CONV-2),
   - a floor remainder built from MUL-7.
2. The first try built with no warnings on both compilers, under `-pedantic-errors` alone, and under the sanitizers. Every build exited 0, and `tidal-fe` gave result 0. The final version is the same as the first try.

### chain.cpp

1. Read RFOR-2 and RFOR-7. Wrote a nested `Iterator` class with `*`, prefix `++`, and `==` only. RFOR-2 says that `!=` "can come from `==`", and DCMP-P2 says that the `==` must return `bool`. The predicate is a member function template that takes any callable. `removeIf` saves `next` before `delete` (NEW-5, OBJ-2).
2. The first try built and ran clean in every configuration, and `tidal-fe` gave result 0. The final version is the same as the first try.
3. A probe of the literal Java form of `removeIf` stops in `tidal-fe` on OBJ-2 (P2-5). The probe is saved as `reductions/removeif-literal-port.cpp`.

### ledger.cpp

1. Declared `std::size_t` and `std::initializer_list` as the language-support page gives them (LSUP-1, LSUP-4), and defaulted `==` on `Point` and `Segment` (DCMP-1). Wrote these parts by hand:
   - the `hash()` members,
   - a `HashSet<T>` with an initializer-list constructor (ILC-3),
   - a reference-counted `Ref<T>` with `operator->` (OPR-5, CPY-P2).
2. The first try built and ran clean in every configuration. `tidal-fe` gave result 0 over 8 explored runs, with the SEQ-1 note of P2-9. The final version is the same as the first try.

## Results

The compilers are GCC 16.2.1 and Clang 22.1.8. The column groups use these flags:

- "Strict" is `-std=c++23 -pedantic-errors -Wall -Wextra`.
- "Pedantic" is `-std=c++23 -pedantic-errors`.
- "Sanitizers" is the pedantic flags plus `-fsanitize=undefined,address`.

Each cell gives the build result and the exit status.

| File | g++ strict | clang++ strict | g++ pedantic | clang++ pedantic | g++ sanitizers | clang++ sanitizers | tidal-fe |
|---|---|---|---|---|---|---|---|
| buckets.cpp | 0 warnings, exit 0 | 0 warnings, exit 0 | exit 0 | exit 0 | exit 0, no report | exit 0, no report | result 0 |
| buckets_first_try.cpp | 0 warnings, exit 0 | 0 warnings, exit 0 | exit 0 | exit 0 | exit 0, no report | exit 0, no report | result 0 |
| chain.cpp | 0 warnings, exit 0 | 0 warnings, exit 0 | exit 0 | exit 0 | exit 0, no report | exit 0, no report | result 0 |
| chain_first_try.cpp | 0 warnings, exit 0 | 0 warnings, exit 0 | exit 0 | exit 0 | exit 0, no report | exit 0, no report | result 0 |
| ledger.cpp | 0 warnings, exit 0 | 0 warnings, exit 0 | exit 0 | exit 0 | exit 0, no report | exit 0, no report | result 0 (8 runs; SEQ-1 note) |
| ledger_first_try.cpp | 0 warnings, exit 0 | 0 warnings, exit 0 | exit 0 | exit 0 | exit 0, no report | exit 0, no report | result 0 (8 runs; SEQ-1 note) |
| reductions/removeif-literal-port.cpp (probe) | not built | not built | not built | not built | heap-use-after-free report | not run | stops: undefined [OBJ-2], exit 126 |

## What worked well

- LAM-P4 answers the exact question that a Java reader has about storing a lambda without the library, and its example works as written.
- RFOR-2 lists every operation that a range-for needs from an iterator, including that `!=` can come from `==`. The iterator worked on the first try.
- CPY-P2 and NEW-P2 explain why an owning class must write or delete its copy functions. A reader without C++ experience can follow the explanation.
- NEW-5 points to OBJ-2 for a read after `delete`, and the front end names OBJ-2 at the exact line.
- MUL-7 states the sign of `%` plainly, so `floorMod` was easy to write.
- The `std::initializer_list` example on the language-support page worked as written on both compilers and in the front end.
- The front end explores unspecified choices (PRIM-9, SEQ-1) and reports the other outcomes, which exposes the string-address trap.
