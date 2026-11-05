# Principle use: batch PU1

- Date: 2026-11-04
- Role: clean-room reader
- Persona: a programmer new to C++
- Programs: 66
- Answers: /home/ryanm/Code/tidal/build/exercises/PU1/answers.json

I read only the batch README, the programs, and the specification copy in the batch directory. I compiled nothing.

There are 65 decided answers and 1 `not-decided` (p61). The decided answers are 31 reject, 17 runs and 17 compiles. Below are the doubtful, unanswered and wrong-seeming cases.

## Findings

**PU1-1**
- Principle: ENUM-P3
- Program: p61
- Kind: no answer
- What happened: The enumerators 1ULL << 63 and -1 need a signed type wider than 64 bits. The table says an implementation "can provide wider integer types of its own" and rejects only if none of its types is wide enough. The principle does not say whether the baseline compilers count such a wider type, so I answered `not-decided`.
- Suggestion: Say whether the baseline compilers use an extended type for an enumeration that no standard type can hold, or say that they reject it.

**PU1-2**
- Principles: DEFARG-P1 and CX-P3
- Program: p13 (also bears on p07)
- Kind: doubtful
- What happened: CX-P3 says a consteval call with arguments that are not constant is an error. DEFARG-P1 says a default argument is evaluated "as if written at the call". Neither principle says whether a consteval call inside a default argument is checked where the default is written or at each call that uses it. I answered p13 `compiles` (no call uses the default) and p07 `runs`, both from DEFARG-P1.
- Suggestion: Add one sentence to DEFARG-P1 or CX-P3 that says when a consteval call in a default argument is evaluated and checked.

**PU1-3**
- Principle: LSUP-P1
- Programs: p37, p64 (also bears on p12)
- Kind: doubtful
- What happened: LSUP-P1 says the implementation "compares ordering values with 0 and converts them, so an ordering class needs those operations too". In p37 and p64, `std::strong_ordering` has no comparison and no conversion, and the only use is built-in `int <=> int`. The principle does not say which uses need those operations. I answered `runs` on the reading that only a defaulted `<=>` or a rewrite needs them.
- Suggestion: List the parts of an ordering class that each use needs: the static members for built-in `<=>`, comparison with 0 for a defaulted `<=>` and for rewrites, and conversions for a common category.

**PU1-4**
- Principle: ALN-P1
- Programs: p25, p39
- Kind: doubtful
- What happened: For a bit-field width, UNI-P4 requires a literal class with exactly one conversion function to an integer type, and that function must be `constexpr`. ALN-P1 says only "a class type whose conversion function gives such a type". The `alignas` operand class has two conversion functions in p25 and a conversion function that is not `constexpr` in p39. I answered `reject` for both, borrowing UNI-P4 for p25 and using CE-P1 for p39.
- Suggestion: Give ALN-P1 the same conditions as UNI-P4, or point to one shared statement. CE-P4 already lists the two places together.

**PU1-5**
- Principle: OPR-P3
- Programs: p21, p24, p28
- Kind: doubtful
- What happened: OPR-P3 says a literal operator for a suffix without an underscore is not valid, with no diagnostic required, and that "Compilers warn". The batch builds with `-pedantic-errors`, and the principle does not say whether that flag makes the warning an error. The README offers no outcome for "not valid, no diagnostic required", so I answered `runs` for all three. For p24, the principle also does not say whether `u8` counts as a suffix the language already uses. I took it as a prefix only.
- Suggestion: State the observed outcome under `-pedantic-errors`. Also list the integer and floating suffixes the language uses, or point to PRIM-P1.

**PU1-6**
- Principle: ACC-P3
- Program: p30
- Kind: doubtful
- What happened: ACC-P3 says befriending a member of a class template "makes that member of every specialization a friend". In p30, `Box<char>` is an explicit specialization, and the program writes its own `peek`. The principle does not say whether that new `peek` counts as "that member". I answered `runs`.
- Suggestion: Add one clause on explicit specializations, and say whether their members are covered.

**PU1-7**
- Principle: COND-P2
- Program: p50
- Kind: doubtful, possibly wrong-seeming
- What happened: The first bullet gives "an lvalue, with the union of their cv-qualifiers" when both operands are lvalues "of one type". In p50, one operand is a `const Box` lvalue and the other a `volatile Box` lvalue. If "one type" ignores cv-qualifiers, the result is a `const volatile Box` lvalue and the program compiles, which is what I answered. If it does not, the operands go to the prvalue path, and copying a `volatile Box` may fail. Neither principle says which.
- Suggestion: Say whether "one type" ignores cv-qualifiers, and say what happens when neither operand's cv-qualifiers include the other's.

**PU1-8**
- Principle: CMP-P3
- Program: p19
- Kind: doubtful
- What happened: CMP-P3 says the end of one row of an array of arrays equals the start of the next row, and that constant evaluation accepts such comparisons inside one complete object. So I answered `compiles`. The rejected item "a pointer past the end of one complete object" says "complete object", so it does not cover a row. Still, a newcomer could read a row as an object of its own.
- Suggestion: Add a short example with `&grid[0][3] == &grid[1][0]` in constant evaluation, so the row case is plainly separate from the separate-object case in p51.
