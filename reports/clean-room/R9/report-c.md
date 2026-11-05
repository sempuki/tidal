# Predict batches: template declarations and deduction, reader C

- Date: 2026-11-04
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R9-declarations (131 programs), R9-deduction (127 programs)

Both answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R9-declarations/answers.json
- /home/ryanm/Code/tidal/build/exercises/R9-deduction/answers.json

I answered `depends` wherever an Evidence segment records that the compilers differ and the question is still open:
- declarations: p06, p56, p95 (Q-0195); p52 (Q-0200); p67 (Q-0198); p70, p84 (Q-0197); p128 (Q-0196)
- deduction: p27, p28 (Q-0295); p36 (Q-0220); p61 (Q-0222); p88 (Q-0220)

For settled questions where a family still differs, I gave the outcome that the rules give: declarations p02, p25 (Q-0199), p69 (Q-0296), p73 (Q-0385); deduction p47 (Q-0208), p78 (Q-0209), p90 (Q-0215), p102 (Q-0219).

I used no `principles` entries, because a rule decided each program.

## R9C-1

- Pages and rules: templates/deduction (DED-39, DED-30, DED-14); expressions/new-delete (NEW-23)
- Kind: unclear
- What happened: Deduction p03 has two placement deallocation templates, one taking `T*` and one taking `const T*`, and the allocation function takes `Arena*`. DED-39 says a template matches if deduction makes its parameter types "the same". DED-30 turns off DED-6 and DED-7 for a target type, but it does not say whether DED-14 still lets a qualification adjustment through. I read "the same" strictly, so only the `T*` template matches, and I predicted `runs`. If DED-14 applied, both templates would match, ordering would pick `const T*`, and the run would fail.
- Suggestion: State in DED-39 that the differences of DED-14 and DED-15 do not apply. Add an example in which a `const T*` template does not match an `Arena*` allocation parameter.

## R9C-2

- Pages and rules: templates/deduction (DED-39); expressions/new-delete (NEW-24, NEW-25); templates/ordering (ORD-3, ORD-9)
- Kind: hard to find
- What happened: Deduction p61, p88, and p36 each turn on what happens when two deallocation functions match. DED-39 covers the case of a function plus a template. It does not cover two templates that ordering cannot separate (p61), and it does not cover a deleted plain function (p88). I found the answers only in the NEW-24 and NEW-25 text and the new-delete Evidence: neither function is called, EDG rejects the tie (Q-0222), EDG and GCC choose the function, and EDG rejects a deleted one. For p61, I also needed ORD-3's sentence that a template parameter appearing in no compared type needs no value. That sentence tells me that `template <class T, class U = int>` and `template <class T>` with the same `T*` parameter are ambiguous.
- Suggestion: In DED-39, link NEW-24 for the tie case and NEW-25 for a deleted function. Add a short example of two templates that differ only in an unused defaulted parameter.

## R9C-3

- Pages and rules: templates/deduction (DED-15, DED-29)
- Kind: needs an example
- What happened: Deduction p120 passes `Derived<int>*` to a `Base<T>*` parameter, and `Derived<int>` has a failing `static_assert`. DED-29 says an error in the body of a class that substitution instantiates is fatal. The search of the bases in DED-15 happens in the match step, though, before substitution. Nothing says that the search instantiates the argument's class. I predicted `reject`, which matches the DED-29 table, but I reached that answer only by assuming the search needs the complete class.
- Suggestion: Add a sentence to DED-15 saying that the search of the bases instantiates the argument's class when it is a specialization, and that DED-29 then applies.

## R9C-4

- Pages and rules: templates/deduction (DED-24)
- Kind: program predicted only from outside knowledge
- What happened: In deduction p16, `empty<int>` makes the type `char (*)[sizeof(T) - sizeof(T)]`, which is an array with bound 0. I predicted a substitution failure. The page does not say that a zero bound is an invalid type, and with GCC extensions in mind I was unsure whether `-pedantic-errors` turns it into a failure or a hard error.
- Suggestion: List an array bound of zero or less among the invalid types in DED-24 or in its examples, with a link to the rule that forbids it.

## R9C-5

- Pages and rules: templates/declarations (TPL-35); templates/deduction (DED-20, DED-23)
- Kind: unclear
- What happened: Declarations p24 has `template <class... Front, class... Back> int packs(Back...)`. TPL-35 says each parameter after a pack needs a default or must be found by deduction. DED-23 says a pack with no value is just empty. The TPL-35 Evidence says EDG rejects "a second type pack after the first". I could not tell whether the rule covers a second pack that a trailing function parameter pack deduces (p24, which I answered `runs`), or only one that nothing deduces (p56 and p95, which I answered `depends`).
- Suggestion: In TPL-35, say whether a trailing type pack counts as a parameter that needs deduction. Give one valid example (deduced from a function parameter pack) and one example that breaks the rule.

## R9C-6

- Pages and rules: batch README; templates/declarations Evidence; templates/deduction Evidence
- Kind: unclear
- What happened: Some questions are settled under decision 0054 "with an override for the family that still differs", such as Q-0199 for declarations p02 and p25, Q-0385 for p73, Q-0209 for deduction p78, and Q-0215 for p90. The README accepts `depends` or the rules' outcome where the compilers differ. I could not tell which of the two the test records for a settled question with an override. I gave the rules' outcome.
- Suggestion: Say in the README or in the Evidence what a settled question with an override expects: the rule's outcome, or a split.

## R9C-7

- Pages and rules: batch README
- Kind: unclear
- What happened: Deduction p104 has no `main`, and it calls `act(1)` at namespace scope, where `act` is declared and never defined. If the program were linked, the link would fail. I assumed that a program with no `main` is only compiled, and I answered `compiles`.
- Suggestion: State in the README that a program with no `main` is compiled and not linked.
