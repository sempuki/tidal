# Predict batches: ordering, packs, and specialization, reader D

- Date: 2026-11-04
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R9-ordering (58 programs), R9-packs (69), R9-specialization (95, where p66 has two files)

All three answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R9-ordering/answers.json
- /home/ryanm/Code/tidal/build/exercises/R9-packs/answers.json
- /home/ryanm/Code/tidal/build/exercises/R9-specialization/answers.json

Some programs fall under an Evidence segment that records a compiler difference. For those I gave the outcome that the rules give, and the `why` names the family that differs. These are ordering p03, p04, p06, p22, p27, p28, p36, p37, p40, p41, p42, p47, p52, p56, packs p34 and p68, and specialization p11, p22, p24, p30, p52, p54, p57, p63, p68, p70, p71, and p80.

Some programs break a no-diagnostic rule on which the compilers differ. I answered `depends` for these: packs p23 and p33 (PACK-30), and specialization p07 and p73 (TSP-17).

I named no principles. A rule decided each program.

## R9D-1

- Pages and rules: templates/ordering, ORD-9, ORD-3, ORD-10
- Kind: unclear, needs example
- What happened: Program p28 in R9-ordering takes the address of `read`, which has two templates. One takes `T&` and the other takes `T&&`, and the target is `int(int&)`. ORD-9 makes the two function types one pair, so ORD-10 does not act inside it. ORD-3 removes the forwarding reference special case. The page then does not say whether deduction of a parameter `T&&` inside a function type succeeds against an invented `A&`. I could not work out an order from the rules. I answered compiles with `read(T&)` only because the ORD-9 note in Evidence says every compiler except GCC 14 chooses `read(T&)`.
- Suggestion: In ORD-9 or ORD-3, say whether `T&&` inside a compared function type deduces from an lvalue reference type. Add a row to the ORD-9 example for `T&` against `T&&` that shows each direction.

## R9D-2

- Pages and rules: templates/ordering, ORD-13, ORD-2, ORD-8
- Kind: unclear
- What happened: In program p40, `operator T*()` and a constrained `operator T* const()` tie in ordering, because ORD-2 strips the top-level `const`. ORD-13 lets constraints decide only if "the result types are the same". The rule does not say whether "the same" means before or after ORD-2. I took the declared types, which differ, so I answered reject. The other reading gives compiles.
- Suggestion: State in ORD-13 whether the result types are compared as declared or after ORD-2. Give the `T*` against `T* const` case as an example.

## R9D-3

- Pages and rules: templates/ordering, ORD-13, ORD-5
- Kind: unclear
- What happened: Program p41 has a constrained `pick(T)` and `pick(T, ...)`. ORD-13 needs parameter lists of the same length. Its ellipsis condition names only functions that are not templates. I could not tell from the rule whether the ellipsis makes the second template's list longer. I answered compiles from the Evidence entry for Q-0293.
- Suggestion: Add a sentence to ORD-13 that says how an ellipsis counts for two templates. One example with a constrained `f(T)` and `f(T, ...)` would show it.

## R9D-4

- Pages and rules: templates/ordering, ORD-P1; overloading, OVL-20
- Kind: hard to find
- What happened: Program p02 in R9-ordering initializes an `auto` variable with the address of two constrained members, `f()` and `f(int)`. The ordering page links only OVL-19 for an address. The case with no target type is in OVL-20, and I found it only by searching the overloading pages. I answered reject because ORD-13's conditions fail and two functions remain.
- Suggestion: In the ORD-P1 table row for the address of an overload set, link OVL-20 next to OVL-19 and say that a variable declared with `auto` gives no target type.

## R9D-5

- Pages and rules: templates/specialization, TSP-14; templates/deduction, DED-12
- Kind: unclear
- What happened: Programs p52 and p70 in R9-specialization pass `1L` and `'c'` to a partial specialization with `int V`, and an `int*` to one with `const auto* P`. TSP-14 says the argument's type must "fit" the declared type. A new reader can take "fit" to mean "converts with no loss", and then `1L` matches `int V`. I answered with the primary template (the reading that the test asserts) only because Evidence says that GCC and EDG do so and that Clang converts.
- Suggestion: Replace "fits" with a precise phrase, such as "has exactly the declared type, with no conversion". Add `1L` and `int*` against `const auto*` to the page examples.

## R9D-6

- Pages and rules: templates/specialization, TSP-11, ORD-14
- Kind: needs example
- What happened: Program p68 declares `Pair<U, T>` as a partial specialization of `Pair<T, U>`. The TSP-11 bullet about "the primary template's parameter list again" does not obviously cover a reordered list. I had to combine the "more specialized" bullet with ORD-14 to see that the reordered list is no more specialized. I answered reject. The Evidence note for Q-0174 confirmed it.
- Suggestion: Add a reordered list to the TSP-11 examples, with a note that the reorder fails the "more specialized" condition.

## R9D-7

- Pages and rules: templates/packs, PACK-3; expressions (variable argument calls)
- Kind: answered from outside knowledge
- What happened: Programs p04 and p22 in R9-packs call `constexpr` functions whose `...` is a variable argument part. The programs pass extra arguments, including a `double` and a string literal, inside a `static_assert`. The packs page does not say whether such a call can be a constant expression when the body never reads the extra arguments. I answered compiles from outside knowledge.
- Suggestion: In the PACK-3 examples, add a constant evaluation of `first_size('a', 1, 2.0)` and link the rule that allows extra arguments in a constant evaluation.
