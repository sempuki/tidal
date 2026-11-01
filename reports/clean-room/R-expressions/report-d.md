# Predict report D: R-postfix, R-primary, R-unary

- Date: 2026-11-01
- Role: clean-room reader
- Persona: programmer new to C++
- Inputs: AGENTS.md, and the three batch directories with their `specification/` copies (the test lists removed). Nothing was compiled.

## Batches

| Batch | Programs | reject | compiles | runs | run-fail | depends |
|---|---|---|---|---|---|---|
| R-postfix | 68 | 35 | 12 | 7 | 6 | 8 |
| R-primary | 114 | 54 | 39 | 13 | 1 | 7 |
| R-unary | 68 | 25 | 25 | 10 | 4 | 4 |

On R-postfix and R-unary, my "depends" answers match the number of disagreements that each page's Evidence segment records (7 on R-postfix plus p47, and 4 on R-unary). On R-primary, the Evidence segment records 7 disagreements. I found only 6 programs that it names (see RED-8). I answered p67 as "compiles" with low confidence.

## Findings

**RED-1**
- Batch and program: R-postfix p51.
- Rules: PFX-10, PFX-11, NAM-37.
- Kind: missing.
- What I needed: whether member access can name an enumerator (`m.red`), a static data member, a base-class member, and a qualified name after the dot (`m.Color::red`, `m.Meter::v`). PFX-11 only says the member must not be a type. Nothing covers a qualified name after `.` or `->`.
- Suggestion: add a rule or a bullet to PFX-10 that lists what member access can name. State that a qualified name can follow `.` and say how that name is looked up. Add an example.

**RED-2**
- Batch and program: R-postfix p18.
- Rules: DCL-13, PFX-11.
- Kind: hard to find.
- What I needed: the program rejects only because `W w(W());` declares a function. I found DCL-13 by searching. The postfix page gives no hint.
- Suggestion: add an example to the postfix page in which a declaration that looks like an object turns out to be a function, so the member access is rejected. Link DCL-13 from it.

**RED-3**
- Batch and programs: R-postfix p19, p25, p31, p38, p50; R-unary p19, p31, p40, p65.
- Rules: PFX-4, PFX-8, PFX-18, PFX-21, UN-4, UN-7, UN-21.
- Kind: outside knowledge.
- What I needed: which sanitizer stops which operation. PFX-21 and PFX-18 say that UBSan reports their cases. PFX-4 does not say whether the UBSan bounds check catches an out-of-range index into a local array (p25), or whether ASan catches a read past the end of a new array (p50). UN-7 does not say whether UBSan catches a read through a null pointer (R-unary p31). I answered all of these from general knowledge.
- Suggestion: in each Evidence segment for an undefined rule, add one line per sanitizer that names the cases it stops at run time.

**RED-4**
- Batch and programs: R-postfix p38; R-primary p81.
- Rules: PFX-8, PRIM-8.
- Kind: outside knowledge.
- What I needed: what happens when a program calls through a null function pointer, or writes to a string literal, under `-fsanitize=address`. Neither operation is a memory error that ASan checks. I assumed that the run faults, that ASan reports the fault, and that the run therefore ends with a nonzero status.
- Suggestion: state the run-time result on the matrix targets in the Evidence. For example, say that string literals live in read-only memory there, so the write faults.

**RED-5**
- Batch and program: R-postfix p45.
- Rules: CLS-13, PFX-20.
- Kind: outside knowledge.
- What I needed: whether the older baseline compilers (Clang 18, GCC 14) support explicit object parameters. The README counts a result on any baseline compiler. I assumed that all of them support explicit object parameters.
- Suggestion: in the CLS-13 Evidence, say which baseline compilers support explicit object parameters.

**RED-6**
- Batch and programs: R-postfix p24, p03.
- Rules: PFX-11, NAM-6.
- Kind: unclear.
- What I needed: PFX-11 says that inside the class being defined, member access sees only members declared before it, and it cites NAM-6. NAM-6, as the names page describes it, makes later members visible in a class body. The two statements read as a contradiction until I worked out that a member declaration is not a place where later members are visible.
- Suggestion: say in PFX-11 which places in a class body see only earlier members (member declarations, such as a decltype in a member's type) and which places see all members (function bodies). Add p24 as an example.

**RED-7**
- Batch and programs: R-postfix p41, p13.
- Rule: PFX-13.
- Kind: unclear.
- What I needed: the Evidence says "Clang rejects ++ on bool as a pedantic diagnostic". That wording suggests Clang would accept the program without `-pedantic-errors`. The Evidence does not say what GCC and EDG do. p13 (`void*` incremented) has the same question for a GCC extension.
- Suggestion: say which compilers give a hard error and which give a warning that `-pedantic-errors` turns into an error, for both bool and `void*`.

**RED-8**
- Batch and programs: R-primary p05, p38, p58, p75; R-primary p67.
- Rules: PRIM-32, PRIM-13, PRIM-15.
- Kind: hard to find.
- What I needed: which compilers run a test that uses `-m32` or `-Werror`. The unary page says that EDG skips such tests. The primary page does not, and the baseline compiler table says only that each test runs on each compiler that supports its flags. I applied the unary note to the primary batch. The primary Evidence says that 7 of 114 tests disagree. I can match only 6 to its bullets: p03, p06, p12, p22, p35, and p104. So I cannot tell which program is the seventh.
- Suggestion: put the note about EDG skipping `-m32` and `-Werror` tests on the baseline compilers page. In each Evidence segment, make the disagreement bullets account for every test that disagrees.

**RED-9**
- Batch and programs: R-primary p67, p12.
- Rules: PRIM-30, PRIM-32.
- Kind: unclear.
- What I needed: PRIM-32's Evidence says that EDG rejects `"é"` because its ordinary encoding is not UTF-8. The Evidence does not say how EDG reads non-ASCII source characters in `u'é'`, `u'€'`, or `L'é'`. So I could not predict EDG on p67.
- Suggestion: state the source encoding that EDG assumes in the matrix. State whether EDG accepts non-ASCII characters in prefixed literals.

**RED-10**
- Batch and program: R-primary p89.
- Rules: PRIM-12, PRIM-25.
- Kind: missing.
- What I needed: whether a member function of a nested class can use a non-static data member of the enclosing class by its bare name. PRIM-25 says "outside a member function". In p89, `g` is a member function, but of the wrong class.
- Suggestion: reword PRIM-25 to say "outside a member function of that class or a class derived from it". Add the nested-class case as an example.

**RED-11**
- Batch and program: R-primary p101.
- Rules: PRIM-25, PRIM-20.
- Kind: unclear.
- What I needed: whether a static member function counts as a "member function" in PRIM-12 and PRIM-25. PRIM-20 forbids `this` there. No rule plainly forbids naming a non-static data member there.
- Suggestion: add the static member function case to PRIM-25.

**RED-12**
- Batch and programs: R-primary p38, p58, p75.
- Rules: PRIM-13, PRIM-6, PRIM-18.
- Kind: outside knowledge.
- What I needed: which compilers warn by default about a multi-character literal and about an out-of-range float literal. Under `-Werror`, the warnings decide the outcome. PRIM-13 recommends the warnings, but the Evidence does not say who gives them. The Evidence also does not say whether an unknown escape such as `\q` is an error or only a warning.
- Suggestion: add an Evidence line that names the warnings Clang and GCC give for these literals.

**RED-13**
- Batch and program: R-unary p01.
- Rules: UN-16, UN-17.
- Kind: unclear.
- What I needed: UN-16 says that an array of unknown bound has its element's alignment. UN-17 says that the operand must be a complete object type. An array of unknown bound is incomplete, so the two rules seem to disagree.
- Suggestion: list the array of unknown bound as an exception in UN-17.

**RED-14**
- Batch and programs: R-unary p11, p63.
- Rule: UN-18.
- Kind: outside knowledge.
- What I needed: p11 asks whether a destructor with no `noexcept` (Calm) counts as a non-throwing function in `noexcept(...)`. p63 asks whether the global deallocation functions are `noexcept`. UN-18 covers allocation functions only. I answered both from general knowledge.
- Suggestion: say in UN-18 that a destructor is non-throwing unless it says otherwise. Say that the global deallocation functions are non-throwing. Link to the exception-specification page.

**RED-15**
- Batch and program: R-unary p53.
- Rules: UN-5, UN-6, UN-1.
- Kind: missing.
- What I needed: whether `**f` works on a function pointer. `*f` gives a function, and UN-6 requires the operand of `*` to be a pointer. UN-1 states the conversion from a function to a pointer only for `+` and `-`.
- Suggestion: say in UN-5 or UN-6 that a function operand of `*` converts to a pointer first. Add `(**f)()` to the examples.

**RED-16**
- Batch and program: R-unary p18.
- Rule: UN-17.
- Kind: unclear.
- What I needed: UN-17's text lists types only. The syntax shows that `alignof` takes only a type, and the Evidence says that an expression operand is an extension. The rule text never says that an expression operand is invalid.
- Suggestion: add a bullet to UN-17 saying that the operand must be a type and not an expression.
