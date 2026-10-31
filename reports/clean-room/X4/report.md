# X4: P7, temporaries live through reference-returning chains

Date: 2026-10-31. Decision 0037. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone.

Inputs read:
- AGENTS.md and decision 0037.
- The Propose section of docs/exercises.md.
- P7 and section 2 of reports/clean-room/roadmap/designer.md.
- These spec pages, in full or in the cited rules: initialization/references, objects, values, range-for, structured-bindings, initializer-lists, constant-evaluation, constexpr, constructors, lambdas, conditional, casts, postfix, members, conversion-functions, operators, unwinding, jumps, evaluation-order, and the glossary.
- Questions Q-0020, Q-0034, Q-0040, Q-0084 and Q-0109.
- The tests named below.

Compiler runs: small programs built with clang++ and g++ (`-std=c++23 -pedantic-errors`) under /tmp/claude-1000/x4, to see what programs do today. The results are quoted where they support a claim.

Totals: 17 rule changes (1 new, 16 amended). No existing test changes its expected outcome. 16 findings, X4-1 to X4-16.

## 1. Design

### 1.1 The feature

Today a reference that binds to a member of a temporary keeps the temporary alive (REF-4). A reference that binds to the result of a member function call on a temporary does not (REF-5, "a binding to a returned reference"). The two forms often name the same object:

```cpp
struct Box { int v; const int& get() const { return v; } };
const int& a = Box{1}.v;       // Box lives as long as a
const int& b = Box{1}.get();   // Box ends at the semicolon; b dangles
```

P7 makes the second line behave like the first.

### 1.2 The rule, stated as precisely as Tidal allows

REF-4 walks from the initializer of a reference through a fixed set of forms:
- parentheses,
- the selected operand of `?:`,
- the right operand of a comma,
- a plain reference cast,
- member access,
- a subscript of an array member.

Each place the walk reaches is an *extending position*. When the walk reaches a temporary, that temporary lives as long as the reference.

P7 adds one form to the walk. If the expression in an extending position is a call to a non-static member function, and the declared return type of that function is a reference to an object type, then the object expression of the call is also an extending position.

Consequences of that one addition:
- The walk repeats, so a chain of reference-returning calls reaches the temporary at its start.
- A class prvalue in the object position becomes a temporary (VC-8). If the walk reaches it, that temporary is extended.
- The walk stops at a call that returns by value. That call's result is a new temporary, which REF-4 already extends. Temporaries before it in the chain end with the full expression, as today.
- Arguments of the call are never extending positions. A function parameter still does not extend (REF-5).
- At most one temporary is extended per reference. The exception is a braced aggregate with several reference members, where each member has its own walk, as today.

What counts as a member call:
- a named call, `x.f()`, or `p->f()` where the object is `*p`,
- an overloaded operator that is a member function (OPR-P1), such as `operator[]`, unary `operator*`, or `operator=`,
- a conversion function to a reference type (CVF-1), called by name, by a cast, or implicitly by the binding itself,
- a function with an explicit object parameter of reference type (CLS-13). The object binds to that parameter, but the call has the member form, so it counts.

What does not count:
- a static member function, even when called through an object, because the call has no object binding,
- a free function or a non-member operator, even when it takes the temporary by reference,
- a member function that returns a pointer, and the built-in `*` and `->` applied to that pointer,
- a function with a by-value explicit object parameter, because no temporary is bound (see X4-10),
- a function that returns a reference to a function type.

The declared return type decides. So the rule depends only on the form of the initializer and needs no knowledge of the function body. A virtual call uses the declared type of the function named in the call.

### 1.3 Which bindings get the new walk

The same bindings that REF-4 extends today get the new walk:
- a reference variable at any scope, including one in a condition or a `for` init statement,
- the hidden reference of a structured binding (SB-6),
- a reference member of an aggregate initialized with braces in a variable definition,
- an init capture by reference (`[&r = e]`), which compilers already treat as an extending binding (X4-6).

These bindings still do not extend:
- a function parameter,
- a `return` statement,
- a reference member of an aggregate initialized with parentheses (Q-0040),
- a member initializer or a default member initializer (CTOR-17).

A range-based `for` already extends every temporary of its range expression (RFOR-4). P7 changes nothing there.

### 1.4 When the extended temporary ends

The rule of REF-4 applies unchanged. The temporary ends when the reference ends, in the reference's place in the order of its scope (DTOR-5). For a reference with static storage, the temporary ends at program end, in reverse order of construction with the other static objects (OBJ-P2). The same rule applies in a constant evaluation (CE-P1).

### 1.5 More examples

A chain:
```cpp
struct B { int n = 0; B& self() { return *this; } };
B& r = B{}.self().self();       // the B temporary lives as long as r
```

A chain that breaks at a by-value call:
```cpp
struct A { B make() const { return {}; } };
B& s = A{}.make().self();       // the B lives as long as s; A ends here
```

An argument stays short-lived:
```cpp
struct P { const int& pick(const int& x) const { return x; } };
const int& t = P{}.pick(2);     // P lives as long as t; the 2 does not
```

A conversion function:
```cpp
struct C { int v; operator const int&() const { return v; } };
const int& u = C{3};            // C lives as long as u
```

A closure that returns a reference to its own capture:
```cpp
const int& w = [x = 1]() -> const int& { return x; }();
// the closure lives as long as w
```

### 1.6 Alternatives considered

- **Extend every temporary of the initializer**, as RFOR-4 does for a range expression. This is simpler to state, but it moves the end of every argument temporary in every reference initializer, so the impact is larger.
- **Also extend through the first reference parameter of a free function.** This would cover non-member operators and free accessors. It changes `tests/declarations/initialization/references/lifetime-not-through-return.cpp` from reject to accept, and it makes a parameter binding extend, which REF-P2 rules out by principle. Rejected.
- **Opt in per function** with a mark on the member function. No valid program changes, but no dangling program is fixed without an edit. This is the fallback if the impact in section 4 is judged too large.
- **Reference members of a temporary** (`T{}.r`, where `r` is a reference member). Compilers do not extend these today (X4-2). P7 leaves them alone, because a reference member refers to an object outside the temporary.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| REF-12 | new | none | required | The object expression of a reference-returning non-static member call in an extending position is itself an extending position, with the inclusions and exclusions of 1.2. | none |
| REF-4 | amended | required | required | Names REF-12 as one more form of the walk. The cast item admits a cast that calls a conversion function returning a reference. States that a reference member of a temporary is not a member for this rule. | none |
| REF-5 | amended | undefined | undefined | The "returned reference" item covers only calls that REF-12 does not reach: free functions, static members, and calls whose object is not a temporary. Programs that read through a reference covered by REF-12 leave this rule. | none |
| REF-P2 | amended | principle | principle | The extension also reaches the object of a member call that returns a reference. It still does not reach a parameter or a `return`. | none |
| OBJ-P3 | amended | principle | principle | "The one exception" also covers a temporary reached through a reference-returning member call. | none |
| OBJ-P2 | amended | principle | principle | The bullet "a temporary ends with the full expression that made it" names the extension as its exception. | none |
| OBJ-8 | amended | required | required | Adds the member-call form beside "a member of a temporary". | none |
| OBJ-9 | amended | required | required | "And no longer" gains the exception for extended temporaries. | none |
| VC-P3 | amended | principle | principle | "A reference can keep a temporary alive longer" cites REF-12 with REF-4. | none |
| VC-6 | amended | required | required | Lists REF-12 among the extensions. The reverse-order sentence covers the temporaries that end with the full expression. An extended one leaves that order and takes its reference's place. | none |
| SB-P1 | amended | principle | principle | The hidden variable keeps alive a temporary that its initializer reaches through REF-4 or REF-12, as well as one it binds to directly. | none |
| SB-6 | amended | required | required | Cites REF-12 with REF-4. | none |
| CE-14 | amended | reject | reject | A temporary extended at namespace scope counts as static, whichever form extended it. | none |
| CX-1 | amended | reject | reject | The ban on a block-scope `constexpr` reference to a temporary concerns the object the reference refers to. A temporary that the initializer extends but the reference does not refer to is allowed. | none |
| LAM-7 | amended | required | required | States that an init capture by reference that binds to a temporary extends it to the life of the closure object, and that REF-12 applies to it. | none |
| CTOR-17 | amended | no-diagnostic | no-diagnostic | States that REF-12 does not reach a member initializer or a default member initializer, so `: r(T{}.get())` still dangles. | none |
| UNW-1 | amended | required | required | The automatic objects of a block include the temporaries extended by its references, and unwinding destroys them in their place in the order. | none |

Counts: 17 changes. 1 new, 16 amended, 0 reclassified, 0 withdrawn, 0 split. No existing test changes its outcome.

A scan of all 3239 tests looked for reference declarations whose initializer contains a call followed by `.`, `[`, or `->`. Outside a range-based `for`, no test binds a reference through a reference-returning member call on a temporary. The tests nearest the boundary keep their outcome:
- `tests/statements/range-for/temporary-in-range-expression.cpp`: RFOR-4 already extends.
- `tests/declarations/initialization/references/lifetime-not-through-return.cpp`: a free function, so still reject.
- `tests/expressions/value-categories/member-reference-of-prvalue.cpp`: a reference member, which is out of scope, and the destructor is trivial.
- `tests/classes/conversion-functions/to-reference.cpp`: a named object.
- `tests/declarations/initialization/references/extend-through-array-member-subscript.cpp` and `lifetime-member-extended.cpp`: REF-4 forms, unchanged.

New tests the proposal needs. These are suggested paths under `tests/declarations/initialization/references/`:
- `extend-through-member-call.cpp` (run; the destructor runs after the scope's last statement),
- `extend-through-call-chain.cpp` (run; two reference-returning calls),
- `chain-breaks-at-value-call.cpp` (run; the first temporary ends at the semicolon and the second lives),
- `member-call-argument-not-extended.cpp` (reject in constant evaluation),
- `extend-through-conversion-function.cpp` (run),
- `extend-through-member-operator.cpp` (run; `operator[]`),
- `extend-through-explicit-object-reference.cpp` (run),
- `static-member-call-not-extended.cpp` (run),
- `member-call-constant-evaluation.cpp` (accept; a read after the declaration),
- `member-call-namespace-constexpr.cpp` (accept; CE-14).

Three more go elsewhere:
- `tests/declarations/structured-bindings/lifetime-member-call.cpp`,
- `tests/expressions/lambdas/init-capture-member-call.cpp`,
- `tests/exceptions/unwinding/extended-temporary-destroyed.cpp`.

## 3. Interactions

**Reference binding forms (REF-4, REF-5, REF-P2, Q-0040, Q-0109).** REF-12 adds to the walk and changes no existing form. The proposal must say that the walk goes through every REF-4 form in any order, so `(c ? X{} : X{}).get()` and `static_cast<const X&>(X{}).get()` extend. Paren-initialized aggregates stay outside (Q-0040). Compilers disagree on the array-member subscript (Q-0109), and REF-12 does not depend on it.

**Value categories and materialization (VC-3, VC-4, VC-8, PFX-9).** A member call on a class prvalue makes the temporary (VC-8). The call's category follows the return type (PFX-9). REF-12 uses the return type itself. It excludes functions that return by value, whose prvalue VC-4 already handles.

**Full expressions and destruction order (VC-P3, VC-6, VC-9, SEQ-5, DTOR-5, OBJ-P2).**
- The object expression is evaluated before the arguments (tested by `tests/expressions/sequencing/object-before-member-call-arguments.cpp`). So the extended temporary is the oldest temporary of its call, and destroying the arguments first keeps the reverse order.
- The proposal must say where the extended temporary sits among the block's objects: in its reference's place (X4-7, X4-8).
- With several declarators in one declaration, each is its own full expression (VC-9). So the extended temporaries end in reverse declarator order.

**Objects and lifetime (OBJ-P3, OBJ-8, OBJ-9, OBJ-2, OBJ-10).** Reads through a reference covered by REF-12 move from OBJ-2 (undefined) to defined. A destructor that runs later can now reach objects that ended first (section 4.3), which moves some programs into OBJ-2.

**Range-based `for` (RFOR-P1, RFOR-4).** RFOR-4 is wider than REF-12, so the loop is unchanged. The proposal should note that RFOR-4 stays a separate rule. Tidal does not describe the loop through a hidden reference variable, so RFOR-4 cannot be derived from REF-12 (X4-16).

**Structured bindings (SB-P1, SB-1, SB-6, SB-5).** The hidden variable gets the walk, so `auto& [a, b] = T{}.pair();` extends the `T`. Today both compilers end it at the semicolon. Calls to `get` in the tuple protocol act on the hidden variable, which is named, so REF-12 does not apply to them.

**Initializer lists (ILC-P3, ILC-5).** The backing array lives as long as its `std::initializer_list`. If that list is a member of a temporary that REF-12 extends, the array lives as long as the temporary. ILC-5 needs no change. A member that returns a pointer, such as `begin()`, is outside REF-12.

**Conditional operator (COND-P2, COND-P3, COND-3, COND-12).** Only the selected operand is in an extending position. With two class prvalues, the result initializes one temporary directly (COND-12), and that temporary is the object of the call. With an lvalue result (COND-3), the walk reaches a named object and nothing is extended.

**Casts (CAST-P1, CAST-1, REF-4 cast item).** A reference cast that calls a conversion function is a member call under REF-12. The REF-4 cast item excludes user-defined conversions, so it must point to REF-12 for that case.

**Conversion functions (CVF-P1, CVF-1).** A conversion function to a reference type is a non-static member function, so an implicit binding through it extends the class temporary. Today it does not: both compilers end `C{3}` at the semicolon (X4-15).

**Overloaded operators (OPR-P1, OPR-2).** A member operator counts and a non-member does not.
- `=`, `[]`, `()` and `->` must be members (OPR-2). So `(T{} = x)` and `T{}[i]` extend when they return references.
- For `+=` and unary `*`, the extension depends on whether the class declares the operator as a member.
- The proposal must state this asymmetry and accept it as the price of excluding parameters.

**Member functions (CLS-6, CLS-13).**
- Ref-qualified accessors (`&&`-qualified, returning a member as an xvalue) are the main case REF-12 fixes.
- An explicit object parameter of reference type counts as the object.
- A by-value explicit object parameter has no temporary to extend (X4-10).
- Static members are excluded, although PFX-P3 evaluates their object expression.

**Lambdas (LAM-7, LAM-19, LAM-P1).**
- A call to a closure prvalue's `operator()` that returns a reference is a member call, so the closure is extended.
- An init capture by reference is an extending binding today (X4-6). So REF-12 applies to it and extends the temporary to the closure's life.
- LAM-19 (a by-reference capture that outlives its variable) is unchanged, because the captured variable is not a temporary.

**Constructors and destructors (CTOR-P6, CTOR-17, DTOR-5, Q-0084).** Member initializers stay outside the walk, so CTOR-17 still applies and its class stays `no-diagnostic`. Clang rejects a direct temporary in a member initializer (Q-0084). The proposal does not ask it to reject the member-call form.

**Constant evaluation (CE-P1, CE-P2, CE-14, CE-23, CX-1, CX-2).**
- The same rule applies in a constant evaluation.
- CE-14 must count a namespace-scope temporary as static, whatever form extended it (X4-4).
- CX-1 must say that a block-scope `constexpr` reference may extend a temporary it does not refer to (X4-5). Otherwise a valid program becomes invalid (section 4.4).
- The extended temporary needs a literal type, as every object of an evaluation does (CE-23).

**Exceptions (UNW-P1, UNW-1, UNW-5, UNW-6, UNW-7).** An extended temporary is destroyed when its block is left by a throw. A destructor declared `noexcept(false)` that throws now throws at scope exit, and during unwinding that ends the program (UNW-5). The proposal must say that extended temporaries are among a block's automatic objects (X4-7).

**Jumps (JMP-6).** A reference always has an initializer, so a jump past one is already rejected. Leaving a block by `break`, `continue`, `return` or `goto` destroys the extended temporary with the block's other objects. No change.

**Templates and `auto` (the forwarding reference, decltype).** In generic code, `auto&& x = obj().get();` extends or not depending on the return type of the instantiated function. This is already true for REF-4. The operands of `decltype`, `sizeof` and `noexcept` are unevaluated and make no temporaries.

**Not touched.** Name lookup, overload resolution and access are unchanged. REF-12 applies after the call is resolved and uses only the chosen function's declared return type.

## 4. Impact on existing programs

No program in the test suite changes outcome (section 2). Tidal has no corpus of real programs, so the claims below come from reasoning about the rule and from compiler runs.

### 4.1 Undefined becomes defined

A program that reads through a reference bound by a reference-returning member call to a member of a temporary is undefined today (REF-5, OBJ-2). Under P7 it is defined.

- **Run time.** Today both compilers end the temporary at the semicolon. This held for an accessor, a call that returns `*this`, a conversion function and an `&&` accessor; in each run the destructor ran before the next statement. GCC warns "possibly dangling reference to a temporary" for three of the four, and Clang gives no warning.
- **Constant evaluation.** Today both compilers reject a read through such a reference in a `constexpr` function ("accessing ... outside its lifetime"). Under P7 it is accepted. This moves programs from the CE-9 rejection list into valid code.

### 4.2 Rejected becomes valid

- A constant evaluation that reads through such a reference (4.1).
- A namespace-scope `constexpr` reference to a member of a temporary, reached through a member call. Both compilers reject it today. Under P7 with the CE-14 change, the temporary counts as static and the program is valid.
- A block-scope `constexpr` reference to a member of the temporary stays rejected (CX-1), because the temporary is automatic.

### 4.3 Valid programs that change meaning

These programs bind a reference through a reference-returning member call on a temporary whose destructor has an effect. The destructor now runs at the end of the reference's scope.

- **The reference refers outside the temporary**, for example to a static object, a global, or an object passed as an argument. The program is valid today and stays valid. Every side effect of the destructor moves later, so reads of state that the destructor changes now see the old value.
- **The reference dangles but is never read.** The program is valid today. Only the destructor timing moves.
- **The destructor touches an object declared later in the block.** Objects end in reverse order (DTOR-5), so a later object ends before the extended temporary. A destructor that reaches that object through a pointer set during the block now reads an ended object, which is undefined (OBJ-2). Today that destructor ran at the semicolon, before the later object existed.
- **The destructor throws** (declared `noexcept(false)`, UNW-6). Today it throws at the end of the declaration. Under P7 it throws at scope exit. If the scope is left by another exception, the program ends (UNW-5).
- **A block-scope `constexpr` reference that refers to a static object through a member call** on a literal temporary. This is valid today. The temporary now lives to the end of the block, with no observable change in constant evaluation.

A front end can find every such declaration from its form alone. It is a binding whose walk passes through a reference-returning member call and reaches a class prvalue with a non-trivial destructor. The front end in `frontend/` could run that probe over a corpus.

### 4.4 Valid programs that become invalid

None by Tidal's text, if CX-1 is amended as in section 2. Without that amendment, one case breaks. A block-scope `constexpr` reference whose initializer is a member call on a temporary, returning a reference to a static object, is valid on both compilers today. Under P7 it would read as binding to a temporary and be rejected.

### 4.5 Evidence in Tidal

- **RFOR-4 and Q-0020.** The range-for Evidence records that GCC 14 and Clang 18 destroy range-expression temporaries before the body. Newer versions keep them for the whole loop, after a version change. This is the one recorded case where the language moved the end of temporaries in valid programs. Tidal handled it with a per-compiler expectation. P7 tests would need the same, since every compiler in the matrix uses today's timing.
- **Q-0040, Q-0109 and Q-0084.** The compilers already disagree at the edges of REF-4 and CTOR-17. Programs at those edges are not portable today, and P7 does not touch them.
- **REF-5 Evidence.** All compilers reject dangling reads in constant evaluation. So every 4.1 program that runs in a constant evaluation is rejected today, and none of them relies on the old timing.

## 5. Findings

**X4-1** (missing; foundations/values.md; VC-9, VC-P3, glossary "full expression"). VC-9 now defines a full expression, which settles most of D17. Its list does not cover a member initializer, a default member initializer or an init capture. Needed: where the unextended temporaries of those positions end, since P7 excludes them. Suggestion: add those positions to the list.

**X4-2** (ambiguous; initialization/references.md; REF-4, VC-3). "A member of one" does not say whether a reference member of a temporary counts. Both compilers do not extend it. The test `member-reference-of-prvalue.cpp` has a trivial destructor, so it cannot show the timing. Needed: the boundary that P7 sits next to. Suggestion: REF-4 says a reference member does not extend, with a run test.

**X4-3** (needs example; initialization/references.md; REF-4, REF-5). D18 is still open. No example sets a member access beside a call that returns a reference to the same member. Suggestion: add the pair to the Examples segment.

**X4-4** (ambiguous; foundations/constant-evaluation.md; CE-14). D19 is still open. CE-14 says a temporary bound to a reference at namespace scope counts as static. It does not say whether this follows the extension or the direct binding. Suggestion: phrase CE-14 in terms of any temporary that REF-4 extends.

**X4-5** (ambiguous; declarations/constexpr.md; CX-1, CX-2). "Must not bind to a temporary" does not say whether it covers a temporary that the initializer extends but the reference does not refer to. Today this arises for braced aggregates with reference members. Under P7 it arises for every member-call chain. Suggestion: state the ban in terms of the object referred to.

**X4-6** (missing; expressions/lambdas.md; LAM-7, REF-4). Nothing says that an init capture by reference extends a temporary, and REF-4 does not list it either. Both compilers extend it to the life of the closure: when a reference init capture was bound through a cast to a temporary, the temporary's destructor ran after the closure's scope ended. Suggestion: a rule in LAM-7 or REF-4, and a run test.

**X4-7** (missing; foundations/objects.md, exceptions/unwinding.md; OBJ-P2, UNW-1, REF-4, CE-14). No rule gives the storage duration of a temporary extended in a block. CE-14 says a namespace-scope one "counts as static". UNW-1 speaks of a block's automatic objects, and REF-4 says "in order with the other objects of its scope". Needed: whether unwinding destroys an extended temporary. Suggestion: say that an extended temporary has the storage duration of its reference.

**X4-8** (ambiguous; foundations/values.md; VC-6). "The temporaries of one full expression go away in the reverse order of their creation" reads as covering extended temporaries too, which contradicts REF-4. Suggestion: limit the sentence to the temporaries that end with the full expression.

**X4-9** (missing; initialization/references.md, expressions/new-delete.md; REF-4). REF-4 extends a temporary bound to a reference member of an aggregate initialized with braces. It does not say whether this holds for an aggregate made by a `new` expression. Needed: whether P7 reaches `new`. Suggestion: name the cases, with a test.

**X4-10** (missing; classes/members.md, foundations/values.md; CLS-13, VC-4, VC-10). Take a by-value explicit object parameter called on a class prvalue. Tidal does not say whether the prvalue initializes the parameter directly or through a temporary, and VC-10 leaves the end of the parameter to the implementation. Needed: whether such a call has a temporary to extend. Suggestion: a sentence in CLS-13.

**X4-11** (missing; initialization/references.md; REF-5). Tidal has no recommended rule for warnings about dangling references. GCC warns "possibly dangling reference to a temporary" and Clang does not. Needed: what the proposal says implementations should still warn about, namely argument temporaries. Suggestion: a `recommended` rule beside REF-5.

**X4-12** (outside the spec; docs/spec-format.md, Q-0007). There is no notation for a rule change and no version marking (designer section 2). The table in section 2 uses this report's own form. The RFOR-4 evidence shows that Tidal already faces a version change in destructor timing, with no way to mark it in the rule. Suggestion: settle the change notation and a version field.

**X4-13** (outside the spec; D32). Implementation cost. The rule can be decided from the form of the initializer and the declared return type, so the cost looks small. Tidal has no model to state it in.

**X4-14** (outside knowledge). Section 4 names the patterns that produce affected programs: accessors on rvalues, builder chains that return `*this`, and guards whose destructor has an effect. Two judgments come from general experience, and Tidal gives no frequency data for either:
- that these patterns are common in real code,
- that a lock or guard held longer is the most likely harm.

The reader also used general experience to ask whether a static object made during constant evaluation must be destroyed in a constant way. Tidal has no rule on that, so section 4.4 rests on Tidal's silence.

**X4-15** (needs example; classes/conversion-functions.md; CVF-1, REF-4). REF-4's cast item excludes user-defined conversions. No example or test shows the implicit binding through a conversion function that returns a reference. Both compilers do not extend the class temporary. Suggestion: an example and a run test.

**X4-16** (missing; statements/range-for.md; RFOR-P1, RFOR-1, RFOR-4). Tidal does not describe the range-based `for` through a hidden variable that holds the range. So a proposal cannot tell whether RFOR-4 is a special case of the reference rules or a rule of its own. P7 treats it as a rule of its own. Suggestion: one sentence in RFOR-P1 saying how the loop holds the range.
