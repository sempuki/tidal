# Predict report B: template batches

- Date: 2026-11-01
- Role: clean-room reader
- Persona: programmer new to C++
- Inputs: `specification/` only. No program was compiled or run.

## Batches

| Batch | Programs | compiles | runs | reject | depends |
|---|---|---|---|---|---|
| `build/exercises/R-class-template-deduction` | 108 | 44 | 0 | 53 | 11 |
| `build/exercises/R-deduction` | 114 | 72 | 2 | 33 | 7 |
| `build/exercises/R-arguments` | 83 | 38 | 4 | 37 | 4 |

Each batch's answers are in its `answers.json`.

The reader answered `depends` when the Evidence segment records a
difference that changes the outcome of a program on a current compiler.
When a `reject` rule has one compiler that accepts (CTAD-12, CTAD-13,
TARG-8), the reader answered `reject`. When only Clang 18 differs, the
reader answered with the outcome of the current compilers and named
Clang 18 in `why`.

## Findings

### RB-1

- Batch and programs: R-class-template-deduction p21, p35, p66, p85
- Rules: CTAD-1, CTAD-6, CTAD-24
- Kind: unclear
- Need: The reader had to know if a variable with no initializer can use
  deduction. CTAD-1 lists "a variable with an initializer" only. CTAD-6
  has a test named `no-initializer.cpp` and gives a guide with no
  parameters, which suggests that `Tag a;` deduces `Tag<int>` and that
  `Tuple e;` deduces `Tuple<>`.
- Suggestion: Add a variable with no initializer to the CTAD-1 list, or
  say in CTAD-6 that a declaration with no initializer runs the choice
  with no arguments. Add `Tag a;` to the examples.

### RB-2

- Batch and programs: R-class-template-deduction p01, p50, p76, p78
- Rules: CTAD-14
- Kind: unclear
- Need: The Evidence says that EDG deduces through an alias that
  "passes its parameters on" and not through one that fixes an
  argument. The reader could not place `Pair<T, T>` or `Box<T*>` in
  either group, so the reader could not tell if EDG accepts p01 and p76.
- Suggestion: List the alias shapes that the probes used, with the
  result of each compiler for each shape.

### RB-3

- Batch and programs: R-class-template-deduction p03, p20
- Rules: CTAD-9, CTAD-10
- Kind: missing
- Need: The parameter list of CTAD-9 names members, arrays, and packs
  of bases. A base that is not a pack is also an element of the list
  (`Base<T>` in p03, `List<T...>` in p20). The reader assumed that such
  a base acts as a member of the base type, so its braces are kept when
  the type is dependent.
- Suggestion: Add a bullet for a base that is not a pack, and say how
  CTAD-10 treats its braces.

### RB-4

- Batch and programs: R-class-template-deduction p20, p37
- Rules: CTAD-27
- Kind: needs example
- Need: CTAD-27 says that the remaining elements do not set the length
  of a pack that an earlier element fixes. The rule does not say what
  happens when fewer elements remain than the fixed length (p20 has
  two `{}` for three bases), or how `{}` deduces nothing for a base.
- Suggestion: Add an example with fewer elements than the fixed length,
  and state that the missing bases are value-initialized.

### RB-5

- Batch and programs: R-class-template-deduction p10
- Rules: CTAD-17, CTAD-20, OVL-27
- Kind: outside knowledge
- Need: For a string literal, the reader had to choose between the
  user guide `template <class T> Box(T&&)` and the user guide
  `Box(const char*)`. The reader used outside knowledge that a
  reference binding and an array-to-pointer conversion rank the same,
  so the non-template guide wins by the usual tie-breaker.
- Suggestion: Add this case to the examples of CTAD-P3, with a link to
  the rule that ranks the array-to-pointer conversion as an exact match.

### RB-6

- Batch and programs: R-class-template-deduction p13, p71, p74
- Rules: CTAD-22, ORD-5
- Kind: hard to find
- Need: A guide from `Mark(T)` ties with a guide from
  `template <class U = int> Mark(T, U* = nullptr)` only because the
  parameter that uses its default argument takes no part in partial
  ordering. The reader found ORD-5 on another page.
- Suggestion: Name ORD-5 in CTAD-22, or add the example of p13 to the
  page.

### RB-7

- Batch and programs: R-class-template-deduction p58
- Rules: CTAD-11, INST-31
- Kind: hard to find
- Need: `later<int>` is used before and after the guide `Cell(int)`.
  CTAD-11 says "the point of instantiation", but a function
  specialization has several. The reader found in INST-31 that a
  constant evaluation uses the declarations before the first point.
- Suggestion: Say in CTAD-11 which point counts when a function
  template has several, and link INST-31.

### RB-8

- Batch and programs: R-class-template-deduction p53, p69, p102
- Rules: CTAD-16
- Kind: unclear
- Need: One Evidence item says that Clang makes no guides from
  inherited constructors. Another item says that every compiler,
  Clang included, deduces `Derived<int>` from a `Base<int>` object "as
  the inherited copy candidate gives". The two items seem to disagree
  about Clang.
- Suggestion: Say whether Clang makes the inherited copy candidate and
  no other inherited guide, or deduces the type by another path.

### RB-9

- Batch and programs: R-deduction p58
- Rules: DED-31, DED-14
- Kind: missing
- Need: For `operator T**` and the needed type `const int* const*`,
  both `T = int` and `T = const int` reach the needed type by a
  qualification adjustment. The program asserts `T = const int`. The
  reader used outside knowledge that the value closest to the needed
  type is chosen.
- Suggestion: State which value deduction picks when more than one
  value allows a qualification adjustment, with this example.

### RB-10

- Batch and programs: R-deduction p35
- Rules: DED-14, DED-15
- Kind: unclear
- Need: `up_ptr(const Base<T>*)` with a `Derived*` argument needs both
  the base class difference and the added `const`. The page lists the
  differences one at a time and does not say if they combine.
- Suggestion: Say that a derived-to-base match can also add
  qualifiers, or give the combined case in the DED-P4 table.

### RB-11

- Batch and programs: R-deduction p01, p112
- Rules: NEW-23, DED-39
- Kind: unclear
- Need: When a function and a template both match a placement `new`,
  the Evidence says that EDG does not instantiate the template. It does
  not say if GCC or Clang instantiate it. p112 puts a failing
  `static_assert` in the template, so the outcome depends on this.
  The reader answered `runs`.
- Suggestion: Say for each compiler if the template is instantiated,
  and state the rule in NEW-23.

### RB-12

- Batch and programs: R-deduction p100, p70
- Rules: DED-24, DED-29
- Kind: needs example
- Need: An error from a lambda in the function type is a hard error
  (DED-29), but a call of a lambda with no viable function is a
  substitution failure (DED-24). The test `substitution-lambda-call-fails.cpp`
  is listed under both rules. The reader had to decide that a generic
  lambda whose written return type fails makes the call not viable.
- Suggestion: Add a short example of each side of the line, and list
  that test under one rule or say why it is under both.

### RB-13

- Batch and programs: R-deduction p114
- Rules: OVL-25, DED-17
- Kind: outside knowledge
- Need: The reader had to know that a braced list `{1.5, 2.5}` for a
  `const long (&)[2]` parameter still forms a conversion sequence that
  is worse than the template's exact match, though the conversion
  narrows.
- Suggestion: Add a note or link on how narrowing inside a braced list
  affects the ranking of a candidate.

### RB-14

- Batch and programs: R-arguments p69
- Rules: CX-6, CE-20, TARG-14
- Kind: wrong
- Need: CX-6 says that a constant evaluation must not reach the
  definition of a `static` variable that is not `constexpr`. The
  Evidence of CE-20 says that every compiler accepts a `static const`
  integer with a constant initializer. p69 has such a variable in a
  `constexpr` function. The reader followed CE-20.
- Suggestion: Change the CX-6 bullet to "a `static` or `thread_local`
  variable that is not usable in constant expressions", to match CE-20.

### RB-15

- Batch and programs: R-arguments p60
- Rules: TARG-14, TARG-19, CE-3
- Kind: outside knowledge
- Need: `Ref<alias>` uses a reference variable that is not `constexpr`
  as a reference template argument. CE-3 lists the variables that are
  usable in constant expressions, and references are not in the list.
  The reader used outside knowledge that a reference with a constant
  initializer is usable.
- Suggestion: Say in CE-3 or TARG-14 whether a reference variable with
  a constant initializer can name its object in a template argument.

### RB-16

- Batch and programs: R-arguments p06
- Rules: TARG-6, TARG-17
- Kind: hard to find
- Need: In a member definition outside the class, the head of a
  template template parameter has no default for its inner parameter,
  so `C<>` has too few arguments. The reader found only the test name
  `template-template-inner-default-missing.cpp` under TARG-6. The rule
  text does not say where the inner default comes from.
- Suggestion: Say in TARG-6 or TARG-17 that the head in scope decides
  the defaults of a template template parameter, with this example.

### RB-17

- Batch and programs: R-arguments p75
- Rules: TARG-3
- Kind: outside knowledge
- Need: `static_cast<Box<Box<int>>>(...)` ends in three `>`
  characters. TARG-3 says how `>>` splits. The reader used outside
  knowledge that the characters form the tokens `>>` and `>`.
- Suggestion: Add `>>>` to the TARG-3 examples.
