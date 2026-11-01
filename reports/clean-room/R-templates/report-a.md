# Predict report A: templates batches

- Date: 2026-11-01
- Role: clean-room reader
- Persona: programmer new to C++
- Inputs: `specification/` only, plus the four batch directories.
  No compiler was run.

## Batches

| Batch | Programs | compiles | runs | reject | depends |
|---|---|---|---|---|---|
| R-packs | 68 | 36 | 1 | 26 | 5 |
| R-concepts | 115 | 46 | 2 | 51 | 16 |
| R-constraints | 118 | 41 | 1 | 49 | 27 |
| R-ordering | 41 | 24 | 1 | 6 | 10 |
| Total | 342 | 147 | 5 | 132 | 58 |

No program was predicted as `link-fail`, `run-fail`, or `undefined`.

## Findings

### RA-1

- Batch and program: R-packs p42
- Rules: PACK-12, PACK-27
- Kind: unclear
- Needed: the result of a call where a pack before the end of a
  function type's parameter list is deduced as empty. The Evidence
  says that this case splits EDG from Clang and GCC, but it does not
  say what each compiler does.
- Suggestion: state the result on each side of the split, for example
  "Clang and GCC fail deduction and choose the other overload; EDG
  does X". Then a reader can answer `depends` with confidence.

### RA-2

- Batch and program: R-packs p04, p22, p32
- Rules: PACK-21, PACK-30
- Kind: unclear
- Needed: whether Clang rejects every using-declaration that expands an
  empty pack, or only a use in the class that depends on it. The
  PACK-30 Evidence says that Clang "rejects the using-declaration that
  expands to nothing". Read alone, the sentence conflicts with PACK-21,
  which says the declaration declares nothing.
- Suggestion: say in the Evidence that Clang rejects only when a name
  in the class finds the empty using-declarator. Add an example where
  the class has its own `f` and the program calls `f` from outside the
  class, which PACK-30 does not cover.

### RA-3

- Batch and program: R-packs p35, p55
- Rules: PACK-12
- Kind: missing
- Needed: what happens when a pack in a non-deduced template argument
  list (`List<Ts..., int>`) gets an explicit argument. PACK-12 says the
  list is non-deduced. The Evidence covers explicit arguments only for
  function types.
- Suggestion: add a sentence or an example. An explicit argument fixes
  the pack, and with no explicit argument the pack is empty, so the
  match fails.

### RA-4

- Batch and program: R-packs p23
- Rules: PACK-31
- Kind: needs example
- Needed: what a `const` bit-field does when it reaches a forwarding
  reference. PACK-31 forbids only the non-const case. I inferred that
  `T` is `const int&` and that the reference binds to a temporary.
- Suggestion: add the const case to the Examples, with the deduced `T`.

### RA-5

- Batch and program: R-concepts p02, p77; and the `depends` answers in
  every batch
- Rules: CPT-3, and the Evidence of each page
- Kind: hard to find
- Needed: the list of the eight baseline compilers. The answer format
  allows `depends` when the compilers differ. Some pages name Clang 18
  and GCC 14 as the only compilers that differ. I could not find the
  baseline list from `specification/` alone, so I could not tell if a
  split on Clang 18 counts.
- Suggestion: list the eight baseline compilers once, on the index page
  or in the glossary, and link to the list from each Evidence segment.

### RA-6

- Batch and program: R-concepts p73
- Rules: CPT-26, CPT-27
- Kind: unclear
- Needed: what GCC 14 does with a private member in a requires
  expression inside a member function of a class that is not a
  template. The Evidence says that GCC 14 finds such an expression
  `false`. CPT-27 says that a requirement with no template parameter
  that is not valid makes the program not valid. The two statements
  give different outcomes for this program.
- Suggestion: say whether the GCC 14 result applies outside templates,
  and whether GCC 14 then gives `false` or rejects.

### RA-7

- Batch and program: R-concepts p28, p93
- Rules: CPT-33, CNS-33
- Kind: unclear
- Needed: what happens when Clang substitutes the extra arguments of a
  compound requirement early, and that substitution instantiates a
  class with a failing `static_assert`. CPT-33 says only that Clang
  substitutes the arguments first. I combined this with CNS-33 to
  predict a Clang rejection.
- Suggestion: add to the CPT-33 Evidence that the early substitution
  can turn into a hard error on Clang, and link to CNS-33.

### RA-8

- Batch and program: R-constraints p16, p24
- Rules: CNS-9
- Kind: unclear
- Needed: the meaning of "an atomic constraint whose type is not `bool`
  for any argument". It can mean that the type is never `bool`, as for
  `sizeof(T)`. It can also mean that the type is not `bool` for the
  argument at hand, as for `T::value` with an `int` member. The two
  readings give different answers for p16.
- Suggestion: say "whose type is not `bool` for every argument, such as
  `sizeof(T)`". Add an example of a skipped dependent atom such as
  `T::value`, which every compiler accepts.

### RA-9

- Batch and program: R-constraints p10, p73
- Rules: CNS-10
- Kind: missing
- Needed: what each compiler does when a satisfaction result changes
  because a function is declared between two checks. The Evidence
  covers only a class completed between two checks. The test list
  names `satisfaction-changes-declaration.cpp`, but the page does not
  describe its results.
- Suggestion: add one Evidence line for the declaration case.

### RA-10

- Batch and program: R-constraints p27, p36
- Rules: CNS-35, CPT-26
- Kind: outside knowledge
- Needed: where the names in a requirement are looked up. To predict
  the recursion I assumed that an unqualified call in a concept finds
  a function declared after the concept, through argument-dependent
  lookup at the point of the check. I did not find this rule on the
  concepts or constraints pages.
- Suggestion: state the lookup point for names in a constraint or a
  requirement, or link to the dependent-names rule that gives it.

### RA-11

- Batch and program: R-constraints p64, p111
- Rules: CNS-22
- Kind: needs example
- Needed: a worked conversion of a constraint into a disjunction of
  conjunctions and a conjunction of disjunctions. I had to distribute
  `(A || B) && C` by hand to see that each form subsumes the other.
- Suggestion: add one worked example with a small table of the two
  forms and the shared atoms. Include a pair where each side subsumes
  the other, so that the call is ambiguous.

### RA-12

- Batch and program: R-ordering p09, p13
- Rules: ORD-10, ORD-11
- Kind: needs example
- Needed: a plain statement of which template wins. The text "the pair
  loses the direction in which the less qualified type's template is at
  least as specialized" took several readings. I had to check it
  against the function-template grid.
- Suggestion: add a sentence such as "the template with the more
  qualified reference is more specialized". Add a table of the four
  combinations of reference kind and `const` for conversion results,
  with the winner of each.

### RA-13

- Batch and program: R-ordering p26
- Rules: ORD-13, ORD-15
- Kind: unclear
- Needed: whether EDG accepts this exact program. The Evidence says
  twice that EDG lets a constraint decide between parameter lists of
  different lengths. The test `constraints-default-parameter.cpp` is
  listed under ORD-15, a reject rule. A reader cannot tell if the test
  is one where all compilers agree.
- Suggestion: merge the two duplicate Evidence bullets. Say if the
  split covers a parameter with a default argument.

### RA-14

- Batch and program: R-packs p30
- Rules: PACK-1, PACK-2
- Kind: unclear
- Needed: whether a named pack can have its `...` inside parentheses,
  as in a pointer-to-function declarator. PACK-2 forbids only the
  unnamed form, and PACK-1 says "any declarator".
- Suggestion: add the named form `R (*... fp)()` to the PACK-1
  examples as valid.
