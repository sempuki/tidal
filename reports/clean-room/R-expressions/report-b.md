# Clean-room report: R-expressions, reader B

- Date: 2026-11-01
- Role: clean-room reader
- Persona: programmer new to C++
- Inputs: AGENTS.md, and the four batch directories with their `specification/` copies (the test lists removed). Nothing was compiled or run.

## Batches

| Batch | Programs | runs | compiles | reject | run-fail | depends |
|---|---|---|---|---|---|---|
| R-conditional | 37 | 9 | 14 | 13 | 0 | 1 |
| R-lambdas | 62 | 23 | 14 | 22 | 1 | 2 |
| R-casts | 54 | 15 | 17 | 18 | 3 | 1 |
| R-new-delete | 125 | 51 | 11 | 37 | 7 | 19 |

R-conditional has two answers (p23 and p37) that I gave from the specification text, although general C++ knowledge says the other outcome. REB-1 explains them.

## Findings

**REB-1**
- Batch and program: R-conditional p23, p37 (also p29).
- Rule IDs: COND-3, CONV-19.
- Kind: wrong.
- What I needed: the result type when both operands are prvalues of one arithmetic or enumeration type, such as two `char` values or two enumerators of one enumeration. The COND-3 table sends every arithmetic or enumeration pair through the usual arithmetic conversions. CONV-19 step 2 promotes both operands, so the specification gives `int` for `true ? 'a' : 'b'` and for `true ? A : B` with both of type E. General C++ knowledge says the result keeps the shared type (`char`, `E`). The programs' assertions expect the shared type. I answered `reject` from the specification text. If the shared-type reading is right, both programs compile.
- Suggestion: add a row before the arithmetic row. Two prvalues of the same type give a prvalue of that type, with no conversion. The "two integer literals 0 give int" note then follows from that row.

**REB-2**
- Batch and program: R-conditional p32.
- Rule IDs: COND-3.
- Kind: missing.
- What I needed: what happens to array operands, such as two string literals of different lengths. No row of the table covers arrays, and the pointer row needs pointers. I assumed both arrays become `const char*` (outside knowledge).
- Suggestion: say that array and function operands become pointers before the table applies, unless both are lvalues of the same type.

**REB-3**
- Batch and program: R-conditional p34.
- Rule IDs: COND-3, CONV-19.
- Kind: unclear.
- What I needed: whether two different enumeration types in `?:` are accepted under `-pedantic-errors`. The Evidence says only that compilers warn the use is deprecated. I guessed that a deprecation warning is not a pedantic error.
- Suggestion: state in the Evidence that every compiler accepts the program under `-pedantic-errors`.

**REB-4**
- Batch and program: R-lambdas p10.
- Rule IDs: LAM-21.
- Kind: outside knowledge.
- What I needed: whether the deprecation warning for an implicit `this` capture through `[=]` is on under `-Wall -Wextra -Werror`, and so whether the program is rejected. LAM-21 says only that an implementation should warn. I relied on outside knowledge that GCC and Clang warn by default in C++20 and later.
- Suggestion: record in the Evidence which compilers warn, and under which flags.

**REB-5**
- Batch and program: R-lambdas p27; R-casts p13, p16; R-new-delete p48, p62, p105.
- Rule IDs: LAM-19, CAST-14, NEW-5.
- Kind: missing.
- What I needed: whether the sanitizer named in the README stops each undefined case. For CV-5, NEW-40, and NEW-32 the Evidence says which sanitizer stops the program, and I used that for R-casts p31 and R-new-delete p20, p36, p106. Several other pages give nothing:
  - LAM-19: use-after-scope through a reference capture (p27).
  - CAST-14: a downcast to the wrong class under `vptr` (p13), and an out-of-range enumeration under `undefined` (p16).
  - NEW-5: a double delete and a `delete`/`new[]` mismatch under `address` (p48, p62, p105).

  I answered `run-fail` from outside knowledge.
- Suggestion: add one Evidence line for each undefined rule that a sanitizer test uses, naming the sanitizer that stops the program.

**REB-6**
- Batch and program: R-casts p31.
- Rule IDs: CAST-5, CV-5.
- Kind: hard to find.
- What I needed: the sanitizer result for a write through `const_cast` to a `const` global. The answer is in the Evidence of `declarations/declarators.md` under CV-5. The casts page does not point there.
- Suggestion: in the CAST-5 Evidence, say that the address sanitizer stops a write to a `const` object with static storage, with a link to CV-5.

**REB-7**
- Batch and program: R-new-delete p05, p79, p104 (also p26).
- Rule IDs: NEW-35, LSUP-8.
- Kind: unclear.
- What I needed: what Clang does for a negative run-time bound and for a size too large. The Evidence says GCC throws `std::bad_array_new_length` in both cases. The following sentence says Clang passes an impossible size to the allocation function, which then throws `std::bad_alloc`. That sentence comes right after the case of an initializer longer than the bound, so I could not tell whether it covers all three cases or only that one. I took it to cover all three and answered `depends` for p05, p79, and p104.
- Suggestion: give Clang's behavior for each of the three erroneous cases separately, for example in a small table with GCC beside it.

**REB-8**
- Batch and program: R-new-delete p11.
- Rule IDs: LSUP-11, NEW-37.
- Kind: hard to find.
- What I needed: whether constant evaluation can call `::operator new` by name inside an allocator class in a namespace other than `std`. NEW-37 covers `new` expressions only. The answer is in LSUP-11 on the language-support page, and the new-delete page never mentions LSUP-11.
- Suggestion: add to NEW-37 or NEW-19 that a call by name of an allocation function is valid during constant evaluation only inside `std::allocator`, with a link to LSUP-11.

**REB-9**
- Batch and program: R-new-delete p117.
- Rule IDs: NEW-1, NEW-2.
- Kind: missing.
- What I needed: whether `new int[3](1, 2)` is valid, and what values it gives. NEW-2 covers `()` and braces for arrays but not a parenthesized list of values. From outside knowledge I assumed it works like a braced list and gives 1, 2, 0.
- Suggestion: say whether a parenthesized list initializes an array in `new`, and whether it differs from braces (for example on narrowing). Add the case to the examples.

**REB-10**
- Batch and program: R-new-delete p03, p76.
- Rule IDs: NEW-19, NEW-26.
- Kind: hard to find.
- What I needed: whether calls by name to the sized deallocation forms compile on every compiler. NEW-19 lists the sized forms as always present. The note that Clang 18 does not declare them sits under NEW-26 and NEW-28 in the Evidence. A reader of NEW-19 alone would answer `runs`.
- Suggestion: put the Clang 18 split for the sized forms (Q-0125) under NEW-19 in the Evidence too, or mark NEW-19 itself with the question.

**REB-11**
- Batch and program: R-new-delete p49, p118.
- Rule IDs: NEW-23, NEW-24.
- Kind: unclear.
- What I needed: whether a compiler that chooses the plain function over a matching template still instantiates the template's body. Only EDG's behavior is given ("does not instantiate"). For p49 I assumed GCC and Clang do not instantiate it either.
- Suggestion: state for each compiler whether the template body is instantiated when the plain function wins.

**REB-12**
- Batch and program: R-new-delete p12, p14, p33, p99.
- Rule IDs: NEW-10, NEW-33, LSUP-7.
- Kind: needs example.
- What I needed: whether a program can write its own definition of the non-allocating placement forms when it includes no header, and whether that definition counts as the form that NEW-33 describes. The tests rely on this, and I assumed it.
- Suggestion: add an example to NEW-10 or LSUP-7 that declares `operator new(size, void*)` without a header, and say that it counts as the non-allocating form.

**REB-13**
- Batch and program: R-new-delete p01, p30.
- Rule IDs: NEW-30, NEW-25.
- Kind: needs example.
- What I needed: confirmation that a class of ordinary alignment that declares only an aligned `operator new` is rejected, and that a private `operator delete` rejects a plain `new` even when nothing can throw. The rules say both, but each fact sits deep in the text or the Evidence.
- Suggestion: add a two-line example for each to the Examples section.

**REB-14**
- Batch and program: R-lambdas p01.
- Rule IDs: LAM-20.
- Kind: unclear.
- What I needed: whether a copy capture of an `int` must give a closure at least as large as an `int`. LAM-20 says the size is unspecified, so `sizeof(bycopy) >= sizeof(int)` is not promised by the text. I answered `runs`.
- Suggestion: say what the size of a closure is guaranteed to be, if anything (for example, at least the size of its copy-captured members), or note that every compiler meets this.
