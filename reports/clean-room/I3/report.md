# I3: overload ranking from the rules alone

Date: 2026-11-05
Commit read: b1d89e77 (Phase 1 read the pages at e9b55afa; between the two commits only `specification/glossary.md` changed among the pages read, by 4 lines)
Kind: Implement (decision 0057, item E)
Role: clean-room reader in the role of an implementer

The reader returned this report as a message, and the integrator saved
it here with the reader's files. The filtered page copies, the
binaries, and the front-end build stay out of the repository.

## Inputs

Phase 1 (blind). The reader read these pages through `filter.py`. The filter drops each `## Evidence` segment and each `:tests:` line, and it asserts that neither reaches its output. Nothing slipped through.

- specification/overloading/conversion-sequences.md
- specification/overloading/overload-resolution.md
- specification/foundations/conversions.md
- specification/declarations/initialization/references.md
- specification/classes/initializer-lists.md (ILC-2, ILC-3, ILC-8, the principles)
- specification/declarations/initialization/list.md (LIST-2, LIST-3, LIST-5, LIST-6)
- specification/glossary.md (terms looked up on the filtered copy)

Output: `ranking.py`, about 1100 lines of Python, and `notes.md` with 22 steps that the text did not give (N1 to N22). `examples_check.py` runs the two Examples blocks of the overloading pages, and all 14 cases agree. `ranking.sha1` holds `8eff41d43a3227b3f7c7a191eeed2cb74ca11837`. The reader made `ranking.py` read-only after that, and `sha1sum -c` still passes at the end.

The component takes candidates (`Fn`: parameters, ellipsis, defaults, member kind, cv and ref-qualifier, template flag, constraint satisfaction, rewritten and swapped flags, inherited flag) and arguments (`Arg`: type and value category, null pointer constant, braced list with designators, string literal, overload set). It returns "ok" with a function, "ambiguous", or "no viable function". Partial ordering and constraints are callbacks that the caller answers. ICS-8 faults are left to the caller.

Phase 2 (scoring). The reader read:

- every test under tests/overloading/conversion-sequences/ and the ranking tests under tests/overloading/overload-resolution/ (best-, better-, tie-, template-exact-over-plain, init-, reference-, viable-, defaults-take-no-part, static-slot-no-part, no-object-, builtin-logical-, builtin-increment-from-, builtin-other-shape-kept),
- the two tests outside tests/overloading whose tidal-rules line cites a ranking rule and which the component can decide (`declarations/functions/definitions/different-parameter-types-overload.cpp`, `foundations/language-support/initializer-list-candidate-not-viable.cpp`),
- all 24 grid files and both `tables.md`.

`score.py` turns 19 grids into calls of the component. `score_tests.py` with `batch2.py` to `batch6.py` turns the test files into calls. `run_score.py` writes `score-output.txt`.

The reader did not score tests about candidate sets, lookup, call contexts, the address of an overload set, surrogate candidates, default arguments from two scopes, or inherited copy shapes. Those rules decide which functions are candidates, and the caller does that work. The `contexts`, `address`, `address-explicit`, and `surrogates` grids are left out for the same reason, as are the surrogate pairs of `object-call`. The `1.0f64` row of `order` and the `long long : 3 bit-field` row of `arithmetic` cannot be represented (see I3-3 and I3-9).

Front end. The reader built it with `cargo build --release --locked` into its scratch directory, wrote 17 programs in `programs/`, compiled each with g++ and clang++ (`-std=c++23 -pedantic-errors`), and ran each with `tidal-fe run`. Seven reductions are in `reductions/`.

## Score

| Set | Pass | Fail | Not scored |
|---|---|---|---|
| Grid cells (19 grids) | 1184 | 5 | 45 split between compilers; 18 three-way rejects that OVL-11 decides after resolution |
| Individual assertions (test files) | 329 | 14 | 4 not representable (extended floating-point types) |
| Total | 1513 | 19 | |

The component agrees with 98.8 % of the assertions it can decide. These are the 19 failures:

| Assertions | Cause | Finding |
|---|---|---|
| 2 grid cells (`pointers`, Middle* / const Middle*) | component | I3-1 |
| 3 grid cells (`references`, bit-field to const volatile int&) | text | I3-2 |
| bit-field-promotion.cpp | text and component | I3-3 |
| list-string-literal-utf8.cpp | component | I3-4 |
| rvalue-reference-before-qualification.cpp | text and reading | I3-5 |
| 5 assertions in user-defined-explicit*.cpp | component | I3-6 |
| 2 assertions in user-defined-explicit-reference-value/xvalue.cpp | text | I3-7 |
| init-direct-explicit-conversion.cpp, builtin-logical-explicit-bool/competes/competes-and.cpp | component design, text hard to find | I3-8 |

Of the 22 open steps in notes.md, the tests confirmed the reader's reading wherever they reached one: N2, N3, N4, N7, N10, N11, N13, and N19. Two readings failed against the tests, N3 applied to a prvalue (I3-5) and N14 (I3-8).

On the 17 programs, `tidal-fe` gave the result that Tidal's pages give in every program it ran (12 of 17). That includes p01, p03, p07, and p16, where g++ differs, and p02 and p13, where clang++ rejects. It stopped on five programs (I3-14 to I3-18).

## Findings

### I3-1. A derived-to-base pointer step and a qualification adjustment are two steps

- Rules: ICS-P2, ICS-1, ICS-26 (conversion-sequences)
- Kind: needs example
- What happened: For `Derived*` to `const Middle*`, the component made one derived-to-base step whose result already had `const`. So `Derived*` to `Middle*` was no longer a subsequence of it, and the call `f(Middle*)` / `f(const Middle*)` came out ambiguous. The grid expects `Middle*`. The component caused the error. The diagram in ICS-P2 gives the order of groups, but no example shows a pointer conversion followed by a qualification adjustment.
- Suggestion: Add a line to ICS-1 or ICS-26 with `Derived*` to `const Base*` as two steps: the derived-to-base conversion to `Base*`, then the qualification adjustment. Also say that `Derived*` to `Base*` is a proper subsequence of it.

### I3-2. A bit-field and a `const volatile` lvalue reference

- Rules: REF-8, REF-2, ICS-8 (references, conversion-sequences)
- Kind: unclear
- What happened: REF-8 says a non-`const` lvalue reference must not bind to a bit-field. REF-2 lets a `const` lvalue reference that is not `volatile` bind to a bit-field. Neither rule says what a `const volatile` reference does. The caller check followed REF-8 and accepted the binding. The grid rejects the calls that choose `const volatile int&` for an `int : 3` bit-field. The reading of the text caused the 3 failures.
- Suggestion: State in REF-8 that an lvalue reference that is not `const`, or that is `volatile`, must not bind to a bit-field. Name the fault in ICS-8.

### I3-3. A sequence needs more of the argument than its type and value category

- Rules: ICS-P1, ICS-2, CONV-1 (conversion-sequences, conversions)
- Kind: wrong
- What happened: ICS-P1 says forming a sequence "looks only at the argument's type, cv-qualifiers, and value category". The reader designed the argument record from that sentence. Several facts outside that list decide the rank. A bit-field promotes by its width (CONV-1). A literal `0` is a null pointer constant (CONV-8). A string literal can initialize a character array (ICS-38). The reader added the last two after reading CONV-8 and ICS-38, but not bit-field width. So `take(bits.v)` with `long long v : 3` came out ambiguous where the test expects `take(int)`. Both the text and the component caused the failure.
- Suggestion: Reword ICS-P1 to list what a sequence looks at: type, cv-qualifiers, value category, and for some arguments the expression itself (a bit-field's width, a null pointer constant, a string literal, an overload set). Keep the sentence about access and deletion as it is.

### I3-4. The LIST-6 table for UTF-8 literals

- Rules: ICS-38, LIST-6 (conversion-sequences, list)
- Kind: component error
- What happened: The component applied the LIST-6 table only to plain literals. For a UTF-8 literal it allowed only `char8_t`, so `{u8"ab"}` had no sequence to `const char (&)[3]`. The text gives the right answer. The component caused the failure.
- Suggestion: None for the text. The ICS-38 cross-reference to LIST-6 was enough.

### I3-5. A non-class prvalue binds a reference through a temporary

- Rules: ICS-11, ICS-46, REF-1, REF-2, ICS-28, ICS-31 (conversion-sequences, references)
- Kind: hard to find
- What happened: ICS-46 says `int* const&` and `const int* const&` both bind directly to an `int*` lvalue. REF-1 does not list that binding (notes N3). The reader extended the rule to any argument, so an `int*` prvalue bound `const int* const&` directly with the identity. The test `rvalue-reference-before-qualification.cpp` says both references bind a temporary, and ICS-28 then picks the rvalue reference. REF-2 does say that a non-class prvalue binds a temporary, but ICS-11 and ICS-46 do not point there. The reading caused the failure, helped by the gap in REF-1.
- Suggestion: In ICS-11, add a row: a non-class prvalue binds a temporary of the referenced type, so its sequence includes the conversion (for example a qualification adjustment). In REF-1, add the direct binding of an lvalue whose type reaches the referenced type by a qualification adjustment below the top level, which ICS-46 relies on.

### I3-6. ICS-45 limits an explicit conversion function for a reference to a qualification adjustment

- Rules: ICS-45 third bullet, OVL-15 (conversion-sequences, overload-resolution)
- Kind: component error
- What happened: In direct initialization the component used an `explicit` conversion function for any direct reference binding, a derived-to-base step included. ICS-45 says such a function serves a reference only when the result reaches the referenced type by a qualification adjustment alone. Five `makes<Base&..., ToDerived...>` assertions failed. The text is clear here, and the component caused the failures.
- Suggestion: None for the text.

### I3-7. The temporary of a reference is copy-initialized, also in direct initialization

- Rules: ICS-45, OVL-15, REF-2, REF-14, REF-P3 (conversion-sequences, overload-resolution, references)
- Kind: missing
- What happened: For `static_cast<const Item&>(ToItem{})` with `explicit operator Item()`, OVL-15 gives no direct binding, because an lvalue reference takes only functions that return an lvalue reference. So the reference binds a temporary Item. The component made that temporary under ICS-45's first bullet (a class target in direct initialization) and found a sequence. The tests say there is none. The test notes say the temporary is copy-initialized. REF-P3 says this for a braced list, but no rule says it for a reference initialized from one expression. The text caused the 2 failures.
- Suggestion: Add to ICS-45 or to REF-2 that the temporary a reference binds is copy-initialized, also in direct initialization, so an `explicit` conversion function cannot make it.

### I3-8. Which parameters are direct-initialized

- Rules: ICS-45, OVL-9, OVL-40, ICS-10 (conversion-sequences, overload-resolution)
- Kind: hard to find
- What happened: The component treats every parameter as copy-initialized, following the last sentence of ICS-45. The text names three exceptions in three places. The operands of built-in `!`, `&&`, and `||` are one (OVL-9). The copy or move constructor parameter in direct initialization of a class is another (OVL-40). A cast is the third. The API had no flag for these, so four tests failed: `!f` with `explicit operator bool`, the two "competes" tests, and `Target t(s)` with an explicit `operator Target()`. The reader recorded the gap as N14 before scoring. The component's design caused the failures, and the scattered text made the gap easy to miss. The same design keeps the ordering and constraint callbacks out of nested user-defined conversions. No test failed for that reason, because OVL-26 or OVL-29 decides first in each such test.
- Suggestion: Add a short list to ICS-P1 or OVL-P2 of the parameters that are direct-initialized, with the rule for each.

### I3-9. Extended floating-point types have no ranks

- Rules: ICS-35, ICS-44, CONV-19 (conversion-sequences, conversions)
- Kind: missing
- What happened: ICS-35 and ICS-44 depend on floating-point types of equal rank. The text marks the extended types as "referenced" and gives no ranks. The component could not represent `std::float64_t`, so 4 tests and one grid row could not be scored.
- Suggestion: Give the ranks of the extended floating-point types relative to `float`, `double`, and `long double` in TYP-9, or name the page that does.

### I3-10. A temporary counts as an rvalue for ICS-28

- Rules: ICS-28, ICS-P5 step 4d (conversion-sequences)
- Kind: unclear (outside knowledge)
- What happened: ICS-28 applies when "both sequences bind references to an rvalue". `const short&` and `int&&` from an enumerator each bind a temporary. The reader needed outside knowledge to treat the temporary as the rvalue (notes N2). The tests agree with that reading.
- Suggestion: Add a sentence to ICS-28 saying that a reference that binds a temporary binds an rvalue.

### I3-11. The CONV-9 rule for const at every level

- Rules: CONV-9 last bullet, ICS-31 (conversions, conversion-sequences)
- Kind: unclear (outside knowledge)
- What happened: CONV-9 forbids a conversion "that adds const at one level without adding it at every level above". Read literally, `int* const*` to `const int* const*` is rejected, because level 1 already has `const` and so does not add it. The reader required `const` to be present at each level above (notes N4). `qualification-fewer.cpp` passes with that reading.
- Suggestion: Write "without `const` at every level above that level". Also give the array-bound case of CONV-16 below the first level (notes N5).

### I3-12. Promotion targets of wchar_t, char8_t, and char16_t

- Rules: CONV-1, glossary "integer promotion" (conversions)
- Kind: outside knowledge
- What happened: CONV-1 gives the matrix promotion only for `char32_t`. The glossary gives the method but not the widths. The reader used `int` for the other three from outside knowledge (notes N6). The `compound-integral` grid passed, but no ranking test separates `int` from `unsigned` for these types.
- Suggestion: Extend the matrix sentence of CONV-1 to every character type.

### I3-13. The worst element when element sequences tie

- Rules: ICS-16, ICS-17, ICS-25 (conversion-sequences)
- Kind: unclear
- What happened: "The worst element conversion" has no rule for two element sequences that tie but differ, such as two user-defined sequences through different functions. ICS-25 later needs to know which function the list sequence holds. The reader kept the first such element (notes N8). No test reached the case.
- Suggestion: Say what a list sequence holds when several elements share the worst kind. A short example with two user-defined element conversions would settle it.

### I3-14. Front end: a conversion function to a function reference

- Rules: OVL-15, OVL-29, ICS-11 (overload-resolution, conversion-sequences)
- Kind: front end stops (unsupported construct)
- What happened: `reductions/r1-function-reference-conversion.cpp` binds `F&` to `Source{}` through `operator F&()`. g++ and clang++ accept it and return 0. `tidal-fe` says "unsupported at line 4: conversion to a function reference". Program p05 (OVL-29 before OVL-27) stops the same way.
- Suggestion: Support a direct binding of a function reference to a conversion function's result, then rerun p05.

### I3-15. Front end: a braced list with one element of a derived class

- Rules: ICS-19 third bullet, ICS-36 (conversion-sequences)
- Kind: front end stops (unsupported construct)
- What happened: `reductions/r2-list-derived-element-class-parameter.cpp` calls `take({d})` with `take(Base)`, where `Derived` derives from `Base` and both have constructors. g++ and clang++ accept it. `tidal-fe` says "unsupported at line 5: constructor call". Without braces, or with aggregates, the program runs. Program p06 stops the same way.
- Suggestion: Support the ICS-19 case where a one-element list of a derived class initializes a base class parameter.

### I3-16. Front end: a converting constructor from a class argument

- Rules: ICS-3, ICS-4 (conversion-sequences)
- Kind: front end stops (unsupported construct)
- What happened: `reductions/r3-converting-constructor-argument.cpp` passes `Source{}` to `take(Wrap)` where `Wrap(Source)` exists. `tidal-fe` says "unsupported at line 4: converting constructor". The same program with `Wrap(int)` and an `int` argument runs. Program p12 (ICS-7, ICS-25) stops the same way.
- Suggestion: Support a converting constructor whose parameter has class type.

### I3-17. Front end: a cast to a reference from a class object

- Rules: ICS-4, OVL-15, ICS-45 (conversion-sequences, overload-resolution)
- Kind: front end stops (unsupported construct)
- What happened: `reductions/r4-reference-cast-from-class.cpp` writes `static_cast<int&>(s)` with `operator int&()`. `tidal-fe` says "unsupported at line 3: reference cast". The same cast inside a requires-expression (`reductions/r5-requires-reference-cast.cpp`) gives "unsupported at line 3: concept". Program p15, which tests ICS-45 through a concept, stops at the requires-expression.
- Suggestion: Support a cast to a reference type whose operand converts through a conversion function. Have the requires-expression report the inner construct, so the message names the real gap.

### I3-18. Front end: an explicit conversion function in direct initialization of a reference

- Rules: ICS-45, OVL-15 (conversion-sequences, overload-resolution)
- Kind: front end stops (unsupported construct, with a wrong message)
- What happened: `reductions/r6-explicit-conversion-reference-block.cpp` (`int& r(s);` in a block, `explicit operator int&()`) gives "unsupported at line 3: conversion to reference". `reductions/r7-explicit-conversion-const-reference-namespace.cpp` (`const int& r(s);` at namespace scope) gives "unsupported at line 4: reference bound to a temporary". By OVL-15 and ICS-45 the reference binds directly to `value`, so "bound to a temporary" describes the binding wrongly. The same programs without `explicit` run and give 0. g++ and clang++ accept all four forms. Program p17 stops the same way as r7.
- Suggestion: Support ICS-45's third bullet in direct initialization of a reference. Fix the message so that it does not call a direct binding a temporary.
