# Roadmap: what to specify next, from a language designer's view

Date: 2026-10-31. Decision 0037. A clean-room reader in the role of a
language designer read AGENTS.md, docs/ (roadmap, exercises, status,
style guide, spec format, decision 0037), the four other roadmap views,
the question titles and Q-0007 and Q-0029, and every page under spec/
(principles, grammar, and rules; Examples and Evidence segments only
where a proposal needed them). Other agents were editing pages into
lists while it read; content did not change. All features below are the
reader's own inventions, designed from Tidal alone.

Findings are D1 to D33. Each names a kind and the page it belongs on.

## 1. Portfolio

Sixteen proposals. Breadth and depth at a glance:

| # | Proposal | Area | Changes existing behavior | Deep interaction | Rating |
|---|---|---|---|---|---|
| P1 | Width-named integer literal suffixes | lexical, types | rarely (reserved suffixes) | conversions, literal operators | partial |
| P2 | Narrowing is an error in every implicit conversion | types, conversions | yes | overload resolution | partial |
| P3 | Left-to-right evaluation everywhere | evaluation order | yes | overloaded operators, constant evaluation | ready |
| P4 | Chained comparisons with their mathematical meaning | expressions | yes | overload resolution, rewritten comparisons, temporaries | partial |
| P5 | Automatic objects start at zero | initialization, lifetime | yes (undefined becomes defined) | constant evaluation, jumps | partial |
| P6 | Named arguments | declarations, calls | no | overload resolution, default arguments | blocked |
| P7 | Temporaries live through reference-returning chains | lifetime | yes (destructor timing) | temporaries, range-for, constant evaluation | ready |
| P8 | Relocation: a move that ends its source | classes, lifetime | no | copy and move, destructors, unwinding | partial |
| P9 | Overload sets as values | generic code | no | overload resolution, templates, lookup | blocked |
| P10 | Expansion over the elements of an object | generic code, statements | no | structured bindings, templates, constant evaluation | blocked |
| P11 | Exceptions that stay inside a constant evaluation | compile time, errors | no | constant evaluation, handlers, lifetime | ready |
| P12 | Constant parameters | compile time | no | overload resolution, templates, constant evaluation | blocked |
| P13 | Checked `noexcept` | error handling | yes | implicit calls, destructors, virtual functions | partial |
| P14 | Scope-exit blocks | statements, errors | no (unless the keyword is taken) | unwinding, jumps, destruction order | ready |
| P15 | Declared initialization order across translation units | program structure, linkage | no | static initialization, linking | partial |
| P16 | Member call syntax for free functions | lookup | no | lookup, argument-dependent lookup, overload resolution | blocked |

Totals: 4 ready, 7 partial, 5 blocked. Six proposals change existing
behavior (P2, P3, P4, P5, P7, P13).

### P1. Width-named integer literal suffixes

An integer literal can end in a suffix that names a signedness and an
exact width, such as `i8`, `u16`, `i32`, or `u64`. The literal gets the
integer type of that width, and a value that does not fit is rejected, as
PRIM-2 rejects a literal with no type today.

- **Cites:** PRIM-P1, PRIM-1 (the candidate list; `z` and `uz` are the
  precedent for a suffix that names a type), PRIM-2, PRIM-3, PRIM-16,
  PRIM-22 (the extended floating suffixes are the closest precedent),
  OPR-P3, OPR-8, OPR-9, OPR-16, CONV-1 (an 8-bit literal promotes in
  arithmetic), CONV-P2, the glossary's usual arithmetic conversions,
  UN-15.
- **Settled from Tidal:** suffixes without a leading underscore are
  reserved for the language (OPR-9), so a program that declares
  `operator""i32` already gets a warning, and the impact section can
  name exactly those programs as the ones that change. Promotion of the
  narrow types follows from CONV-1 with no new rule.
- **Not settled:**
  - D1 (missing, foundations/types.md): the page is a title and a
    summary. No rule lists the integer types, their widths, or their
    ranks, so the proposal cannot say which existing type `i64` names on
    each target, or whether it must be a new type.
  - D2 (contradictory, expressions/primary.md): PRIM-16 says a suffix
    must be one of the forms in the grammar, and the `integer_suffix`
    production lists only `u`, `l`, `ll`, and `z`. OPR-8 accepts any
    literal with a suffix that has a literal operator. A proposal that
    adds suffixes cannot tell which rule to amend.
  - D3 (missing, a lexical page): no rule says how the end of a numeric
    literal is found, so the proposal cannot show that `1u8` and the
    `u8` character prefix do not collide.

Rating: **partial**. Main blocker: the fundamental types page (D1).

### P2. Narrowing is an error in every implicit conversion

The narrowing conversions that LIST-2 forbids in braces become errors in
`=` initialization, assignment, argument passing, and `return`, for
scalar targets. An explicit cast is the way to ask for the old
conversion. Programs that rely on a silent truncation stop compiling.

- **Cites:** LIST-P1, LIST-2 (the definition of narrowing, reused
  whole), INIT-P1, INIT-4 ("a conversion that loses information is
  permitted", the rule this proposal reverses), INIT-5, ASN-P1, ASN-1,
  ASN-5, ASN-14, PFX-6, JMP-3, THR-1, CAST-P2, CAST-11, CE-19, SA-3,
  IF-16, ILC-4, CTOR-21, BF-7, BF-11, CMP-13, CONV-P2, CONV-P4, CONV-2,
  CONV-4, CONV-5.
- **Settled from Tidal:** the set of narrowing conversions and the
  constant-fits exception exist and are tested. Several places already
  ban narrowing (CE-19, CMP-13, ILC-4), and two already permit it on
  purpose (SA-3, IF-16), which gives the design a principled boundary.
  ILC-4 gives a precedent for the overload question: narrowing in braces
  makes the program invalid and never selects another constructor.
- **Not settled:**
  - D4 (missing, foundations/conversions.md): no list of the contexts
    where an implicit conversion happens. CVF-2 lists them for
    conversion functions only. The proposal must name every context it
    changes, and it cannot show the list is complete (default arguments,
    handler parameters, `?:` operands, compound assignment through
    ASN-5).
  - D5 (missing, a new overload resolution page): whether a narrowing
    argument makes a candidate not viable or makes the chosen call
    invalid. The two choices change which programs break.
    DEL-1, ACC-3, COND-11, OPR-6, CPY-9, CVF-4, and ILC-3 all point at
    overload resolution *(referenced)*.
  - D6 (ambiguous, assignment.md): ASN-5 converts the result of `op` to
    the type of `E1` in `E1 op= E2`. The proposal must decide whether
    `i += 0.5` narrows, and Tidal does not say whether that conversion is
    an implicit conversion in the sense of ASN-1.

Rating: **partial**. Main blocker: overload resolution (D5).

### P3. Left-to-right evaluation everywhere

Every operand pair and every argument list is evaluated from left to
right, and the unspecified orders go away. The exception is the existing
right-before-left order of assignment, which the proposal keeps. A
program whose output depends on argument order keeps its meaning on one
implementation and changes it on another.

- **Cites:** SEQ-P1, SEQ-P2, SEQ-1 to SEQ-7, ADD-P4, ADD-13, MUL-P4,
  MUL-13, MUL-14, SHF-P4, SHF-8, ASN-P3, ASN-4, ASN-11, COMMA-1, LOG-5,
  OPR-7, LIST-7, NEW-12, DEFARG-3, FN-10, VC-10, TR-P5, CE-P3, and the
  glossary entry for unsequenced.
- **Settled from Tidal:** every fixed order is stated with an ID, and
  the SEQ-1 Evidence says that Clang evaluates arguments left to right
  and GCC right to left. That fact is the core of the impact section:
  every program whose result depends on the order already differs
  between two matrix compilers. SEQ-4 says which programs are undefined
  today and so gain a meaning. TR-P5 names the optimization freedom the
  change removes.
- **Not settled:**
  - D7 (ambiguous, foundations/evaluation-order.md): ADD-P4 and MUL-P4
    cite SEQ-1 for the order of two operands, and SEQ-1 is about the
    arguments of a call. No rule says in general that the operands of a
    binary operator are unordered. The glossary says "most binary
    operators".
  - D8 (missing, comparison.md and bitwise.md): these pages have no rule
    on operand order, where additive.md and multiplicative.md have one.
    The proposal cannot list every unspecified order it removes.
  - D9 (missing, foundations/evaluation-order.md): the order of the
    placement arguments, the array bound, and the initializer of a `new`
    expression is given only in part (NEW-12).

Rating: **ready**. The gaps are inventory gaps, and the design can be
written now.

### P4. Chained comparisons with their mathematical meaning

`a < b < c` means `a < b && b < c`, with `b` evaluated once, for a chain
of relational operators that all point one way. Today the expression
compares a `bool` with `c`, so every such program changes meaning.

- **Cites:** CMP-P1, CMP-1, CMP-2, CMP-3, CMP-P6, CMP-14, the `cmp`
  grammar, Q-0029, LOG-P2, LOG-4, LOG-7, DCMP-6, DCMP-12, OPR-6, OPR-7,
  OPR-11, SEQ-P1, VC-P3.
- **Settled from Tidal:** CMP-1 states today's meaning, and CMP-1's own
  example is the case that changes. CMP-14 asks for a warning, and
  Q-0029 records that Clang 22 rejects a chain by default. That is the
  impact evidence: on one matrix compiler the changed programs do not
  compile today. LOG-4 and LOG-7 give the short-circuit and constant
  evaluation behavior the new form inherits.
- **Not settled:**
  - D10 (ambiguous, comparison.md): CMP-14 names `a < b < c`. It does not
    say whether `a == b == c` counts as a chain. Equality of two `bool`
    results is a plausible idiom, so the proposal must exclude equality,
    and Tidal gives no evidence either way.
  - D5 again: when `<` is an operator function that returns a class, the
    proposal must say whether the chain applies. Without overload
    resolution it cannot say which candidate a rewritten `<` (DCMP-6)
    produces.
  - D11 (missing, foundations/values.md): the middle operand is
    evaluated once and used twice. Tidal has no rule for a value that
    one expression uses twice (the closest is the single evaluation of
    `E1` in ASN-9), so the proposal must define a temporary and its
    lifetime itself.

Rating: **partial**. Main blocker: overload resolution for class
operands (D5).

### P5. Automatic objects start at zero

A scalar automatic variable, a scalar member left without a value by
default initialization, and an object from `new` without an initializer
get the value zero, as static storage already does (STO-3). An attribute
restores today's state for code that measures the cost. Reads that are
undefined today gain a meaning, and constant evaluation stops rejecting
them.

- **Cites:** INIT-P2, INIT-1, INIT-2, STO-1, STO-3, NEW-P1, NEW-3, OBJ-P1,
  OBJ-P4, OBJ-2, CTOR-P2, CTOR-4, CTOR-12, CV-3, CX-1, CE-21, CE-22,
  LIST-8, JMP-P2, JMP-6, SW-5, ATTR-P1, ATTR-P2, TR-P5, Q-0013, Q-0039,
  Q-0115.
- **Settled from Tidal:** INIT-1 and NEW-3 are the two `undefined` rules
  that become `required`, and STO-3 is the precedent. ATTR-P2 says an
  attribute does not change what a valid program does; an opt-out
  attribute fits, because every program that observes the difference has
  undefined behavior under the attribute. CE-21 and CE-22 show which
  constant evaluations become valid.
- **Not settled:**
  - D12 (missing, initialization/forms.md): "default-initialized" is a
    term in a Syntax note. No rule says what default initialization does
    to a class, its scalar members, or an array of classes. CTOR-12 says
    only that a read of such a member is undefined.
  - D13 (ambiguous, statements/jumps.md and storage-linkage.md): JMP-6
    lets a jump skip a scalar declaration without an initializer, and
    STO-1 says each pass through a declaration creates the object. Tidal
    does not say whether the object exists when the jump skips its
    declaration, so the proposal cannot say when the zero is written.
  - D14 (missing, constexpr.md): CX-1 lets a `constexpr` class object
    have no initializer when the default constructor gives every member a
    value. The proposal turns every scalar member into one with a value,
    and CX-1 does not say whether "a value" includes a zero that
    nothing wrote.

Rating: **partial**. Main blocker: no rule-level account of default
initialization (D12).

### P6. Named arguments

A call can name parameters, as in `f(1, .width = 3)`, and named
arguments follow the positional ones in any order. Parameter names
become part of a function's interface, so redeclarations must agree on
them.

- **Cites:** PFX-P2, PFX-5, PFX-6, DEFARG-P1, DEFARG-P2, DEFARG-1 to
  DEFARG-9, FN-1, FN-13, DCL-P3, DCL-8, LIST-5 and LIST-14 (designators
  are the syntax precedent), SEQ-1, SEQ-5, VIRT-1, DEFARG-5, OPR-2,
  CTOR-10, ILC-3, LAM-1, LSP-P2.
- **Settled from Tidal:** DCL-P3 says the function type holds parameter
  types and not names, so pointers and aliases carry no names (as
  DEFARG-5 says for defaults). LIST-14's rules for designators give a
  tested model for order and repetition.
- **Not settled:**
  - D15 (missing, functions/definitions.md): no rule says whether two
    declarations of one function may use different parameter names, or
    what an override (VIRT-1) does with names.
  - D5 again: the proposal must say how names filter candidates before
    viability. Overload resolution is *(referenced)*.
  - D16 (missing, grammar): `expression_list`, the argument list of a
    call, is used in postfix.md and never defined, so the proposal cannot
    show that `.name =` in an argument list is free syntax.

Rating: **blocked**. Main blocker: overload resolution (D5).

### P7. Temporaries live through reference-returning chains

When a reference variable binds to the result of a member function call
on a temporary, the temporary lives as long as the reference, as it does
today for a member access (REF-4). This turns some dangling references
into valid code. It also moves the destructor call of temporaries in
programs that are valid today, which is a behavior change.

- **Cites:** OBJ-P3, OBJ-8, OBJ-9, VC-P3, VC-6, VC-9, REF-P2, REF-4,
  REF-5, REF-7, Q-0040, Q-0109, RFOR-P1, RFOR-4, SB-6, ILC-5, CE-14,
  CTOR-17, LAM-19, COND-12, DTOR-5.
- **Settled from Tidal:** REF-4 lists every binding that extends a
  temporary, and REF-5 lists the ones that do not, with a returned
  reference among them. RFOR-4 extends all temporaries of the range
  expression, which is a precedent for extension by position instead of
  by binding. VC-6 gives the destruction order the extended temporaries
  must keep. The impact section can name the programs that change: those
  where the returned reference does not refer into the temporary and
  the temporary's destructor has an effect.
- **Not settled:**
  - D17 (missing, foundations/values.md): "full expression" has no
    definition with an ID. VC-P3 and VC-9 list some separate full
    expressions, SEQ-5 uses the term, and the glossary has no entry.
  - D18 (needs example, initialization/references.md): the boundary
    between REF-4 and REF-5 for a member access and a member call that
    returns a reference to the same member has no example. The proposal
    rests on that boundary.
  - D19 (ambiguous, foundations/constant-evaluation.md): CE-14 counts a
    temporary extended at namespace scope as static. The proposal adds a
    new way to extend, and CE-14 does not say whether the rule follows
    the extension or the binding form.

Rating: **ready**.

### P8. Relocation: a move that ends its source

A relocation expression moves a local object into a new object and ends
the source's lifetime without running its destructor. A class opts in
with a defaulted relocation function. A class that is trivially copyable
relocates by copying bytes. The scope exit then skips the source.

- **Cites:** OBJ-P1, OBJ-P2, OBJ-3, OBJ-10, CPY-P1, CPY-P2, CPY-P4,
  CPY-P6, CPY-5, CPY-7, CPY-11, CPY-19, DTOR-5, DTOR-6, DTOR-7, JMP-9,
  UNW-1, LAM-4, VC-P4, VC-7, DEL-P2, DEL-4, STO-1, CE-P1.
- **Settled from Tidal:** OBJ-P2's "reverse of the order in which they
  began" and DTOR-6 (an explicit destructor call ends an object and
  keeps storage) give the model. CPY-P2 shows how a new special member
  interacts with the implicit ones, and the proposal can follow its
  pattern. DEL-P2 limits `= default` to functions the implementation
  knows how to write, which the proposal extends.
- **Not settled:**
  - D20 (missing, a lexical page): no keyword list and no rule for
    contextual keywords beyond the note in virtual.md that `override`
    and `final` are identifiers with a special meaning. The proposal
    cannot show that a new keyword is free.
  - D21 (missing, copy-move.md and members.md): CPY-19 and CLS-19 end
    with "others that this page does not list yet", and "trivial" in
    JMP-6, SW-5, and UNI-5 has no definition. Byte-wise relocation rests
    on these terms.
  - D22 (missing, foundations/objects.md): no rule covers an object
    whose end depends on a path (relocated on one branch, not on
    another). OBJ-3 assumes every automatic object ends at its block.

Rating: **partial**. Main blocker: class properties (D21).

### P9. Overload sets as values

The name of an overloaded function, used where a value is needed and no
target type selects a function, becomes a closure object. Each call of
the closure runs overload resolution on the call's arguments. An
overload set can then go to a function that takes any callable.

- **Cites:** DTY-P1, DTY-4, AUTO-8, AUTO-11, MPTR-2, CONV-13, LAM-P1,
  LAM-2, LAM-10, LAM-12, LAM-15, NAM-P3, NAM-5, DEL-1, ACC-3.
- **Settled from Tidal:** AUTO-8 and DTY-4 reject an overloaded name as
  an initializer of `auto` and as an operand of `decltype`, so these
  positions are free for a new meaning, and no valid program changes.
  MPTR-2 states the rule that a target type selects a function.
- **Not settled:**
  - D5 again, and the address of an overload set *(referenced)*.
  - D23 (missing, foundations/names.md): the proposal must say whether
    argument-dependent lookup (NAM-5) runs where the closure is made or
    where it is called. NAM-5 does not define the associated
    namespaces for class arguments with bases.
  - LAM-15 marks templates *(referenced)*, and a closure with a call
    operator for any arguments is a template.

Rating: **blocked**. Main blockers: overload resolution and templates.

### P10. Expansion over the elements of an object

A `for` statement marked for translation-time expansion repeats its body
once per element of an array, an aggregate, or a tuple-protocol object,
with each element's own type. `break` and `continue` keep their meaning.

- **Cites:** SB-P2, SB-2, SB-3, SB-5, SB-12, SB-13, SB-14, RFOR-P1,
  RFOR-1 to RFOR-7, IF-P3, IF-13, IF-14, IF-17, IF-22, IF-23, LOOP-P2,
  JMP-7, CE-P4, CE-11, Q-0048.
- **Settled from Tidal:** SB-P2's fixed order of tests says what the
  elements are. RFOR-P1 says what a range loop means, and `if
  constexpr` shows a statement keyword followed by `constexpr`.
- **Not settled:**
  - D24 (missing, structured-bindings.md and a language-support page):
    the tuple protocol and the lookup of `get` are *(referenced)*
    (SB-5, Q-0048).
  - IF-13 and IF-22 describe discarded statements in a template. A body
    whose variable has a different type in each copy is an instantiation,
    and N2 has no page.

Rating: **blocked**. Main blocker: templates and instantiation.

### P11. Exceptions that stay inside a constant evaluation

A `throw` reached during constant evaluation is allowed when a handler
in the same evaluation catches it. Only an exception that leaves the
evaluation makes it not constant. A `constexpr` function can then report
errors the same way in both modes.

- **Cites:** CE-P1, CE-P2, CE-P3, CE-1, CE-7, CE-8, CE-14, CE-22, TR-P4,
  TR-5, THR-P1, THR-P2, THR-P5, THR-1 to THR-10, TRY-P1, TRY-1, TRY-2,
  TRY-8, UNW-1, UNW-5, EXS-4, UN-18, CEV-1, IF-18.
- **Settled from Tidal:** CE-8 and THR-10 are the rules that change, and
  CE-1's "try blocks, if no exception is thrown" shows the boundary.
  CE-P3 gives today's reason ("an exception has no handler at
  translation time"), which the proposal answers directly, and CE-P1
  ("the same program run at translation time") supports it. TRY-2
  matches handlers by type relations Tidal already defines, so no
  run-time type library is needed.
- **Not settled:**
  - D25 (missing, exceptions/throw.md): THR-P2 says the exception object
    belongs to no scope. No rule says where its storage comes from, so
    the proposal cannot apply CE-7's rule that storage made in an
    evaluation is freed in it.
  - D26 (ambiguous, constant-evaluation.md and throw.md): CE-8 and
    THR-10 state the same rule on two pages. A proposal must amend both
    and cannot tell which one owns the behavior. THR-11 and UNW-8 repeat
    each other the same way, with a difference in what they exclude.
  - D27 (missing, constant-evaluation.md): UNW-5 lists the ways a
    program terminates. No rule says what constant evaluation does when
    it reaches one (for example an exception that leaves a `noexcept`
    function inside the evaluation).

Rating: **ready**.

### P12. Constant parameters

A parameter declared `constexpr` takes only constant arguments, and the
body can use it in constant expressions. Each distinct argument value
gives a distinct function. Overloads can differ in whether a parameter
is constant, so a call with a constant argument can choose another body.

- **Cites:** CX-5 (rejects a `constexpr` parameter today, so the syntax
  is free), CE-3, CE-P4, CE-11, CE-19, CEV-1, CEV-2, IF-16, IF-19, DCL-P3,
  DCL-8, EXS-1, PFX-6, CONV-13, LAM-1.
- **Settled from Tidal:** CX-5 makes the syntax free. CE-3 says a
  parameter is usable only during an evaluation that called the
  function, which is the rule the proposal extends.
- **Not settled:**
  - D28 (missing, glossary and constant-evaluation.md): "usable in
    constant expressions" is not a defined term. CE-3 and CV-8 define it
    for variables in prose.
  - CE-19 marks non-type template arguments *(templates referenced)*.
    The identity of each specialized function, and the one-definition
    rule across units, need templates and item I.
  - D5 again, for the overloads on constant parameters.

Rating: **blocked**. Main blockers: templates and overload resolution.

### P13. Checked `noexcept`

A function declared `noexcept` must not contain a `throw` or a call to a
potentially throwing function outside a `try` block whose last handler
catches everything. The implementation rejects a violation. A separate
spelling keeps today's run-time termination for code that cannot be
checked.

- **Cites:** EXS-P1, EXS-P2, EXS-P3, EXS-1 to EXS-4, THR-P4, THR-9,
  THR-11, UNW-P3, UNW-5, UNW-6, UNW-8, UN-18, DTOR-4, CTOR-P2, CONV-10,
  LAM-10, TRY-3, CPY-P2, DCMP-1, NEW-P3, TR-P1.
- **Settled from Tidal:** UN-18 lists what makes an expression
  potentially throwing, and the check can reuse it. DTOR-4 makes most
  destructors `noexcept`, so the impact section can name the class of
  programs that change: a destructor that calls a function without
  `noexcept`. THR-11 and UNW-8 show that implementations already warn
  about the direct case.
- **Not settled:**
  - D29 (missing, unary.md or exception-specifications.md): UN-18 names
    implicit calls by example ("such as the constructor of a temporary or
    a conversion function"). Tidal has no complete list of the calls an
    expression makes without naming them (destructors of temporaries,
    rewritten comparisons, allocation and deallocation functions). The
    check must cover them all.
  - The default allocation function throws, and its exception type is
    *(library referenced)*.
  - Conditional `noexcept` in templates needs the template chapter.

Rating: **partial**. Main blocker: the inventory of implicit calls
(D29).

### P14. Scope-exit blocks

A `defer` compound statement registers a block that runs when control
leaves the enclosing block by any route. Deferred blocks run in the
reverse order of registration, interleaved with the destruction of the
block's automatic objects.

- **Cites:** OBJ-P2, OBJ-3, STO-1, JMP-P2, JMP-4, JMP-6, JMP-9, UNW-P1,
  UNW-1, UNW-5, DTOR-4, TRY-15, LOOP-3, SW-5, IF-17, CE-P1, STMT-P1.
- **Settled from Tidal:** OBJ-3 ("by any route", "each iteration of a
  loop") and JMP-4 ("first evaluates the returned value, then destroys")
  fix where a deferred block runs. UNW-5 gives the answer for a deferred
  block that throws during unwinding: treat it as a destructor. JMP-6
  gives the rule for a jump forward past a registration.
- **Not settled:**
  - D20 again: whether `defer` is a free identifier, and how many
    programs use it as a name.
  - D16 again: the `statement` nonterminal has no single definition, so
    the proposal cannot show where the new statement fits in the grammar.
  - A jump out of a deferred block (`return`, `break`) has no analogue in
    a destructor. The proposal must forbid it, and Tidal has no rule for
    a region that a jump must not leave other than TRY-17 and JMP-12.

Rating: **ready**.

### P15. Declared initialization order across translation units

A namespace-scope variable with dynamic initialization can name the
variables its initializer reads. The implementation initializes those
first, across translation units, and rejects a cycle when the program is
linked.

- **Cites:** SINIT-P1, SINIT-P2, SINIT-1 to SINIT-6, STO-3, STO-10, OBJ-4,
  OBJ-P2, LNK-1, LNK-4, LNK-6, INL-P1, TR-P1, TR-P2, TR-4, CINIT-1, CE-24,
  UNW-4, Q-0014.
- **Settled from Tidal:** SINIT-3 is the `unspecified` rule that the
  proposal narrows, and SINIT-2 says how `inline` variables are ordered
  today. STO-10 makes destruction follow automatically.
- **Not settled:**
  - D30 (missing, foundations/translation.md): TR-P1 defines what a
    translation diagnostic is. Linking appears only as "fails to link"
    (TR-4). No rule class covers a diagnostic at link time, and the
    proposal needs one for a cycle.
  - The one-definition rule and a multi-file program model are roadmap
    items I and J. The proposal needs both to say that every unit sees
    the same dependency list.

Rating: **partial**. Main blocker: the translation and linking model
(D30).

### P16. Member call syntax for free functions

`x.f(args)`, where the class of `x` has no member `f`, calls `f(x,
args)` found by ordinary and argument-dependent lookup. A member always
wins, so a class keeps control of its own interface.

- **Cites:** PFX-P3, PFX-10, PFX-11, NAM-P2, NAM-P3, NAM-5, DER-P2,
  DER-3, DER-4, ACC-3, ACC-7, ACC-17, CLS-13, CLS-17, OPR-6, RFOR-2.
- **Settled from Tidal:** PFX-11 rejects a member access that names no
  member, so the space is free and no valid program changes meaning.
  RFOR-2 already falls back from members to free `begin` and `end`,
  which is a precedent for the design and for its edge cases (Q-0098).
  CLS-13 and CLS-17 show that a member with an explicit object
  parameter is close to a free function.
- **Not settled:**
  - D23 again: the associated namespaces of argument-dependent lookup.
  - D5 again: overload resolution between the fallback candidates.
  - The fallback must decide what "no member" means when DER-4 makes a
    lookup ambiguous. Tidal says an ambiguous name is valid when unused,
    and a fallback would use it.

Rating: **blocked**. Main blockers: lookup (item H) and overload
resolution.

## 2. Cross-cutting needs

These needs come from the proposals as a whole.

- **A notation for a change to a rule.** A proposal changes rules in a
  few ways: new, amended, withdrawn, split, or reclassified. Several
  proposals reclassify: P3 turns SEQ-1 from `unspecified` into
  `required`, P5 turns INIT-1 and NEW-3 from `undefined` into
  `required`, P13 adds a `reject` rule beside THR-9. Tidal has the class
  `withdrawn` and the rule "IDs are never reused" (docs/spec-format.md),
  and nothing for the other kinds. Propose exercises need a table form:
  rule ID, change kind, old class, new class, tests affected.
- **Principles as rationale.** Principle IDs are the strongest tool
  Tidal gives a designer. P11 argues against CE-P3's stated reason, P5
  against INIT-P2, P3 against SEQ-P2. A proposal can cite the principle
  it changes and the principle that supports it. The format should say
  how a principle is amended, since rules point to principles through
  `:rules:` lists.
- **Item-level anchors.** Many rules are lists (SEQ-3, LIST-2, CE-1,
  UN-18). A proposal that changes one item must cite the whole rule. An
  anchor per list item (for example LIST-2, item 1) would let a proposal
  and its similarity-checked summary point at one condition.
- **An index of free space.** Several proposals rest on a `reject` rule
  that makes a construct invalid today: AUTO-8 and DTY-4 for P9, CX-5 for
  P12, PFX-11 for P16, OPR-9 (a warning) for P1. A designer finds these
  only by reading every page. An index of `reject` rules by syntactic
  position, generated from the rule classes, would show the space a new
  feature can take without changing any valid program. It needs item C
  of the roadmap first, so that every prohibition sits in a `reject`
  rule.
- **Keywords and tokens.** Three proposals (P8, P14, P1) stop at the
  question of whether a word or a suffix is free (D3, D20).
- **Version marking.** Q-0007 keeps Tidal at one version. DEL-3 and
  SB-15 mention a later version in passing. A designer cannot tell
  space that is free from space that a later version has taken, and an
  invented feature can collide with one. Rules need a version field, or
  a list of constructs rejected now because a later version gives them
  a meaning.
- **Cross-page inventories.** A proposal's interaction section lists
  every place a mechanism occurs. Tidal states these mechanisms page by
  page with no index: the contexts of implicit conversion (D4), the
  full expressions (D17), the unordered operand pairs (D7, D8), the
  implicit calls of an expression (D29), and the immediate function
  contexts (CEV-1, IF-19). The usual arithmetic conversions are cited on
  five pages as CONV-3, which is the conversion to `bool`; they exist
  only as a glossary entry with no rule ID (D31 below).
- **Impact evidence.** Tidal has no corpus of real programs, so an impact
  section can only reason. Two things in the repository help. Evidence
  segments and overrides show where matrix compilers already disagree
  (SEQ-1, Q-0029), and programs that depend on such a point are already
  not portable. The front end in `frontend/` could run the 3000-test
  suite with one rule changed and list the tests whose outcome moves.
  That is a cheap impact probe for P2, P3, P4, P5, and P13.
- **Implementation cost.** Tidal describes no translation phases and no
  cost model, so a cost section is outside the spec (D32). The front end
  is the nearest stand-in.

D31 (ambiguous, foundations/conversions.md): ADD-P1, BIT-P1, CMP-P1,
MUL-P1, and COND-P2 cite CONV-3 for the usual arithmetic conversions.
CONV-3 is the conversion to `bool`. The steps of the usual arithmetic
conversions are in the glossary only, so a proposal cannot cite one
step by ID.

D32 (outside the spec): implementation cost. Tidal has no model of what
an implementation knows at each point of translation.

D33 (outside knowledge): the impact sections of P2, P3, P4, and P5 lean
on a general sense that real code seldom relies on silent truncation,
argument order, chained comparisons, or reads of uninitialized
variables. Tidal gives the compiler-disagreement evidence above and
nothing on frequency. The reader also used general experience for the
claim in P5 that zeroing automatic storage has a measurable cost, which
is why P5 includes an opt-out.

## 3. Ranked topics

Ranked by how many proposals each topic moves forward, weighed against
cost. "Unblocks" means the proposal's rating rises.

1. **Cross-page inventories on the Foundations pages** (new item; small,
   low similarity risk because the content is Tidal's own). Give rule IDs
   to the usual arithmetic conversions and integer promotion (D31); add a
   rule that lists the contexts of implicit conversion (D4), one that
   defines a full expression (D17), and one that lists the unordered
   operand pairs (D7, D8). Unblocks P2, P3, P7, and P13, and P1 in part.
   The roadmap's lane V thickens evaluation-order.md; this goes further.
2. **Fundamental types (item E).** Unblocks P1, and gives P2 and P5 the
   types they name. Agree with the roadmap.
3. **Implicit conversion sequences and overload resolution (items L
   and M).** Unblocks P6, P9, P12, P16, and moves P2 and P4 to ready.
   Six proposals depend on it, more than on any other topic, and seven
   rule pages mark it *(referenced)*. Agree with the roadmap's ranking
   and its method controls.
4. **Templates, declarations and instantiation (items N1 and N2).**
   Unblocks P9, P10, and P12, and lets P13 cover conditional `noexcept`.
   A designer needs the instantiation model (N2) as much as deduction
   (O), because an expansion statement and a constant parameter are
   instantiations without deduction. The roadmap orders O before N2; the
   designer view would put N2 first.
5. **A narrow lexical slice (part of item G).** Keywords and contextual
   keywords, reserved identifiers, the end of a numeric literal, and
   the literal suffix grammar (D2, D3, D20). Unblocks P1, P8, and P14.
   The roadmap defers G for similarity risk. The designer view disagrees
   for this slice: every proposal that adds a word needs it, and D2 is a
   contradiction in the current text.
6. **The translation and linking model with the one-definition rule
   (items I and J).** Unblocks P15 and part of P12. Agree, with one
   addition: a rule class or a stated policy for diagnostics at link
   time (D30).
7. **Class properties: trivial, trivially copyable, standard-layout
   (item S).** Unblocks P8 and settles a term P5 and P14 touch through
   JMP-6. The roadmap defers S until an observation channel is decided.
   The designer view disagrees in part: "trivial" is already used
   undefined in JMP-6, SW-5, and UNI-5, and a definition does not need
   the library.
8. **Language-support types (item K).** The tuple protocol (P10), the
   exception types of allocation (P13), and `std::initializer_list`.
   Agree.
9. **Version marking and the change notation** (process, from Q-0007).
   Needed by every Propose exercise. Not in the roadmap.
10. **The index of `undefined` rules (part of item D).** P3 and P5
    define behavior that is undefined today and need the full list. Agree.

Where this view agrees with docs/roadmap.md: the repair phase first,
types early, lookup before overload resolution, and overload resolution
as the topic of highest value. Where it differs: the cross-page
inventories (item 1) are new, the lexical slice and class properties
should not wait, and N2 should come before O.

## 4. Propose exercises

Four proposals, chosen so that their design and impact sections reach
the most pages between them. All four are ready or partial, so the
exercise produces design text and not only a list of blockers. The
blocked proposals would each end at overload resolution.

| Exercise | Proposal | Pages it reaches |
|---|---|---|
| X1 | P3, left-to-right evaluation | evaluation-order, additive, multiplicative, shift, comparison, bitwise, logical, assignment, comma, operators, postfix, member-pointers, new-delete, initialization/list, default-arguments, functions/definitions, values, translation (18) |
| X2 | P13, checked `noexcept` | exception-specifications, throw, handlers, unwinding, constructors, copy-move, comparisons, virtual, lambdas, unary, conversions, declarators, new-delete (13; 12 new) |
| X3 | P5, automatic objects start at zero | initialization/forms, initialization/static, objects, storage-linkage, constructors, constexpr, constant-evaluation, jumps, switch, attributes, unions, members, new-delete, initialization/list (14; 11 new) |
| X4 | P7, temporaries through reference-returning chains | initialization/references, range-for, structured-bindings, initializer-lists, conditional, casts, objects, values, constant-evaluation, lambdas, constructors (11; 6 new) |

Together they reach 47 of the 68 spec pages and every chapter.
X1 and X3 each test a reclassification (`unspecified` and `undefined`
to `required`), so they test the change notation of section 2. X2 adds a
`reject` rule over existing run-time behavior. X4 changes the timing of
a destructor in valid programs, the hardest impact case.

Pages none of the four reaches: names, types, primary, decltype,
alignment, volatile, linkage-specifications, namespaces, inline,
type-names, enumerations, static-assert, deleted, return-deduction,
auto, access, derived, conversion-functions, blocks, if, loops. A fifth exercise on P16 (lookup) or P1 (literals) would cover
names, primary, derived, and access, and would end at a blocker.
