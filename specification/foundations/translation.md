# Translation, execution, and behavior classes

A program is a set of translation units that an implementation
translates and links into one executable. This page states the
following:

- what an implementation must do with a program that breaks a rule,
- where execution begins,
- how constant evaluation relates to execution,
- what limits each implementation sets on translation,
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
:rules: TR-1 TR-6 TR-12 TR-17

A rule in this specification is a *rule on the program* or a *fact about
the implementation*. A rule on the program says what a valid program
must or must not do. If a program breaks a `reject` rule, the program
is not valid, and an implementation that translates the program must
say so with a diagnostic, whether or not the program would ever reach
the broken part. A `no-diagnostic` rule makes the program not valid
with no diagnostic needed. An `undefined` rule constrains only a run
that reaches the operation, so a program whose undefined operation
never runs is valid. A fact about the implementation says what every
implementation does with a valid program. The class of each rule says
which kind it is and how strong it is:

| Class | Kind | Meaning |
|---|---|---|
| `reject` | rule on the program | The implementation must diagnose a program that breaks the rule. The grammar of the `Syntax` segments is such a rule (TR-12). |
| `no-diagnostic` | rule on the program | The program is not valid, and an implementation need not notice. |
| `required` | fact | Every implementation behaves this way. |
| `undefined` | rule on the program | This specification sets no requirement on what happens. |
| `impl-defined` | fact | Each implementation chooses and documents its behavior. |
| `unspecified` | fact | Each implementation chooses from the given set, and need not document or be consistent. |
| `conditionally-supported` | fact | An implementation can reject the construct with a diagnostic, and it documents the constructs that it does not support. |
| `recommended` | advice | An implementation should follow the rule, usually by warning. |

One requirement needs a diagnostic when the units are linked: a program
defines exactly one `main` ({rule}`TR-17`). Every other rule whose break can show
only when the units are linked, such as {rule}`LNK-7` or {rule}`ODR-8`,
is a `no-diagnostic` rule.

A diagnostic for a `recommended` rule is a warning. The warning
leaves the program valid, and a program can ask the implementation
to treat the warning as an error.

A `reject` rule asks for at least one diagnostic. After the diagnostic,
an implementation can go on and translate the program, for example as
an extension. The tests run each compiler in its strict mode, where the
diagnostic stops the translation.
:::

:::{principle} TR-P2
:rules: TR-4

Each {term}`translation unit` is translated on its own, with only the
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
- A name with internal {term}`linkage` is a different entity in each unit.
- A name that is used but defined nowhere is found out only when the
  program is linked.

Units share declarations through {term}`header` files. Each unit
includes the headers it needs, and an {term}`include guard` keeps a
header from entering one unit twice ({principle}`ODR-P3`).
:::

:::{principle} TR-P3
:rules: TR-2 TR-3 TR-9 TR-11

Execution begins at `main`. The program's exit status is the value that
`main` returns, and reaching the end of `main` returns zero. `main` is
the entry point and nothing else, so a program cannot call `main` or
take its address. Every implementation accepts the two forms of `main`
that the runtime knows how to call. Each implementation chooses whether
to accept other forms ({rule}`FN-6`). The name `main` at global scope is
kept for the entry point.

The environment that receives the status can keep only part of the
value. On the matrix targets the environment keeps the low eight bits.
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
free to reorder them. {term}`Constant evaluation <constant evaluation>` runs the abstract machine
exactly, so constant evaluation stops at that point and the program is
rejected, even when the value that the undefined step produced is never
used.
:::

:::{principle} TR-P4
:rules: TR-5 TR-10 TR-16

Constant evaluation runs the same functions as execution, with the
same meaning, at translation time. Constant evaluation differs from
execution in the following ways:

- It happens where a constant is needed. It is also tried on the
  initializer of each static or `thread_local` variable, and if that
  try fails, the variable is initialized at run time ({rule}`SINIT-1`).
- It tracks every object and every operation, so an evaluation that
  would be undefined at run time is not a constant, and the program is
  rejected there.
- A floating-point operation can give a result that differs from the
  result at run time (TR-16).

A program can ask which mode it is in and act differently in each.
:::

:::{principle} TR-P6
:rules: TR-13 TR-14 TR-15

An implementation translates a program on a machine with finite
memory, so translation has limits. Each implementation sets its own
limits and documents them. This specification sets no minimum for any
limit. If a program goes past a limit, the implementation rejects the
program. Tidal asks for this rejection so that a program never runs
with a wrong size. The matrix compilers reject, apart from the class
sizes of {question}`Q-0350`. The limits are
of these kinds:

| Limit | Rule |
|---|---|
| the depth of nested instantiations | TR-13 |
| the size of an array type | TR-14 |
| the size of a class type | TR-15 |
| the depth of recursion and the number of steps in one constant evaluation | {rule}`CE-10` |

Each configuration of a compiler is an implementation. So a compiler
option that changes a limit gives an implementation with a different
limit. A program that needs a high limit is portable only to the
implementations whose limits are high enough.
:::

## Constraints

:::{rule} TR-1
:kind: reject
:tests: foundations/translation/ill-formed-diagnosed.cpp foundations/translation/ill-formed-in-unused-function.cpp

An implementation issues at least one diagnostic for a program that
breaks a `reject` rule. This rule applies also when the broken code is
in a function that the program never calls. In the strict mode of each
matrix compiler, the diagnostic stops the translation.
:::

:::{rule} TR-3
:kind: reject
:tests: foundations/translation/main-return-type-not-int.cpp foundations/translation/main-called.cpp foundations/translation/main-address.cpp foundations/translation/main-template.cpp foundations/translation/main-wrong-parameter-types.cpp foundations/translation/main-inline.cpp foundations/translation/main-static.cpp foundations/translation/main-constexpr.cpp foundations/translation/main-consteval.cpp foundations/translation/main-deleted.cpp foundations/translation/main-c-linkage.cpp foundations/translation/main-c-linkage-in-namespace.cpp foundations/translation/main-c-linkage-variable-in-namespace.cpp foundations/translation/main-variable.cpp foundations/translation/main-deduced-return-type.cpp foundations/translation/main-trailing-return-int.cpp foundations/translation/main-in-namespace.cpp foundations/translation/main-cpp-linkage-specification.cpp foundations/translation/main-cpp-linkage-block.cpp

These rules apply to `main` at global scope:

- The declared return type of `main` must be `int`. A placeholder type
  that `return` deduces is not valid.
- `main` must not be a template, or be `inline`, `static`, `constexpr`,
  `consteval`, or deleted.
- `main` must not have C language linkage
  ({question}`Q-0366`).
- The parameter list of `main` must be one that the implementation
  accepts (FN-6).
- A program must not call `main` or take its address.

A variable at global scope must not be named `main`. No variable or
function in any namespace that has C language linkage can be named
`main`.
:::

:::{rule} TR-10
:kind: reject
:tests: foundations/translation/constant-evaluation-rejects-ub.cpp foundations/translation/constant-evaluation-rejects-overflow.cpp

An operation that would be undefined, such as division by zero, makes a
constant evaluation not constant (TR-5). The implementation rejects the
program there.
:::

:::{rule} TR-12
:kind: reject
:tests: foundations/translation/grammar-for-constexpr.cpp foundations/translation/grammar-else-without-if.cpp foundations/translation/grammar-mismatched-bracket.cpp foundations/translation/grammar-missing-operand.cpp exceptions/handlers/param-default-argument.cpp exceptions/handlers/param-storage-class.cpp declarations/enumerations/scoped-unnamed.cpp

The tokens of each translation unit must form a sequence of zero or
more `declaration` that the grammar derives. The grammar is the set of
productions in the `Syntax` segments. A diagnostic for a token sequence
that the grammar does not derive need not name a rule other than TR-12.
:::

:::{rule} TR-17
:kind: reject
:tests: foundations/translation/main-no-definition.cpp foundations/translation/main-two-definitions.cpp

A program defines exactly one function `main` at global scope. A
program with no `main`, or with a definition of `main` in each of two
units, is not valid. The implementation can give the diagnostic when it
links the units, and every baseline compiler does.
:::

## Runtime Behavior

:::{rule} TR-2
:kind: required
:tests: foundations/translation/main-exit-status.cpp foundations/translation/main-nonzero-status.cpp foundations/translation/main-falls-off-end.cpp foundations/translation/main-with-args.cpp

Execution begins with a call to `main`. The value that `main` returns is
the exit status of the program, and zero means success. If control
reaches the end of `main`, the status is zero. With two parameters,
`int` and `char**` ({rule}`FN-6`), the first is the number of arguments.
The second is an array of that many strings followed by a null pointer.
:::

:::{rule} TR-4
:kind: required
:tests: foundations/translation/units-translated-separately.cpp foundations/translation/unit-sees-own-declarations-only.cpp foundations/translation/undefined-at-link.cpp

Each translation unit sees only its own declarations. A function with
internal linkage is a separate function in each unit that defines it. A
name that only another unit declares cannot be used (NAM-8).

A function that is declared and used but defined in no unit breaks
{rule}`LNK-7`. LNK-7 is a `no-diagnostic` rule ({principle}`TR-P1`).
An implementation can report its break when it links the units, and the
link then fails. The implementation can also accept the program. Every
implementation links a valid program.
:::

:::{rule} TR-5
:kind: required
:tests: foundations/translation/constant-evaluation-same-function.cpp foundations/translation/constant-evaluation-only-where-needed.cpp foundations/translation/if-consteval-selects.cpp foundations/translation/constant-initializer-tried.cpp

A `constexpr` function gives the same result in constant evaluation and
at run time, apart from floating-point results (TR-16). Constant
evaluation occurs where a constant is needed, such as a `static_assert`,
and in a try of a static initializer ({rule}`SINIT-1`). A call that TR-10
rejects in constant evaluation is valid at run time if it reaches no
undefined operation.
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
:tests: foundations/translation/undefined-taints-whole-evaluation.cpp foundations/translation/undefined-on-taken-path-only.cpp foundations/translation/undefined-on-taken-path-sanitizer.cpp

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

:::{rule} TR-11
:kind: impl-defined
:tests: foundations/translation/main-status-low-bits.cpp foundations/translation/main-status-low-bits-nonzero.cpp foundations/translation/main-status-negative.cpp

The status that the environment receives from the value of `main`
(TR-2) is implementation-defined. On the matrix targets the environment
receives the value modulo 256, as a number from 0 to 255. A value of 256
gives status 0, and a value of -1 gives status 255.
:::

:::{rule} TR-13
:kind: impl-defined
:tests: foundations/translation/limit-depth-crossed.cpp foundations/translation/limit-depth-crossed-ilp32.cpp foundations/translation/limit-depth-ilp32.cpp foundations/translation/limit-depth-one-template-100.cpp foundations/translation/limit-depth-two-templates-100.cpp foundations/translation/limit-depth-two-templates-1000.cpp foundations/translation/limit-depth-flag-lowered.cpp foundations/translation/limit-depth-edg-flag-lowered.cpp templates/instantiation/recursion-within-limit.cpp templates/instantiation/recursion-function-within-limit.cpp templates/instantiation/recursion-endless.cpp templates/instantiation/recursion-endless-function.cpp

An implementation rejects a program whose {term}`instantiations <instantiation>` nest
deeper than a limit. The limit, and the way that the implementation
counts the depth, are implementation-defined. The matrix compilers
count and limit the depth as follows, on both targets:

| Compiler | What the limit counts | Limit | Option that sets the limit |
|---|---|---|---|
| Clang 22, 18 | the nested instantiations of all templates | 1024 | `-ftemplate-depth=N` |
| GCC 16, 14 | the nested instantiations of all templates | 900 | `-ftemplate-depth=N` |
| EDG 7.0 | the nested instantiations of one template | 64 | `--pending_instantiations=N` |

A chain that alternates between two templates counts its whole length
on Clang and GCC, and the length of each template's part on EDG. A
chain of 100 through `Even` and `Odd` has depth 100 on Clang and GCC
and depth 50 on EDG, so every matrix compiler accepts it. A chain of
1000 has depth 500 on EDG and 1000 on GCC, so both reject it, and
Clang accepts it.

An implementation that does not have an option of the table rejects a
command line that gives the option.
:::

:::{rule} TR-14
:kind: impl-defined
:tests: foundations/translation/limit-array-largest-clang-lp64.cpp foundations/translation/limit-array-exceeded-clang-lp64.cpp foundations/translation/limit-array-largest-gcc-lp64.cpp foundations/translation/limit-array-exceeded-gcc-lp64.cpp foundations/translation/limit-array-largest-edg-lp64.cpp foundations/translation/limit-array-largest-gcc-ilp32.cpp foundations/translation/limit-array-exceeded-gcc-ilp32.cpp foundations/translation/limit-array-largest-ilp32.cpp declarations/declarators/array-too-large.cpp declarations/declarators/array-4gib-lp64.cpp

An implementation rejects an array type whose size in bytes is larger
than a limit. The limit is implementation-defined, can depend on the
target, and is never larger than {rule}`STG-1` allows. The largest
array of the matrix compilers has this size in bytes:

| Compiler | x86-64 Linux | `-m32` |
|---|---|---|
| Clang 22, 18 | 2<sup>61</sup> - 1 | 2<sup>32</sup> - 1 |
| GCC 16, 14 | 2<sup>63</sup> - 1 | 2<sup>31</sup> - 1 |
| EDG 7.0 | 2<sup>64</sup> - 1 | 2<sup>32</sup> - 1 |
:::

:::{rule} TR-15
:kind: impl-defined
:tests: foundations/translation/limit-class-largest-clang-lp64.cpp foundations/translation/limit-class-exceeded-clang-lp64.cpp foundations/translation/limit-class-largest-clang-18-lp64.cpp foundations/translation/limit-class-wrong-size-clang-18-lp64.cpp foundations/translation/limit-class-largest-gcc-lp64.cpp foundations/translation/limit-class-exceeded-gcc-lp64.cpp foundations/translation/limit-class-largest-edg-lp64.cpp foundations/translation/limit-class-largest-gcc-ilp32.cpp foundations/translation/limit-class-exceeded-gcc-ilp32.cpp foundations/translation/limit-class-largest-ilp32.cpp

If the size of a class type is larger than a limit, an implementation
rejects each use that needs the size, such as an object definition or
`sizeof`. The limit is implementation-defined, can depend on the
target, and is never larger than {rule}`STG-1` allows. The largest
class of the matrix compilers has this size in bytes:

| Compiler | x86-64 Linux | `-m32` |
|---|---|---|
| Clang 22 | 2<sup>60</sup> - 1 | no limit (Evidence) |
| Clang 18 | no limit (Evidence) | no limit (Evidence) |
| GCC 16, 14 | 2<sup>63</sup> - 1 | 2<sup>31</sup> - 1 |
| EDG 7.0 | 2<sup>64</sup> - 1 | 2<sup>32</sup> - 1 |
:::

:::{rule} TR-16
:kind: unspecified
:tests: foundations/translation/floating-constant-and-run-either.cpp foundations/translation/floating-sum-constant-and-run-either.cpp

Whether a floating-point operation gives the same result in constant
evaluation and at run time is unspecified. Constant evaluation on the
matrix rounds each operation to its type. At run time {rule}`MUL-17`
applies, so GCC with `-m32` can give another result. An operation
whose exact result the type can represent, such as `2.0 * 3.0` in
`double`, needs no rounding, and the matrix gives that result in both
modes.
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
  return helper();            // not valid if no unit defines helper (LNK-7)
}

constexpr int half(int v) { return v / 2; }
static_assert(half(8) == 4);  // constant evaluation
int at_run_time = half(9);    // the same function, at run time
```

```cpp
template <int N> struct Odd;
template <int N> struct Even { static constexpr int value = Odd<N - 1>::value + 1; };
template <int N> struct Odd { static constexpr int value = Even<N - 1>::value + 1; };
template <> struct Even<0> { static constexpr int value = 0; };

// Depth 100 on Clang and GCC, 50 of each template on EDG: accepted (TR-13).
static_assert(Even<100>::value == 100);
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 65
of 89 tests. EDG takes no optimization flag, so the three `-O2` tests
are skipped there. EDG runs and links no programs, so its results for
the run and link tests say only that the programs are valid.

### Divergences

- {question}`Q-0012` (TR-14, TR-15; settled on TR-P6): whether Tidal
  sets a minimum for the largest array or class. It sets none, and each
  limit is implementation-defined. No family differs.
- {question}`Q-0024` (TR-3; settled on TR-P3): which parameter lists
  `main` may have. `()` and `(int, char**)` everywhere, and others by
  the implementation's choice. EDG accepts a first parameter of type
  `long`, which the answer allows, so no family differs.
- {question}`Q-0029` (CMP-14; settled on CMP-P6): whether a chained
  comparison is valid. It is, and a warning is recommended. Clang 22
  rejects some forms with no flag, and Clang 18 gives no warning.
- {question}`Q-0312` (TR-12; open): whether the specification lists
  token sequences that no production accepts. It does not, and TR-12
  gives the grammar its effect. No family differs, and the question
  waits on the maintainer.
- {question}`Q-0350` (TR-15; open): whether Clang conforms when it
  accepts a class whose size it cannot represent. It does not. Clang 22
  and 18 with `-m32`, and Clang 18 on x86-64, accept such a class and
  give `sizeof` a wrong value.
- {question}`Q-0351` (TR-13; settled on TR-P6): how the depth of nested
  instantiations is counted, and whether it must reach a minimum. Both
  are implementation-defined, with no minimum. No family differs.
- {question}`Q-0366` (TR-3; open): whether `main` can be declared with
  `extern "C++"`. It can, as Clang 22 and 18 and GCC 16 and 14 say.
  EDG rejects it,
  and the gap review of the rules that phase 7 added reads the
  reference as EDG does (`documentation/decisions/pending.md`, row 4).

### Split tests

Each row is a test that carries an override for a baseline compiler. The
Expected column gives the outcome that the rules predict, and each
family column gives the outcome that the test expects on that family.
EDG runs no programs, so its cell for a run test gives the result of the
compile.

| Test | Rules | Expected | Clang | GCC | EDG | Question |
|---|---|---|---|---|---|---|
| `declarations/declarators/array-too-large.cpp` | TR-14 | accept | reject | accept | accept | {question}`Q-0012` |
| `limit-array-exceeded-clang-lp64.cpp` | TR-14 | accept | reject | accept | accept | {question}`Q-0012` |
| `limit-array-exceeded-gcc-ilp32.cpp` | TR-14 | accept | accept | reject | accept | {question}`Q-0012` |
| `limit-array-exceeded-gcc-lp64.cpp` | TR-14 | reject | reject | reject | accept | {question}`Q-0012` |
| `limit-array-largest-edg-lp64.cpp` | TR-14 | reject | reject | reject | accept | {question}`Q-0012` |
| `limit-array-largest-gcc-lp64.cpp` | TR-14 | accept | reject | accept | accept | {question}`Q-0012` |
| `limit-array-largest-ilp32.cpp` | TR-14 | accept | accept | reject | accept | {question}`Q-0012` |
| `limit-class-exceeded-clang-lp64.cpp` | TR-15 | accept | Clang 22: reject; Clang 18: accept | accept | accept | {question}`Q-0012` |
| `limit-class-exceeded-gcc-ilp32.cpp` | TR-15 | accept | accept | reject | accept | {question}`Q-0012` |
| `limit-class-exceeded-gcc-lp64.cpp` | TR-15 | reject | Clang 22: reject; Clang 18: accept | reject | accept | {question}`Q-0012`, {question}`Q-0350` |
| `limit-class-largest-clang-18-lp64.cpp` | TR-15 | accept | Clang 22: reject; Clang 18: accept | accept | accept | {question}`Q-0012` |
| `limit-class-largest-edg-lp64.cpp` | TR-15 | reject | reject | reject | accept | {question}`Q-0012`, {question}`Q-0350` |
| `limit-class-largest-gcc-lp64.cpp` | TR-15 | accept | reject | accept | accept | {question}`Q-0012`, {question}`Q-0350` |
| `limit-class-largest-ilp32.cpp` | TR-15 | accept | accept | reject | accept | {question}`Q-0012` |
| `limit-class-wrong-size-clang-18-lp64.cpp` | TR-15 | accept | reject | accept | accept | {question}`Q-0350` |
| `limit-depth-one-template-100.cpp` | TR-13 | accept | accept | accept | reject | {question}`Q-0351` |
| `limit-depth-two-templates-1000.cpp` | TR-13 | reject | accept | reject | reject | {question}`Q-0351` |
| `main-c-linkage-in-namespace.cpp` | TR-3 | reject | Clang 22: reject; Clang 18: accept | GCC 16: reject; GCC 14: accept | reject | |
| `main-c-linkage-variable-in-namespace.cpp` | TR-3 | reject | Clang 22: reject; Clang 18: accept | GCC 16: reject; GCC 14: accept | reject | |
| `main-c-linkage.cpp` | TR-3 | reject | Clang 22: reject; Clang 18: accept | GCC 16: reject; GCC 14: accept | reject | |
| `main-cpp-linkage-block.cpp` | TR-3 | run | run | run | reject | {question}`Q-0366` |
| `main-cpp-linkage-specification.cpp` | TR-3 | run | run | run | reject | {question}`Q-0366` |
| `main-deleted.cpp` | TR-3 | reject | reject | GCC 16: reject; GCC 14: accept | reject | |
| `main-wrong-parameter-types.cpp` | TR-3, FN-6 | reject | reject | reject | accept | {question}`Q-0024` |

### Notes

- TR-17: the gap review of the text phase 8 wrote on expressions, foundations,
  and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) doubts the class of a
  program with no `main`. The reference waives the diagnostic for a
  used function with no definition, and it states no waiver for the
  rule that a program has one `main`. TR-17 follows the gap review of 2026-11-05 here: a
  program with no `main` needs a diagnostic, which the implementation
  can give when it links the units, and LNK-7 keeps the used function
  with no definition. Every compiler of the matrix that links reports
  both at link time
  (`main-no-definition.cpp`). `main-no-definition.cpp` is the one test with no `main`
  that the matrix links. `tools/run_tests.py` links a test of one file
  only when the test expects `run`, `run-fail`, or `link-fail`, and
  always links a test with `tidal-sources`. So every other test of one
  file with no `main` is only compiled.
- TR-4: 45 tests expect a link failure on at least one compiler. Each
  of these tests cites a `no-diagnostic` rule that its program breaks:
  LNK-6, LNK-7, ODR-8, INL-6, INL-7, VIRT-17, VIRT-20, CLS-21, or
  INST-28. A test can also cite a `required` rule that explains why the
  program breaks the `no-diagnostic` rule, such as a declaration that
  is not a definition. A link failure is one result that these rules
  allow, and the Clang and GCC linkers give it. When a configuration
  gives another allowed result, such as a configuration at `-O2` that
  removes the use, the test records that result with an override.
  `tools/check_specification.py` fails for a test that expects a link
  failure and cites no `no-diagnostic` rule.
- TR-11: on every Clang and GCC configuration, `return 256;` gives
  status 0, `return 257;` gives status 1, and `return -1;` gives status
  255.
- TR-3: Clang 22, GCC 16, and EDG reject each form of `main` in the
  list. GCC 14 accepts a deleted `main`. GCC 14 and Clang 18 accept the
  forms with C language linkage.
- TR-1: Clang 22 reports C language linkage on `main` through a warning
  flag (`-Wmain`), and GCC 16 through `-Wpedantic`. In the strict mode
  of the tests, both diagnostics are errors. Without
  `-pedantic-errors`, both compilers warn and translate the program,
  which TR-P1 allows.
- TR-5: every compiler tries the initializer of a static variable as a
  constant, also when no constant is needed there
  (`constant-initializer-tried.cpp`).
- TR-16: on x86-64 every configuration gives the same floating-point
  result at translation and at run time. The test accepts both results
  of {rule}`MUL-17`. The gap review of the rules that phase 7 added
  reads the reference as recommending that the two results agree.
  Tidal states no `recommended` rule for that. Every configuration that
  the matrix runs gives the same results, so no test could show a
  difference.
- TR-P6, TR-13, TR-14, TR-15: the gap review of program structure reads
  the C++23 text as asking only that an implementation document its
  limits, with no requirement on a program past a limit. Tidal keeps the
  rejection that the compilers give (`documentation/decisions/pending.md`,
  row 4).
- TR-12: every compiler rejects each token sequence of the tests. Each
  message names the token that the parser expected, or says that an
  expression or a statement was expected.
- TR-15: Clang 18 on x86-64 gives the right `sizeof` for a class of up
  to 2<sup>61</sup> - 1 bytes (`limit-class-largest-clang-18-lp64.cpp`).
  From 2<sup>61</sup> bytes on, Clang 18 accepts the class and gives
  `sizeof` a wrong value (`limit-class-wrong-size-clang-18-lp64.cpp`).
  With `-m32`, Clang 22 and 18 give the right `sizeof` for a class of up
  to 2<sup>32</sup> - 1 bytes, the largest value of `std::size_t`
  (`limit-class-largest-ilp32.cpp`). A larger class gets its size
  modulo 2<sup>32</sup> ({question}`Q-0350`).
- TR-13: the flag tests lower each limit with the option of the table.
  The other compilers reject the unknown option. So each flag test
  expects a reject from every implementation, also from an
  implementation outside the table.
- TR-13: on an endless chain of function templates, Clang 22 and 18
  print the depth message and then end with a segmentation fault. Each
  compiler rejects the program with a diagnostic first
  ({rule}`INST-15`).
- TR-9, TR-11 with `-m32`: the matrix runs no program built with
  `-m32`, because the host compilers have no 32-bit C library. In the
  writer's probes, the run tests of the two rules give the same
  results with `-m32` in the GCC 14, Clang 18, and Clang 22 containers.

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

- the chained comparison of CMP-14 with operands of built-in type, a
  fold expression over a relational operator, and a chain in a requires
  expression that depends on no template parameter
  ({question}`Q-0029`),
- the literal operator of the OPR-9 test, which has no space between
  `""` and the suffix (`-Wreserved-user-defined-literal`).

The table lists the warnings for the cases that each rule names. A
compiler can give other warnings on the same code. For example, Clang
22 warns under `-Wall` about a self-move (`-Wself-move`), and the
Evidence of {rule}`CPY-13` records that warning outside the rule.

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
| {rule}`REF-15` | on: `-Wdangling-field` | no warning |
| {rule}`SEQ-7` | on: `-Wunsequenced` | `-Wall`: `-Wsequence-point` |
| {rule}`SHF-9` | on: `-Wshift-count-overflow`, `-Wshift-count-negative` | on: `-Wshift-count-overflow`, `-Wshift-count-negative` |
| {rule}`STMT-4` | on: `-Wunused-value` | `-Wall`: `-Wunused-value` |
| {rule}`SW-7` | on: `-Wswitch` | `-Wall`: `-Wswitch` |
| {rule}`SW-8` | on: `-Wswitch` | `-Wall`: `-Wswitch` |
| {rule}`TRY-4` | on: `-Wexceptions` | on: `-Wexceptions` |
| {rule}`TRY-12` | on: `-Wreturn-type` | no warning |
| {rule}`TRY-13` | on: `-Wexceptions` | no warning |
| {rule}`UN-19` | on: `-Wsizeof-array-argument` | on: `-Wsizeof-array-argument` |
| {rule}`UNEV-3` | on: `-Wunused-comparison` | `-Wall`: `-Wunused-value` |
| {rule}`UNEV-4` | on: `-Wunevaluated-expression` | no warning |
| {rule}`UNW-8` | on: `-Wexceptions` | on: `-Wterminate` |
| {rule}`VIRT-13` | `-Wall`: `-Wdelete-non-abstract-non-virtual-dtor`, `-Wdelete-abstract-non-virtual-dtor`, `-Woverloaded-virtual`; on: `-Winconsistent-missing-override` | `-Wall`: `-Wdelete-non-virtual-dtor`, `-Woverloaded-virtual`; `-Wsuggest-override` only, for every override with no `override` or `final` |
| {rule}`VOL-5` | on: `-Wdeprecated-volatile` | on: `-Wvolatile` |

Inputs: clang-sema-decl, clang-diag-sema, edg-decls.
- TR-17: the gap review of the text phase 12 wrote on classes,
  declarations, and foundations (`reports/review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md`, RV5-A-14) reads the case of two
  definitions of `main` as open in the C++23 text. With external
  linkage, the second definition can fall under the rule for one
  definition of a function instead, which needs no diagnostic. Both
  readings give a failed link on every baseline compiler. TR-17 takes
  the reading with a diagnostic.
