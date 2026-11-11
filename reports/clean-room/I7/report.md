# I7: class template argument deduction, from the rules alone

Date: 2026-11-11
Commit read: `bd97540b4f8b3b0a0d3c71848920d9285df65214`
Kind: Implement (decision 0062, item E)
Role: clean-room reader in the role of an implementer

The reader returned this report as a message, because the write of `report.md` was refused. The integrator saved it here with the reader's files.

## Inputs

Blind phase. I read these pages only through `filter.py`, which is a copy of the I6 filter. The filter drops each `## Evidence` segment with its subsections and each line with `:tests:`.

- `templates/class-template-deduction.md`, all of it
- the glossary entries for class template argument deduction, deduction guide, copy candidate, aggregate candidate, forwarding reference, non-deduced position and partial ordering
- the linked rules OVL-14, OVL-25 to OVL-28, OVL-31, OVL-35, OVL-36, ORD-1, ORD-P2, ORD-5, ILC-3, ILC-8, LIST-3 to LIST-5, LIST-P2, DER-8, TARG-9, TARG-10, DED-8 and DED-13

I did not read `tests/`, `questions/` or `frontend/` in the blind phase. `notes.md` lists 28 steps that the text did not give me. I wrote them before I read any test.

The component is `ctad.py`. It takes a world of class templates, alias templates and user guides, and an initializer with its form. A class template has these parts:

- its parameters
- its constructors, each with its own parameters, `explicit`, constraint, deleted and access
- whether it is defined
- whether it is an aggregate, with its elements (bases first, and a pack of bases as an expansion)
- the bases whose constructors it inherits

The component does these steps:

- checks the place of the name (CTAD-1, CTAD-2, CTAD-4, CTAD-15)
- forms the guides: from constructors or the guide with no parameters, the copy candidate, the user guides declared before the use, the aggregate candidate for this initializer, and the guides through an alias or inherited constructors (CTAD-5 to CTAD-16, CTAD-27 to CTAD-29)
- runs the two phases of a braced list, deduces each guide, and chooses one with the usual ranking, OVL-27, partial ordering with ORD-5, and CTAD-19 to CTAD-22
- checks CTAD-23 and CTAD-24, and a narrow part of CTAD-26
- checks the declarations of user guides (CTAD-12, CTAD-13) and of several declarators (CTAD-3)

The component uses I5's `deduce.py` as its callback, unchanged on disk: `deduce()` for one guide, `ics()` for the ranks, `at_least_as_specialized()` for partial ordering, and `subst()` and `unify()`. The interface fits. The component replaces two I5 functions when it loads. `is_forwarding()` now treats a class parameter `T&&` as no forwarding reference (CTAD-8). `ics_braced()` now lets a braced list convert to an aggregate class template of the world (notes 5).

`blind_check.py` runs the page's own examples and 14 rules with no example: 42 of 42.

The sha1 of `ctad.py` is `39d182fd20a3c1b0977123bfe99bfc67a268685c`, recorded in `ctad.sha1`. `score.py` checks the hash before it runs. The file is read-only and did not change after the blind phase.

Scoring phase. I read these tests:

- the 121 tests in `tests/templates/class-template-deduction/`
- the probe grid `grids/forms.json` with `grids/tables.md`
- the 14 tests elsewhere whose tidal-rules line names a CTAD rule: two rule tests and 12 program tests

I built the front end into my scratch directory with no build failure. I wrote 12 programs in `programs/`. Two reductions are in `reductions/`.

## Score

290 of 296 assertions are right (`score-output.txt`). 126 of 132 scored tests have every assertion right. Each of the 52 rejects that the component got right names the rule of the test.

| Source | Tests scored | Assertions | Right |
|---|---|---|---|
| `tests/templates/class-template-deduction/*.cpp` | 117 | 185 | 179 |
| probe grid `forms.json` (10 forms by 9 class shapes) | 1 | 90 | 90 |
| tests elsewhere that cite a CTAD rule | 14 | 21 | 21 |

For a grid cell where the compilers split, I scored the outcome that the page gives. That is accept for the alias column, accept for the inherited column, and accept for an aggregate in a `new` expression.

The score leaves out 4 tests whose assertions the component cannot express. Each is a bullet of CTAD-12 that needs a part of a declaration the tree does not hold:

- `guide-access`: the access of a guide and of a member template
- `guide-definition`: a guide with a body
- `guide-specifier`: a specifier other than `explicit`
- `guide-wrong-scope`: the scope of the guide

The score needs these caveats:

- 16 assertions pass because the caller does a step before the call. Ten name a class template after substitution: a template template parameter, a dependent qualifier, and a deduction in a function template. Four take the point of a use from the caller (CTAD-11). Two take a flag that marks an alias as CTAD-15's kind. The caller also resolves a member alias to the class parameter (CTAD-8, last bullet).
- The ranks come from I5 and are coarse (notes 22). No test needed a finer rank.
- Constraints and the sizes of types are lambdas in the test world. The component only calls them.

These are the wrong assertions:

- `aggregate-elision`, `Mixed b{1, 2, 3L}`. The member `Plain p` has a type that is not dependent. The component gave it one element of the list, so `2` went to `z` and `3L` had no member (I7-1).
- `aggregate-string-fixed-array`, `Named b{"ab", 1}`. The member `char v[3]` does not depend on a parameter. The component elided its braces and gave `"ab"` and `1` to its elements (I7-1).
- `guide-redeclared`. `check_user_guide()` finds a guide declared twice only for guides that are not templates (I7-4).
- `result-then-initialize`, `inherited-copy-candidate` and `inherited-copy-candidate-user-guide`. The component returns the deduced type, and the initialization that follows fails. The component does not run that initialization (I7-2, I7-3).

## Findings

### I7-1. Brace elision for a member whose type is not dependent

- Pages and rules: `templates/class-template-deduction.md`, CTAD-9, CTAD-10; LIST-4 (notes 4).
- Kind: missing.
- What happened: CTAD-10 says when braces are not elided: a dependent type that is not an array, and an array with a dependent bound. It then says an array with a dependent element type and a fixed bound is elided. It does not say what happens to a member whose type is not dependent. I gave such a member one element. Two tests read it otherwise. In `aggregate-elision`, `Plain p` takes the elements `1` and `2`, as LIST-4 elides for a plain aggregate. In `aggregate-string-fixed-array`, `char v[3]` takes `"ab"` whole, as LIST-4 keeps the braces of an array that a value can initialize directly. Both answers follow from LIST-4 once the reader knows that LIST-4 applies unchanged. The text never says so.
- Suggestion: add a bullet to CTAD-10: "A member or base whose type is not dependent follows LIST-4 as in any aggregate initialization." Add `Mixed b{1, 2, 3L}` and `Named b{"ab", 1}` to the page's examples.

### I7-2. Step 5 needs a second overload resolution that a reader can miss

- Pages and rules: CTAD-25, CTAD-26, CTAD-P1 step 5 (notes 21).
- Kind: needs example.
- What happened: CTAD-25 says the initialization runs as for a declaration with the deduced type. I read the chosen guide as the constructor that runs, and flagged CTAD-26 only for a deleted or private constructor of that guide. `result-then-initialize` shows the gap. A user guide gives `Pair<int>`, and the constructors of `Pair<int>` then tie for `(1, 2)`. The guide chosen in step 3 says nothing about the constructor chosen in step 5. The principle says step 5 "is a separate initialization", which is right. The page's one example of CTAD-26 (a deleted constructor) is a case where the two choices agree.
- Suggestion: add the `result-then-initialize` program to the examples, with a comment that the constructors of the deduced type are chosen again and tie. Say in CTAD-25 that the constructor which runs can differ from the constructor that gave the chosen guide.

### I7-3. The inherited copy candidate gives a type that the initialization rejects

- Pages and rules: CTAD-16, CTAD-26, DER-8; the tests `inherited-copy-candidate` and `inherited-copy-candidate-user-guide` (notes 10).
- Kind: unclear.
- What happened: CTAD-16 gives a derived class template the copy candidate of its base. DER-8 says copy constructors are not inherited. So `Derived d(b)` with `b` of type `Base<int>` deduces `Derived<int>`, and the initialization fails. I followed CTAD-16 and got `Derived<int>`, which matches the test's first step. My component has no step 5, so it missed the reject (I7-2). The text does not say why a guide that can never initialize is in the set. A reader must put CTAD-16 and DER-8 together to see the failure.

  `inherited-copy-candidate-user-guide` cites CTAD-24 for its reject. Its note says "The user guide removes the aggregate candidate of Derived". DER-8 says a class with `using B::B;` is not an aggregate, so no aggregate candidate exists to remove. By CTAD-16, the inherited copy candidate gives `Derived<int>`, and the failure is CTAD-26, as in `inherited-copy-candidate`. The note says GCC and EDG do that.
- Suggestion: add a note to CTAD-16: "The inherited copy candidate can give a type whose initialization then fails, since the base's copy constructor is not inherited (DER-8, CTAD-26)." In `inherited-copy-candidate-user-guide`, cite CTAD-26 with CTAD-16, and drop the sentence on the aggregate candidate.

### I7-4. The component missed a template guide declared twice

- Pages and rules: CTAD-12 (fifth bullet).
- Kind: none (a mistake in the component).
- What happened: CTAD-12 says "One guide is not declared twice". My check compared only guides that are not templates, which is the sixth bullet.
- Suggestion: none for the text.

### I7-5. Implicit constructors and the guide with no parameters

- Pages and rules: CTAD-6, CTAD-5 (notes 1).
- Kind: unclear.
- What happened: CTAD-6 gives the guide with no parameters when the class "declares no constructor". The text does not say whether an implicitly declared default, copy or move constructor gives a guide. I gave none. No test separates the readings, because the copy candidate covers a copy.
- Suggestion: add to CTAD-5: "Only the constructors that the class declares give guides. The implicitly declared ones give none."

### I7-6. Which guides are initializer-list guides, and an empty list

- Pages and rules: CTAD-18, CTAD-17, ILC-3 (notes 2, 3).
- Kind: missing.
- What happened: CTAD-18 speaks of "the phase of ILC-3 that tries only the initializer-list guides". No rule defines an initializer-list guide. ILC-3 defines the phase for constructors. I took a guide whose first parameter is `std::initializer_list<E>`, with defaults for the rest, from a constructor or a user guide. The text also does not say whether `C c{};` runs the first phase. I skipped it, from my own knowledge. No test has an initializer-list constructor with an empty list.
- Suggestion: define the initializer-list guide in CTAD-17 or CTAD-18. Say whether an empty list skips the first phase for guides, as it does for constructors when a default constructor exists.

### I7-7. The match of CTAD-14 needs one worked example

- Pages and rules: CTAD-14, CTAD-28 (notes 6, 7, 8).
- Kind: needs example.
- What happened: CTAD-14 says "The result of the guide is matched with the type of the alias". I had to make three choices. The first was which side is deduced. The second was what "the alias parameters that the match uses" means for a parameter in a non-deduced position. The third was how to test "a specialization of the alias". I deduced the guide's parameters from the alias's type, held the alias's parameters fixed, and later deduced the alias's parameters back from the result. The 14 alias assertions came out right. In `alias-template-result-check`, the note says the user guide forms and then fails CTAD-28. My component drops it earlier, because its result `Pair<A, typename Id<B>::type>` does not match `Pair<T*, T*>`. The outcome is the same. The text allows both readings.
- Suggestion: add one example with the steps written out: the guide `Pair(T, U) -> Pair<T, U>`, the alias `IntPair<U> = Pair<int, U>`, the match `T = int, U = U`, and the new guide `IntPair(int, U) -> Pair<int, U>`. Say whether a guide whose result does not match is dropped at once or kept to fail CTAD-28.

### I7-8. An alias of an alias

- Pages and rules: CTAD-14, CTAD-28 (notes 9).
- Kind: missing.
- What happened: CTAD-14 says "directly or through another alias", and gives the procedure for one step. I flattened the chain to the class template and ran CTAD-14 once. `alias-template-fixed-argument` agrees for `LongFirst`. The text does not say whether the constraints of the inner alias also apply.
- Suggestion: say that the steps run once per alias, inner first, and that each alias's constraints apply.

### I7-9. Which guides a derived class inherits

- Pages and rules: CTAD-16, CTAD-6, DER-8 (notes 10, 11).
- Kind: unclear.
- What happened: CTAD-16 names the base's guides "with the user guides and the copy candidate". It does not say whether the base's guide with no parameters (CTAD-6) or its aggregate candidate comes along. I left both out, since DER-8 does not inherit a default constructor. No test decides it. My handling of a derived parameter that the base does not use matched `inherited-guide-unused-parameter-default` and `inherited-not-deducible`.
- Suggestion: list in CTAD-16 the guides that a derived class takes from its base, and the ones it does not.

### I7-10. Is every guide a function template?

- Pages and rules: CTAD-P1, CTAD-P3, CTAD-11, CTAD-22, OVL-27 (notes 13, 14).
- Kind: unclear.
- What happened: CTAD-P1 says "Each guide is a function template". CTAD-11 says a user guide "can be a template or not". The page's `Text t("ab")` example needs OVL-27, under which a function that is not a template wins. CTAD-P3 says the fourth tie-breaker "follows OVL-27" for guides from constructors. A guide from a constructor that is not a template is still a template over the class's parameters, so OVL-27 does not order it. I ran OVL-27 only for user guides with no template head, then partial ordering, then CTAD-19 to CTAD-22. Every tie test agreed.
- Suggestion: in CTAD-P1, say that a user guide with no template head is a function and not a template, and that OVL-27 applies to it. In CTAD-P3, say that CTAD-22 is modeled on OVL-27, which does not itself apply to two guides from constructors.

### I7-11. Copy initialization and the conversions a guide may use

- Pages and rules: CTAD-17, OVL-14, ICS-10 (notes 15).
- Kind: unclear.
- What happened: OVL-14 limits a constructor in copy initialization from another type to a standard sequence. CTAD-17 says only that copy initialization leaves out the explicit guides. I put no limit on the guides. No test needs a user-defined conversion into a guide's parameter in copy initialization.
- Suggestion: say in CTAD-17 whether the limit of OVL-14 and ICS-10 applies to the guides.

### I7-12. CTAD-18 and the sentence on one object in ILC-3

- Pages and rules: CTAD-18, ILC-3 (Q-0081).
- Kind: hard to find.
- What happened: ILC-3 says a list with one object of the same class also uses the first phase. CTAD-18 says the choice of a guide skips the first phase for one element of a specialization. Both are right, for different choices. A reader who reads ILC-3 first, as CTAD-17 sends them, gets CTAD-18 backwards until they see the contrast.
- Suggestion: add "unlike ILC-3 for constructors" to CTAD-18.

### I7-13. A new expression and a static member with no initializer

- Pages and rules: CTAD-1, CTAD-2 (notes 23).
- Kind: unclear.
- What happened: CTAD-1 lists a `new` expression and a static data member, each "with an initializer". CTAD-2's list says "in particular" and names neither form with no initializer. I rejected both under CTAD-2. No test covers them.
- Suggestion: add both forms to CTAD-2's list.

### I7-14. When a conditional explicit is evaluated

- Pages and rules: CTAD-5, CTAD-17, CTAD-23 (notes 20).
- Kind: unclear.
- What happened: CTAD-5 says the guide is explicit "if the condition is true for the deduced arguments". In copy initialization, CTAD-17 leaves out explicit guides before the choice. I evaluated the condition after each guide's deduction and before the choice. `conditional-explicit` agrees for `Narrow b = 'c'`. The text does not say whether a guide whose condition is true is dropped before the ranking or after it.
- Suggestion: say in CTAD-17 that each guide's condition is evaluated for its own deduced arguments before the choice.

### I7-15. Knowledge the pages did not give

- Pages and rules: CTAD-9, ICS-21 (notes 5, 22).
- Kind: outside knowledge.
- What happened: I took these from my own knowledge, or from I5:
  - the rank of a braced list that initializes an aggregate base or member (notes 5). I found no linked rule that gives it.
  - the ranks of the conversions, through I5's coarse ranks (notes 22)
  - the empty list and the first phase (I7-6)
- Suggestion: link from CTAD-9 the rule that gives the conversion sequence of a braced list to an aggregate.

### I7-16. The front end stops on an aggregate candidate with designators

- Pages and rules: CTAD-9, LIST-5.
- Kind: front end.
- What happened: `reductions/r1-aggregate-designator.cpp` is two lines: `template <class K> struct Entry { K key; };` and `Entry a{.key = 3};`. GCC and Clang accept it. The front end prints "unsupported at line 2: check: deduction of class template arguments". The same class with `Entry b(4L, 'x')` runs, and a plain struct with a designator runs. So the front end forms no aggregate candidate for a designated list. I found the stop through `programs/p01-aggregate-designators.cpp`. The test `aggregate-candidate` has the same construct.
- Suggestion: form the aggregate candidate from the designated members (CTAD-9). Add the reduction as an accept test.

### I7-17. The front end stops on a braced copy of a class with an initializer-list constructor

- Pages and rules: ILC-3 (the sentence on one object of the same class, Q-0081).
- Kind: front end.
- What happened: `reductions/r2-braced-copy-initializer-list-class.cpp` declares `struct Bag { Bag(std::initializer_list<int>) {} };` and runs `Bag a{1}; Bag b{a};`. GCC and Clang run it with exit 0. The front end prints "unsupported at line 11: constructor call". No deduction is needed. `Bag b(a)` runs, and `Bag b = {a}` stops the same way. ILC-3 tries the first phase with `a`, finds no conversion from `Bag` to `int`, and then takes the copy constructor in the second phase. I found the stop through `programs/p07-initializer-list.cpp`, where `Bag b{a}` deduces through the copy candidate (CTAD-18).
- Suggestion: when the first phase of ILC-3 finds no viable initializer-list constructor, go on to the second phase with the element as an argument. Add the reduction as a run test.

## Programs

Each program ran under g++ 16.2.1 and clang++ 22.1.8 with `-std=c++23 -pedantic-errors`, and under `tidal-fe run`.

| Program | Rules | g++ | clang++ | tidal-fe |
|---|---|---|---|---|
| p01-aggregate-designators | CTAD-9, LIST-5 | 0 | 0 | unsupported (I7-16) |
| p02-alias-fixed-argument | CTAD-14 | 0 | 0 | 0 |
| p03-inherited-guide | CTAD-16 | 0 | rejects (Q-0239) | 0 |
| p04-string-guide | CTAD-11 | 0 | 0 | 0 |
| p05-overload-set | CTAD-9, CTAD-27 | 0 | 0 | 0 in every explored order |
| p06-conditional-explicit | CTAD-5, CTAD-17 | 0 | 0 | 0 |
| p07-initializer-list | CTAD-18, ILC-3 | 0 | 0 | unsupported (I7-17) |
| p08-new-and-cast | CTAD-1 | 0 | 0 | 0 |
| p09-constrained-guides | CTAD-29 | 0 | 0 | 0 |
| p10-copy-and-user-guide | CTAD-20, CTAD-21 | 0 | 0 | 0 |
| p11-template-template | CTAD-1 | 0 | 0 | 0 |
| p12-aggregate-base-braces | CTAD-9, CTAD-10, CTAD-27 | 0 | 0 | 0 |

The page accepts p03 (CTAD-16). The page's CTAD-19 example says Clang makes no inherited guides (Q-0239). The front end follows the page.

## Reductions

| Reduction | From | Stop |
|---|---|---|
| `reductions/r1-aggregate-designator.cpp` | p01 | unsupported at line 2: check: deduction of class template arguments |
| `reductions/r2-braced-copy-initializer-list-class.cpp` | p07 | unsupported at line 11: constructor call |

r1 stops in `check` as well as in `run`. r2 needs no template.

## Host-only claims

These claims rest on the host compilers alone, with no matrix run. The hosts are g++ 16.2.1 and clang++ 22.1.8 with `-std=c++23 -pedantic-errors`.

- p01, p02 and p04 to p12 build and exit 0 on both hosts. p03 builds and exits 0 on g++.
- clang++ rejects p03 with "no viable constructor or deduction guide".
- r1 compiles on both hosts.
- r2 builds and exits 0 on both hosts. `Bag b(a)` and `Bag b = {a}` build and exit 0 on both hosts (I7-17).
- In the narrowing of r1, `Entry b(4L, 'x')` and a designated list for a plain struct build and exit 0 on g++ (I7-16).

## What worked well

- CTAD-P2's table and CTAD-5 to CTAD-7 gave every guide from a class. All 90 grid cells came out right, the split columns included.
- CTAD-P3's table of tie-breakers, with CTAD-19 before CTAD-20 and CTAD-22, gave all 12 tie tests. The tests that place CTAD-19 first (`tie-own-constructor-template`, `tie-own-constructor-user-guide`) came out right from the table alone.
- CTAD-27 and the `Bases` example gave all five fixed-pack tests, the conflict through a conversion included.
- CTAD-14 and CTAD-28, with the `Same` and `IntDuo` examples, gave all 14 alias assertions, the CTAD-28 forwarding case included.
- CTAD-8's bullets gave every forwarding test once I5's `is_forwarding()` learned the class parameters.
- I5's deduction served as the callback with two small changes. It needed no change for packs, constant parameters from array bounds, or default arguments in guides.
