# I4: the conditional operator with class operands, from the rules alone

Date: 2026-11-05
Commit read: 8d7f3bb3
Kind: Implement (decision 0058, item E)
Role: clean-room reader in the role of an implementer

The reader returned this report as a message, and the integrator saved it here with the reader's files.

## Inputs

Phase 1 (blind). I read these pages only through `filter.py`, which drops each `## Evidence` segment with its subsections and each line with `:tests:`:

- `expressions/conditional.md`
- the glossary
- `declarations/initialization/references.md`
- `overloading/conversion-sequences.md`
- `overloading/overload-resolution.md` (OVL-P4, OVL-1 to OVL-17)
- `classes/conversion-functions.md`
- `classes/constructors.md` (CTOR-9, CTOR-10, CTOR-22)
- `foundations/conversions.md` (CONV-17, CONV-19)
- `classes/access.md` (ACC-2, ACC-12)
- `declarations/functions/deleted.md` (DEL-1)

The first version of the filter stopped skipping at the first subheading inside Evidence. I found the bug by listing the headings of `conditional.md` before reading any page, and fixed it. No Evidence text or `:tests:` line reached me. `notes.md` records this slip. It also lists 16 steps the text did not give me, written before I read any test.

The sha1 of `conditional.py` is 37da55eb14efdda80e8671e0c1121a2fc905575b, recorded in `conditional.sha1`. The file did not change after Phase 1, and `score.py` checks the hash.

Phase 2. I read the 68 tests under `tests/expressions/conditional/`. I also read the three other tests whose tidal-rules line cites a COND rule: `tests/programs/W2/clamp_sort.cpp`, `W2/order.cpp` and `W15/largest.cpp`. Those three use only `int` operands that the conditional tests already cover.

I built the front end into `OUT/target` and wrote 13 programs in `programs/`. Each ran under g++ and clang++ with `-std=c++23 -pedantic-errors` and under `tidal-fe run`. Six reductions are in `reductions/`.

## Score

85 of 85 assertions are right and none is wrong (`score-output.txt`). The 85 come from 50 tests: 43 assertions with a class operand and 42 on the non-class rows. Expected values come from each test's `tidal-expect` line.

One rejection cites COND-11 where the score case expected COND-5. The test (`two-classes-several-conversions`) cites both rules, so the difference is not an error.

No test covers the three cases where the frozen component and the compilers disagree (I4-1, I4-2, I4-3). The programs found those cases, and the score does not show them.

## Findings

### I4-1. The base row of COND-16 gives the wrong target

- Page and rules: `expressions/conditional.md`, COND-16 (target table, second row), COND-5.
- Kind: wrong.
- What happened: the row says the target is "that base class with the cv-qualifiers of X". For a `Derived` prvalue against a `const Base` lvalue, the row gives a `Base` prvalue. That prvalue differs from `const Base` in cv-qualifiers, so the pages reject with COND-5. The component and the front end both reject (`programs/p12-derived-prvalue-const-base-lvalue.cpp`).
- GCC and Clang both accept and give a `const Base` prvalue. Their answer matches the rule of the first row: the target is `T` when `T` has every cv-qualifier of `X`, and otherwise `X` does not convert.
- With that rule, every test that cites the base row still passes. A `const Derived` operand against a `Base` operand is still rejected, because neither operand converts. The note in `class-base-target-keeps-cv.cpp` already says "the base type must have at least those cv-qualifiers".
- Suggestion: give the base row the same target as the one-class row: `T`, if `T` has every cv-qualifier of the type of `X`. Rewrite the paragraph after the table, because a `const Derived` lvalue then does not convert to `Base` at all. Add a test with a `Derived` prvalue and a `const Base` lvalue.

### I4-2. Step 1 of COND-16 does not say whether an xvalue can bind

- Page and rules: COND-16 steps 1 and 2, REF-2, the glossary entry "bind directly".
- Kind: unclear.
- What happened: REF-2 says a `const T&` binds directly to an xvalue and to a class rvalue. COND-16 excludes only class prvalues. So the text gives an lvalue for a `const A` lvalue against an `A` xvalue, and the component returned `lvalue const A`. GCC and Clang both give a `const A` prvalue (`programs/p1-const-lvalue-xvalue.cpp`), and the front end agrees with them.
- For a `Derived` xvalue against a `const Base` lvalue, the compilers split (`programs/p13-derived-xvalue-const-base-lvalue.cpp`). GCC copies and gives a prvalue (run result 1). Clang binds and gives an lvalue (run result 0).
- The front end rejects that case with COND-5, "T and const T" (`reductions/r6-derived-xvalue-const-base-lvalue.cpp`). Neither compiler rejects, and the pages give an lvalue, so the front end contradicts the pages.
- If step 1 needs an lvalue (the GCC reading) and I4-1 is fixed, the result is a `const Base` prvalue.
- Suggestion: say in step 1 whether `X` must be an lvalue. For one class type, both compilers agree with "an lvalue only". Record the Clang and GCC split for a derived xvalue as a question. Add a test for the one-class case.

### I4-3. Unrelated cv-qualifiers on one class type do not always reject

- Page and rules: COND-P2 (second bullet), COND-3 row 2, COND-11, COND-5.
- Kind: wrong.
- What happened: COND-P2 says "Two such lvalues of one class type are rejected". COND-11 runs when the operands have different types and neither converts, and a `const A` and a `volatile A` meet that condition. I followed COND-P2 (notes.md item 2), so the component rejects.
- When `A` has `operator int() const volatile`, GCC and Clang accept and give `int`. So COND-11 runs.
- The front end stops with "unsupported: class conditional" (`reductions/r2-cv-class-lvalues-builtin.cpp`). When each side is a different class, the front end handles the expression.
- Suggestion: in COND-P2 and in the note of row 2, say that neither operand converts, so COND-11 decides. The expression is rejected only when no built-in candidate is viable, as for a class with no conversion function. In COND-11, say that types that differ only in cv-qualifiers count as different types.

### I4-4. The front end gives a prvalue for a conversion to `int&`

- Page and rules: COND-16 step 1, OVL-15, ICS-11.
- Kind: front end result that the pages contradict.
- What happened: `reductions/r1-conv-to-int-ref-lvalue.cpp` has a class with `operator int&()` against an `int` lvalue. Step 1 binds `int&` directly to the result of the conversion function, so the result is an `int` lvalue. GCC and Clang accept an assignment to the result. The front end rejects with ASN-2, "assignment to an rvalue".
- `class-converts-to-reference.cpp` covers the class-to-class case (`operator B&()`). No test covers a reference to a non-class type.
- Suggestion: add a test with a conversion to `int&` against an `int` lvalue, and fix the front end.

### I4-5. The front end stops on an `explicit` conversion function

- Page and rules: COND-16 step 3, CVF-11, COND-11, COND-5.
- Kind: front end stop.
- What happened: in `reductions/r3-explicit-conversion.cpp`, a class has only `explicit operator int()`, and the other operand is `0`. Neither operand converts, and COND-11 has no candidate, so the pages reject. GCC and Clang reject. The front end prints "unsupported: class conditional".
- Suggestion: reject with COND-5. Add the program as a reject test.

### I4-6. The front end accepts a conversion to an ambiguous base

- Page and rules: COND-16, CONV-17, DER-10, ICS-8.
- Kind: front end result that the pages contradict.
- What happened: in `reductions/r4-ambiguous-base.cpp`, the class `B` has two `T` bases, and a `B` lvalue meets a `T` lvalue. The conversion counts (ICS-8) and then fails (CONV-17). GCC and Clang reject. The front end accepts and runs.
- Suggestion: check the base conversion after COND-16 picks it. Add a reject test.

### I4-7. The front end accepts a conversion to a private base

- Page and rules: COND-16, ACC-12, CONV-17, REF-1.
- Kind: front end result that the pages contradict.
- What happened: in `reductions/r5-private-base.cpp`, a `P` lvalue meets a `T` lvalue, where `P` derives privately from `T`. GCC and Clang reject. The front end accepts.
- COND-16 says access does not stop step 3. It does not say what happens in step 1, where REF-1 asks for an accessible base (notes.md item 5). Both readings reject.
- Suggestion: reject with ACC-12. In COND-16, say that access does not stop step 1 either, so that both readings cite the same rule.

### I4-8. Rows 3 and 4 of COND-3 state no type condition

- Page and rules: COND-3 rows 3 and 4, COND-16.
- Kind: unclear.
- What happened: "Two xvalues" and "An lvalue and an xvalue" do not say that the operands share a type, or how cv-qualifiers combine. Taken literally, row 4 matches an `int` lvalue and a `long` xvalue.
- I read both rows as "one type apart from cv-qualifiers". For class operands I required identical types and sent the rest to COND-16. For an `A` xvalue against a `const A` xvalue, the component gives a `const A` xvalue, and both compilers agree.
- Suggestion: write the type condition into rows 3 and 4. For a class, say that operands with different cv-qualifiers go to the last row.

### I4-9. COND-8 covers only an lvalue and a prvalue

- Page and rules: COND-8, COND-3 row 4, COND-16 step 3.
- Kind: missing.
- What happened: an lvalue and an xvalue of one class type give a prvalue (row 4). The xvalue is moved into the result, so a deleted move constructor rejects the program. GCC, Clang and the front end (with CPY-23) all reject `programs/p9-lvalue-xvalue-deleted-move.cpp`. COND-8 names only the lvalue and prvalue case.
- The text also does not state the copies that step 3 makes. These are a `Box` lvalue to a `const Box` prvalue, and a `Derived` lvalue to a `Base` prvalue. The component checks all of these copies (notes.md items 10 and 11).
- Suggestion: extend COND-8. A class glvalue that becomes a prvalue is copied from an lvalue or moved from an xvalue, and the copy or move must be valid.

### I4-10. COND-11 and OVL-11 disagree on the value category

- Page and rules: COND-11, OVL-11.
- Kind: unclear.
- What happened: OVL-11 says that after a built-in candidate wins, only the conversion function applies, and then the operator's own rules decide. For two classes that convert to `int&`, those rules (COND-3 row 1) give an `int` lvalue. COND-11 says each candidate gives a prvalue.
- GCC, Clang and the front end treat the result as a prvalue (`programs/p3-two-int-refs.cpp`). I followed COND-11.
- Suggestion: add a sentence to OVL-11, or to the `?:` row of OVL-P4, saying that `?:` keeps the prvalue of COND-11.

### I4-11. COND-11 does not say which `(T, T)` candidates to try

- Page and rules: COND-11, OVL-P4.
- Kind: missing.
- What happened: the set of pointer types is infinite. The text does not say how an `int*` from one operand and a `const int*` from the other meet. `two-classes-promoted-types.cpp` expects `const int*`.
- I tried each pointer type that an operand gives, plus the composite pointer type of each pair. That choice passes the test, but it is my own.
- Suggestion: say that the only candidates that matter use the types the operands reach. Say that two pointer types meet at their combined type, as in COND-3 row 7, and give an example.

### I4-12. Knowledge the pages did not give

- Page and rules: COND-16 steps 1 and 2, ICS-45, CVF-11, CONV-19, CONV-1.
- Kind: outside knowledge.
- What happened: COND-16 steps 1 and 2 do not say that `explicit` conversion functions are left out. I took that from the copy context of step 3 (notes.md item 3).
- Some facts came from my own knowledge (LP64). These are the promotions and ranks of `char8_t`, `char16_t`, `char32_t` and `wchar_t`, and the widths that decide a signed type against an unsigned one.
- Suggestion: in COND-16, say that each step is a copy context, so `explicit` functions take no part. Link CONV-19 to a table of promotions for the character types.

### I4-13. Two readings of COND-16 needed the glossary or a guess

- Page and rules: COND-16 steps 2 and 3, OVL-15, the glossary entry "bind directly".
- Kind: hard to find.
- What happened: COND-16 excludes class prvalues from steps 1 and 2. It says nothing about `operator int()` against an `int` xvalue. OVL-15 says an rvalue reference takes conversion functions that return a value. Only the glossary ("no temporary object in between") told me to leave out every prvalue result (notes.md item 4).
- The "Otherwise" of step 3 can mean that step 3 runs only when no earlier step applies. I read it as "when step 1 or 2 does not bind". GCC, Clang and the front end agree with that reading (`programs/p11-step3-after-step1.cpp`).
- Suggestion: in steps 1 and 2, say "binds directly to a glvalue result". In step 3, say "If step 1 or 2 does not bind".

### I4-14. The choice among conversion functions is spread out

- Page and rules: COND-11, COND-16 step 3, ICS-7, ICS-25, ICS-37, CVF-4, ICS-8.
- Kind: hard to find.
- What happened: two jobs needed four ICS rules on one page and CVF-4 on another. One job was to pick one conversion function per operand. The other was to compare two built-in candidates. CVF-4 says a conversion to the class itself or to a base is never used.
- The two tests with an ambiguity (`class-ambiguous-conversion`, `two-classes-several-conversions`) depend on ICS-25, under which two different functions tie.
- Suggestion: give COND-11 one worked example that names ICS-25, using the `Wide` and `Narrow` classes of the test.
