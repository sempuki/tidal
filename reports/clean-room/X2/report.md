# X2: Propose P13, checked `noexcept`

Date: 2026-10-31. Decision 0037, Propose exercise X2. A clean-room
reader in the role of a language designer wrote this report. It read
AGENTS.md, decision 0037, the Propose section of `docs/exercises.md`,
proposal P13 in `reports/clean-room/roadmap/designer.md`, the spec pages
on exception specifications, throw, handlers, unwinding, and the parts
of other pages that the proposal touches. It read the tests under
`tests/` that contain `noexcept`, the tests with destructor bodies, and
questions Q-0094 and Q-0095. It compiled small probes with Clang and GCC
(`-std=c++23 -pedantic-errors`). The feature is the reader's own design,
made from Tidal alone.

Findings are X2-1 to X2-14.

## 1. Design

### 1.1 Summary

A function whose `noexcept` the program writes is checked. Its body
must not contain a construct that can let an exception out of it. The
implementation rejects a definition that breaks this. A new spelling,
`noexcept(default)`, gives today's behavior: the function is
non-throwing in its type, its body is not checked, and an exception that
reaches its boundary ends the program (THR-9, UNW-5).

```cpp
void log(const char*);
void a() noexcept { log("x"); }                          // rejected
void b() noexcept { try { log("x"); } catch (...) {} }   // valid
void c() noexcept(default) { log("x"); }                 // valid, as today
```

The check is local. It looks at one function body and at the declared
types of the functions the body calls. It needs no knowledge of other
bodies, because a callee's promise is in its type (EXS-P1, EXS-2).

### 1.2 Which functions are checked

A function definition is *checked* when the program writes one of these
on it:

- `noexcept`,
- `noexcept(c)` where the constant `c` is `true` (EXS-1),
- `noexcept` on a function defaulted on its first declaration, which
  EXS-4 permits.

These are *unchecked*, and keep today's run-time termination:

- `noexcept(default)`, on any function or lambda,
- `throw()`, where an implementation accepts it (EXS-5). It means
  `noexcept(default)`, so old code keeps its meaning,
- a destructor that is non-throwing by DTOR-4 without a written
  specifier,
- an implicit or defaulted special member or comparison that is
  non-throwing by EXS-4 without a written specifier. These are correct
  by construction, because EXS-4 makes them non-throwing only when every
  function they call is,
- a deallocation function without a written specifier (EXS-4, Q-0094),
- an inherited constructor (EXS-4). Its body is the base constructor's.

EXS-P3 supports this line. A function the program writes is potentially
throwing "unless it says otherwise". The proposal adds that when the
program says otherwise, the implementation holds it to it. A function
that gets its specifier from the implementation is not checked, because
the program made no promise.

### 1.3 Type and redeclaration

`noexcept(default)` gives the same function type as `noexcept`. So it
changes nothing in EXS-2, EXS-8, CONV-10, LAM-10, CMP-19, or DCL-14. A
declaration can say `noexcept` and the definition `noexcept(default)`,
or the reverse. Only the spelling on the definition decides the check.

```cpp
void close() noexcept;                // header
void close() noexcept(default) { }    // valid: same type, unchecked
```

### 1.4 Potentially throwing constructs

A checked body is rejected if it contains a potentially throwing
construct that is not handled (1.5). The constructs are:

1. a throw-expression with an operand (THR-1, THR-5),
2. a rethrow, `throw` with no operand (THR-6, TRY-8),
3. an evaluated expression for which `noexcept(e)` would be `false` by
   UN-18: a call to a potentially throwing function, an implicit call
   (constructor of a temporary, conversion function), a `new` whose
   allocation function is potentially throwing, a `dynamic_cast` to a
   reference that can fail,
4. the destruction of an object whose destructor is potentially
   throwing: a local variable, a temporary, a member or base in a
   checked destructor, or the object of a `delete` (EXS-4),
5. in a checked constructor, the initialization of a base or member,
   including from a default member initializer (EXS-4, CTOR-P2),
6. the initialization of a static or thread-local variable declared in
   the body (UNW-4),
7. in a checked function, the initialization of the result object by a
   `return`, when that initialization calls a potentially throwing
   constructor or conversion,
8. the end of a handler of a constructor's or destructor's function try
   block, because control there rethrows (TRY-10),
9. a handler parameter by value whose copy is potentially throwing
   (TRY-2). X2-4 explains why this item is a guess.

These are not checked:

- unevaluated operands (UNEV-1),
- a discarded statement (IF-13). It never runs,
- the first branch of `if consteval` (IF-P4). It runs only in constant
  evaluation, where a reached `throw` already makes the program invalid
  (CE-8, THR-P5),
- the body of a lambda defined in the checked function. It is a
  separate function with its own specifier. A call to it is construct
  3,
- the destruction of the exception object at the end of a handler
  (THR-8). Section 3.6 and X2-3 explain.

### 1.5 Handled constructs

A construct is handled when it lies inside a try block of the same
function (a function try block counts), and one of these holds:

- the try block has a catch-all handler (TRY-3),
- the construct is a throw-expression with an operand, the
  initialization of the exception object from the operand is
  non-throwing (THR-1, THR-3), and a handler of that try block matches
  the operand's static type by TRY-2.

The second form is sound because THR-4 fixes the exception object's
type at the `throw` as the static type of the operand. Every other
construct throws an exception of unknown type, so it needs a catch-all.

```cpp
struct Err {};
int f() noexcept { try { throw Err{}; } catch (Err&) { return 1; } }  // valid
int g() noexcept { try { throw 1; } catch (long) { return 1; } }     // rejected: TRY-2 has no int-to-long match
```

A handler's own body is outside its try block. An exception from a
handler goes outward (TRY-8), so the handler body is checked like any
other code in the function. A try block nested inside a handler can
handle it.

```cpp
void h() noexcept {
  try { log("a"); }
  catch (...) { throw; }    // rejected: the rethrow is not handled
}
```

### 1.6 What a rejection says

The diagnostic names the construct and, for a call, the callee. This
follows the existing practice of Clang and GCC for the direct case
(UNW-8 Evidence) and costs nothing more, since the check is local.

## 2. Rule changes

Classes follow TR-P1. "—" means the item has no class (a grammar
production, the glossary, or a rule that does not exist yet). Test paths
are relative to `tests/`.

| ID | Change | Old class | New class | Description | Tests whose expected outcome changes |
|---|---|---|---|---|---|
| `exs` grammar | amended | — | — | The specifier also takes the form `noexcept(default)`. | none |
| lambda `specifiers` grammar | amended | — | — | A lambda takes `noexcept(c)` and `noexcept(default)`, not only bare `noexcept` (X2-7). | none |
| EXS-P1 | amended | principle | principle | The specifier is a constant or the word `default`. The type carries only "non-throwing or not", never "checked or not". | none |
| EXS-P3 | amended | principle | principle | When the program writes a non-throwing specifier, the implementation proves it from the body, unless the program asks for `noexcept(default)`. | none |
| EXS-P5 | new | — | principle | A written promise is checked where it is made. The check is local: a body is judged against the declared types of what it calls. | none |
| EXS-1 | amended | required | required | The operand is a constant expression that converts to `bool`, or `default`. `noexcept(default)` makes the function non-throwing. | none |
| EXS-7 | amended | reject | reject | `default` is an exception to the rule that the operand must be a constant expression. | none |
| EXS-4 | amended | required | required | A defaulted function with a written `noexcept` is checked: if a function it calls for a base or member is potentially throwing, the program is not valid. Implicit specifiers stay unchecked. | none |
| EXS-5 | amended | impl-defined | impl-defined | Where `throw()` is accepted, it means `noexcept(default)`, not checked `noexcept`. | none |
| EXS-9 | new | — | reject | The body of a checked function must not contain a potentially throwing construct that is not handled (1.4, 1.5). | `exceptions/throw/throw-in-noexcept-terminates.cpp` (run-fail to reject), `exceptions/unwinding/leaves-noexcept-function-terminates.cpp` (run-fail to reject), `exceptions/unwinding/noexcept-boundary-is-the-function.cpp` (run-fail to reject), `exceptions/unwinding/call-in-noexcept-no-warning.cpp` (accept to reject), `exceptions/throw/throw-in-noexcept-warns.cpp` and `exceptions/unwinding/throw-in-noexcept-warns.cpp` (reject with `-Wall -Werror` to reject with no flags; rule tag UNW-8 to EXS-9) |
| EXS-10 | new | — | required | `noexcept(default)` gives the same type as `noexcept`. Its body is not checked. An exception that reaches its boundary ends the program. Declarations may mix the two spellings. | none (new tests needed) |
| EXS-11 | new | — | required | The list of potentially throwing constructs and the list of exclusions (1.4), shared by EXS-9 and UN-18. | none |
| UN-18 | amended | required | required | `noexcept(e)` is `false` if `e` contains a construct from EXS-11. The values of existing operands do not change; the list becomes complete (X2-1). | none |
| THR-P4 | amended | principle | principle | Termination at a `noexcept` boundary remains for unchecked functions. For a checked function the implementation has already proved that no exception arrives. | none |
| THR-9 | amended | required | required | The rule names every non-throwing function, not only those "declared `noexcept`" (X2-10). Its examples use `noexcept(default)`. | none (the test moves to EXS-9; a `noexcept(default)` test replaces it) |
| UNW-P3 | amended | principle | principle | Same change as THR-P4. | none |
| UNW-5 | amended | required | required | The first item ("leaves a function declared `noexcept`") covers `noexcept(default)` functions and implicitly non-throwing functions. A checked function cannot reach it, except through X2-5. | covered by the EXS-9 row |
| UNW-6 | amended | required | required | A checked `noexcept` function can throw and catch inside itself when the throw is handled by 1.5. | none (`exceptions/unwinding/noexcept-function-handles-inside.cpp` stays `run`) |
| UNW-8 | amended | recommended | recommended | The warning moves to unchecked functions: a direct `throw` in a `noexcept(default)` function or in a destructor that is non-throwing by DTOR-4. Checked functions get the EXS-9 error instead. | `exceptions/unwinding/call-in-noexcept-no-warning.cpp` and both `throw-in-noexcept-warns.cpp` tests leave this rule (listed under EXS-9) |
| glossary | new | — | — | New terms: *checked function*, *potentially throwing construct*, *handled*. | none |

Totals: 20 rows (4 new rules or principles, 2 grammar changes, 1
glossary change, 13 amended rules and principles). Six tests change.
Four change outcome, from `run-fail` or `accept` to `reject`. Two keep
`reject` but lose their `-Wall -Werror` flags and move from UNW-8 to
EXS-9. Each of the six needs a twin that spells `noexcept(default)`, so
THR-9, UNW-5, and UNW-8 keep their coverage.

Rules that the proposal reads but does not change: EXS-2, EXS-3, EXS-6,
EXS-8, DTOR-4, DCL-14, CONV-10, LAM-10, CMP-19, THR-1 to THR-8, THR-10,
TRY-1 to TRY-17, UNW-1 to UNW-4, UNW-7, CE-8, IF-13, UNEV-1, VC-10.

## 3. Interactions

### 3.1 The type system and pointers

Cites EXS-P1, EXS-2, EXS-8, DCL-14, CONV-10, CMP-19, LAM-10.

The check adds no new type. A checked and an unchecked function have the
same type, so overloads, redeclarations, pointer conversions, and
comparisons do not change. The proposal must say that "checked" is a
property of a definition, like a function body, and not of a type.

A call through a pointer to a non-throwing function is non-throwing
(EXS-2), so it needs no handler. CONV-10 keeps a potentially throwing
function from becoming such a pointer implicitly. An explicit
`reinterpret_cast` can still make one (CAST-6). The proposal must say
that such a call is undefined, or the check is not sound. Tidal does not
say (X2-5).

### 3.2 Virtual functions

Cites EXS-P2, EXS-3.

A call to a virtual function that is non-throwing in the static type is
non-throwing, because EXS-3 makes every override non-throwing too. An
override can be `noexcept(default)`. Then the promise is kept at run
time by termination, as today. The proposal keeps EXS-3 as it is, and
says that an override need not be checked.

### 3.3 Constructors and destructors

Cites CTOR-P2, DTOR-4, EXS-4, UNW-2, TRY-10, TRY-17.

A checked constructor includes its member initializers, its base
constructors, and the default member initializers it uses. A checked
destructor includes the destructors of its members and bases.

A constructor's function try block cannot absorb an exception, because
control at the end of its handler rethrows (TRY-10) and a `return` there
is not valid (TRY-17). So a checked constructor whose parts can throw is
always rejected. It must be `noexcept(default)`, or its parts must be
non-throwing. The proposal must state this outcome with an example.

A destructor without a written specifier is non-throwing by DTOR-4 but
is not checked. The roadmap entry for P13 names these destructors as the
class of programs that would change. This design leaves them out.
Section 4.4 counts what including them would cost.

### 3.4 Implicit calls

Cites UN-18, EXS-4, DCMP-1, DCMP-6, SB-5, RFOR-1, RFOR-2, NEW-P3,
UNW-3, UNW-4, VC-10, REF-4.

The check must find every call the body makes without naming it. UN-18
gives two by example. A complete list must also cover:

- the destructors of temporaries and locals,
- rewritten and reversed comparisons (DCMP-6) and defaulted ones
  (DCMP-1, Q-0095),
- the calls to `get` for a structured binding (SB-5),
- the calls to `begin`, `end`, and the iterator operations of a
  range-based `for` (RFOR-1, RFOR-2),
- the allocation and deallocation functions of `new` and `delete`
  (NEW-P3, UNW-3, Q-0094),
- the initialization of a static local (UNW-4),
- the copy into a by-value parameter and the destruction of a parameter
  (VC-10).

Tidal has no such list (X2-1). The proposal must either supply one or
define the construct set by reference to one page that has it.

### 3.5 Arguments and parameters

Cites VC-P4, VC-10, UNW-5.

A by-value argument is initialized somewhere around the call. VC-10 says
the parameter is destroyed either at the function's return or at the
end of the caller's full expression. If the parameter's destructor can
throw and it runs at the return, the exception may leave the callee,
which is checked. The proposal takes the safe course: a by-value
parameter with a potentially throwing destructor makes a checked
function invalid, on every implementation. Argument initialization is
the caller's construct. Tidal says neither where arguments are
initialized nor which side of a `noexcept` boundary a parameter's
destruction is on (X2-2).

### 3.6 Handlers and the exception object

Cites THR-3, THR-4, THR-8, TRY-2, TRY-3, TRY-8, TRY-10.

THR-3 permits an exception object whose destructor is potentially
throwing. That object is destroyed when the last handler for it ends
(THR-8), and that point is inside the checked function. A catch-all
handler cannot know the type, so if this destruction counted, no
`catch (...)` in a checked function could ever end. The proposal
exempts it and keeps termination for it. Tidal does not say what happens
when that destructor throws (X2-3).

A by-value handler parameter is copied from the exception object. Tidal
does not say where an exception from that copy goes (X2-4).

### 3.7 Constant evaluation

Cites TR-P1, TR-P4, THR-P5, THR-10, CE-8, IF-P4.

EXS-9 is a `reject` rule, so TR-P1 applies. The program is rejected
whether or not the construct is ever reached. A `constexpr` function
marked `noexcept` that throws on an untaken path is valid today (THR-10,
CE-8) and is rejected under the proposal. The first branch of
`if consteval` runs only in constant evaluation, where a reached `throw`
is already an error, so the proposal exempts it.

### 3.8 Templates

Cites EXS-1, IF-13, IF-22, CLS-2.

`noexcept(c)` with a `c` that depends on a template parameter decides,
for each specialization, whether that specialization is checked. A
discarded statement in a template is not instantiated (IF-13), so it is
not checked. The proposal must say that the check runs on each
specialization and not on the template. Tidal has no template chapter to
cite for when a specialization's body is formed (X2-6).

### 3.9 The `noexcept` operator

Cites UN-18, UNEV-1, CVF-3.

Values do not change. A checked function and its `noexcept(default)`
twin give the same result from `noexcept(f())`. UN-18 and EXS-9 share
one construct list (EXS-11), so a generic wrapper written
`noexcept(noexcept(expr))` around the same `expr` always passes the
check. The proposal must show this as an example, because it is the main
way generic code stays checked.

### 3.10 The library

Cites UN-18, NEW-P3.

The default allocation function throws. Its exception type, the
non-throwing allocation form, and a function that ends the program
without returning are all library referenced. The proposal cannot say
which library calls a checked function can make (X2-8).

## 4. Impact on existing programs

### 4.1 Valid programs that change meaning

None. The check adds no run-time behavior, and it changes no type and no
`noexcept(e)` value. A program that stays valid does what it did.

### 4.2 Valid programs that become invalid

A program becomes invalid when a function with a written non-throwing
specifier contains a construct from 1.4 that 1.5 does not handle. The
cases, with the tests that show them:

- A direct `throw` in a `noexcept` function. Today it terminates
  (`exceptions/throw/throw-in-noexcept-terminates.cpp`,
  `exceptions/unwinding/leaves-noexcept-function-terminates.cpp`).
  Clang and GCC already warn under `-Wall` (UNW-8 Evidence;
  `exceptions/throw/throw-in-noexcept-warns.cpp`). The reader's probe
  shows both also warn on a rethrow in a handler of a `noexcept`
  function.
- A call to a potentially throwing function from a `noexcept` function
  (`exceptions/unwinding/noexcept-boundary-is-the-function.cpp`,
  `exceptions/unwinding/call-in-noexcept-no-warning.cpp`). No compiler
  warns today, and UNW-8 says none need to. This is the large class in
  real code.
- A `noexcept` constructor whose member or base construction can throw
  (3.3). No test covers it.
- A `noexcept` function that uses `new` without a catch-all (UN-18;
  `expressions/unary/noexcept-new-false.cpp` shows the value). No test
  covers the check.
- A `constexpr` `noexcept` function with a `throw` on an untaken path
  (3.7). No test covers it.

The fix for each is mechanical. Writing `noexcept(default)` in place of
`noexcept` on the failing definition restores the old meaning exactly,
so a tool can migrate a code base with no change in behavior.

Where the compilers disagree today, they would disagree on validity:

- Q-0094: GCC treats a class-specific `operator delete` without a
  specifier as potentially throwing. A checked function that runs
  `delete` on such a class would be valid on Clang and EDG and rejected
  on GCC.
- Q-0095: EDG treats a defaulted `==` as non-throwing when a member `==`
  is potentially throwing. A checked function that compares such
  objects would be valid on EDG only.
- Q-0077: EDG rejects `throw()`. The proposal maps `throw()` to
  `noexcept(default)`, so this disagreement does not grow.

The provisional answers of these questions would become the line
between valid and invalid programs (X2-11).

### 4.3 Programs that become valid

Programs that spell `noexcept(default)`. Both Clang and GCC reject that
spelling today ("expected expression"), so no valid program uses it and
the space is free. A lambda with `noexcept(c)` is accepted by both
compilers today, though Tidal's grammar does not list it (X2-7).

### 4.4 The alternative: check every non-throwing destructor

If destructors that are non-throwing by DTOR-4 were checked too, every
destructor that calls a function without `noexcept` would become
invalid. A search of `tests/` finds 47 files with such a destructor,
mostly a `~T() { mark(id); }` where `mark` has no specifier
(for example `foundations/objects/block-reverse-order.cpp`,
`exceptions/unwinding/constructor-throws-parts-destroyed.cpp`,
`classes/constructors/dtor-order.cpp`), plus
`exceptions/unwinding/leaves-implicit-noexcept-destructor-terminates.cpp`
(run-fail to reject). The count is from a pattern search and is
approximate. The proposal rejects this alternative. The break is too
wide for the gain, and EXS-P3 ties a check to a promise the program
wrote. UNW-8, amended, gives these destructors a warning for the direct
case instead.

### 4.5 The alternative: catch-all only

P13 in the roadmap requires a catch-all handler. Under that rule
`exceptions/unwinding/noexcept-function-handles-inside.cpp`
(`try { throw 2; } catch (int v)`) becomes invalid too. The typed match
in 1.5 keeps it valid at the cost of one more condition, which THR-4 and
TRY-2 already make decidable.

### 4.6 Implementation cost

The check is one pass over each checked body, with no analysis across
functions. It reuses the construct list that `noexcept(e)` needs
already. The two hard parts are the complete implicit-call list (X2-1)
and per-specialization checking in templates (X2-6).

## 5. Findings

- **X2-1** (missing; `expressions/unary.md`,
  `declarations/exception-specifications.md`; UN-18, EXS-4, DCMP-6,
  SB-5, RFOR-2). The check needs every call an expression or statement
  makes without naming it. UN-18 gives two by example. There is no list
  that covers temporaries' and locals' destructors, rewritten
  comparisons, structured-binding `get`, range-for calls, static local
  initialization, and allocation and deallocation. This is D29 from the
  roadmap, now confirmed as the main blocker. Suggestion: one list of
  implicit calls on the exception-specifications page, cited by UN-18
  and by each page that makes such a call.
- **X2-2** (missing; `foundations/values.md`,
  `exceptions/unwinding.md`; VC-10, UNW-5). Tidal does not say where an
  argument is initialized, or whether a parameter destroyed at the
  function's return is inside the callee's `noexcept` boundary. Needed:
  which side owns each. Suggestion: a rule beside VC-10 that names the
  side for initialization and for each of the two destruction points.
- **X2-3** (missing; `exceptions/throw.md`; THR-3, THR-8). THR-3 permits
  an exception object with a potentially throwing destructor. Nothing
  says what happens when that destructor throws as the last handler
  ends. Needed: whether the program ends or the new exception
  propagates. Suggestion: a rule in THR-8 and a run test.
- **X2-4** (missing; `exceptions/handlers.md`; TRY-2, THR-8). A
  handler parameter by value is copied from the exception object. No
  rule says where an exception from that copy goes, or whether the
  handler is then active. Suggestion: a rule beside TRY-2 and a test.
- **X2-5** (missing; `expressions/casts.md`, `expressions/postfix.md`;
  CAST-6, CONV-10, PFX-8). `reinterpret_cast` can turn a pointer to a
  potentially throwing function into a pointer to a non-throwing one.
  No rule says what a call through a pointer of the wrong function type
  does. PFX-8 covers only a null pointer. Without a rule, a checked
  function can be bypassed. Suggestion: an `undefined` rule for a call
  through a function pointer whose type differs from the function's,
  with the `noexcept` case as its example.
- **X2-6** (missing; no template page; EXS-1, IF-13, IF-22). Conditional
  `noexcept` depends on template arguments. A designer cannot say when a
  specialization's body is formed and checked, or how a dependent
  `noexcept` operand is evaluated. The roadmap names this gap.
  Suggestion: the template chapter, or at least a rule on when a
  specialization's body is checked.
- **X2-7** (missing; `expressions/lambdas.md`; LAM-10, the `specifiers`
  production). The lambda grammar lists only bare `noexcept`. Clang and
  GCC accept `noexcept(sizeof(int) == 4)` on a lambda (reader's probe).
  Suggestion: let the production take the full `noexcept_specifier` and
  add a test.
- **X2-8** (outside the spec; `expressions/new-delete.md`; UN-18,
  NEW-P3). The default allocation function's exception, the
  non-throwing allocation form, and any function that ends the program
  without returning are library referenced. A checked function's use of
  them cannot be analyzed. Suggestion: a short list of the library
  functions whose specifier the language pages rely on.
- **X2-9** (ambiguous; `exceptions/unwinding.md`; UNW-8). "Outside any
  try block that can catch it" has no static definition. The reader's
  probe shows Clang and GCC both warn on a rethrow inside a handler, so
  they read "can catch" narrowly. Needed: what makes a try block able to
  catch a given `throw`. Suggestion: define it by TRY-2 against the
  operand's static type (THR-4), as 1.5 does, with an example for a
  rethrow.
- **X2-10** (contradictory; `exceptions/throw.md`,
  `exceptions/unwinding.md`; THR-9, UNW-5, EXS-4). THR-9 and UNW-5 say
  "declared `noexcept`". EXS-4 makes implicit special members, defaulted
  comparisons, deallocation functions, and inherited constructors
  non-throwing without any declaration. UNW-5 names destructors
  separately but not the others. Needed: whether an exception that
  leaves one of them ends the program. Suggestion: state THR-9 and UNW-5
  for every non-throwing function, using the term the Syntax segment of
  the exception-specifications page defines.
- **X2-11** (ambiguous; `declarations/exception-specifications.md`;
  EXS-4, Q-0094, Q-0095). Under P13 the provisional answers of Q-0094
  and Q-0095 decide which programs are valid, and the compilers split.
  Needed: settled answers. Suggestion: settle both before any feature
  that depends on the non-throwing status of an implicit function.
- **X2-12** (ambiguous; `expressions/unary.md`; UN-18). "A
  `dynamic_cast` to a reference type that can fail" does not say when a
  cast cannot fail. The test shows a downcast. A cast to the same class
  or to a base is the open case. Suggestion: say which casts count and
  add a test for a cast to a base.
- **X2-13** (missing; `docs/spec-format.md`; all rows of section 2).
  Grammar productions have no IDs, so a proposal cannot cite a change to
  `exs` or to the lambda `specifiers` by ID. The format has `withdrawn`
  and no way to mark a rule amended or reclassified. The roadmap raised
  the second point. Suggestion: IDs or anchors for productions, and a
  change-table form in the spec format.
- **X2-14** (outside knowledge; `declarations/exception-specifications.md`;
  EXS-1, EXS-7). The design depends on `default` being unable to start
  an expression, so that `noexcept(default)` is free. Tidal has no
  keyword list. The reader confirmed it by compiling with Clang and GCC
  and by its own knowledge that `default` is a keyword. This is the
  roadmap's keyword gap (D20). Suggestion: a keyword list on a lexical
  page.

## Rule IDs cited

EXS-P1, EXS-P2, EXS-P3, EXS-P5 (new), EXS-1 to EXS-8, EXS-9 to EXS-11
(new), THR-P4, THR-P5, THR-1 to THR-10, TRY-2, TRY-3, TRY-8, TRY-10,
TRY-17, UNW-P3, UNW-2 to UNW-8, UN-18, UNEV-1, DTOR-4, CTOR-P2, CONV-10,
LAM-10, CMP-19, DCL-14, DCMP-1, DCMP-6, SB-5, RFOR-1, RFOR-2, NEW-P3,
VC-P4, VC-10, REF-4, CAST-6, PFX-8, CE-8, TR-P1, TR-P4, IF-P4, IF-13,
IF-22, CLS-2, CVF-3.
