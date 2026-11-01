# Predict exercise: statements (R-statements)

- Date: 2026-11-01
- Role: clean-room reader
- Persona: programmer new to C++
- Inputs: AGENTS.md, and the six batch directories with their `specification/` copies (the test lists removed). Nothing was compiled.

## Batches

| Batch | Programs | runs | compiles | reject | depends |
|---|---|---|---|---|---|
| R-blocks | 26 | 13 | 4 | 7 | 2 |
| R-if | 67 | 18 | 16 | 33 | 0 |
| R-jumps | 57 | 18 | 11 | 26 | 2 |
| R-loops | 19 | 9 | 0 | 9 | 1 |
| R-range-for | 29 | 14 | 2 | 8 | 5 |
| R-switch | 36 | 15 | 3 | 16 | 2 |
| Total | 234 | 87 | 36 | 99 | 12 |

I predicted no link-fail, run-fail, or undefined outcomes. I answered `depends` only where an Evidence segment records a compiler split:
- R-blocks p22, p26 (Q-0096)
- R-jumps p46, p50 (Q-0041)
- R-loops p01 (Q-0044)
- R-range-for p02 (Q-0020), p06, p07, p10 (Q-0026), p25 (Q-0098)
- R-switch p07 (Q-0041), p14 (SW-7, EDG gives no warning)

## Findings

**RS-1.** R-blocks p25; R-switch p14. Rules STMT-4, SW-7. Kind: unclear.
- I needed to know what EDG does with a program built with `-Werror` and a warning flag. The STMT-4 Evidence says that EDG "skips these tests". The SW-7 Evidence says that EDG "gives no warning". So I answered `reject` for p25 and `depends` for p14, though the two cases look the same.
- Suggestion: for each recommended rule, say what outcome a reader should predict for EDG, or say that EDG is outside the comparison.

**RS-2.** R-blocks p23, p22, p26. Rule STMT-5. Kind: unclear.
- The Evidence says that GCC rejects a `case` or `default` label at the end of a block. It does not say which GCC versions do so. It also does not say whether every compiler accepts a plain label at the end of a block (p23).
- Suggestion: name the GCC versions. State that a plain label at the end of a block is accepted everywhere, if it is.

**RS-3.** R-blocks p10, p21. Rule STMT-6. Kind: needs example.
- STMT-6 is abstract. I needed cases like `T(x)->m = 5;`, which is an expression, and `T(c)(a);` or `T(*p) = nullptr;`, which are declarations.
- Suggestion: add two or three short example lines under STMT-6, one for each result.

**RS-4.** R-if p12. Rules IF-18, SINIT-1. Kind: hard to find.
- p12 asks which global initializers take the constant branch of `if consteval`. if.md does not say. I found the answer in `declarations/initialization/static.md` (SINIT-1 and its Evidence note), which has nearly the same example.
- Suggestion: link IF-18 to SINIT-1, and add the `fixed` and `moved` lines to the if.md examples.

**RS-5.** R-if p14. Rule IF-18 and the glossary entry for "constant evaluation". Kind: unclear.
- IF-18 says "at other times", and the glossary lists places where constant evaluation occurs only "for example". I could not confirm from the specification that a local non-constexpr initializer such as `int r = where() + ...` never takes the constant branch.
- Suggestion: give the full list of contexts that are constant-evaluated, or state that every other evaluation happens at run time.

**RS-6.** R-if p63. Rules IF-22, IF-13. Kind: outside knowledge.
- IF-22 says that parts "that are the same for every template argument" must be valid. I used outside knowledge to decide that `never_declared()`, a call with no arguments, is such a part, and that the compiler looks it up when it reads the template.
- Suggestion: add an example with a discarded call that has no dependent arguments, and link the term for a dependent name.

**RS-7.** R-if p11, p59. Rule IF-16. Kind: needs example.
- "A conversion that loses information is permitted" covers `2` to `bool`. I was less sure that it covers a pointer to `bool` (`&g`).
- Suggestion: add one example with an integer and one with an address.

**RS-8.** R-jumps p03. Rules JMP-15, CPY-4. Kind: hard to find.
- JMP-15 mentions a "move-only type" without a link. I needed CPY-4 to learn that a user-declared move constructor deletes the implicit copy constructor.
- Suggestion: link "move-only" in JMP-15 to CPY-4.

**RS-9.** R-jumps p22, p51. Rules JMP-16, OBJ-2. Kind: hard to find.
- p22 reads a skipped variable before any write, inside a `static_assert`. JMP-16 says only that the variable "has no value". I had to infer from the glossary that constant evaluation rejects the read.
- Suggestion: say in JMP-16 that constant evaluation rejects such a read. Add the p51 pattern (write, then read) as a valid example.

**RS-10.** R-jumps (whole batch). Rules JMP-6, JMP-17, JMP-11. Kind: unclear.
- The Evidence says 54 of 57 tests agree, but only the JMP-6 entry describes a split in outcome. The JMP-17 and JMP-11 entries describe diagnostics or elision that do not change the outcome. I could not find the third disagreeing test.
- Suggestion: name the rule for each disagreeing test, or correct the count.

**RS-11.** R-range-for p06, p07, p10. Rule RFOR-5. Kind: unclear.
- The Evidence says that GCC 14 "accepts a storage class". It does not say whether that covers all three of `static`, `thread_local`, and `extern`.
- Suggestion: list which storage classes GCC 14 accepts.

**RS-12.** R-range-for p15, p25. Rules RFOR-2, RFOR-7. Kind: unclear.
- The Evidence says that EDG uses the member form only for member functions. It does not say what EDG does next: fall back to free functions, or reject.
- For p25 (callable data members, no free functions), I guessed that EDG rejects. For p15 (data member `begin` of type `int`), I answered `reject` from RFOR-7, though EDG may use the free functions there.
- Suggestion: state EDG's outcome for both cases.

**RS-13.** R-range-for p18. Rule RFOR-7. Kind: missing.
- No rule rejects a range of pointer type, or of any other type that is not an array, a braced list, or a class. The page intro lists the three kinds of range, but RFOR-7 lists only the conditions for class ranges.
- Suggestion: add to RFOR-7 that the range must be one of the three kinds.

**RS-14.** R-switch (whole batch). Evidence of switch.md. Kind: unclear.
- The Evidence says "All seven compilers". The README and the other pages say there are eight baseline compilers.
- Suggestion: correct the count, or say which compiler is left out and why.

**RS-15.** R-switch p25. Rule SW-2. Kind: needs example.
- SW-2 allows several conversion functions to one type. It does not say which one runs. In p25 that choice would decide the value (1 or 2). I assumed the non-const function is chosen for a non-const object.
- Suggestion: say that ordinary overload resolution picks the function, and link that rule.

**RS-16.** R-switch p05, p30. Rules SW-2, SW-3. Kind: outside knowledge.
- From outside knowledge, I expect some compilers to warn about a `bool` condition and about a case value outside the range of `char`, even without `-Wall`. The specification is silent. These programs have no `-Werror`, so the outcome does not change.
- Suggestion: mention these default warnings in the Evidence, so a reader with `-Werror` can predict them.
