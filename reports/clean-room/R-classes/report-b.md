# Predict batches: classes, reader B

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-copy-move (103 programs), R-derived (77 programs), R-members (90 programs)

The answers are written to these files:

- `/home/ryanm/Code/tidal/build/exercises/R-copy-move/answers.json`
- `/home/ryanm/Code/tidal/build/exercises/R-derived/answers.json`
- `/home/ryanm/Code/tidal/build/exercises/R-members/answers.json`

I answered `depends` for R-copy-move p04, p35, p41, for R-derived p77, and for R-members p12, p82, p84. Every other answer is a single outcome.

## RCB-1

- Batch and program: R-copy-move p04, p35, p41
- Rules: CPY-14, CPY-26
- What I needed: The three programs each default a constructor inside the class with a parameter type that differs from the implicit one, and none of them uses the constructor. The Evidence segment says GCC 14 rejects such a declaration and that the compilers agree on 101 of 103 tests. That leaves room for only two disagreements, but three programs fit the pattern. I could not tell which forms GCC 14 rejects: `const T&` where the implicit one takes `T&`, `const volatile T&`, or `const T&&`. I answered `depends` for all three.
- Suggestion: In the CPY-14 evidence line, list each parameter form and say which compilers reject it.

## RCB-2

- Batch and program: R-copy-move p35, p77
- Rules: CPY-2, CPY-14
- What I needed: CPY-2 calls a constructor taking `T&&` "with any cv-qualifiers" a move constructor. CPY-14 deletes a defaulted constructor whose type differs from the implicit one. I had to combine the two rules to conclude that a defaulted `const T&&` constructor is a move constructor and is deleted.
- Suggestion: Add a sentence or example to CPY-14 that names the `const T&&` case.

## RCB-3

- Batch and program: R-copy-move p37
- Rules: UNI-3, CPY-4, CPY-7
- What I needed: The union member has a deleted copy constructor, and that member's copy is not just non-trivial. UNI-3 talks only about non-trivial member functions. I applied CPY-7, which deletes a copy when a part's copy is deleted, to the union, but neither page says CPY-7 covers variant members.
- Suggestion: In UNI-3, say that a deleted or inaccessible function of a variant member also deletes the union's matching function.

## RCB-4

- Batch and program: R-copy-move p95
- Rules: CPY-7, CPY-5
- What I needed: Holder's implicit move assignment has to move members that have no move functions, because their destructors are user-declared. CPY-7 says the function is deleted when a part "cannot be moved in the same way". I had to infer that a part with no move function still counts as movable through its copy function.
- Suggestion: State in CPY-7 that a part with no move function is moved by its copy function, so that part does not delete the move.

## RCB-5

- Batch and program: R-copy-move p10
- Rules: CPY-1, OVL-27
- What I needed: CPY-1 says a constructor template is never a copy constructor. The page does not say what happens when a template specialization has the same signature as the implicit copy constructor. I found the tie-break (the non-template wins) on the overload resolution page.
- Suggestion: Add an example to copy-move.md that shows a template constructor competing with the implicit copy constructor, with a reference to OVL-27.

## RCB-6

- Batch and program: R-derived p60
- Rules: DER-8, DER-20
- What I needed: DER-8 says an inherited constructor keeps its access. The rule does not say where the access check happens. In p60 the base constructor is private, D is a friend of B, and the call is at namespace scope. I guessed that the check happens at the point of use as if B's own constructor were named there, so friendship with D does not help, and answered reject.
- Suggestion: State the context of the access check for an inherited constructor, and add an example with a friend.

## RCB-7

- Batch and program: R-derived p04
- Rules: DER-8, DER-20
- What I needed: D inherits constructors from B, and the program builds a D from a B. The copy constructor is not inherited, but parenthesized aggregate initialization could still build a D from a B. I found in declarations/initialization/list.md that inherited constructors make D a non-aggregate.
- Suggestion: Add to DER-8 a note that a class that inherits constructors is not an aggregate, with a reference to the aggregate rule.

## RCB-8

- Batch and program: R-derived p47, p57, p50
- Rules: DER-4, NAM-36
- What I needed: In p57 the injected class name of one base is reached through two non-virtual paths. In p50 two different classes share the name. DER-4 says a type found through two paths to the same class is not ambiguous, and NAM-36 covers different bases. I applied the DER-4 exception to the injected class name, but no rule says the injected class name counts as such a type.
- Suggestion: Give an example of each case in derived.md: one class through two non-virtual paths, and two different bases that share a name.

## RCB-9

- Batch and program: R-members p25
- Rules: CLS-10, CLS-19
- What I needed: The program checks that an empty base subobject has the address of the derived object. CLS-10 says the base "can" have that address. CLS-19 promises the shared address only for the first data member. I predicted `runs` because I assumed every compiler places the empty base at the start, but the specification does not promise it.
- Suggestion: Say whether a standard-layout class guarantees the shared address for an empty base, or record what the compilers do in Evidence.

## RCB-10

- Batch and program: R-members p82
- Rules: CLS-13
- What I needed: The program counts destructor runs of an explicit object parameter that was built directly from a prvalue, and expects one run before the next statement. The specification does not say when a by-value explicit object parameter is destroyed. I needed that point to be no later than the end of the full-expression.
- Suggestion: State when a by-value parameter, including an explicit object parameter, ends its life, or give a reference to the rule that does.

## RCB-11

- Batch and program: R-members p59, p88
- Rules: CLS-21, CLS-3
- What I needed: CLS-21 says an implementation need not diagnose a missing definition before linking. The rule does not say the linker reports it. For p59 the outcome depends on whether the compiler emits a reference to the member at `-O0`, and I assumed it does and answered `link-fail`.
- Suggestion: Add to the CLS-21 Evidence line what the matrix compilers do at `-O0` when a reference binds to an undefined const static member.
