# tidal-fe

A C++ front end written in Rust from the Tidal specification, to exercise
the specification. The front end checks programs and runs them on an
interpreter of the abstract machine. It does not generate code. The
front end can use any crate that does not encode knowledge of C++
(decision 0035).

## Build

```sh
cd frontend
cargo build --release
```

The binary is `target/release/tidal-fe`.

## Check one file

```sh
target/release/tidal-fe check FILE [--m32] [--werror] [-Wname ...]
```

| Exit status | Meaning |
|---|---|
| 0 | accept |
| 1 | reject: at least one error, printed with a Tidal rule ID when one applies |
| 2 | unsupported: the front end met a construct it does not understand and found no error |

A token sequence that the parser does not derive is an error that names
TR-12. The parse stays unsupported when the front end knows that it
does not model the construct (an extension outside the specification,
an `asm` declaration, a named escape, a case range), or when the stop is
inside a class template or a scope where an earlier name was unknown,
since there the stop can come from the front end and not from the
program.

Diagnostics go to standard error, one per line:

```
FILE:LINE: error [SW-3]: duplicate case value 1
FILE:LINE: error [CX-1, CE-22]: initializer is not a constant expression (...)
FILE:LINE: unsupported: static_assert: new expression
FILE:LINE: warning [STMT-4]: expression result unused
```

An error names a rule of a class that allows rejection (decision 0034).
When a constant was needed and the evaluation failed, the error names
two rules: the rule that needs the constant, then the rule that the
evaluation broke. The conformance count of matching rules accepts either.

`--m32` switches the target from LP64 (x86-64 Linux) to ILP32.
`--werror` turns the warnings for `recommended` rules into errors.
Some warnings run only when a `-W` flag asks for them, following the
warning-flag table in the Evidence of `specification/foundations/translation.md`:
ATTR-6 with `-Wall`, `-Wunused`, `-Wunused-variable`, or
`-Wunused-but-set-variable`, SEQ-7 with `-Wall`, ATTR-7 with
`-Wimplicit-fallthrough` or `-Wextra`, CTOR-13 with `-Wall`, the
deprecated-copy part of CPY-13 with `-Wextra`, and the pessimizing-move
part of CPY-13 with `-Wall`. ODR-11 is on with no flag, and every test
passes `-pedantic-errors`, which makes it an error. The front end names
ODR-11 with ODR-9.

## Run one program

```sh
target/release/tidal-fe run FILE [FILE...] [--m32]
```

`run` checks each file as a translation unit of its own, links the units
together, and runs the program on the interpreter (decision 0048). The
members of a closure type never link, since each lambda has a type of
its own. A call through a pointer to a function of another unit runs
that unit's function when its parameters and result have scalar types
(LNK-2). `run` prints one line:

```
result: 0
unspecified: result 0 with fixed choices: SEQ-1 at line 7 (arguments evaluated from left to right)
undefined [MUL-9] at line 8: division by zero
terminated [UNW-5] at line 4: an exception leaves main
unsupported at line 3: pointer to member constant
rejected [CTOR-25] at line 5: no constructor of 'S' accepts the arguments
```

| Exit status | Meaning |
|---|---|
| the program's status, reduced to 0 to 255 | `result` or `unspecified`: the program returned from `main` |
| 125 | the check rejected a translation unit |
| 126 | `undefined`: the run stopped at undefined behavior and gives no result |
| 127 | `unsupported`: the check or the interpreter met a construct it does not model |
| 134 | `terminated`: the program ended without returning from `main` (UNW-5, THR-7), as `std::abort` does on the matrix targets |

A program's own status can also be 125 to 127 or 134. The printed line
tells the cases apart.

An `unspecified` line lists the choice points that the first run
passed, each with the rule that leaves the choice open. The first run
takes the first option at each point. `run` then runs the program again
with the other options at the choice points of SEQ-1 (every order of
the arguments; the second run of a call reverses the order), SEQ-8 (left or right operand first), PRIM-9 (one object
or two), CMP-8 (either order), CMP-20 (equal or not), and MPTR-7, up to
64 runs in all (decision 0048). A second line gives the other outcomes:

```
explored: 4 runs; other outcomes: result 1; result 0
explored: 64 runs (the bound of 64 runs stopped the exploration; the set is partial); other outcomes: result 0
```

When every explored run returns the result of the first run, the first
line says so before it lists the choices (finding P2-9). The second line
stays:

```
result: 0 in every explored order (8 runs); unspecified choices: SEQ-1 at line 90 (arguments evaluated from left to right)
explored: 8 runs; other outcomes: result 0
```

The arguments of a multi-argument `operator[]`, and the operands of the
other operators that OPR-7 lists, keep their order, so they are no
choice point. A run with more than 4096 choice points settles a line
once another option there gives the first outcome. The set is then
partial too, and the second line says so.

If one of the other runs stops at undefined behavior, the program's
outcome is that stop. The conformance harness uses the same outcome.

The first evaluation of a string literal makes its object. Each later
evaluation of a literal with the same characters makes an object of
its own, and a comparison of pointers into two such objects is the
choice point of PRIM-9: one object, with the offsets compared, or two.
A copy of one pointer points into one object, so it compares equal to
the original with no choice.

Within the operands of a built-in operator whose operands are
unsequenced (SEQ-8), the run records the reads and writes of objects in
the caller's frame. Two accesses to one memory location, one of them a
write, stop the run with SEQ-4. Adjacent bit-fields are one location.
Two reads of a volatile object do not stop the run (finding G102 of
report-10b). A read or write of a volatile object through a glvalue
that is not volatile stops the run with VOL-6, after the object's
construction and before its destruction. The check covers a volatile
complete object, reached by any glvalue, and a volatile member, reached
by a member glvalue. It does not cover the bytes of a volatile member.

A floating conversion to a type whose range the value exceeds gives
infinity, in a run and in constant evaluation (CONV-11, the matrix
table for Clang and GCC on both targets).

After `main` returns, a pass through the declaration of a block-scope
`static` variable that has ended stops the run with OBJ-12.

A `goto` or a `case` label can enter a branch of an `if` or the body of
a loop. The jump skips the init statement and the condition, and the
variables of a skipped declaration exist with no value. After the
body, a `while` or `for` loop goes on with its step and condition, and
a `do` loop with its condition (finding G161 of report 24).

An exception unwinds the stack as the UNW-7 table gives for Clang. With
no matching handler before `main`, no automatic object is destroyed. At
the boundary of a non-throwing function, the objects of the functions
it calls are destroyed and its own objects are not, also when a try
block of the function has no matching handler.

### C library functions

The specification states no library. As a front-end choice under
decision 0035, the interpreter knows a closed list of C library
functions. A program declares each one itself, with C language linkage
at global scope and the usual parameters:

| Function | What the interpreter does |
|---|---|
| `_Exit(int)` | ends the program with the status; no object is destroyed |
| `exit(int)` | ends the program with the status after the static objects end; no automatic object is destroyed |
| `abort()` | ends the program as `terminated` |
| `malloc(n)` | gives storage of `n` bytes, as the library's `operator new` does, or a null pointer for a size above the interpreter's limit |
| `free(p)` | releases storage, as the library's `operator delete` does |
| `puts(s)` | returns 1; the run does not record the output |
| `putchar(c)` | returns `c` as an `unsigned char` value; the run does not record the output |

Any other function without a definition leaves the run unsupported.
A program that declares `std::nothrow` as LSUP-6 gives it, with no
definition, gets the library's object at its first use. The library's
nothrow allocation forms return a null pointer when they fail (finding
G162 of report 24).

### Choices of the run that follow the matrix

Where a rule leaves the result to the implementation, or does not
require a diagnostic, the run follows Clang and GCC on the matrix
targets, as the tests do:

| Rule | What the interpreter does |
|---|---|
| OBJ-11 | a `thread_local` object at namespace scope, or a `thread_local` static data member, with dynamic initialization is constructed at the first use of such a variable of its unit; a use of a variable with constant initialization starts nothing |
| INST-11 | the definition of an object of a class specialization makes the virtual member functions that are not pure, at the end of the unit; `sizeof` of the class makes none |
| LSP-8 | a global variable with C++ language linkage and a C variable of the same spelling in another unit are one variable |

`main` can have the parameters `()`, `(int)`, `(int, char**)`, and
`(int, char**, char**)`. These are the lists that Clang accepts on the
matrix targets (FN-6), and the check rejects the others under TR-3. The
run gives one argument, the program's name, and an empty environment.

## Run the conformance tests

```sh
target/release/tidal-fe conform ../tests [-v] [--out FILE]
```

`check` reads `#include "file"` relative to the directory of the file
that holds the directive (INC-2). The other preprocessing is a small
slice:

- the conditional groups of the ODR page's grammar: `#if`, `#ifdef`,
  `#ifndef`, `#elif`, `#else`, and `#endif`, with integer expressions
  and `defined`,
- `#define` of a name with or without a value, and `#undef`. A name with
  a value is replaced by its tokens.
- `#pragma once` anywhere in a file (INC-5).

A function-like `#define` and other directives are unsupported.

The harness reads the `// tidal-` directives at the top of each test. The
harness judges each test by its base `tidal-expect` line, and treats
`run`, `run-fail`, and `link-fail` as accept for the check. The
`tidal-expect-edg` override only names a second answer. A
`tidal-allows-edg: RULE-ID` line names the rules that allow that second
answer (G134). A test with no base line gets the status `NO-BASE` and
does not pass with either answer (G135). The
kind of each rule comes from `../specification` beside the test
directory, or from the directory that `--spec DIR` names. `-Werror` in `tidal-flags`
passes `--werror`, `-m32` passes `--m32`, and the other `-W` flags
pass through. A test with
`tidal-sources` checks each translation unit on its own.

The output is a table per directory and per subdirectory with these
columns:

- **Pass**: the answer matches the expectation.
- **False reject**: the front end rejected a valid program.
- **Missed reject**: the front end accepted an invalid program.
- **Unsupported**: the front end gave up on a construct and found no error.
- **Differs from EDG only**: the answer differs from the EDG override
  but matches the test's base `tidal-expect` line. A pass.
- **Agrees with EDG only**: the answer matches the EDG override and not
  the base line. A failure.
- **EDG, allowed**: the answer matches the EDG override and not the
  base line, and the rules allow a second answer. A pass. With a
  `tidal-allows-edg` line, each rule that the line names is
  `impl-defined`, `unspecified`, `no-diagnostic`, or
  `conditionally-supported`. Without one, every rule the test cites is
  of one of these kinds.
- **No base expectation**: the test has no base `tidal-expect` line, so
  the harness has no answer to judge by. A failure.

A table per page (test subdirectory) follows. It counts the rejects that
pass and the rule that the first error names: a rule the test cites, a
rule that the text of a cited rule links to with {rule}`ID` (as decision
0052 lets a run stop match), another rule, or no rule. The share is the
first two columns over the passing rejects. With `-v`, each passing
reject whose first error names another rule or no rule has a line that
starts with `RULE-ID`, then `other-rule` or `no-rule`, the test, its
rules, and the rules that the error names.

The harness then runs each test whose base expectation is `run` or
`run-fail`, if the check accepted every translation unit, and links the
units of a test with `tidal-sources`. A `run` test passes when the
exit status is 0. The status is the value of `main` modulo 256
(TR-11). A `run-fail` test passes when the run stops at undefined
behavior that names one of the test's rules, or the program ends with a
nonzero status or terminates. A separate table counts the run
tests:

- **Pass**: every explored run gives an outcome that the expectation
  allows.
- **Pass, partial**: a pass whose exploration is partial, because the
  bound stopped it, a line was settled, or another run is unsupported.
  These tests are in **Pass** too.
- **Other outcome**: the first run gives an allowed outcome, and another
  explored run does not.
- **Wrong result**: a `run` test ended with a nonzero status or
  terminated.
- **Missed undefined behavior**: a `run-fail` test ended with status 0.
- **Unexpected undefined behavior**: the run stopped at undefined
  behavior that the test does not expect.
- **Unsupported**: the interpreter met a construct it does not model.
- **Check did not accept**: the check rejected a unit or gave up, so the
  program did not run.

The output then lists the unsupported constructs, with the number of tests each
one affects, first for the interpreter and then for the check. A
variable template argument of unknown type names the construct that
gave the unknown type, such as `new expression (in a variable template
argument)`, so that the list counts root causes. `-v`
also lists every test that did not pass. `--out FILE` writes the tables
and one line per test to FILE.

Each test line starts with a status. For a test that expects a run, the
status gives the run's outcome when the check passed:

| Status | Meaning |
|---|---|
| `pass` | the check matches the expectation, and a `run` or `run-fail` test also passed its run |
| `EDG-ONLY` | the check differs from the EDG override and matches the base expectation |
| `EDG-AGREES` | the check matches only the EDG override; a failure |
| `EDG-ALLOWED` | the check matches only the EDG override, and the rules allow either answer; a pass |
| `NO-BASE` | the test has no base `tidal-expect` line; a failure |
| `pass-partial` | as `pass`, but the exploration of the choice points is partial |
| `FALSE-REJECT` | the check rejected a valid program |
| `MISSED-REJECT` | the check accepted a program that the test expects rejected |
| `unsupported` | the check gave up on a construct and found no error |
| `run-unsupported` | the check passed, and the interpreter met a construct it does not model |
| `RUN-WRONG-RESULT` | the check passed; a `run` test ended with a nonzero status or terminated, or a `run-fail` test whose rules name no undefined behavior ended with status 0 |
| `RUN-MISSED-UB` | the check passed; a `run-fail` test ended with status 0 |
| `RUN-UNEXPECTED-UB` | the check passed; the run stopped at undefined behavior that the test does not expect |
| `RUN-OTHER-OUTCOME` | the check passed; the first run passed, and another explored run gives an outcome that the expectation does not allow |

The run tests also have lines of their own that start with `RUN`, with
the run status second (`run-pass`, the run statuses above, and
`run-not-run` when the check did not accept a unit).

## Layout

| File | Contents |
|---|---|
| `src/lexer.rs` | tokens, literal checks (PRIM rules), and `#include "file"` with include guards and `#pragma once` (INC rules) |
| `src/parser.rs` | parser state, entities, scopes, name lookup |
| `src/decl.rs`, `src/decl2.rs` | decl-specifiers, declarators, declarations, functions, templates |
| `src/class.rs` | class and enumeration specifiers |
| `src/expr.rs`, `src/expr2.rs` | expressions, conversions, calls, casts, initialization |
| `src/ics.rs` | implicit conversion sequences and their comparison (ICS rules), and overload resolution for calls (OVL-22 to OVL-25, OVL-34 to OVL-36) |
| `src/ovl.rs` | overload resolution for operators, constructors, calls on a class object, and the address of an overload set (OVL rules) |
| `src/templ.rs` | templates: the record of each template, template argument lists, specializations made by parsing a template's tokens again with its parameters bound, selection of explicit and partial specializations, deduction, partial ordering, packs, and the definitions that a use needs (TPL, TARG, TSP, DED, ORD, INST, DEP rules) |
| `src/cons.rs` | concepts and constraints: normal forms, satisfaction, subsumption, requires expressions, abbreviated function templates, and the use checks of packs (CPT, CNS, DED-34, PACK-8) |
| `src/ctad.rs` | deduction of class template arguments: the guides, their choice, and the sites (CTAD rules) |
| `src/dcmp.rs` | the bodies of defaulted comparison operators, made at their first use (DCMP rules) |
| `src/stmt.rs` | statements and jump checks |
| `src/interp/mod.rs` | the interpreter's machine: values, objects, places, the objects that end with a block, a full expression, or the program, the stops (decision 0048), the choice points, and the SEQ-4 record of accesses |
| `src/interp/bytes.rs` | the byte view of objects (STG-11 to STG-16, STG-23, STG-26): object representations, byte pointers, addresses, `reinterpret_cast` (CAST-6, CAST-16), pointer-interconvertible objects (STG-16, CLS-19), the access check of STG-14, and the exceptions of LSUP-8 |
| `src/interp/expr.rs` | expressions, conversions, initialization, constructors, copies, assignments, calls, pointers to members, `new` (also placement `new` in a byte array or in the place of an object, STG rules, and the allocation and deallocation functions that a class or the program declares, NEW-13 and NEW-21 to NEW-24), allocation functions called by name, explicit destructor calls, `delete`, `throw`, and `dynamic_cast` |
| `src/interp/stmt.rs` | statements, blocks, `goto`, `switch`, try blocks and handlers, and static locals |
| `src/interp/run.rs` | a whole program: linking the units, static initialization, `main`, and the end of the static objects |
| `src/consteval.rs` | constant evaluation, as a client of the interpreter with the limits of `constant-evaluation.md` |
| `src/conform.rs` | the conformance harness |

## The rewrite

`next/` holds the rewrite (decision 0060, `DESIGN-rewrite.md`), a crate
of its own with its own `Cargo.lock` and `target/`. Build steps 1 and 2
are in place: the source map, the diagnostics, the lexer, the parser
with the `Names` oracle, and the driver. The old front end in `src/`
stays the front end of record and judges every test.

```sh
cd next
cargo build --release
cargo test --release
target/release/tidal-next parse FILE...   # tree, or stop LINE: why
target/release/tidal-next dump FILE       # the syn trees
target/release/tidal-next lex FILE...     # one token a line
```

`tidal-next check` gives `reject` with a TR-12 error when the parser
stops and `unsupported` when the unit parses, since analysis comes at
build step 3. `tidal-fe lex` prints the same form as `tidal-next lex`,
and `tools/frontend_check.py --next` compares the two, runs the unit
tests, and counts the tests that parse as their outcome needs.

## The conditional operator

When an operand has class type, `class_cond` in `src/expr.rs` follows
the page as one procedure. The rows of COND-3 before the last come
first (`cond_rows`). Otherwise COND-16 tries each operand against the
other (`cond16_try`): step 1 for two lvalues, step 2 for an xvalue
target, then the table of step 3, whose sequence is formed without
access or deleted checks. If exactly one operand converts, the
conversion is made (`cond16_apply`), and a private or ambiguous base
or an ambiguous sequence is an error there. The rows of COND-3 then
run again. If neither converts, COND-11 runs overload resolution over
the built-in candidates (`cond11`): every pair of promoted arithmetic
types, a `(T, T)` candidate for each pointer, pointer-to-member, and
scoped enumeration type that the operands reach, and the composite
pointer types of the reached pointers. A class prvalue result copies or
moves each glvalue operand, and the copy or the move must be valid and
accessible (COND-8).

## Templates

A template keeps the tokens of its declaration. The front end parses
the pattern once, with each template parameter as a dependent type, for
the checks that need no arguments and as the patterns of deduction,
selection, and partial ordering. A specialization is made by parsing the
tokens again with each parameter bound to its argument. A class
specialization is made complete when a use needs it (INST-1), the body
of a member function when a use needs it, and the definition of a
function specialization at its first use if constant evaluation can call
it, or else at the end of the unit (INST-31). Names in namespaces are
looked up as at the template's definition (DEP-5), and argument-dependent
lookup of a dependent call also sees later declarations (DEP-9).
A constraint is kept as its normal form: conjunctions and disjunctions
over atomic constraints, each with its tokens and its parameter mapping
(CNS-P1). A check parses an atom's tokens again with the mapping bound,
and an error in that parse makes the atom not satisfied (CNS-7).
Subsumption compares atoms by their place in the source and their
mapped arguments (CNS-4, CNS-22). A function with placeholder parameters
becomes a template with invented parameters (DED-34). A class template
named without arguments deduces them through its guides, each a hidden
function template (CTAD-P1). A constant parameter declared with a
placeholder pattern, such as `auto*`, matches an argument whose type has
that form (DED-12).

Partial ordering in a call gives each member template an object
parameter (ORD-6). For two members, the object parameters name the
more derived of the two classes, so a member that a using declaration
brings in from a base counts as the derived class's own member. For the
whole function types of ORD-9, a `T&&` in one type takes an lvalue
reference `X&` of the other as `T = X&` (G146). A requires clause of a
member of a local class in a specialization is parsed with its errors
held back. A clause with an error is a constraint that is not
satisfied (INST-12).

In a member template of a class specialization, a function parameter
pack whose pattern names a pack of the class expands when the member
template is declared, because the class's pack is known (INST-5,
PACK-1). A constant template argument of a pointer-to-member type is the
member it names (TARG-12). In a template, a functional cast with a
class template name and no argument list deduces at the instantiation
(CTAD-1). A designated list has an aggregate candidate with one
parameter for each named member, in member order (CTAD-9). Deduction of
a converting constructor template that ICS-4 considers takes only a
standard sequence for its first parameter (ICS-10), so the deduction
does not consider the constructor again.

A member function with an explicit object parameter of a deduced type
deduces it from the object of the call, as the first argument (CLS-13).

A lambda with a placeholder parameter or a template head is a generic
lambda (LAM-15). Its body is parsed as a pattern, so a check that needs
the parameter types waits for a call (SA-5). A call deduces the
template arguments, also a trailing pack and explicit type arguments,
and parses the lambda again with them bound to make the call operator.
A lambda with an explicit object parameter gets the call operator's
declaration before its body is parsed, so the body can call itself
(LAM-16). The requires clause of a generic lambda, after its template
head or after its parameters, is checked for each call when the call
operator is made. A call operator whose clause is not satisfied gives
no candidate (CNS-1, OVL-24). The placeholder parameters of a lambda are
the lambda's own invented parameters, also inside an abbreviated
function template (LAM-15, DED-34). A generic lambda with no captures converts to
a pointer to function when the specialization for the target's
parameters has the target's type (LAM-10).

An error in the body of such a call operator is held, and the front end
reports it only when a call selects that operator. So a generic lambda
in an overload set does not make the program invalid for a call that
another candidate wins (G152 in report 22).

A lambda that the look-ahead of class template argument deduction
parses keeps its closure type when the initializer is parsed again, so
the deduced class names that type (CTAD-1).

A guide made from a constructor template keeps the constructor's
constraint, and the constraint can name the class's parameters, which
the guide deduces first (CTAD-29). In a template, a class temporary and
an operator call whose operands depend on no parameter are formed as
outside a template, so that such a value can be a constant template
argument of a partial specialization (TSP-14, TARG-19). A substitution
failure of an alias template is remembered, and a muted use of the same
specialization fails again, so a requires expression whose parameter
type is not valid is false (CPT-28).

## Bytes

The value of each object stays the interpreter's record. The bytes of an
object are computed from its value when a program reads them through
`unsigned char`, `char`, or `std::byte`, and a write of a byte changes
the scalar that holds it. Integers are little-endian two's complement
(STG-26), and `float` and `double` use their IEEE formats (MUL-20).
Padding bytes, and the bytes of a scalar with no value, are kept apart
from the values. A pointer's bytes are its address. The bytes of a
pointer to member hold a number that the interpreter gives each value
it encodes, and 0 for the null value, so a copy of the bytes gives the
same pointer (STG-12). The interpreter
gives each object an address when a program first needs one, aligned
for its type. The interpreter computes bytes only for classes without
bases and bit-fields, and not for `long double`.

A `reinterpret_cast` to a byte type gives a byte pointer, which steps
over the bytes of the complete object. A cast to another object type
gives the pointer-interconvertible object of that type (STG-16, CLS-19),
or else the same object. A read or write through a glvalue whose type
STG-14 does not allow for the object stops the run with STG-23.

A `T*` that points to a byte of a byte array, where `T` is not a byte
type, counts the byte as an array of one `T` (ADD-14). Arithmetic on it
steps `sizeof(T)` bytes, so `p + 1` is the address `sizeof(T)` bytes on
(Q-0430, provisional answer). A step of more than one element is
undefined. The front end checks each step on its own and does not
remember where the pointer started, so a second step of one element
from the position one past is not caught. Subtraction of two such
pointers counts in units of `T`. In storage from an allocation
function, a cast to `T*` of a pointer to the byte at the offset of
element `k` of the array of `T` that the storage holds gives a pointer
to element `k` (STG-28, finding G164).

## Allocated storage in constant evaluation

In constant evaluation, the elements of storage from `std::allocator`
begin their lifetimes only when `std::construct_at` builds them, also
for an implicit-lifetime type. An assignment to an element, or to a
member of an element, before that is not a constant expression (LSUP-11,
CE-9, Q-0421). A run outside constant evaluation keeps the implicit
objects of STG-8.

## Exception specifications

The `noexcept` operator (UN-18) follows EXS-4: a function is
potentially throwing unless its declaration says `noexcept`, except
for a deallocation function, a destructor (DTOR-4), and a defaulted or
implicit special member or comparison, which follow the functions that
they call for bases and members. A destructor of a class temporary in
the operand counts. A placement `new` counts when every allocation
function with that number of arguments is non-throwing.

The operand of a member's `noexcept` that names a later member is
parsed again at the end of the class (CLS-2, EXS-1). When the operand
names the function itself (EXS-6), the function stays potentially
throwing.

The findings about the specification are in
`reports/clean-room/rust-frontend/report.md`, `report-2.md` to
`report-26.md`, and `report-10b.md`.
