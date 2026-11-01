# Clean-room report: R-expressions, reader A

- Date: 2026-11-01
- Role: clean-room reader
- Persona: programmer new to C++
- Inputs: AGENTS.md, and the four batch directories with their `specification/` copies (the test lists removed). The four copies are identical. Nothing was compiled or run.

## Batches and counts

| Batch | Programs | reject | compiles | runs | run-fail | depends | undefined |
|---|---|---|---|---|---|---|---|
| R-additive | 56 | 23 | 17 | 6 | 2 | 7 | 1 |
| R-multiplicative | 52 | 16 | 18 | 9 | 4 | 5 | 0 |
| R-shift | 25 | 13 | 9 | 1 | 2 | 0 | 0 |
| R-comparison | 87 | 24 | 35 | 17 | 0 | 11 | 0 |
| Total | 220 | 76 | 79 | 33 | 8 | 23 | 1 |

The Evidence segments decided most programs, including every `depends` answer. The rule pages alone did not settle them. The shift page was complete for its batch, so it has no findings.

## Specification findings

### REA-1
- Batch and program: R-additive p35
- Rules: ADD-10, ADD-8
- Kind: missing, outside knowledge
- What I needed: The program adds a run-time nonzero value to a null pointer under `-fsanitize=undefined`. The ADD Evidence covers this case only during constant evaluation. It does not say whether the undefined sanitizer of Clang or GCC stops it at run time. I answered `undefined`. From outside knowledge, I expect Clang's sanitizer to stop it and GCC's to maybe not.
- Suggestion: Add an Evidence line for each sanitizer and a nonzero offset from a null pointer at run time. If the compilers differ, say so, so a reader can answer `depends`.

### REA-2
- Batch and program: R-additive p02
- Rules: ADD-1, ADD-10, ADD-3
- Kind: needs example, unclear
- What I needed: `arr + 'a' - 'a'` groups left to right, so the first step moves 97 elements past a 4-element array. That step is undefined. The page has no example of a temporary out-of-range step inside a longer expression. It also does not say what happens to a namespace-scope initializer that fails constant evaluation. My guess is that it becomes a run-time initialization, which would then be undefined. I answered `compiles` because the program has no main.
- Suggestion: Add an example where `p + n - n` is undefined although the final position is valid. Point to the rule that says what a non-constant namespace-scope initializer does.

### REA-3
- Batch and programs: R-multiplicative p03, p06, p08
- Rules: CONV-7, MUL-18, MUL-17, TYP-10
- Kind: missing, outside knowledge
- What I needed: These programs depend on which of the two neighbors a result rounds to when it falls exactly halfway. CONV-7 leaves the choice to the implementation. TYP-10 gives the precision but not the rounding mode. I assumed round to nearest, ties to even, on the matrix targets.
- Suggestion: State the default rounding mode on the matrix targets, near TYP-10 or MUL-20. Add one example, such as 1 plus half an ulp giving 1.

### REA-4
- Batch and programs: R-comparison p56, p73. Also R-additive p07, p41, p53 and R-multiplicative p07, p38, p44.
- Rules: ADD-16, MUL-16, CMP-21, TR-6
- Kind: missing
- What I needed: For the deprecated enumeration mixes, which warning flag each compiler uses, and whether that warning is on by default in C++23 mode. p56 has only `-Werror`, and p73 has `-Werror -Wdeprecated`. I assumed the warning is on by default, so both are rejected. For p41 and p07 (multiplicative), I assumed `-pedantic-errors` does not turn a deprecation warning into an error. The specification does not say that either.
- Suggestion: In the Evidence for ADD-16, MUL-16, and CMP-21, name each compiler's flag, say whether it is on by default, and say whether `-pedantic-errors` turns it into an error.

### REA-5
- Batch and programs: R-comparison p42, p54, p86, p65
- Rules: CMP-1, CMP-14, TR-6
- Kind: unclear
- What I needed: These programs use `-Werror -Wparentheses` on chained comparisons of variables, such as `a < b < c`, `low <= x <= high`, and `left < middle > right`. The Evidence says that Clang 22 treats the warning as an error by default, that Clang 18 does not warn for variables, and that GCC warns about some equality chains. It does not say that GCC warns on each relational chain under `-Wparentheses`, which I assumed. It does not say whether EDG accepts the flag or skips the test, or whether Clang 18 warns for constants, which matters for p65. I answered `depends` for all four.
- Suggestion: Add a small table: rows are the compilers, and columns are a relational chain of variables, a relational chain of constants, and an equality chain. Each cell gives the result with no flags and with `-Werror -Wparentheses`.

### REA-6
- Batch and program: R-comparison p62
- Rules: CMP-7, CMP-20
- Kind: unclear
- What I needed: `record.values + 2 <= &record.tail` compares a pointer one past an array member with the next member, during constant evaluation. CMP-20 rejects such a comparison during constant evaluation, but its wording is about equality. The CMP-7 Evidence says every compiler orders "an array member and the member after it". I could not tell whether that includes the one-past pointer. I answered `compiles`.
- Suggestion: Say whether CMP-20 covers relational operators, and add an example of the one-past pointer of an array member compared with the next member, both with `==` and with `<`.

### REA-7
- Batch and program: R-comparison p37
- Rules: CMP-8
- Kind: missing
- What I needed: CMP-8 says `a < b` and `a > b` on unrelated pointers need not agree. p37 returns success only if they disagree. The specification gives no run-time evidence of what the compilers actually do. I answered `runs` from the idea that two distinct objects have different addresses, which the specification does not promise.
- Suggestion: Add run-time evidence for relational comparison of two unrelated globals. If the outcome is not reliable, say so.

### REA-8
- Batch and program: R-comparison p52
- Rules: CMP-24, CMP-11, CONV-13, CMP-P2
- Kind: hard to find
- What I needed: One array operand of `<=>` with one pointer operand. CMP-11 covers only the case where both operands are arrays, and CMP-24 bans only two arrays. The principle CMP-P2 says an array operand becomes a pointer, but no rule says so for a single array operand of `<=>` or `==`. I answered `compiles` with the result `less`.
- Suggestion: Add a rule or a sentence in CMP-2 or CMP-6: an array operand alone converts to a pointer for every comparison operator, including `<=>`. Add an example.

### REA-9
- Batch and program: R-additive p54
- Rules: ADD-10, CE-9
- Kind: unclear
- What I needed: A pointer into row 0 of a 2x2 array is moved by 3, and the program reads through it during constant evaluation. ADD-10 makes this undefined because each row is a separate array. The Evidence says GCC accepts out-of-range pointers and rejects only reads. It does not say whether GCC treats a read of another row as a read outside the array. I answered `reject`.
- Suggestion: Add an Evidence line for each compiler on a read of the next row through a pointer into the current row, during constant evaluation.

### REA-10
- Batch and programs: R-additive p46, p56
- Rules: ADD-18, ADD-12
- Kind: needs example
- What I needed: With `-m32`, the cutoff cases sit right at INT_MAX: EDG "rejects every position past INT_MAX", GCC "rejects an array of more than INT_MAX bytes", and Clang rejects a difference of 2147483648. I read these to mean that an array of exactly INT_MAX bytes, with a difference of exactly INT_MAX, is accepted by all three (p56 compiles), and that a 2^31-byte array is rejected by all three (p46 rejects).
- Suggestion: Give the boundary values in one example: the largest array and the largest difference that each compiler accepts.

### REA-11
- Batch: all four (README outcome categories)
- Programs: R-additive p30, p31. R-multiplicative p09, p19, p35, p48. R-shift p05, p24.
- Rules: ADD-9, MUL-9, MUL-10, SHF-7, TR-8
- Kind: unclear
- What I needed: These programs reach undefined behavior while built with a sanitizer that stops the run. The README offers both `undefined` and `run-fail`, and it does not say which one fits. I used `run-fail` whenever the Evidence says the sanitizer stops the operation. I used `undefined` only where the specification does not say (REA-1).
- Suggestion: State in the README or in TR-8 that a program whose undefined operation a sanitizer stops is answered `run-fail`.

### REA-12
- Batch and program: R-multiplicative p18
- Rules: MUL-14
- Kind: missing
- What I needed: `(i = 7) % i` under `-Wall -Werror`. The MUL-14 Evidence quotes each compiler's warning text but does not name the flags. It also does not say whether `-Wall` turns on GCC's warning or whether Clang warns by default. I assumed both compilers warn under `-Wall`, so the program is rejected.
- Suggestion: Name the warning flag for each compiler, and say whether `-Wall` enables it.

### REA-13
- Batch and programs: R-comparison p34, p48, p85 (in context, not blocking)
- Rules: CMP-8
- Kind: needs example
- What I needed: CMP-8 lists the cases where the result is unspecified, but it does not say directly that the comparison is still valid at run time and in a function that is not evaluated as a constant. I inferred from the "Implementation Requirements" placement that a program that ignores the result compiles and runs.
- Suggestion: Add one sentence to CMP-8 saying that these comparisons are valid and give `true` or `false`. Only constant evaluation rejects the unrelated-object case.
