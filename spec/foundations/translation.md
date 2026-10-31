# Translation, execution, and behavior classes

A program is a set of translation units that an implementation
translates and links into one executable. This page states the
following:

- what an implementation must do with a program that breaks a rule,
- where execution begins,
- how constant evaluation relates to execution,
- what each class of rule in this specification requires.

The later pages use these classes without explaining them again.

Depends on: storage duration and linkage, constant expressions, function
definitions.

Readers who know the vocabulary of the ISO standard will find that this
specification says the same things in different words. The words
differ on purpose, for the following reasons:

- The writing follows Simplified Technical English, which is meant for
  readers who are not specialists in the text, and *shall* and *may*
  are the two words that such readers misread most often.
- The strength of each rule is carried by its class, printed in the
  rule's header and checked against the implementations by tests, so
  the verb in the sentence does not have to carry the strength.
- *Ill-formed* and its relatives describe a program from the standard's
  side, while this specification describes what an implementation does
  with the program. The tests of this specification observe what the
  implementation does.

The table below maps one vocabulary to the other. The table is
informative.

| ISO term | Tidal word or class |
|---|---|
| well-formed | valid |
| ill-formed | not valid; a `reject` rule |
| ill-formed, no diagnostic required | a `no-diagnostic` rule |
| undefined behavior | `undefined`; "the behavior is undefined" |
| unspecified behavior | `unspecified`; "it is unspecified if" |
| implementation-defined behavior | `impl-defined`; "is implementation-defined" |
| conditionally-supported | `conditionally-supported` |
| deprecated | a `recommended` rule that asks for a warning |
| shall, shall not | must, must not |
| may | can |
| should | should, in `Recommended Practice` only |
| diagnostic | diagnostic; "rejects" when it stops translation |
| potentially evaluated, odr-use | evaluated operand, use ({rule}`ODR-3`) |

## Principles

:::{principle} TR-P1
:rules: TR-1 TR-6

A rule in this specification is a *rule on the program* or a *fact about
the implementation*. A rule on the program says what a valid program
must or must not do. If a program breaks such a rule, the program is not
valid, and an implementation that translates the program must say so
with a diagnostic, whether or not the program would ever reach the
broken part. A fact about the implementation says what every
implementation does with a valid program. The class of each rule says
which kind it is and how strong it is:

| Class | Kind | Meaning |
|---|---|---|
| `reject` | rule on the program | The implementation must diagnose a program that breaks the rule. |
| `no-diagnostic` | rule on the program | The program is not valid, and an implementation need not notice. |
| `required` | fact | Every implementation behaves this way. |
| `undefined` | rule on the program | This specification sets no requirement on what happens. |
| `impl-defined` | fact | Each implementation chooses and documents its behavior. |
| `unspecified` | fact | Each implementation chooses from the given set, and need not document or be consistent. |
| `conditionally-supported` | fact | An implementation can reject the construct. If the implementation accepts the construct, it documents the behavior. |
| `recommended` | advice | An implementation should follow the rule, usually by warning. |

A diagnostic for a `recommended` rule is a warning. The warning does
not make the program invalid, and a program can ask the implementation
to treat the warning as an error.
:::

:::{principle} TR-P2
:rules: TR-4

Each translation unit is translated on its own, with only the
declarations it contains, and the units are then linked into one
program:

```text
unit 1 --> translated with its own declarations --+
                                                  |
unit 2 --> translated with its own declarations --+--> linked into
                                                  |    one program
unit 3 --> translated with its own declarations --+
```

So the following facts hold:

- A name that one unit uses and another defines needs a declaration in
  the first unit.
- A name with internal linkage is a different entity in each unit.
- A name that is used but defined nowhere is found out only when the
  program is linked.
:::

:::{principle} TR-P3
:rules: TR-2 TR-3 TR-9

Execution begins at `main`. The program's exit status is the value that
`main` returns, and reaching the end of `main` returns zero. `main` is
the entry point and nothing else. A program cannot do the following:

- call `main`,
- take the address of `main`,
- give `main` a form other than the two that the runtime knows how to
  call.
:::

:::{principle} TR-P5
:rules: TR-7 TR-8

This specification describes an *abstract machine*: what each
construct does, in what order, with what effect on objects. An
implementation need only produce the same {term}`observable behavior`
as that machine. The observable behavior consists of the following:

- the reads and writes of `volatile` objects in their order,
- the data the program writes out,
- the exit status.

Everything else is invisible, so an implementation can reorder, merge,
or drop work that the observable behavior does not depend on.
Optimization uses that freedom. The rules about evaluation order
({principle}`SEQ-P2`) and copies ({principle}`VC-P4`) leave choices open
for the same reason.

Undefined behavior is the point where the abstract machine has no next
step. From there this specification requires nothing of the execution,
including the parts that came before, because the implementation was
free to reorder them. Constant evaluation runs the abstract machine
exactly, so constant evaluation stops at that point and the program is
rejected, even when the value that the undefined step produced is never
used.
:::

:::{principle} TR-P4
:rules: TR-5 TR-10

Constant evaluation runs the same functions as execution, with the
same meaning, at translation time. Constant evaluation differs from
execution in the following ways:

- It happens only where a constant is needed, and nowhere else.
- It tracks every object and every operation, so an evaluation that
  would be undefined at run time is not a constant, and the program is
  rejected there.

A program can ask which mode it is in and act differently in each.
:::

## Constraints

:::{rule} TR-1
:kind: reject
:tests: foundations/translation/ill-formed-diagnosed.cpp foundations/translation/ill-formed-in-unused-function.cpp

An implementation rejects a program that breaks a `reject` rule, also
when the broken code is in a function that the program never calls.
:::

:::{rule} TR-3
:kind: reject
:tests: foundations/translation/main-return-type-not-int.cpp foundations/translation/main-called.cpp foundations/translation/main-address.cpp foundations/translation/main-no-definition.cpp foundations/translation/main-template.cpp foundations/translation/main-wrong-parameter-types.cpp

These rules apply to `main`:

- `main` must return `int`.
- `main` must not be a template.
- The parameter list of `main` must be one that the implementation
  accepts (FN-6).
- A program must not call `main` or take its address.
:::

:::{rule} TR-10
:kind: reject
:tests: foundations/translation/constant-evaluation-rejects-ub.cpp

An operation that would be undefined, such as division by zero, makes a
constant evaluation not constant (TR-5). The implementation rejects the
program there.
:::

## Runtime Behavior

:::{rule} TR-2
:kind: required
:tests: foundations/translation/main-exit-status.cpp foundations/translation/main-nonzero-status.cpp foundations/translation/main-falls-off-end.cpp foundations/translation/main-with-args.cpp

Execution begins with a call to `main`. The value that `main` returns is
the exit status of the program, and zero means success. If control
reaches the end of `main`, the status is zero. With two parameters, the
first is the number of arguments and the second is an array of that
many strings followed by a null pointer.
:::

:::{rule} TR-4
:kind: required
:tests: foundations/translation/units-translated-separately.cpp foundations/translation/unit-sees-own-declarations-only.cpp foundations/translation/undefined-at-link.cpp foundations/translation/main-no-definition.cpp

Each translation unit sees only its own declarations. A function with
internal linkage is a separate function in each unit that defines it. A
function that is declared and used but defined in no unit makes the
program fail to link. This rule includes `main`, which the program's
start uses, so a program with no `main` fails to link. A name that only
another unit declares cannot be used (NAM-8).
:::

:::{rule} TR-5
:kind: required
:tests: foundations/translation/constant-evaluation-same-function.cpp foundations/translation/constant-evaluation-only-where-needed.cpp foundations/translation/if-consteval-selects.cpp

A `constexpr` function gives the same result in constant evaluation and
at run time. Constant evaluation occurs only where a constant is needed,
such as a `static_assert`. A call that TR-10 rejects in constant
evaluation is valid at run time if it reaches no undefined operation.
`if consteval` selects its first branch in constant evaluation and its
second at run time. A `constexpr` function that never gives a constant
is still valid (CX-3), as long as no constant evaluation reaches the
function.
:::

:::{rule} TR-7
:kind: required
:tests: foundations/translation/observable-volatile-kept-optimized.cpp foundations/translation/volatile-order-kept-optimized.cpp foundations/translation/exit-status-kept-optimized.cpp

An implementation produces the observable behavior of the abstract
machine at every optimization level. Each read and write of a
`volatile` object happens, in the order of the program, and the exit
status is the value that `main` returns. Work whose result the
observable behavior does not depend on can be removed.
:::

## Undefined Behavior

:::{rule} TR-8
:kind: undefined
:tests: foundations/translation/undefined-taints-whole-evaluation.cpp foundations/translation/undefined-on-taken-path-only.cpp

If an execution reaches an operation that has undefined behavior, this
specification sets no requirement on the whole execution, before or
after that point. Constant evaluation rejects such an evaluation, also
when the result of the operation is not used. An operation on a path
that the evaluation does not take has no effect on the evaluation.
:::

## Implementation Requirements

:::{rule} TR-9
:kind: impl-defined
:tests: foundations/translation/main-with-args.cpp foundations/translation/main-program-name.cpp

The environment that starts a program supplies the values of `main`'s
two parameters. On the matrix targets, the first parameter is at least
1, and the first string holds the name by which the program was started.
:::

## Recommended Practice

:::{rule} TR-6
:kind: recommended
:tests: foundations/translation/warning-not-error.cpp foundations/translation/warning-as-error-with-flag.cpp

A diagnostic for a `recommended` rule is a warning. The warning does not
stop translation. An implementation should give a way to turn warnings
into errors.
:::

## Examples

```cpp
int helper();                 // declared here, defined in another unit

int main(int argc, char** argv) {
  if (argc > 2) return 2;     // exit status 2
  return helper();            // fails to link if no unit defines helper
}

constexpr int half(int v) { return v / 2; }
static_assert(half(8) == 4);  // constant evaluation
int at_run_time = half(9);    // the same function, at run time
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 26
of 27 tests. EDG takes no optimization flag, so the three `-O2` tests
are skipped there. EDG runs and links no programs, so its results for
the run and link tests say only that the programs are valid.

- TR-3: EDG accepts `main` with a first parameter of type `long`. Clang
  and GCC reject it ({question}`Q-0112`).

The table below gives, for each `recommended` rule that asks for a
warning, the flag that turns the warning on in Clang 22 and GCC 16. The
results come from the rule's tests, compiled with no flag, with
`-Wall`, and with `-Wextra`. A cell has these forms:

- "on" means that the warning is on with no flag.
- "`-Wall`" or "`-Wextra`" names the group that turns the warning on.
- The flag after the colon is the warning's own flag. The compiler
  prints this flag with the warning, and the flag alone also turns the
  warning on.

`-Werror` makes each warning an error. Older versions can differ. For
example, Clang 18 gives no warning for CMP-14 ({question}`Q-0029`).

Every test passes `-pedantic-errors`. On both Clang 22 and GCC 16, that
flag makes the warnings of two rules errors, DCMP-7 and ODR-11. The
table says "an error under `-pedantic-errors`" in both rows. On these
two compilers, a test of the two rules fails to compile with no
`-Werror`. The other
warnings in the table stay warnings under `-pedantic-errors`. Clang 22
reports two further cases as errors with no flag at all:

- the chained comparison of CMP-14,
- the literal operator of the OPR-9 test, which has no space between
  `""` and the suffix (`-Wreserved-user-defined-literal`).

A test of a warning passes `-Werror` and a flag that turns the warning
on in both compilers, so the two columns give one result. The flag is
the warning's own flag, or a group that holds the warning in both
columns. For SEQ-7 the tests pass `-Wall`, because GCC needs `-Wall`
and Clang warns with no flag.

| Rule | Clang | GCC |
|---|---|---|
| {rule}`ADD-16` | on: `-Wdeprecated-enum-enum-conversion` | on: `-Wdeprecated-enum-enum-conversion` |
| {rule}`ATTR-4` | on: `-Wdeprecated-declarations` | on: `-Wdeprecated-declarations` |
| {rule}`ATTR-5` | on: `-Wunused-result`, `-Wunused-value` | on: `-Wunused-result` |
| {rule}`ATTR-6` | `-Wall` or `-Wunused`: `-Wunused-variable`, `-Wunused-but-set-variable` | `-Wall` or `-Wunused`: `-Wunused-variable`, `-Wunused-but-set-variable` |
| {rule}`ATTR-7` | `-Wimplicit-fallthrough` only | `-Wextra`: `-Wimplicit-fallthrough` |
| {rule}`ATTR-14` | on: `-Winvalid-noreturn` | on, with no flag name |
| {rule}`BF-8` | on: `-Wbitfield-constant-conversion`, `-Wbitfield-width` | on: `-Woverflow`; the width warning has no flag name |
| {rule}`BIT-6` | on: `-Wparentheses` | `-Wall`: `-Wparentheses` |
| {rule}`BIT-7` | `-Wall`: `-Wbool-operation` | `-Wall`: `-Wbool-operation` |
| {rule}`BIT-8` | on: `-Wdeprecated-enum-enum-conversion` | on: `-Wdeprecated-enum-enum-conversion` |
| {rule}`CMP-4` | `-Wextra`: `-Wsign-compare` | `-Wall`: `-Wsign-compare` |
| {rule}`CMP-14` | on: `-Wparentheses`, as an error | `-Wall`: `-Wparentheses` |
| {rule}`CMP-16` | on: `-Wdeprecated-array-compare` | on: `-Warray-compare` |
| {rule}`CMP-21` | on: `-Wdeprecated-enum-compare`, `-Wdeprecated-enum-float-conversion` | on: `-Wenum-compare`, `-Wdeprecated-enum-float-conversion` |
| {rule}`COMMA-5` | on: `-Wunused-value` | `-Wall`: `-Wunused-value` |
| {rule}`CPY-13` | `-Wextra`: `-Wdeprecated-copy-with-user-provided-copy`; `-Wall`: `-Wpessimizing-move` | `-Wextra`: `-Wdeprecated-copy`; `-Wall`: `-Wpessimizing-move` |
| {rule}`CTOR-13` | `-Wall`: `-Wreorder-ctor` | `-Wall`: `-Wreorder` |
| {rule}`CV-9` | on: `-Wdeprecated-volatile` | on: `-Wvolatile` |
| {rule}`CVF-6` | on: `-Wclass-conversion` | on: `-Wclass-conversion` |
| {rule}`DCMP-7` | on: `-Wambiguous-reversed-operator`, an error under `-pedantic-errors` | on, with no flag name, an error under `-pedantic-errors` |
| {rule}`FN-7` | on: `-Wreturn-type` | on: `-Wreturn-type` |
| {rule}`LAM-21` | on: `-Wdeprecated-this-capture` | on: `-Wdeprecated` |
| {rule}`LOG-8` | `-Wall`: `-Wlogical-op-parentheses` | `-Wall`: `-Wparentheses` |
| {rule}`LOG-9` | on: `-Wlogical-not-parentheses` | `-Wall`: `-Wlogical-not-parentheses` |
| {rule}`MUL-16` | on: `-Wdeprecated-enum-float-conversion` | on: `-Wdeprecated-enum-float-conversion` |
| {rule}`ODR-11` | on: `-Wundefined-internal`, an error under `-pedantic-errors` | on, with no flag name, an error under `-pedantic-errors`; `-Wall` or `-Wunused-function` replaces the warning with a `-Wunused-function` warning, which stays a warning |
| {rule}`OPR-9` | on: `-Wuser-defined-literals`; with no space after `""`, also an error with no flag: `-Wreserved-user-defined-literal` | on: `-Wliteral-suffix` |
| {rule}`PRIM-13` | on: `-Wmultichar`, `-Wliteral-range` | on: `-Wmultichar`, `-Woverflow` |
| {rule}`SEQ-7` | on: `-Wunsequenced` | `-Wall`: `-Wsequence-point` |
| {rule}`SHF-9` | on: `-Wshift-count-overflow`, `-Wshift-count-negative` | on: `-Wshift-count-overflow`, `-Wshift-count-negative` |
| {rule}`STMT-4` | on: `-Wunused-value` | `-Wall`: `-Wunused-value` |
| {rule}`TRY-4` | on: `-Wexceptions` | on: `-Wexceptions` |
| {rule}`TRY-12` | on: `-Wreturn-type` | no warning |
| {rule}`TRY-13` | on: `-Wexceptions` | no warning |
| {rule}`UN-19` | on: `-Wsizeof-array-argument` | on: `-Wsizeof-array-argument` |
| {rule}`UNEV-3` | on: `-Wunused-comparison` | `-Wall`: `-Wunused-value` |
| {rule}`UNEV-4` | on: `-Wunevaluated-expression` | no warning |
| {rule}`UNW-8` | on: `-Wexceptions` | on: `-Wterminate` |
| {rule}`VIRT-13` | `-Wall`: `-Wdelete-non-abstract-non-virtual-dtor`, `-Woverloaded-virtual` | `-Wall`: `-Wdelete-non-virtual-dtor`, `-Woverloaded-virtual` |
| {rule}`VOL-5` | on: `-Wdeprecated-volatile` | on: `-Wvolatile` |

Inputs: clang-sema-decl, clang-diag-sema, edg-decls.
