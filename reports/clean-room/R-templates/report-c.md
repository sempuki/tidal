# Predict exercise report: templates batches, reader C

- Date: 2026-11-01
- Role: clean-room reader
- Persona: programmer new to C++
- Inputs read: AGENTS.md, specification/ pages (index, glossary, foundations/translation, foundations/constant-evaluation for CE-23, templates/declarations, templates/specialization, templates/dependent-names, templates/instantiation, templates/arguments for TARG-2 and TARG-23), and the four batch directories. I did not compile or run anything, and I made no git changes.

## Batches and counts

All four answers.json files are written and parse as valid JSON. Each has one entry per program in its README.

| Batch | Programs | runs | compiles | reject | link-fail | depends |
|---|---|---|---|---|---|---|
| build/exercises/R-declarations | 117 | 16 | 32 | 61 | 1 | 7 |
| build/exercises/R-specialization | 93 | 9 | 32 | 37 | 4 | 11 |
| build/exercises/R-dependent-names | 108 | 42 | 15 | 33 | 0 | 18 |
| build/exercises/R-instantiation | 127 | 16 | 27 | 64 | 4 | 16 |

I gave no `run-fail` or `undefined` answers. Every `depends` answer comes from a compiler split that the page's Evidence records, and its `why` names the compilers. For programs where a no-diagnostic rule is broken but every compiler accepts, I answered with what the compilers do (`runs` or `compiles`) and said so in `why`.

## Specification findings

**RC-1.** R-declarations p70. Rules TPL-40, DEP-30. Kind: unclear.
- **Needed:** A member defined outside the class uses its own head, and that head has no inner default. The body then writes `C<>`, and the class is never instantiated. I needed to know whether the missing argument is diagnosed at the definition. TPL-40 says which default applies but not when the error is found. I answered reject.
- **Suggestion:** Say whether a template-id built from a template template parameter is checked when the template is defined, or only when it is instantiated.

**RC-2.** R-declarations p08, p25, p79. Rules TPL-6, TPL-7, CE-23. Kind: unclear.
- **Needed:** Whether a class used as a constant parameter type needs a constexpr constructor. The literal type definition in CE-23 and the glossary mentions only the destructor, the bases, and the members. I could decide p79 (user-provided destructor) from it. A new reader may still wonder about constructors.
- **Suggestion:** Add one sentence to TPL-6 or CE-23 that says what a literal class needs, or does not need, about its constructors.

**RC-3.** R-declarations p96, p105. Rule TPL-11. Kind: needs example.
- **Needed:** For `Hold<4>` with `template <Wrap W>`, how the argument becomes `Wrap<int>`. I also needed to know that a user deduction guide applies (`Hold<3>` gives `Size<long>`). TPL-11 refers to CTAD-1, but the page has no example with a guide.
- **Suggestion:** Add an example with a class-template-named parameter and a deduction guide, and show the resulting parameter type.

**RC-4.** R-declarations p22. Rule TPL-18. Kind: hard to find.
- **Needed:** The statement that a friend function named after a template parameter is valid sits inside a `reject` rule. EDG rejects it only when the class is used. I had to read the Evidence to learn that the use matters.
- **Suggestion:** Move the friend case into its own `required` rule, or into an exception, with the EDG split noted there.

**RC-5.** R-declarations p73, p115. Rule TPL-37. Kind: unclear.
- **Needed:** The Evidence gives the GCC split only for a friend declaration of a class template. It says nothing about a friend function template declaration (not a definition) with a default. I assumed every compiler rejects that case.
- **Suggestion:** State the result for each friend form: class template friend, function template friend declaration, and function template friend definition.

**RC-6.** R-declarations p43. Rule TPL-14. Kind: needs example.
- **Needed:** The Evidence says GCC checks only the defaults on one declaration. I had to work out that a single declaration that gives a default before a parameter with none is rejected by GCC too.
- **Suggestion:** Give one example that every compiler rejects and one that only Clang and EDG reject.

**RC-7.** R-specialization p38. Rule TSP-14 (Q-0295). Kind: missing.
- **Needed:** The Evidence covers `int V` with the argument `1L`. It does not cover a `char` argument such as `'c'` against `int V`, where Clang may also convert and match.
- **Suggestion:** Record the result for an integral argument of a narrower type as well as a wider one.

**RC-8.** R-specialization p52, and the Clang 18 notes in TSP-14 and TSP-21. Kind: unclear.
- **Needed:** The Evidence names differences in Clang 18 that Clang 22 does not have. I could not tell whether an old compiler in the matrix counts as "the compilers differ" for a `depends` answer. I answered with the current compilers' result.
- **Suggestion:** Say on each page, or in the exercise README, which compiler versions decide a `depends` outcome.

**RC-9.** R-specialization p77. Rules TSP-25, plus the concepts pages. Kind: outside knowledge.
- **Needed:** A requirement that names a type built from a call that is not a constant gives `false`. It is not an error. I used general knowledge for this and did not read the concepts pages.
- **Suggestion:** Link from TSP-25 to the rule that makes such a requirement false, or add this case as an example.

**RC-10.** R-dependent-names p09. Rules DEP-8, DEP-9, TARG-23. Kind: hard to find.
- **Needed:** `probe<T>(x)`, where nothing named `probe` is visible at the definition. The dependent-names page does not say that the `<` opens a template argument list. TARG-23 on the arguments page says so, but DEP-8 does not link to it.
- **Suggestion:** Add a link to TARG-23 in DEP-8, next to "explicit template arguments that are dependent".

**RC-11.** R-dependent-names p04, p08. Rules DEP-13, DEP-35. Kind: outside knowledge.
- **Needed:** Why `this->m` and `Both::m` are not ambiguous when `Fixed` is a virtual base of both `Both` and `Extra`. Contrast p68 and p95, where the base is not virtual. I used general knowledge that a member of a derived class hides the same member of a shared virtual base.
- **Suggestion:** Link to the class-member lookup rule that covers hiding through a virtual base, and add a contrasting example to DEP-13.

**RC-12.** R-dependent-names p55. Rules DEP-19, DEP-23. Kind: missing.
- **Needed:** A using-declaration that names a dependent base's member type without `typename`. DEP-23 covers only the form with `typename`, and DEP-20 does not list using-declarations. I inferred reject from DEP-19.
- **Suggestion:** State directly that without `typename` the using-declaration does not declare a type name.

**RC-13.** R-dependent-names p69, p62. Rules DEP-9, DEP-31 (Q-0249). Kind: unclear.
- **Needed:** DEP-31 says a default added later breaks the rule "also for a call with a dependent argument". But DEP-9 lists a test of that kind (`argument-lookup-later-default`) as required behavior. The Evidence gives the compiler split only for the qualified call. I could not tell what the compilers do for the unqualified dependent call. I answered runs.
- **Suggestion:** Give the result for the unqualified form in the Evidence, and say how DEP-9 and DEP-31 fit together.

**RC-14.** R-dependent-names p86. Rule DEP-34 (Q-0245). Kind: unclear.
- **Needed:** The Evidence names only the arrow form `p->template ~T()`. The rule's tests include a dot form. I assumed the dot form splits the compilers the same way.
- **Suggestion:** Name both forms in the Evidence bullet.

**RC-15.** R-dependent-names p23, p98. Rules DEP-32, DEP-4. Kind: outside knowledge.
- **Needed:** `sizeof(T) - sizeof(T) - 1` is an unsigned value that wraps to a very large bound. It is not negative. Such a bound makes the array invalid for every `T`. The specification did not tell me the type of `sizeof` or the wrap.
- **Suggestion:** Link DEP-32's example to the rules on the type of `sizeof`, unsigned arithmetic, and array bound limits.

**RC-16.** R-dependent-names p02, p108, p98. Rules DEP-33, DEP-32. Kind: outside knowledge.
- **Needed:** `void(T)` and `int(T)` inside `sizeof( )` are read as function types. `sizeof` of a function type is not valid. I used general knowledge for both facts.
- **Suggestion:** Add an example to DEP-33 that says why the operand is a function type and why it is never valid with an element.

**RC-17.** R-dependent-names p64, R-instantiation p41. Rule INST-6. Kind: outside knowledge.
- **Needed:** An array bound of zero is not valid under `-pedantic-errors`. I did not find this on the pages I read.
- **Suggestion:** Link from INST-6 or DEP-4 to the declarator rule on array bounds.

**RC-18.** R-dependent-names p70. Rule DEP-19. Kind: unclear.
- **Needed:** `int n(T::count);` in a function body could declare a function or a variable. DEP-19 implies that `T::count` is a value, so the line declares a variable. The specification does not say how this choice is made.
- **Suggestion:** Add this form to the DEP-19 grid, or give an example in DEP-19.

**RC-19.** R-dependent-names p11, p18, p04, p08; R-instantiation p13, p67, p72, p92, p101. Rules DEP-35, DEP-38, INST-32, INST-35. Kind: unclear.
- **Needed:** The README outcomes have no entry for a program that is not valid when no diagnostic is required. I answered with what the compilers do (runs or compiles). A reader cannot tell whether the expected answer is the compiler behavior or a mark that the program is invalid.
- **Suggestion:** Add an outcome such as `no-diagnostic` to the README, or say which outcome to use for these programs.

**RC-20.** R-instantiation p56, p75. Rules INST-15, INST-14. Kind: unclear.
- **Needed:** The rule class is `undefined`, but the endless chain happens during translation, and every compiler rejects. The README's `undefined` outcome means behavior at run time. I answered reject.
- **Suggestion:** Say in INST-15 that implementations stop translation in practice, or give the README an outcome for undefined behavior during translation.

**RC-21.** R-instantiation p68. Rule INST-28. Kind: unclear.
- **Needed:** The result depends on the optimization level: `-O0` fails to link and `-O2` runs. The README gives no `-O` flag. I assumed the default level and answered link-fail.
- **Suggestion:** State the optimization level in the README, or answer such cases as `depends`.

**RC-22.** R-instantiation p36. Rule INST-22 (Q-0255). Kind: unclear.
- **Needed:** INST-22 says an exception specification on an explicit instantiation must match. It does not say whether leaving out the specification counts as a match, when the template is `noexcept`. I answered that leaving it out is valid.
- **Suggestion:** Say whether leaving out the specification is allowed.

**RC-23.** R-instantiation p67, p101. Rule INST-35. Kind: outside knowledge.
- **Needed:** What the linker does with two explicit instantiation definitions in two units. Also what it does with an explicit instantiation in one unit and an explicit specialization in another. The Evidence for these tests gives no link results. I used general knowledge that the linker keeps one copy and does not report the clash.
- **Suggestion:** Add the observed link and run results for these tests to the Evidence.

**RC-24.** R-instantiation p89. Rule INST-30. Kind: needs example.
- **Needed:** Where the point of a class specialization falls when a function specialization needs the class and is itself used later. I had to work out the order of the two points by hand.
- **Suggestion:** Add a diagram like the one in INST-P4 for this nested case.

**RC-25.** R-instantiation p72, p92. Rules INST-31, INST-33, INST-32. Kind: unclear.
- **Needed:** A function specialization has two points. INST-33 says an instantiation does not see a declaration after its point, but only constant evaluation is tied to the first point. For an ordinary call, I needed the Evidence ("every compiler takes the declarations at the end of the unit") to decide.
- **Suggestion:** State in INST-31 or INST-33 that a non-constant use can see declarations up to the last point, and that any difference between the points falls under INST-32.
