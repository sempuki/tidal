# Predict batches: unwinding and names, reader D

- Date: 2026-11-04
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R8-unwinding (60 programs), R8-names (240 programs)

Both answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R8-unwinding/answers.json
- /home/ryanm/Code/tidal/build/exercises/R8-names/answers.json

I answered `depends` wherever an Evidence segment records that the compilers differ:
- unwinding: p06, p11 (UNW-3, Clang 18 calls the unsized form), p13, p18, p26 (UNW-1, Q-0365), p19 (UNW-8 table), p23 (UNW-2, GCC 14), p60 (UNW-7 table)
- names: p30 (NAM-38, Q-0364), p47 (PFX-11, Q-0316), p170 (NAM-27, Q-0311), p213 (NAM-39, Q-0362)

I did not compile or run any program. I read only the specification copies in the two batch directories, the two READMEs, and the form report.

## R8D-1

- Pages and rules: exceptions/unwinding.md, UNW-1, Evidence table for Q-0365
- Kind: unclear
- What happened: Programs p13, p18 and p26 build a function's result and then a local destructor throws. The Evidence table lists "Clang 22" as the family that still differs. The Evidence bullet below it says "Clang never destroys the result" and does not name a version. I could not tell whether Clang 18 behaves like Clang 22 or like GCC. I answered `depends` for all three (GCC runs, Clang run-fail).
- Suggestion: Name every compiler that leaves the result alive, Clang 18 included or excluded, in the same place as the table.

## R8D-2

- Pages and rules: exceptions/unwinding.md, UNW-11, UNW-3; expressions/new-delete.md, NEW-23
- Kind: missing
- What happened: In p48 the program declares a global placement `operator new` with one extra size parameter and no deallocation function. The implicit global sized `operator delete` has the matching parameters, and it is a usual deallocation function. UNW-11's example and its Evidence cover only the case where a class declares both functions. Nothing says whether the implicit global sized form counts as the selected function. I answered reject from the text of UNW-11, with low confidence.
- Suggestion: State whether UNW-11 applies when the matching function is the implicit global sized form, and add the compilers' behavior for that case to the Evidence.

## R8D-3

- Pages and rules: foundations/names.md, NAM-6, NAM-37; expressions/postfix.md, PFX-11 and its Evidence
- Kind: hard to find
- What happened: p47 and p66 use a member access in a member declaration of the class being defined. The names page says a member declaration sees only earlier members, so I first predicted p47 compiles. The compiler split on p47 (Clang rejects because the class is incomplete, Q-0316) appears only on the postfix page. The names page also gives no `reject` rule for a member access that finds nothing or finds a type (p26, p98, p174). The rule for those cases is PFX-11, on the postfix page. I answered `depends` for p47 and cited PFX-11 for the others.
- Suggestion: In NAM-6 or NAM-37, cite PFX-11 for member access and mention Q-0316.

## R8D-4

- Pages and rules: exceptions/unwinding.md, UNW-2 and its Evidence
- Kind: unclear
- What happened: The Evidence says "GCC 14 differs for the temporary of a reference member", and p23 tests that case. The sentence does not say what GCC 14 does instead, so I could not predict the GCC 14 outcome. It could be a different order, or a temporary that never ends. I answered `depends`.
- Suggestion: Say what GCC 14 does, for example "destroys the temporary after the member a" or "does not destroy the temporary".

## R8D-5

- Pages and rules: exceptions/unwinding.md, UNW-2; expressions/lambdas.md
- Kind: needs an example
- What happened: p09 has a lambda with an init-capture, then a simple capture, then an init-capture that throws. UNW-2 says the initialized captures are destroyed in reverse order. The page does not say in which order the captures are initialized. I assumed list order and answered runs.
- Suggestion: Cite the rule that gives the initialization order of captures, or add a sentence to UNW-2.

## R8D-6

- Pages and rules: foundations/names.md, NAM-3; statements/blocks.md, STMT-8
- Kind: unclear
- What happened: In p115 a function's outermost block declares a variable with the parameter's name. NAM-3 says the outermost block is one scope with the parameter list. NAM-3 also says STMT-8 is the specific rule for a block. STMT-8 speaks only of "a block" and does not mention parameters. I could not tell which rule a test cites, so I cited NAM-3. The outcome (reject) was clear.
- Suggestion: Say in NAM-3 or STMT-8 which rule is the specific one when a block declaration clashes with a parameter.

## R8D-7

- Pages and rules: foundations/names.md, NAM-30 (last paragraph)
- Kind: missing
- What happened: In p138 the argument's class has a private base, and the function that ADL finds takes a pointer to that base. NAM-30 says the call "can then fail the access check" but gives no rule ID for that check. I answered reject and cited NAM-30 and ACC-14. ACC-14 covers the injected class name, so it is probably not the right rule.
- Suggestion: Cite the access rule that rejects the conversion to a private base.

## R8D-8

- Pages and rules: exceptions/unwinding.md, UNW-8 Evidence table
- Kind: needs an example
- What happened: The rule's examples and the Evidence table helped with p19, p22, p36, p40, p49 and p59. One part was still hard. p40 is a rethrow inside a handler, and the rule says the rethrow is not enclosed by that handler's try block. I had to read TRY-8 to confirm that a test with `-Wall` still gets the warning, because the Evidence says `-Wall` adds none of these warnings. I read "adds none" to mean the warnings already fire by default.
- Suggestion: Write "Clang and GCC give these warnings by default. `-Wall` neither adds nor removes them."
