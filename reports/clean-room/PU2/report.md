# Principle use: batch PU2

- Date: 2026-11-04
- Role: clean-room reader
- Persona: a programmer new to C++
- Programs: 44 (p01 to p44; p11 has two units)
- Answers file: /home/ryanm/Code/tidal/build/exercises/PU2/answers.json

I read only the batch README, the programs, and the principles-only copy of the specification in that directory. I compiled nothing. The principles gave an outcome for 43 programs. One program (p35) is `not-decided`. Two answers (p03, p22) are doubtful, and one (p13/p15) relied on a second page to settle the reason.

## Findings

**PU2-1**
- Principle: CMP-P5 (with CMP-P1, CONV-P2, and the glossary entry "narrowing conversion")
- Program: p35
- Kind: no answer
- What happened: The first two assertions are settled by the built-in candidates in OVL-P4 and the result table in CMP-P5. The third assertion compares `int` with `double` by `<=>`. CMP-P5 rejects "a narrowing conversion". CONV-P2 and the glossary define narrowing as a conversion that "can change a value or drop a fraction". On x86-64, `int` to `double` changes no `int` value, so by that wording the conversion does not narrow and the result is `partial_ordering`. CMP-P5 also singles out "an unscoped enumeration against a floating-point value" as rejected, and says the enumeration has no lossless common type there, which hints that an integer against a floating-point value is rejected too. The principles never state the integer-to-floating case.
- Suggestion: In CONV-P2 or CMP-P5, say whether a conversion from an integer type to a floating-point type narrows when the target can hold every value of the source. Add an `int <=> double` line to the CMP-P5 table or its list of rejected forms.

**PU2-2**
- Principle: ORD-P2 and ORD-P4 (with ORD-P1 and OVL-P3)
- Program: p03
- Kind: doubtful
- What happened: `pick(T) requires (sizeof(T) > 1)` competes with `pick(T, ...)` in a call with one argument. The sequences tie, and only the first parameter takes part, so each template is at least as specialized as the other. Tie-breaker 6c speaks of a "trailing pack", and the model does not say whether a C-style ellipsis counts as one. ORD-P4 says 6d needs "parameter lists of the same length", and the model does not say whether the ellipsis counts toward the length. I answered `reject` (ambiguous), but the ellipsis could plausibly let 6c or 6d pick the constrained template.
- Suggestion: In ORD-P2 step 6, or in ORD-P4, say how an ellipsis parameter acts in 6c and in the "same parameter lists" condition of 6d. Add an example with a constrained template against one that has an ellipsis.

**PU2-3**
- Principle: COND-P2
- Program: p22
- Kind: doubtful
- What happened: COND-P2 says that when "two class types each convert to the other" the expression is rejected. `Wrap` converts to `Tag` only through a deleted conversion function. OVL-P1 says deletion is checked after the best function is chosen, so I read the deleted function as still counting as a conversion and answered `reject`. COND-P2 does not say this itself. A reader who treats a deleted conversion as "no conversion" would answer that `Tag` converts to `Wrap` and the program compiles.
- Suggestion: In COND-P2, say whether a conversion through a deleted or inaccessible function counts when the operator checks whether each operand converts to the other.

**PU2-4**
- Principle: UNI-P4 and ALN-P1
- Programs: p13, p15
- Kind: doubtful (the reason, not the outcome)
- What happened: Both principles require "a literal type". To see why `W` with `~W() {}` fails, I had to read the glossary entry. CE-P1 then says that a temporary inside an evaluation "can have any type", and fails only at an operation it cannot do. That sentence seems to permit `W{}`, until the reader works out that the destructor call is a non-constexpr call. Both paths give `reject`, but they give different reasons.
- Suggestion: In UNI-P4 and ALN-P1, add one sentence saying that a class whose destructor is user-provided and not `constexpr` fails the literal-type condition. Alternatively, say in CE-P1 how the literal-type demand for a width or an `alignas` operand relates to "a temporary can have any type".

**PU2-5**
- Principle: OPR-P3
- Programs: p17, p20, p34
- Kind: doubtful (outcome category)
- What happened: OPR-P3 says that a program declaring a literal operator for a reserved suffix "is not valid, and no diagnostic is required". It then gives a compiler table in which the outcome depends on the compiler. I answered `depends` for p17 and p20 from the table, and `runs` for p34, because the language suffixes keep their built-in meaning and no compiler rejects the declarations. The README's outcome list has no category for a program that is not valid with no diagnostic required, so I could not tell whether such a program should be answered `undefined`, `depends`, or by what the compilers do.
- Suggestion: State in OPR-P3, or in the exercise README, which outcome a test expects for a program that is not valid with no diagnostic required but that every compiler accepts.

**PU2-6**
- Principle: ORD-P1
- Program: p12
- Kind: doubtful
- What happened: The ORD-P1 example records that GCC does not reverse the parameters of a swapped candidate (Q-0211). That example uses two non-member templates. p12 uses a swapped member template, so ORD-6 (the object parameter) and ORD-7 (the reversal) both apply. I extended the recorded GCC behavior to this case and answered `depends`. Under that reading, Clang and EDG choose the swapped member and compile, and GCC chooses the written non-member and fails the `static_assert`. The model does not say whether GCC's deviation also covers a member's object parameter.
- Suggestion: In the ORD-P1 example, say whether GCC's failure to reverse also applies when the swapped candidate is a member template with an object parameter, or add such a case to the table.
