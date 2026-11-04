# Predict batches: constant evaluation and evaluation order, reader A

- Date: 2026-11-04
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R8-constant-evaluation (154 programs), R8-evaluation-order (40)

I wrote both answers files:
- /home/ryanm/Code/tidal/build/exercises/R8-constant-evaluation/answers.json
- /home/ryanm/Code/tidal/build/exercises/R8-evaluation-order/answers.json

I answered `depends` wherever an Evidence segment records that the compilers differ:
- constant evaluation (25 programs): p08, p39, p42, p44, p45, p49, p58, p63, p69, p71, p74, p77, p82, p84, p101, p103, p117, p118, p119, p120, p123, p135, p143, p148, p152. The Evidence says the compilers agree on 130 of 154 tests, which leaves 24 that differ. So one of my `depends` answers is probably wrong. p49 and p08 are the least certain.
- evaluation order: none. The Evidence says all eight compilers agree on all 40 tests.

## R8A-1

- Pages and rules: foundations/constant-evaluation.md, CE-26, the undefined-rule table (row SEQ-4); foundations/evaluation-order.md, SEQ-4 Evidence
- Kind: contradiction
- What happened: In constant-evaluation p49, two assignments to `i` in one expression and a read of `i` beside `++i` are each evaluated in a constant. The CE-26 Evidence says all eight compilers accept unsequenced accesses. It also says the table shows "no test" for SEQ-4. The table's SEQ-4 row instead records "gcc-14: rejects". The evaluation-order Evidence says no compiler rejects such a pair during constant evaluation. I answered `depends` (GCC 14 rejects) for p49, and `compiles` for p130, which matches the three expressions that the CE-26 Evidence names.
- Suggestion: Make the CE-26 text agree with the table. Name the SEQ-4 case that GCC 14 rejects.

## R8A-2

- Pages and rules: constant-evaluation.md, CE-10 and its Evidence
- Kind: unclear
- What happened: In p15, four loops of 100000 iterations run in one evaluation. The Evidence says such loops pass, but the sentence is in the GCC bullet. The Clang bullet says Clang counts every step of the evaluation together, and it gives no step figure for loops. I answered `compiles` without knowing whether Clang's step limit allows 400000 iterations. p46 (fib(24), about 150000 calls) sits at the CE-10 table's "accepted" figure of 150000 calls, so the table does not settle p46 either. I answered `compiles` for p46.
- Suggestion: Say in the Evidence whether every compiler accepts the four loops. Give a Clang loop figure, or make the accepted call count in the table a round margin below what the tests use.

## R8A-3

- Pages and rules: constant-evaluation.md, CE-18, CE-3, Evidence for CE-18
- Kind: needs an example
- What happened: p117 calls a static member function through a reference parameter inside a new evaluation (a `constexpr` variable in a non-constexpr function). p152 does the same with a non-static `constexpr` member function that does not use the object. CE-18 covers a member function that does not use the object. It does not say whether a static member call through the reference counts as a use of the reference. The Evidence says only that Clang 18 "rejects the use of the reference". I answered `depends` (Clang 18 rejects) for both.
- Suggestion: Add a static member call to the CE-18 example. Say which form the Clang 18 split covers.

## R8A-4

- Pages and rules: constant-evaluation.md, CE-3, CE-14
- Kind: unclear
- What happened: In p91 a block-scope `constexpr` array has automatic storage. A plain `auto&&` reference binds to the array, and a `constexpr int` reads through the reference. CE-3 makes "a reference variable with a constant initializer" usable. Only CE-14 suggests that binding to an automatic object is not a constant initializer, and CE-14 is about the value of a constant expression. p93 is the same program at namespace scope. I answered `reject` for p91 and `compiles` for p93.
- Suggestion: In CE-3, say that a reference bound to an object with automatic storage has no constant initializer. Add p91's case to the examples.

## R8A-5

- Pages and rules: constant-evaluation.md, CE-22, undefined-rule table rows INIT-1 and OBJ-2
- Kind: unclear
- What happened: p104 copies an uninitialized `unsigned char` in a constant evaluation. CE-22 says the copy is rejected. The table gives EDG "some cases" for INIT-1, so I could not tell whether EDG rejects this case. p85 reads a local through a pointer after the local's function has returned. I found the governing rule only through the table (OBJ-2, STG-22, both "some cases" for GCC or EDG). I answered `reject` for both.
- Suggestion: Name the cases that each "some cases" entry rejects and accepts. Alternatively, add a line to the CE-22 Evidence about the `unsigned char` copy and a CE example of a dangling read.

## R8A-6

- Pages and rules: constant-evaluation.md, Evidence for CE-3 and CE-14 (Q-0193)
- Kind: unclear
- What happened: p08 declares a `constexpr` reference to a temporary aggregate whose reference member binds another temporary, then reads through the member. The Evidence states two facts. EDG rejects the declaration, and GCC rejects a read of a `const` class temporary. It does not say whether reading the int through the aggregate's reference member counts as the second case for GCC. I answered `depends`, but cannot say whether GCC also rejects.
- Suggestion: Give the outcome of each compiler for the declaration plus the read.

## R8A-7

- Pages and rules: classes/constructors.md, CTOR-17 Evidence; constant-evaluation.md
- Kind: hard to find
- What happened: In p126 a reference member has a default member initializer that binds a temporary, and a constant evaluation reads the member. The constant-evaluation page says nothing on this case. I found the answer (every compiler rejects) only in the CTOR-17 Evidence on the constructors page. I answered `reject`.
- Suggestion: Add a pointer from CE-9 or CE-14 to CTOR-17 for reference members bound to temporaries.

## R8A-8

- Pages and rules: declarations/constexpr.md, CX-3 Evidence; constant-evaluation.md
- Kind: hard to find
- What happened: Clang 18 rejects a `constexpr` function that can never give a constant. The constant-evaluation page mentions this only under CE-12. Several programs have such a function: p73's `fail`, and the functions that construct a class with a virtual base (p32, p59, p149). Each of those programs is rejected for another reason too, so my answers did not change. I could not tell whether a program that only defines such a function and never evaluates it would be rejected by Clang 18.
- Suggestion: In CE-P1 or CE-2, add one sentence that points to CX-3 and the Clang 18 behavior.

## R8A-9

- Pages and rules: constant-evaluation.md, CE-17 and CE-23; declarations/constexpr.md, CX-1
- Kind: needs an example
- What happened: p05, p19, p70, p113 and p150 each declare a `constexpr` variable of a class that is not a literal type. p68 is a union with one volatile and one non-volatile member. I needed CE-23, CX-1 and the CE-P1 sentence "Only a `constexpr` variable needs a literal type" together to tell p68 (literal, `compiles`) from p19 (all members volatile, `reject`). Each is stated, but in three places.
- Suggestion: Add to the CE examples one union with all volatile members and one with a non-volatile member.

## R8A-10

- Pages and rules: foundations/evaluation-order.md, SEQ-4, SEQ-7 and their Evidence
- Kind: unclear
- What happened: p24 (`i + i++`, one read and one write) and p30 (`i++ + i++`) build with `-Wall -Werror`. The Evidence quotes the Clang and GCC warnings for "multiple unsequenced modifications". It does not say that the warnings also cover a read beside a write. I answered `reject` for both, using outside knowledge for p24.
- Suggestion: Say in the SEQ-4 Evidence which of the two forms each compiler warns about under `-Wall`.
