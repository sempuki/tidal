# Predict batches: template arguments, class template deduction, and concepts, reader B

- Date: 2026-11-04
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R9-arguments (93 programs), R9-class-template-deduction (117), R9-concepts (124)

All three answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R9-arguments/answers.json
- /home/ryanm/Code/tidal/build/exercises/R9-class-template-deduction/answers.json
- /home/ryanm/Code/tidal/build/exercises/R9-concepts/answers.json

None of the three batches has extra flags. Some Evidence segments record that a compiler family differs. For those programs I gave the outcome of the rules, and in `why` I named the family and the question. These are the programs:
- arguments: p32 (Q-0170), p39 and p56 (Q-0168), p57 (Q-0117), p84 (Q-0201)
- class template deduction: p18 (Q-0277), p22 (Q-0242), p35 (Q-0244), p36 and p54 (Q-0352), p56, p74, and p117 (Clang 18 and EDG on CTAD-14), p60 (Q-0280), p69, p77, and p103 (Q-0239), p75 (Q-0279), p76 and p89 (Q-0241), p90 (Q-0276), p102 (Q-0278), p112 (Q-0243)
- concepts: p10 and p85 (Q-0270), p15 and p101 (Q-0287), p19, p74, and p78 (Q-0260), p27 (Q-0288), p33, p65, and p105 (Q-0258), p34 (Q-0286), p37 (Q-0289), p44 and p52 (Q-0259), p86 (Q-0285), p87 (Q-0290)

I used `principles` on only three arguments programs (p19, p57, p82). The comparison table of TARG-P4 gave the answer for each.

## R9B-1

- Pages and rules: templates/arguments.md TARG-6, TARG-20, TARG-21; templates/deduction.md DED-34
- Kind: hard to find
- What happened: In arguments p37, `second_is_long<int, long>(1)` gives two explicit arguments to a template whose head has one parameter, and the function has an `auto` parameter. TARG-20 says explicit arguments go to the leading parameters. TARG-6 says a template-id must not have more arguments than parameters. From the arguments page alone I would have answered reject. DED-34 is on the deduction page. It says the invented parameters of `auto` come after the head and can take explicit arguments, and that a pack at the end of the head takes them all. With DED-34 I answered compiles.
- Suggestion: In TARG-6 or TARG-20, say that the invented parameters of a function with `auto` parameters count as template parameters after the head, and cite DED-34. Add a short example with one `auto` parameter and one extra explicit argument.

## R9B-2

- Pages and rules: templates/arguments.md TARG-11, TARG-27
- Kind: needs an example
- What happened: In arguments p08, a class inherits from `Base<int>` and `Base<char>` and declares a member of type `Base<long>*`. TARG-27 says the bare name is ambiguous. TARG-27 does not say that the name with an argument list is still valid. TARG-11 implies it, and the Evidence bullet for TARG-27 states it. I answered compiles from the Evidence.
- Suggestion: Add one sentence to TARG-27 that says the name with an argument list names the template and is valid. Show both forms in an example.

## R9B-3

- Pages and rules: templates/arguments.md TARG-14; foundations/constant-evaluation.md CE-3, CE-17
- Kind: hard to find
- What happened: Arguments p45 uses `Ptr<address()>`. The `constexpr` function `address()` returns the address of a `static const int` declared in its body. The arguments page covers the address. CE-17, on another page, decides whether the call is a constant expression. CE-17 rejects a static variable that is not usable in constant expressions, and CE-3 says which variables are usable. I answered compiles. I was not sure that a `static const int` with a constant initializer is usable in constant expressions.
- Suggestion: Add an example to CE-17 with a `static const int` and a `static int` in a `constexpr` function. The first is accepted and the second rejected.

## R9B-4

- Pages and rules: templates/class-template-deduction.md CTAD-19, CTAD-22; templates/ordering.md
- Kind: needs an example
- What happened: Deduction p69 has an own guide from `Derived(const T&)` and an inherited guide from `Base(T)`. CTAD-19 applies only when the two guides have the same parameter type for each argument. These types differ, so CTAD-19 does not apply. I also had to know that partial ordering treats `T` and `const T&` as equal, so the guides tie and the call is ambiguous. The page gives that result only in an Evidence bullet. Programs p31, p34, and p95 use the same tie-breaker with equal parameter types, and the rule predicted them easily.
- Suggestion: Add two examples to CTAD-19. In the first, `Derived(T)` competes with `Base(T)` and the own guide wins. In the second, `Derived(const T&)` competes with `Base(T)` and the call is ambiguous.

## R9B-5

- Pages and rules: templates/class-template-deduction.md CTAD-6, CTAD-16
- Kind: unclear
- What happened: In deduction p114, `Derived<T>` inherits the constructors of `Base<int>`, a base that does not use `T`. CTAD-16 says the base type acts as an alias of the derived parameters. The rule does not cover a base type that uses none of them. I also could not tell whether a class that only inherits constructors "declares no constructor" for CTAD-6, which would give it the guide with no parameters. I answered reject, because no guide gives `T` from the argument `1`.
- Suggestion: In CTAD-16, say that an inherited guide cannot deduce a derived parameter that its base does not use. In CTAD-6, say whether inherited constructors count as declared constructors.

## R9B-6

- Pages and rules: templates/class-template-deduction.md CTAD-9, CTAD-10
- Kind: needs an example
- What happened: Deduction p93 initializes `template <int N> struct Bytes { unsigned char s[N]; }` with `{"abc"}`. For a string literal, CTAD-9 makes the parameter a reference to a `const` array. To answer, I had to know that `const unsigned char[N]` does not deduce from `const char[4]`, because the element types differ. The page does not say this, so I used outside knowledge. I answered reject.
- Suggestion: Add two cases to the CTAD-9 example. A string literal for a `char` array deduces. A string literal for an `unsigned char` array gives no guide.

## R9B-7

- Pages and rules: templates/class-template-deduction.md CTAD-10, Evidence on Q-0277
- Kind: unclear
- What happened: Deduction p33 gives `{"ab"}` to `template <class T> struct Three { T v[3]; }`. CTAD-10 says the braces are elided, and the Evidence says every compiler gives `T = const char*`. The Evidence also says that the source-reader review reads "the first form" of Q-0277 as giving `Three<char>`, and that this part waits on the maintainer. I could not tell which answer the rules give now. I followed the rule text and answered compiles with `Three<const char*>`.
- Suggestion: In the Evidence, say that CTAD-10 gives `Three<const char*>` and that the other reading is a doubt under review.

## R9B-8

- Pages and rules: templates/class-template-deduction.md CTAD-11; templates/instantiation.md INST-31, INST-32
- Kind: unclear
- What happened: In deduction p85, `later<int>` is instantiated before the guide `Cell(int) -> Cell<long>` and is used again after it. CTAD-11 says that INST-32 applies when two points of instantiation give different meanings. A function template also has a point of instantiation at the end of the translation unit, which comes after the guide. I could not tell whether INST-32 makes the program not valid with no diagnostic required, or whether the first point decides. The Evidence says every compiler uses the guides before the first point. I answered compiles from that bullet.
- Suggestion: In CTAD-11, give the outcome of this case. Say whether it needs a diagnostic, and that every compiler accepts it and uses the first point.

## R9B-9

- Pages and rules: templates/concepts.md CPT-27, CPT-35; Evidence (inventory item D1)
- Kind: unclear
- What happened: Concepts p111 defines `template <class T> concept Never = requires { requires false; };` and asserts `!Never<int>`. The nested requirement depends on no template parameter and fails for every argument. CPT-27 forbids only requirements that are not valid. `false` is a valid expression, so I answered compiles. The Evidence also says the page leaves out inventory item D1, a requirement that fails for every argument, because such a program needs no diagnostic. So p111 may be not valid.
- Suggestion: State the rule for a requirement that is valid and never satisfied, with its kind. Say what the compilers do when such a concept is used.

## R9B-10

- Pages and rules: templates/concepts.md CPT-33, Evidence on Q-0287
- Kind: needs an example
- What happened: Concepts p101 has a compound requirement with the expression `t.missing()`, which is not valid. Its type constraint has an extra argument, and substituting that argument instantiates a class whose `static_assert` fails. The Evidence says Clang substitutes the extra arguments before it checks "the expression and `noexcept`", and names two tests, one on the order of steps and one on `noexcept`. I could not tell whether Clang also rejects when step 1 fails, or only when step 2 fails, as in p15. I answered compiles for both programs by the rule.
- Suggestion: In the Evidence for Q-0287, say for step 1 and for step 2 whether Clang rejects when that step fails.
