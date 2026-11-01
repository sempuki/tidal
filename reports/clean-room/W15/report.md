# W15 Write exercise: generic code

- Date: 2026-11-01
- Role: clean-room reader
- Persona: Haskell programmer who has never written C++. Knows type
  classes, parametric polymorphism, and type-level programming.
- Inputs: `specification/` only (index, glossary, translation,
  language-support, the templates chapter, storage, unions,
  constructors, copy-move, new-delete, conversions, conditional,
  decltype, primary, constant-evaluation).
- Compilers: GCC 16.2.1 and Clang 22.1.8, each with
  `-std=c++23 -pedantic-errors -Wall -Wextra`, and again with
  `-fsanitize=undefined,address`.

## Summary

All five programs build with no warning on both compilers, run, and
exit 0, also under the undefined and address sanitizers. Each program
checks its results with `static_assert` and with the value that `main`
returns. Four of the five built on the first try. The fifth
(`optional.cpp`) needed one change for a Clang warning on
self-assignment.

The three biggest problems:

1. **Raw storage gives no path back to the object (W15-2).** After
   placement `new` into a byte array, the only valid pointer to the new
   object is the one that `new` returns. The launder operation is
   "library referenced", and a program with no header cannot declare
   it (a declared `std::launder` fails to link). No page says
   "keep the pointer that placement `new` returns", and no example
   shows an optional or a stack built on raw storage.
2. **No rule makes a variant member active by construction (W15-1).**
   The usual way to hold a `T` with no default constructor, also in
   constant evaluation, is a union slot filled by `std::construct_at`.
   The union page makes a member active only by initialization or by
   assignment. Both compilers accept the construction in constant
   evaluation, but no rule says why.
3. **The requires-expression trap in `static_assert` (W15-3).** The
   natural test "a type without an area fails" is
   `static_assert(!requires (Label l) { l.area(); })`, and every
   compiler rejects it. CPT-27 says so, but the page does not show
   the two ways out (a concept with a parameter, or a nested
   requirement on a concept-id).

## Rule and principle IDs noted before writing

| Program | Construct | IDs |
|---|---|---|
| all | no headers; declare library names | LSUP-P1, LSUP-P3, LSUP-4, LSUP-7, LSUP-10 |
| all | exit status, constant evaluation | TR-2, TR-5, TR-10, CE-1 |
| stack | class template, constant parameter | TPL-1, TPL-6, TPL-9 |
| stack | requires clause on a class template | CPT-8, CPT-9, CPT-10 |
| stack | storage for a T with no default constructor | UNI-1, UNI-3, UNI-4, UNI-11, LSUP-10, STG-5, NEW-37 |
| stack | end an element | DTOR-6, DTOR-7, DTOR-8 |
| stack | forward arguments | PACK-1, PACK-22, PACK-23 |
| stack | test a failed constraint | CNS-13, CPT-32, CPT-27 |
| largest | variadic function template | TPL-30, PACK-P1, PACK-5, PACK-9 |
| largest | constrained pack | CPT-17, CPT-19 |
| largest | result type | CONV-19, DTY-1, COND-3, RET-1 |
| largest | type equality | TPL-26, TSP-10 |
| shapes | concept, requires expression | CPT-P2, CPT-P3, CPT-1, CPT-24, CPT-28, CPT-33 |
| shapes | constrained placeholder parameters | CPT-20, DED-34 |
| shapes | sum with a fold | PRIM-P6, PRIM-27, PRIM-28, PRIM-29 |
| shapes | more specific overload wins | CNS-P1, CNS-P4, CNS-2, CNS-4, CNS-22, CNS-23, CNS-24, ORD-13, OVL-41 |
| typelist | list as a pack | TPL-30, PACK-19 |
| typelist | recursion by partial specialization | TSP-P2, TSP-9, TSP-14, TSP-15, ORD-14 |
| typelist | property as a parameter | TPL-12, TARG-7, TARG-17 |
| typelist | all-of | PRIM-28 (empty `&&` fold is `true`) |
| optional | byte array hosts a T | STG-P2, STG-4, STG-5, STG-6, ALN-1 |
| optional | pointer to the new T | STG-P4, STG-9, STG-16, STG-19 |
| optional | end the T | DTOR-6, DTOR-7, STG-22 |
| optional | copy and move | CPY-P1, CPY-P2, CPY-P4 |

## Findings

### W15-1. Construction into a union member does not say it makes the member active

- Pages and rules: `classes/unions.md` UNI-4, UNI-5, UNI-13, UNI-6;
  `foundations/storage.md` STG-5, STG-9; `foundations/language-support.md`
  LSUP-10.
- Kind: missing.
- What happened: `stack.cpp` stores each element in a
  `union Slot { T value; constexpr Slot() {} constexpr ~Slot() {} };`
  and fills it with `std::construct_at(&slot.value, args...)`. The
  slot starts with no active member (UNI-4, default initialization).
  UNI-5 and UNI-13 make a member active only by assignment, and only
  when its default constructor is trivial. STG-9 makes a new object
  become a member only when it replaces an old object of that member,
  and an inactive member has no old object. So from the text, reading
  `slot.value` after `construct_at` reads an inactive member (UNI-6,
  CE-P3), and the constant evaluation in `static_assert` should fail.
  Both compilers accept it and give the expected values.
- Suggestion: add a rule to the union page. A new object that a
  placement `new` (or `std::construct_at`) creates at the address of a
  variant member, with the member's type, becomes that member and the
  active member, also in constant evaluation. Add the slot union to the
  Examples, since it is the standard way to hold a type that has no
  default constructor.

### W15-2. Raw storage: the specification gives no usable way back to the object except the pointer from `new`

- Pages and rules: `foundations/storage.md` STG-P4, STG-16, STG-19;
  `foundations/language-support.md` (launder is "library referenced").
- Kind: missing.
- What happened: in `optional.cpp` I first planned to keep a `bool`
  and get the object with `reinterpret_cast<T*>(storage)`. STG-16 says
  the byte array and the `T` are not pointer-interconvertible, so the
  cast gives a pointer to the original object (the bytes), and STG-19
  says only the launder operation reaches the new object. The
  language-support page gives no shape for launder. A probe that
  declares `std::launder` itself fails to link on both compilers. The
  working design keeps the `T*` that placement `new` returns in a
  member. Nothing on the storage page says this is the intended path.
- Suggestion: in STG-P2 or the storage Examples, show a class that
  keeps the result of placement `new` and uses only that pointer. In
  the language-support page, say that a program with no header cannot
  declare the launder operation, so such a program keeps the pointer.

### W15-3. Requires expressions in `static_assert` need an example of the way around CPT-27

- Pages and rules: `templates/concepts.md` CPT-24, CPT-26, CPT-27,
  CPT-32, CPT-35; `templates/constraints.md` CNS-13.
- Kind: needs example.
- What happened: the task asks for a requires expression in
  `static_assert` that shows a type without an area fails the concept.
  The direct form `static_assert(!requires (const Label& l) { l.area(); });`
  is rejected by both compilers ("no member named 'area'"). In the same
  way, `static_assert(!requires { typename Stack<int, 0>; });` is
  rejected ("constraints not satisfied"). CPT-27 predicts both. I read
  CPT-27 before writing and avoided the trap, then confirmed it with
  probes. Two forms work:
  - a nested requirement on a concept-id, `requires { requires Shape<Label>; }`
    (CPT-35), because the concept-id is valid and only its value is
    `false`;
  - a concept with a parameter, such as
    `template <class T> concept Summable = requires (const T& t) { total_area(t); };`,
    because the requirement then depends on a template parameter
    (CPT-26).
- Suggestion: the concepts page has a rejected example
  (`constexpr bool b = requires (int n) { n.size(); };`). Add the two
  working forms beside it. Also mention CPT-27 in CPT-32, since a type
  requirement on a class template whose constraint fails is the most
  likely way to meet it.

### W15-4. A concept cannot be a template argument, and TARG-7 does not list it

- Pages and rules: `templates/arguments.md` TARG-7, TARG-17;
  `templates/declarations.md` TPL-12.
- Kind: missing.
- What happened: a Haskell programmer wants `all_of<Integral, List>`,
  passing a constraint the way a type class is passed as a
  higher-kinded argument. TPL-12 and TARG-17 say that a template
  template parameter takes a class template or an alias template.
  TARG-7 lists "a class, or a variable template" as invalid arguments
  and does not list a concept. The probe confirms that both compilers
  reject a concept there (GCC: "expected 'auto' or 'decltype(auto)'
  after 'Integral'"). `typelist.cpp` wraps the concept in a class
  template `IsIntegral<T>` with `value = Integral<T>`.
- Suggestion: add "a concept" to the TARG-7 list. In the concepts page,
  add one sentence and an example that wraps a concept in a class
  template to pass it as a property.

### W15-5. "Arithmetic type" is used but not defined

- Pages and rules: `specification/glossary.md`;
  `expressions/conditional.md` COND-3; `expressions/multiplicative.md`;
  `foundations/conversions.md`; `foundations/overload-resolution.md`.
- Kind: missing.
- What happened: `largest.cpp` needs a concept for "arithmetic type".
  The term appears on several pages, and the glossary has only
  "promoted arithmetic type". I built the list from the fundamental
  types on `foundations/types.md` (bool, the character types, the
  integer types, and the three floating-point types). I left out the
  extended floating-point types because the specification marks them
  as referenced. I am not sure the list is the intended set.
- Suggestion: add "arithmetic type" to the glossary, as the integral
  types plus the floating-point types, with a link to the types page,
  and say whether `bool` and the character types count.

### W15-6. The common type of the usual arithmetic conversions needs an example

- Pages and rules: `foundations/conversions.md` CONV-19;
  `expressions/decltype.md` DTY-1, DTY-P1; `expressions/conditional.md`
  COND-3.
- Kind: needs example.
- What happened: the task asks for the result type that the usual
  arithmetic rules choose. No page names a way to get that type. I
  combined three rules. CONV-19 says a binary `+` converts both
  operands to the common type. DTY-1 says `decltype` of a prvalue gives
  the bare type. So `decltype(A() + B())` names the common type. The
  conditional operator looked like a natural choice, but COND-3 gives
  an lvalue when both operands are lvalues of one type, so
  `decltype(x < y ? y : x)` would be a reference. `char` and `short`
  promote to `int` through `+`, which matches CONV-19 step 2.
- Suggestion: add an example to CONV-19 or to the decltype page:
  "`decltype(a + b)` is the common type of the usual arithmetic
  conversions; `decltype(c ? a : b)` is a reference when `a` and `b`
  are lvalues of one type".

### W15-7. The slot-union idiom is outside knowledge

- Pages and rules: `classes/unions.md` UNI-3, UNI-11;
  `foundations/language-support.md` LSUP-10; glossary "literal type";
  `foundations/constant-evaluation.md` CE-P1.
- Kind: outside knowledge.
- What happened: the specification has every piece. UNI-3 deletes the
  union's constructor and destructor when the member's are not trivial,
  UNI-11 says the union must then declare them, LSUP-10 makes
  `construct_at` work in constant evaluation, and the glossary says a
  class with a `constexpr` destructor is a literal type. I knew from
  outside the specification to put these together into
  `union Slot { T value; constexpr Slot() {} constexpr ~Slot() {} };`
  and an array of slots. A plain array `T items[N]` needs a default
  constructor for `T`, which the task's `Point` does not have.
- Suggestion: one example on the unions page or the storage page that
  builds a fixed-capacity container from slots, with the explicit
  destructor call (DTOR-6) for removal.

### W15-8. A class that points into its own storage needs its own copy operations

- Pages and rules: `classes/copy-move.md` CPY-P1, CPY-P2;
  `foundations/storage.md` STG-P2.
- Kind: needs example.
- What happened: CPY-P1 says the implicit copy copies member by
  member. For `Optional<T>`, that copies the `T*` member, which then
  points into the other object's bytes. A copy would destroy the other
  object's `T` twice. The rule implies this, but I knew to look for it
  from outside the specification. `optional.cpp` writes all four copy
  and move functions.
- Suggestion: in the copy-move page, add an example of a class with a
  pointer to its own member and the copy constructor it needs.

### W15-9. Passing `reinterpret_cast<T*>(storage)` to `construct_at` is not clearly valid

- Pages and rules: `foundations/storage.md` STG-10, STG-16;
  `foundations/language-support.md` LSUP-10.
- Kind: unclear.
- What happened: `Optional::emplace` calls
  `std::construct_at(reinterpret_cast<T*>(storage), args...)`, because
  LSUP-10 gives `construct_at` a `T*` parameter. STG-16 says the cast
  gives a `T*` value that points to the bytes. `construct_at` only
  converts it to `void*`. STG-10 allows that conversion for a pointer
  to storage outside a lifetime, but does not say whether a `T*` that
  points to an object of another type counts. Both compilers and both
  sanitizers accept it.
- Suggestion: say in STG-16 or STG-10 that such a pointer value can be
  formed and converted to `void*`, and that only access through it is
  undefined.

## Attempt log

1. Read AGENTS.md, the index, translation, language-support, and the
   templates pages (declarations, concepts, constraints, packs, the
   start of specialization), then storage, unions, constructors, and
   the parts of conversions, conditional, decltype, and primary that
   the programs need.
2. `stack.cpp`: designed with a slot union after reading UNI-3 and
   LSUP-10. Built clean on the first try with both compilers and the
   sanitizers. Probe: replaced the concept test with
   `static_assert(!requires { typename Stack<int, 0>; })`. Both
   compilers reject it ("template constraint failure", "constraints not
   satisfied"), as CPT-27 and CNS-13 predict. Kept the concept.
3. `largest.cpp`: built clean on the first try. The comment named
   CPT-20, but the program uses a constrained pack in the head, so I
   corrected the comment to CPT-17 and CPT-19. `largest(-1, 1u)` gives
   4294967295, as CONV-19 step 4 says.
4. `shapes.cpp`: built clean on the first try. Probe 1: replaced the
   nested-requirement test with
   `static_assert(!requires (const Label& l) { l.area(); })`. Both
   compilers reject it, as CPT-27 says (W15-3). Probe 2: wrote
   `Polygon` with a copy of the `Shape` requirements in place of
   `Shape<T> && ...`. Both compilers report the `describe` calls as
   ambiguous, as CNS-4 and CNS-24 say.
5. `typelist.cpp`: built clean on the first try. Probe: passed the
   concept `Integral` to the template template parameter. Both
   compilers reject it (W15-4).
6. `optional.cpp`: chose to keep the pointer from placement `new`
   after reading STG-16 and STG-19 (W15-2). Probe: a self-declared
   `std::launder` fails to link on both compilers. The first build
   gave a Clang warning, `-Wself-assign-overloaded` under `-Wall`, for
   `b = b;` and `a = a;`. GCC gave none. The specification's warning
   table covers only `recommended` rules, so it says nothing about
   this warning. I changed the test to assign through a reference.
   After that, both compilers built it clean, and both sanitizer
   builds ran with exit 0 and no report.

## What worked well

- The concepts and constraints pages match the compilers exactly on
  everything I tried. CPT-10 told me to write `requires (Capacity > 0)`
  with parentheses before I tried it without. CPT-19 told me that an
  empty constrained pack is satisfied.
- CNS-P1 and CPT-P2 explain subsumption in a way a Haskell programmer
  can use. A concept that names another concept acts like a superclass
  constraint, and a copy of the text does not. The probe matched the
  page.
- The diagram in PRIM-P6 and PRIM-28 gave me the fold with an initial
  operand and the empty-pack values (`true` for `&&`, the extra operand
  otherwise) at a glance.
- TSP-P2 and ORD-14 made the type-level recursion in `typelist.cpp`
  read like pattern matching. A more specific pattern wins, as in an
  overlapping instance.
- The language-support page gave declarations for `std::construct_at`
  and the placement form that compiled on both compilers with no
  change. The LSUP-10 evidence said that the placement form need not be
  `constexpr`, which saved a question.
- STG-P2 and DTOR-6 made the life cycle of an object in raw storage
  clear: construct, explicit destructor call, construct again.
