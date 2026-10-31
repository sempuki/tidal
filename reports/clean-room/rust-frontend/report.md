# A C++ front end built from the Tidal specification

Date: 2026-10-31. Decision 0029. The reader was a clean-room reader that
built `tidal-fe`, a hobby front end in Rust, in `frontend/`. Its only
inputs were the files under `spec/`, `docs/`, `tests/`, and `questions/`.
Where the text did not settle a question, the front end makes a choice,
and the finding says which. The numbers below are for the 3058 tests of
that day. `tidal-fe conform ../tests` reproduces them in about 0.2
seconds.

## 1. Summary

### What the front end does

`tidal-fe check FILE` lexes and parses one translation unit, runs
semantic checks, and evaluates constant expressions. It gives one of
three answers:

- accept (exit 0),
- reject (exit 1, with a rule ID when one applies),
- unsupported (exit 2), when it met a construct it does not model and
  found no error.

It has these parts:

- A lexer with the literal checks of the PRIM rules.
- A parser for declarations, declarators, classes, enumerations,
  namespaces, templates (parsed, not instantiated), lambdas, statements,
  and expressions.
- Scopes and name lookup that follow NAM-P1 to NAM-P4, with a small
  argument-dependent lookup.
- Type rules for the built-in operators and the standard conversions.
- Initialization of scalars, arrays, references, and simple aggregates.
- `auto`, `decltype`, and return type deduction.
- Checks for about 150 rules across the six chapters. The statements,
  enumerations, arithmetic operators, and constant evaluation are the
  most complete.
- A constant evaluator. It runs `constexpr` functions (loops, `switch`,
  recursion, local arrays and aggregates, pointers into them). It
  rejects each operation that a CE rule or an `undefined` rule names.
- A variable-template matcher, enough to evaluate the `same<A, B>` idiom
  that 182 tests use.

It does not instantiate templates, run constructors or destructors in
constant evaluation, check access control, or do full overload
resolution. Where one of these decides the answer, it reports
"unsupported", or it treats the type as unknown and checks nothing that
depends on it.

### Conformance

Expectations use `tidal-expect-edg` where present. `run`, `run-fail`,
and `link-fail` count as accept.

| Directory | Tests | Pass | False reject | Missed reject | Unsupported | Differs from EDG only |
|---|---|---|---|---|---|---|
| classes | 848 | 476 | 0 | 318 | 54 | 0 |
| declarations | 890 | 709 | 0 | 125 | 41 | 15 |
| exceptions | 133 | 106 | 0 | 25 | 2 | 0 |
| expressions | 908 | 737 | 0 | 87 | 68 | 16 |
| foundations | 82 | 74 | 0 | 4 | 4 | 0 |
| statements | 197 | 189 | 0 | 5 | 2 | 1 |
| **total** | 3058 | 2291 | 0 | 564 | 171 | 32 |

"Differs from EDG only" counts tests where the answer matches the base
`tidal-expect` line and the rule text, and differs from the EDG override
(F20).

Accepting is cheap for a front end, so the reject side says more:

- 1350 tests expect a rejection. The front end rejects 717 of them (53%).
- In 491 of those 717, the diagnostic names a rule that the test lists.
- It rejects none of the 1708 valid programs.

The areas chosen for milestones 3 and 4:

| Area | Tests | Pass | Missed reject | Unsupported |
|---|---|---|---|---|
| statements (all) | 197 | 189 | 5 | 2 |
| declarations/enumerations | 53 | 51 | 0 | 0 |
| declarations/static-assert | 28 | 26 | 1 | 1 |
| declarations/constexpr | 53 | 45 | 2 | 6 |
| expressions/additive, multiplicative, shift, bitwise, logical | 145 | 136 | 1 | 4 |
| expressions/constant-expressions | 46 | 33 | 1 | 11 |

The largest groups of unsupported tests are `noexcept` of class
operations (19), designated initializers in constant evaluation (19),
`same<...>` with an argument the front end cannot type (16), `new` in
constant evaluation (15), and calls through objects or lambdas (12).
Most missed rejections are in classes: constructors, copy and move,
access, virtual functions, and overload resolution.

### The three biggest problems for an implementer

1. **The grammar cannot drive a parser.** The Syntax segments define 156
   productions and refer to 53 more that no page defines. Among the
   missing ones are `statement`, `declaration`, `simple_declaration`,
   `decl_specifiers`, `type_id`, `cast_expression`, `condition`, and
   `literal`. The expression productions do not connect into one
   precedence chain. Names and notation change from page to page (F1 to
   F8).
2. **The rule class does not say what to diagnose.** 578 of the 1374
   reject tests cite no rule of class `reject`. 420 of them cite only
   `required` rules that contain a prohibition, such as JMP-3, ENUM-3,
   DCL-5, or CMP-10. Some rules contradict each other or a test (F15 to
   F18).
3. **Everything before the in-scope rules is outside the spec, and it
   decides most outcomes.** A front end needs lexing, name lookup beyond
   blocks, overload resolution, templates, and the target's sizes and
   layout before any page applies. The `same<A, B>` idiom alone puts
   partial specialization of variable templates in front of 182 tests
   (F9 to F14).

## 2. Findings

### Grammar

**F1. Central productions are missing.**
- Page: all Syntax segments. Kind: grammar gap.
- These 53 nonterminals are used and never defined: alias_declaration,
  assignment_expression, attr_namespace, binary_digits,
  brace_or_equal_init, braced_init, capture_list, cast_expression,
  class_name, compound_statement, condition, constant,
  constant_expression, ctor_initializer, decimal, decl_specifier,
  decl_specifiers, declaration, delimiter, equality_expr,
  expression_list, function_decl, hex_digits, identifier,
  identifier_list, inclusive_or, init_stmt, lambda_expression, literal,
  logical_or_expr, member_declaration, member_function_decl, name,
  noexcept, octal_digits, param, parameters, postfix_expression,
  primary_expr, qualified_name, raw_char, ref_qualifier, return_type,
  s_char, simple_declaration, statement, tokens, type, type_constraint,
  type_id, type_name, type_specifiers, unary_expression.
- Choice: these parts of the parser came from general knowledge.
- Suggestion: give the chapter index pages Syntax segments for
  `statement`, `declaration`, `decl_specifiers`, and `type_id`. Add a
  check in `tools/check_spec.py` that every referenced nonterminal is
  defined.

**F2. The expression productions do not form one chain.**
- Pages: multiplicative.md, member-pointers.md, unary.md, logical.md,
  bitwise.md, casts.md, throw.md, conditional.md. Kind: grammar gap.
- `mul_expr` is built on `unary_expression`, which skips casts and `.*`.
- `unary_expr` lacks `!`, `~`, casts, `new`, `delete`, and `alignof` of
  an expression. `logical_not` and `complement` are separate productions
  that nothing refers to.
- `throw_expr` is not an alternative of any production.
- `conditional_expr` uses `logical_or_expr`, but logical.md defines
  `logical_or`.
- The prose rules on precedence (MUL-1, ADD-1, SHF-1, CMP-1, BIT-1,
  LOG-6) were clear and gave the order used.
- Suggestion: put the whole chain on expressions/index.md, with
  consistent names.

**F3. The notation changes between pages.**
- Pages: auto.md, declarators.md, primary.md, and enumerations.md use
  `: |` continuation lines. multiplicative.md uses `( )*`, additive.md
  uses `{ }`, blocks.md uses `x*`. `cv` is defined twice (declarators.md
  and structured-bindings.md). Kind: grammar gap.
- Choice: `: |` read as a new alternative.
- Suggestion: a notation section in the style guide, and one definition
  for each nonterminal.

**F4. The declarator grammar is incomplete.**
- Page: declarators.md. Kind: grammar gap.
- The function suffix has no ref-qualifier, no `noexcept`, and no
  attributes. Pointer to member is a separate production on
  member-pointers.md. `params` uses `param`, but default-arguments.md
  defines `parameter`. No rule says how to build a type from a nested
  declarator.
- Choice: apply the pointer operators first, then the suffixes from the
  innermost outward, then the parenthesized inner declarator. The test
  function-returning-pointers-ok.cpp confirms this order.
- Suggestion: one complete production, and a worked example for
  `int (*f())[3]` and `void (S::*p)() const`.

**F5. Case labels are not in the statement grammar.**
- Pages: blocks.md, jumps.md, switch.md. Kind: grammar gap.
- `labeled_stmt` covers only `identifier ":"`. `case_label` and
  `default_label` are defined but nothing uses them. STMT-5 says a case
  or default label can end a block, but `compound_stmt` allows only
  `identifier ":"` there.
- Suggestion: add the case and default forms to `labeled_stmt`, and use
  that production in `compound_stmt`.

**F6. Condition and init-statement productions have three sets of
names.**
- Pages: if.md uses `if_init`, `if_cond`, and `braced_init`. loops.md
  and switch.md use `init_stmt` and `condition`. list.md uses `braced`.
  range-for.md uses `for_init`. Kind: grammar gap.
- `if_init` lists both `expression_stmt` and `";"`. `init_stmt` and
  `condition` are never defined.
- Suggestion: define `init_statement`, `condition`, and
  `braced_init_list` once and use them on all pages.

**F7. Literals have no complete grammar.**
- Page: primary.md. Kind: grammar gap.
- There are no productions for floating literals or character literals.
  The digit and character productions (`decimal`, `octal_digits`,
  `s_char`, and so on) are undefined. User-defined literal suffixes
  appear only in operators.md. The PRIM rules were enough to write the
  literal checks.
- Choice: scan a preprocessing number greedily, then split it into
  parts.
- Suggestion: add the missing literal productions.

**F8. "The whole statement decides" is not a procedure.**
- Rules: STMT-6, DCL-13. Kind: needs example.
- The front end needs a decision for each of these: `T(x);`,
  `T(x).f();`, `T(*p);`, `void (V::* p)() = &V::f;`, `int y(S{});`, and
  `T(x)(y);`.
- Choice: a token look-ahead heuristic.
- Suggestion: a table of example statements and how each one reads.

### Outside the specification

**F9. Lexical structure.** Kind: outside the spec. The front end needed
the token set and maximal munch, the keywords, the alternative tokens
(LOG-13 lists them), digraphs (no page mentions them; not supported),
`>>` closing two template argument lists, the encoding of non-ASCII
characters in plain strings, which decides `sizeof` (UTF-8 was used),
and the Unicode name table for `\N{...}` (PRIM-5 and PRIM-18; reported
as unsupported). Suggestion: a short lexical page that includes the
target's character sets and the Unicode version.

**F10. Name lookup beyond blocks.** Rules: NAM-5, DER-3, DER-4, TYN-4.
Kind: outside the spec. The front end needed lookup into base classes,
injected class names, and the rule that a function or variable hides a
class of the same name in the same scope. elaborated-hides-function.cpp
needs `stat(1)` to name the function; TYN-4 states only the reverse
direction. NAM-5's "namespaces of the argument types" had to cover
pointers to classes, enumerations, and hidden friends. Without
argument-dependent lookup, adl-plus-ordinary.cpp gives a false
rejection. Suggestion: list the associated namespaces and classes, and
add the hiding rule.

**F11. Overload resolution.** Rules that depend on it: MUL-3, ADD-3,
SHF-3, BIT-3, CLS-5, CLS-6, and the DCMP rules. Kind: outside the spec.
MUL-3 rejects a class operand "unless overload resolution finds" an
operator. Choice: rank each argument as exact, promotion, conversion, or
user-defined, and report an unknown type on ties. For class operands,
reject only when the class has no conversion function and no operator
is visible. `operator==` counts for `!=` and `operator<=>` for the
relational operators; without these rewritten forms, ten comparison
tests gave false rejections. Suggestion: a minimal informative ranking,
and a list of the candidates each built-in operator considers.

**F12. Templates.** 182 tests check types with `same<A, B>` and a
partial specialization, and docs/spec-format.md recommends this idiom.
Kind: outside the spec. Choice: implement only variable templates whose
specialization patterns are plain parameters; every other template use
gives an unknown type. IF-13, IF-14, and SA-5 describe behavior that
depends on instantiation (F22). Suggestion: specify the template subset
the tests rely on, or state that it is out of reach for a front end
without templates.

**F13. The target model is scattered.** Rules: UN-15, MUL-15, PRIM-6,
ALN-5, BF-9, BF-10, CLS-11, VC-10. Kind: outside the spec
(implementation-defined). Tests need exact values: sizes, alignments,
the signedness of `char` and `wchar_t`, the `size_t` and `ptrdiff_t`
types, padding, the size of an empty class, bit-field allocation, and
`long double`. Four tests use `-m32`. The glossary's promotion rule for
`wchar_t`, `char16_t`, and `char32_t` needs their widths and signedness.
Choice: LP64 with natural layout, and ILP32 under `-m32`; layouts with
bases, virtual functions, or bit-fields are unsupported. Suggestion: one
"Matrix targets" page with the full table.

**F14. "Literal type" and "constant initializer" are not defined.**
Rules: CX-1, CV-8, CE-3. Kind: missing. CE-3 points to CV-8, and CV-8
uses the same undefined phrase. CX-1 needs a literal type, which no page
defines. Choice: a constant initializer is one whose evaluation succeeds
as a constant expression, and a class with a user-provided destructor
that is not `constexpr` is not a literal type. Suggestion: add both
terms to the glossary.

### Rule classes and contradictions

**F15. Prohibitions sit in `required` rules.** Kind: ambiguous. 420
reject tests cite only `required` rules, 268 distinct rules in all, for
example JMP-3, ENUM-3, ENUM-4, CMP-10, DCL-3, DCL-5, DCL-6, DCL-7, CLS-4,
and NS-7. Another 84 reject tests cite only `undefined` rules
(consistent with CE-9), and 59 cite only `recommended` rules and use
`-Werror`. Suggestion: split each prohibition into a `reject` rule, and
add a check that every rule a reject test cites is `reject` or
`undefined`.

**F16. TR-3 and FN-6 disagree about `main`'s parameters.** Kind:
contradictory. TR-3 (reject) allows only `()` and `(int, char**)`. FN-6
(implementation-defined) says other lists are implementation-defined.
The EDG overrides accept `(int)` and a four-parameter `main`. Choice:
follow FN-6 and accept with a warning; following TR-3 gave five false
rejections. Suggestion: TR-3 should require only that the two standard
forms are accepted.

**F17. CX-1 and CINIT-1 say "must have an initializer".** Kind:
contradicts tests. ctor-on-const-this-type.cpp and virtual-call.cpp
accept `constexpr S s;` when `S` has a constexpr default constructor.
Choice: class types need no written initializer. Suggestion: say that
default initialization through a constexpr constructor counts.

**F18. The type of an enumerator inside its own list.** Rules: ENUM-10,
ENUM-3. Kind: missing. Read together, ENUM-10 and ENUM-3 reject
`enum class E { A = 4, B = A * 2 };`, but enumerator-refers-earlier.cpp
expects it to be valid. Choice: until the closing brace, an enumerator
has the fixed underlying type; without a fixed type, it has the first
integer type that holds its value. Suggestion: add a rule for this.

**F19. CE-9 names a few undefined rules, but many more apply.** Kind:
missing. The front end also needed SHF-7, UN-4, UN-7, UN-21, MUL-10,
MUL-11, MUL-12, ADD-9, ADD-11, ADD-17, CONV-5, ENUM-6 with CAST-14, FN-3,
PFX-4, PFX-8, and NEW-5, found only by reading every page. Suggestion: a
generated index of the `undefined` rules on constant-evaluation.md, each
marked with whether constant evaluation rejects it in each compiler.

**F20. Some EDG overrides contradict the rule text.** Kind: test
expectation disagrees with the spec. Affected rules and questions:
MUL-11 (Q-0002), MUL-10 (Q-0003), INIT-1 (Q-0013), ENUM-6 (Q-0017),
CMP-9 (Q-0030), CE-13 and PRIM-9 (Q-0033), LAM-6 (Q-0037), PRIM-2
(Q-0064), REF-5 (Q-0110), and others. In 32 tests the front end follows
the rule and loses against EDG. Example: floating-divide-by-zero.cpp
expects EDG to accept, while MUL-11 is `undefined` and CE-9 says
constant evaluation rejects it. Suggestion: note the divergence in each
rule, and count either answer as conforming for front ends.

### Ambiguities and smaller gaps

**F21. MUL-10's second bullet is hard to parse.** Suggestion: "`a % b`
is undefined if `a / b` is undefined."

**F22. Which parts of an uninstantiated template are checked.** Rules:
IF-13 and SA-5. Kind: ambiguous, and untestable without templates.
Choice: check only fully known types, and skip every static assertion
inside a template. Suggestion: state which diagnostics are required,
allowed, or forbidden, with examples.

**F23. Return type deduction.** Rules: RET-1, RET-2. Kind: needs
example. RET-2 says the variable rules apply to return types, but
`return g();` for a `void g()` deduces `void`, and AUTO-8 forbids `void`
for a variable. Suggestion: say this in RET-1.

**F24. Bit-field promotion.** The glossary entry said bit-field
promotion is "(referenced)", while CONV-1 and BF-6 define it.
Suggestion: link the glossary entry to BF-6.

**F25. Default arguments accumulate across declarations.**
added-later.cpp depends on this, and constant evaluation must use the
default from whichever declaration gave it. Suggestion: add an example
with three declarations.

**F26. CLS-9 and data members with the class name.** Kind: none found;
noted because the front end first got it wrong. CLS-9 lists the members
that must not have the class name and leaves out non-static data
members, so `struct S { int S; };` is valid. Suggestion: add this
positive case to the Examples segment.

## 3. What worked well

- **Rule IDs.** They are short and stable, and easy to print in
  diagnostics. In 491 of the 717 rejections, the rule printed is one
  the test lists.
- **The statement pages.** IF-8 to IF-10, LOOP-4, LOOP-7, SW-3 to SW-5,
  JMP-5 to JMP-7, and IF-17 are precise and decide the corner cases in
  the text. 189 of 197 statement tests pass.
- **The enumeration page.** 51 of 53 tests pass. The other two differ
  only from EDG. F18 was the one gap.
- **Constant evaluation.** CE-1 to CE-18, together with the `undefined`
  rules on each operator page, read as a checklist for an interpreter.
- **The arithmetic pages.** Each lists operand types, result type, and
  undefined cases in the same order. The glossary's usual arithmetic
  conversions and integer promotion are numbered algorithms that
  translate directly into code.
- **The tests.** One small program per behavior, plus the EDG overrides
  and `tidal-note` links, made each bug quick to find.
- **Examples segments.** They settled several readings, such as the
  DCL-1 binding order and SW-3 narrowing.
