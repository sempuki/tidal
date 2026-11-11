# P4: Port suite, Swift persona

Date: 2026-11-11

Commit read: a91ccc775ea22f6c94c2eac153d8aee62577de72

The reader returned this report as a message, and the integrator saved it here.

Role: clean-room reader (port suite P4, decision 0063, item E). The persona is an experienced Swift programmer who has never written C++. The reader read only files under `specification/` and its own directory `reports/clean-room/P4/`.

## Pages read

- `specification/index.md`
- `specification/foundations/translation.md`
- `specification/foundations/language-support.md`
- `specification/foundations/storage.md`
- `specification/foundations/conversions.md` (CONV-4 only)
- `specification/statements/range-for.md`
- `specification/expressions/new-delete.md` (principles, NEW-19, NEW-26)
- `specification/expressions/lambdas.md` (principles)
- `specification/expressions/multiplicative.md` (MUL-P2, MUL-6, MUL-7)
- `specification/classes/operators.md`
- `specification/classes/comparisons.md` (principles and rules up to DCMP-6)
- `specification/classes/constructors.md` (CTOR-P4, DTOR-6, DTOR-7)
- `specification/classes/copy-move.md` (CPY-P2, CPY-5, CPY-8, CPY-12)
- `specification/classes/members.md` (CLS-P1, CLS-2, CLS-22)
- `specification/declarations/functions/definitions.md` (FN-12)
- `specification/declarations/functions/deleted.md` (DEL-1)
- `specification/declarations/declarators.md` (DCL-20)
- `specification/templates/concepts.md` (principles)
- `specification/templates/class-template-deduction.md` (principles)
- `specification/templates/instantiation.md` (INST-1, INST-2)
- `specification/glossary.md` (searched, not read in full)

## Rules noted before writing

Sequences (`Sequences.swift`):

- RFOR-P1, RFOR-2, RFOR-3, RFOR-7: a computed sequence is a class whose iterator computes the next value in prefix `++`. `end` can return its own type, and the loop compares with `!=`, which can come from `==`. RFOR-P1 says that coroutines are out of scope, so the Swift `IteratorProtocol` maps to this shape.
- OPR-4, OPR-13: the loop uses prefix `++` only.
- LSUP-1: a braced list needs `std::initializer_list` in the given shape, declared by the program.
- CPT-P2, CPT-P3: a concept with a requires expression stands in for the Swift `Sequence` protocol.
- CTAD-P1, CTAD-P2 (the aggregate guide, CTAD-9): `Window{Collatz{6}, 3}` deduces the base type.
- LAM-P1, LAM-P2, LAM-P4: each closure has its own type, so an adaptor takes the closure type as a template parameter.

Buffer (`Buffer.swift`):

- STG-P3, STG-8, STG-28: storage from `::operator new` holds an array of `T`, and pointer arithmetic over elements that are not yet built is valid. The `Stack` example on the storage page is the model.
- NEW-19: the global allocation functions need no declaration.
- LSUP-7, STG-5: the program defines the non-allocating placement `operator new` and builds each element with it.
- DTOR-6, DTOR-7: an explicit destructor call ends an element and keeps the storage.
- NEW-P2, CPY-P2: a class that owns storage deletes or writes its copy functions.
- STG-P2, STG-29 and the `Maybe` example: a stand-in for Swift's `Optional` keeps the pointer that placement `new` returns.

Money (`Money.swift`):

- OPR-P3, OPR-8, OPR-9, OPR-10: a literal suffix that starts with `_`. Integer literals use the `unsigned long long` form and decimal literals use the `long double` form.
- CONV-4: a floating value converts to an integer by dropping the fraction, so rounding adds one half first.
- MUL-6, MUL-7: integer `/` and `%` round toward zero, as Swift does.
- LSUP-2, LSUP-12: a defaulted `<=>` needs `std::strong_ordering` with its static members and the operators `v == 0` and `v < 0`. The page example gives the whole class.
- DCMP-4, DCMP-6: a defaulted `<=>` declares a defaulted `==`, and `<` is rewritten from `<=>`.
- TR-2: the exit status is the value that `main` returns.

## Summary

All three programs exit 0 on g++ and clang++ in every build: with warnings on, with `-pedantic-errors` alone, and with the sanitizers. Their final versions also run on `tidal-fe`. The three biggest problems are these:

1. The specification reads as rejecting a member function, defined in its class, whose return type is a class template specialization over that class (`List<Money, 16> split() const { ... }` inside `Money`). FN-12 asks for a complete return type in a definition. CLS-2 lists only the function body as a place where the class is complete. g++ and clang++ accept the program. `tidal-fe` rejects it, which matches the text (P4-1).
2. `tidal-fe` stops with "unsupported ... overloaded operator" on an operator function whose parameter is `std::initializer_list<int>` (P4-2).
3. The specification gives no example of a generic adaptor that wraps any sequence (the lazy `filter` and `map` stages and `Window`). The reader had to work out how to name a sequence's iterator type and element type with no library, and used outside knowledge for it (P4-3, P4-4).

## Findings

### P4-1

Pages: `classes/members.md`, `declarations/functions/definitions.md`, `templates/instantiation.md`, `declarations/declarators.md`. Rules: FN-12, CLS-2, CLS-22, INST-1, DCL-20, CLS-1.

Kind: wrong (and outside knowledge, front end).

What happened: the first `money.cpp` defined `split` inside `Money` with the return type `List<Money, 16>`, where `List` has a member `T items[Capacity]`. g++ and clang++ accept the program and run it. `tidal-fe` rejects it with "rejected [DCL-20] at line 55: array of incomplete class type". The reduction `reductions/in-class-definition-returns-template-of-own-class.cpp` uses a `Box<Money>` return type and stops with "rejected [CLS-1] at line 1: data member of incomplete class type". A parameter of type `Box<Money>` stops in the same way. A return type of plain `Money` is accepted.

The text supports the front end. FN-12 asks that a definition have a complete return type. INST-1 instantiates the specialization "when the definition is made". CLS-2 lists the member function body as a place where the class is complete, and does not list the return or parameter types. The reader's outside knowledge says that for a member function defined in its class, the completeness check of the parameter and return types waits until the class is complete. The two host compilers behave that way.

Fix in the program: declare `split` in the class and define it after the class.

Suggestion: state in CLS-2 or FN-12 that the parameter and return types of a member function defined in its class are checked for completeness at the end of the class. Add a test whose return type is a specialization over the enclosing class, and fix the front end to match.

### P4-2

Pages: `foundations/language-support.md`, `classes/operators.md`. Rules: LSUP-1, OPR-6.

Kind: front end.

What happened: the first `sequences.cpp` had a member `bool operator==(std::initializer_list<T>) const` in `List` and the check `steps == std::initializer_list<long long>{6, 3, ...}`. g++ and clang++ run it. `tidal-fe` stops with "unsupported at line 198: overloaded operator". The reduction `reductions/operator-initializer-list-parameter.cpp` puts the operand in a named variable and stops in the same way. A free function with the same parameter type runs. So does an operator whose parameter is another class.

Fix in the program: a member function `matches(std::initializer_list<T>)`, called with a braced list.

Suggestion: let overload resolution for an operator accept a candidate whose parameter is `std::initializer_list<E>`.

### P4-3

Pages: `statements/range-for.md`, `templates/concepts.md`. Rules: RFOR-P1, RFOR-2, CPT-P3.

Kind: needs example.

What happened: RFOR-P1 and its example show one computed sequence with its own end type. The Swift program also wraps sequences in lazy `filter` and `map` stages and in a generic `Window`. The specification has no example of an adaptor over any sequence. The reader wrote these parts:
- a concept for "a sequence",
- adaptors that hold the base iterator and the base end,
- a `Lazy` wrapper whose member templates return the next stage.

All of it ran on the first try on every compiler and on `tidal-fe`.

Suggestion: add an informative example to the range-for page. It would show an adaptor class template that wraps any range and filters or maps its elements, with the closure type as a template parameter (LAM-P4), and how the adaptor names the base's iterator type.

### P4-4

Pages: `templates/concepts.md`. Rules: CPT-P3, CPT-28.

Kind: outside knowledge, hard to find.

What happened: to name the iterator type and the element type of a sequence `S` with no library, the reader wrote `decltype(static_cast<const S*>(nullptr)->begin())`. The reader knew from outside the specification that an operand of `decltype` is not evaluated, so it can hold an expression that would be undefined if it ran. The concept's requirement for prefix `++` used the same trick. The reader found no page that shows how to form such a type without the library.

Suggestion: add an informative example, perhaps on the decltype or concepts page, that names the result type of a member call when no object of the type is at hand. A requires-expression parameter (`requires(IteratorOf<S> it) { ++it; }`) is the cleaner form for the concept, and the example could show it.

### P4-5

Pages: `foundations/storage.md`, `classes/copy-move.md`. Rules: STG-P2 (the `Maybe` example), CPY-5, CPY-8, CPY-12, DEL-1.

Kind: needs example.

What happened: Swift's `removeLast()` returns `Element?`. With no library `optional`, the reader built a `Maybe<T>` from the example on the storage page, which keeps the pointer that placement `new` returns. The reader deleted its copy functions, as `FixedVec` does. Then `return last;` in `removeLast` was rejected by all three:
- g++: "use of deleted function"
- clang++: "call to deleted constructor"
- `tidal-fe`: "rejected [DEL-1] at line 70: the move constructor of 'Maybe' is deleted"

The text after the `Maybe` example says that `Maybe` must write its own copy and move functions, and it cites CPY-8. CPY-8 gives the order of an implicit copy. CPY-5 is the rule that a declared copy constructor removes the implicit move constructor. CPY-P2 says that an rvalue then falls back to the copy, which is deleted here. The reader found CPY-5 only after the rejection.

Fix in the program: a move constructor that builds the value in the new object's own bytes from the old one.

Suggestion: show the move constructor in the `Maybe` example, and cite CPY-5 and CPY-P2 beside CPY-8 in the sentence after it.

### P4-6

Pages: `declarations/functions/deleted.md`. Rule: DEL-1.

Kind: front end.

What happened: in the case of P4-5, `tidal-fe` says "the move constructor of 'Maybe' is deleted". By CPY-5, `Maybe` has no move constructor at all, and overload resolution selects the deleted copy constructor. The rule ID is right, and the message names the wrong function.

Suggestion: name the selected function (the copy constructor) in the message, and perhaps add that no move constructor is declared.

### P4-7

Pages: `classes/operators.md`, `foundations/conversions.md`. Rules: OPR-8, CONV-4.

Kind: unclear.

What happened: Swift's `init(floatLiteral:)` receives a `Double` and calls `.rounded()`. The C++ `long double` form receives the literal's value, and the reader rounded by adding `0.5L` and converting (CONV-4). OPR-8 gives the parameter type. It does not say what value the parameter holds for `19.99_usd`, which would be the literal's value rounded to `long double`, as for an unsuffixed literal of that type. The program depends on that value being close to 19.99. Both compilers and `tidal-fe` give 1999 cents, and a `static_assert` checks it.

Suggestion: state in OPR-8 that the numeric form receives the value of the literal without its suffix, as a literal of type `unsigned long long` or `long double`, and cite the rule that gives the rounding of a floating literal.

## Attempt log

### sequences.cpp

1. First try (`sequences_first_try.cpp`) has these parts:
   - a `Collatz` struct with a nested `Iterator` and an empty `Done` end type,
   - a `Sequence` concept,
   - `Filtered`, `Mapped`, and `Lazy` adaptors,
   - a generic `Window` over any sequence,
   - a fixed-capacity `List`,
   - a self-declared `std::initializer_list`.

   g++ and clang++ build it with no warning, and it exits 0 in every build. `tidal-fe` stops with "unsupported at line 198: overloaded operator" (P4-2).
2. Final (`sequences.cpp`): the `operator==` with an `std::initializer_list` parameter becomes the member function `matches`. The program exits 0 everywhere, `tidal-fe` included.

The Swift `it.next()` checks map to `*it`, `++it`, and `it == Done{}`. Swift's `next()` gives the value and then advances. The C++ iterator gives the value with `*` and advances with `++`. `++` on an ended iterator leaves it ended, so the second check after the end holds, as in Swift.

### buffer.cpp

1. Attempt 1 (not kept, because it did not build): `Maybe<T>` with deleted copy functions and no move constructor. g++, clang++, and `tidal-fe` reject `return last;` (P4-5, P4-6).
2. First try that builds (`buffer_first_try.cpp`), which is also the final version (`buffer.cpp`): `Maybe<T>` gets a move constructor. `Buffer<Element>` works as follows:
   - it takes storage from `::operator new`,
   - it builds each element with placement `new` at `storage + count_`,
   - it ends elements with explicit destructor calls,
   - it frees the storage with `::operator delete`.

   `Token` counts live objects in its constructors and its destructor. The program exits 0 everywhere. `tidal-fe` explores 4 orders, gets result 0 in each, and reports the unspecified operand order of `+` at line 121 (SEQ-8).

In C++ `Token` is a value type, so its copy constructor also counts. In Swift a `Token` is a reference, and `alive` counts instances. The counts still match, because each C++ copy is a live object that ends in turn.

### money.cpp

1. First try (`money_first_try.cpp`) has these parts:
   - `Money` with a `long long` count of cents,
   - `_usd` literal operators for integer and decimal literals,
   - `+`, `-`, `*`, and a defaulted `<=>`,
   - a `description` that fills a fixed `Text`,
   - `split`, defined in the class,
   - a self-declared `std::strong_ordering`, copied from the language-support page.

   g++ and clang++ build it with no warning, and it exits 0 in every build. `tidal-fe` rejects it with "rejected [DCL-20] at line 55: array of incomplete class type" (P4-1).
2. Final (`money.cpp`): `split` is declared in the class and defined after it. The program exits 0 everywhere, `tidal-fe` included.

Swift's `[Money(cents: 500), 1.5, 3].sorted()` relies on implicit literal conversion. In C++ each element is written as `1.5_usd` or `3_usd`, and a small insertion sort stands in for `sorted()`. `max(price, 20)` becomes `maximum(price, 20_usd)`.

## Results

Each cell gives the exit status of the run, or the stop. The g++ and clang++ columns use `-std=c++23 -pedantic-errors -Wall -Wextra`. "pedantic" is `-std=c++23 -pedantic-errors`. "sanitizers" is `-std=c++23 -pedantic-errors -fsanitize=undefined,address -fno-sanitize-recover=all`. The pedantic and sanitizer columns give g++ first and clang++ second.

| Program | g++ | clang++ | pedantic | sanitizers | tidal-fe |
|---|---|---|---|---|---|
| `sequences.cpp` | 0 | 0 | 0, 0 | 0, 0 | 0 |
| `sequences_first_try.cpp` | 0 | 0 | 0, 0 | 0, 0 | unsupported: overloaded operator (line 198) |
| `buffer.cpp` | 0 | 0 | 0, 0 | 0, 0 | 0 (4 orders) |
| `buffer_first_try.cpp` (same as the final version) | 0 | 0 | 0, 0 | 0, 0 | 0 (4 orders) |
| `money.cpp` | 0 | 0 | 0, 0 | 0, 0 | 0 |
| `money_first_try.cpp` | 0 | 0 | 0, 0 | 0, 0 | reject DCL-20 (line 55) |

No build gave a warning.

Reductions:

| File | g++ | clang++ | tidal-fe |
|---|---|---|---|
| `reductions/operator-initializer-list-parameter.cpp` | 0 | 0 | unsupported: overloaded operator |
| `reductions/in-class-definition-returns-template-of-own-class.cpp` | 0 | 0 | reject CLS-1 |

## Host-only claims

Each claim below rests on the host g++ and clang++ alone. A writer should probe each one on the matrix before acting on it.

- g++ and clang++ accept a member function defined in its class whose return type or parameter type is a class template specialization with a member of the enclosing class's type (P4-1, the `Box<Money>` shape).
- g++ and clang++ accept an operator function with a parameter of a self-declared `std::initializer_list<int>`, called with a named `std::initializer_list` operand (P4-2).
- g++ and clang++ accept a range-based `for` over an adaptor whose iterator compares with an empty end type by `==` only, with `!=` rewritten (RFOR-2). They also accept the aggregate deduction for `Window{Collatz{6}, 3}` (CTAD-9).
- g++ and clang++ give `19.99_usd` the value 1999 cents through the `long double` form plus one half (P4-7).
- The three final programs run clean under `-fsanitize=undefined,address` on both host compilers.

## What worked well

- RFOR-P1 says that coroutines are out of scope and that a computed sequence is a class whose iterator computes the next value in `++`, with its own end type. That sentence mapped the Swift `IteratorProtocol` to C++ at once. The `Squares` example was the model for `Collatz`.
- The storage page's `Stack` example gave the whole shape of `Buffer`: storage from `::operator new`, placement `new` at `items + count`, explicit destructor calls, and deleted copies. The program ran on its first build, under the sanitizers, and on `tidal-fe`.
- The language-support page's `std::strong_ordering` example worked as written for a defaulted `<=>`, a rewritten `<`, and a `static_assert`. The table in LSUP-P1 said which parts each use needs.
- The literal operator rules (OPR-8, OPR-9) and the example `operator""_m` made the integer and decimal suffix forms clear, including the need for a leading `_`.
- `tidal-fe` reported the unspecified operand order in `buffer.cpp` and explored each order. The rule IDs in its two stops led to the right pages.
