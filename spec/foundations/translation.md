# Translation, execution, and behavior classes

A program is a set of translation units that an implementation
translates and links into one executable. This page states what an
implementation must do with a program that breaks a rule, where
execution begins, how constant evaluation relates to execution, and what
each class of rule in this specification requires. The later pages use
these classes without explaining them again.

Depends on: storage duration and linkage, constant expressions, function
definitions.

## Principles

:::{principle} TR-P1
:rules: TR-1 TR-6

A rule in this specification is one of two kinds. A *rule on the
program* says what a valid program must or must not do. If a program
breaks one, the program is not valid, and an implementation that
translates it must say so with a diagnostic, whether or not the program
would ever reach the broken part. A *fact about the implementation* says
what every implementation does with a valid program. The class of each
rule says which kind it is and how strong it is:

| Class | Kind | Meaning |
|---|---|---|
| `reject` | rule on the program | The implementation must diagnose a program that breaks it. |
| `no-diagnostic` | rule on the program | The program is not valid, and an implementation need not notice. |
| `required` | fact | Every implementation behaves this way. |
| `undefined` | rule on the program | This specification sets no requirement on what happens. |
| `impl-defined` | fact | Each implementation chooses and documents its behavior. |
| `unspecified` | fact | Each implementation chooses from the given set, and need not document or be consistent. |
| `recommended` | advice | An implementation should do this, usually by warning. |

A diagnostic for a `recommended` rule is a warning. It does not make
the program invalid, and a program can ask the implementation to treat
it as an error.
:::

:::{principle} TR-P2
:rules: TR-4

Each translation unit is translated on its own, with only the
declarations it contains, and the units are then linked into one
program. So a name that one unit uses and another defines needs a
declaration in the first unit, a name with internal linkage is a
different entity in each unit, and a name that is used but defined
nowhere is found out only when the program is linked.
:::

:::{principle} TR-P3
:rules: TR-2 TR-3

Execution begins at `main`. The program's exit status is the value that
`main` returns, and reaching the end of `main` returns zero. `main` is
the entry point and nothing else: a program cannot call it, take its
address, or give it a form other than the two that the runtime knows how
to call.
:::

:::{principle} TR-P4
:rules: TR-5

Constant evaluation runs the same functions as execution, with the
same meaning, at translation time. It differs in two ways. It happens
only where a constant is needed, and nowhere else. And it tracks every
object and every operation, so an evaluation that would be undefined at
run time is not a constant, and the program is rejected there. A program
can ask which mode it is in and act differently in each.
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

`main` must return `int`, must not be a template, and must have no
parameters or the two parameters `int` and `char**`
({question}`Q-0112`). A program must not call `main` or take its
address. A program with no `main` does not link.
:::

## Dynamic Semantics

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
:tests: foundations/translation/units-translated-separately.cpp foundations/translation/unit-sees-own-declarations-only.cpp foundations/translation/undefined-at-link.cpp

Each translation unit sees only its own declarations. A function with
internal linkage is a separate function in each unit that defines it. A
function that is declared and used but defined in no unit makes the
program fail to link.
:::

:::{rule} TR-5
:kind: required
:tests: foundations/translation/constant-evaluation-same-function.cpp foundations/translation/constant-evaluation-rejects-ub.cpp foundations/translation/constant-evaluation-only-where-needed.cpp foundations/translation/if-consteval-selects.cpp

A `constexpr` function gives the same result in constant evaluation and
at run time. Constant evaluation occurs only where a constant is
needed, such as a `static_assert`. An operation that would be undefined,
such as division by zero, makes the evaluation not constant, and the
program is rejected there, while the same call at run time with a
value that is never zero is valid. `if consteval` selects its first
branch in constant evaluation and its second at run time.
:::

## Recommended Practice

:::{rule} TR-6
:kind: recommended
:tests: foundations/translation/warning-not-error.cpp foundations/translation/warning-as-error-with-flag.cpp

A diagnostic for a `recommended` rule is a warning. It does not stop
translation. An implementation should give a way to turn warnings into
errors.
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

The results are in `tests/RESULTS.md`. All seven compilers agree on 20 of
21 tests. EDG runs and links no programs, so its results for the run and
link tests say only that the programs are valid.

- TR-3: EDG accepts `main` with a first parameter of type `long`. Clang
  and GCC reject it ({question}`Q-0112`).

Inputs: clang-sema-decl, clang-diag-sema, edg-decls.
