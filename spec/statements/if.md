# The if statement

An `if` statement runs one of up to two statements depending on a condition.
Tidal covers three forms:

- the ordinary form, which tests a condition at run time,
- `if constexpr`, which tests a condition during translation and discards the
  branch not taken,
- `if consteval` and `if !consteval`, which test whether the statement is
  being evaluated as part of constant evaluation.

The ordinary form and `if constexpr` may begin with an initializer statement
that runs first and whose names are visible in the rest of the statement.

Depends on: expression statements, declarations *(referenced)*, scopes
*(referenced)*, [contextual conversion to bool](../glossary.md#contextual-conversion-to-bool),
[constant evaluation](../glossary.md#constant-evaluation), templates *(referenced)*.

Inputs: clang-parse-stmt, clang-sema-stmt, clang-sema-expr (see
`inputs/MANIFEST.md`).

## Syntax

    if-stmt     = "if" [ "constexpr" ] "(" [ if-init ] if-cond ")" branch [ "else" branch ]
                | "if" [ "!" ] "consteval" block [ "else" block ]
    if-init     = expression-stmt | simple-declaration | alias-declaration | ";"
    if-cond     = expression | cond-decl
    cond-decl   = decl-specifiers declarator ( "=" expression | braced-init )
    branch      = statement
    block       = compound-statement

`expression` here excludes a top-level comma only where the grammar of
expressions *(referenced)* says so. Each `if-init` alternative ends with its
own `;`.

In the text below, the *then branch* is the first `branch` or `block` and the
*else branch* is the one after `else`.

## Rules

### IF-1 (required)

The ordinary form evaluates its condition exactly once. If the result is
`true`, the then branch runs. Otherwise the else branch runs, or nothing
runs if there is no else branch.

Tests: statements/if/basic-branches.cpp, statements/if/condition-evaluated-once.cpp

### IF-2 (required)

An `else` belongs to the closest `if` before it that doesn't already have
an `else`.

    if (a)
      if (b) x();
      else y();     // runs when a is true and b is false

Tests: statements/if/dangling-else.cpp

### IF-3 (required)

When the condition is an expression, its value is converted to `bool` by
[contextual conversion](../glossary.md#contextual-conversion-to-bool). An
`explicit operator bool` takes part. Any expression is allowed, including an
assignment.

Tests: statements/if/condition-converted-to-bool.cpp, statements/if/condition-assignment.cpp

### IF-4 (reject)

A program is rejected if the condition expression cannot be converted to
`bool` by contextual conversion.

Tests: statements/if/condition-not-convertible.cpp

### IF-5 (required)

When the condition is a declaration (`cond-decl`), it declares one variable
and initializes it. The variable's value, converted to `bool` by contextual
conversion, decides the branch. `auto` and `constexpr` may be used, and the
variable may have any type the conversion accepts, including a pointer to
function.

    if (Widget* w = find(id)) w->show();

Tests: statements/if/condition-declaration.cpp, statements/if/condition-declaration-constexpr.cpp, statements/if/condition-declaration-function-pointer.cpp

### IF-6 (reject)

A program is rejected if a condition declaration

- has no initializer,
- declares more than one name, including a structured binding,
- declares an array,
- defines a class or enumeration in its type, or
- uses `static`, `extern`, or `thread_local`.

Tests: statements/if/condition-declaration-no-init.cpp, statements/if/condition-declaration-two.cpp, statements/if/condition-structured-binding.cpp, statements/if/condition-declaration-array.cpp, statements/if/condition-declaration-class.cpp, statements/if/condition-declaration-static.cpp, statements/if/condition-declaration-extern.cpp, statements/if/condition-declaration-thread-local.cpp

### IF-7 (required)

The ordinary form and `if constexpr` may start with an initializer
(`if-init`) before the condition. The initializer runs before the condition
is evaluated. It may be an expression statement, an empty statement, an alias
declaration (`using T = ...;`), or any declaration allowed as a statement in a
block, including several variables, a `static` variable, a class definition,
or a function declaration.

    if (auto it = m.find(k); it != m.end()) use(it->second);

Tests: statements/if/init-expression.cpp, statements/if/init-declaration.cpp, statements/if/init-alias.cpp, statements/if/init-empty.cpp, statements/if/init-declaration-kinds.cpp, statements/if/constexpr-with-init.cpp

### IF-8 (required)

Each branch has its own scope, even when the branch is a single statement
without braces. A name declared directly as a branch is not visible after the
`if` statement.

Tests: statements/if/substatement-scope.cpp

### IF-9 (required)

A name declared in the initializer or the condition is visible from its
declaration to the end of the whole `if` statement, which includes the else
branch. It is not visible after the `if` statement. An `else if` is a new
`if` statement nested in the else branch, so its names are visible in its own
else branch.

Tests: statements/if/condition-declaration.cpp, statements/if/init-declaration.cpp, statements/if/init-scope-ends.cpp, statements/if/else-if-scope.cpp

### IF-10 (reject)

A program is rejected if it declares a name a second time in the same `if`
statement at any of these places, when the name was already declared in the
initializer or the condition:

- the condition, for a name from the initializer,
- the outermost block of either branch,
- a branch that is a single declaration without braces.

A nested block inside a branch may reuse the name and hides the outer one.

Tests: statements/if/redeclare-in-branch.cpp, statements/if/redeclare-in-else.cpp, statements/if/redeclare-init-in-branch.cpp, statements/if/redeclare-init-and-condition.cpp, statements/if/redeclare-in-else-statement.cpp, statements/if/redeclare-in-nested-block.cpp

### IF-11 (required)

Temporary objects created while evaluating the condition are destroyed
before either branch runs. Variables declared in the initializer or the
condition live until the `if` statement finishes, after whichever branch
ran, and are destroyed in reverse order of their declaration.

    if (Lock l{m}; Entry e = load())   // after the branch: ~e, then ~l
      use(e);

Tests: statements/if/lifetimes.cpp

### IF-12 (required)

In the ordinary form, a `goto` or a `case` or `default` label of an enclosing
`switch` may transfer control into a branch from outside the `if` statement.
The condition is not evaluated in that case. Jumps are still subject to the
general rules about skipping variable initialization *(referenced)*.

Tests: statements/if/goto-into-branch.cpp, statements/if/case-into-branch.cpp

### IF-13 (required)

In `if constexpr`, the condition is evaluated during translation. The branch
that is not selected is a [discarded statement](../glossary.md#discarded-statement).
Inside a template, a discarded statement is not instantiated, so it may
contain code that would be invalid for the template arguments in use.

    template <class T> int size_of(const T& t) {
      if constexpr (requires { t.size(); }) return t.size();
      else return 1;          // fine for T = int: the first branch is discarded
    }

Tests: statements/if/constexpr-discard-in-template.cpp

### IF-14 (required)

`return` statements in a discarded statement do not take part in deducing a
function's return type. This applies in templates and in ordinary functions.

Tests: statements/if/constexpr-discard-return-type.cpp, statements/if/constexpr-non-template-return-type.cpp

### IF-15 (required)

Outside a template, a discarded statement must still be valid code and is
checked like any other. A function or variable that is used only in a
discarded statement does not need a definition anywhere in the program.

Tests: statements/if/constexpr-non-template-checked.cpp, statements/if/constexpr-non-template-no-definition.cpp

### IF-16 (reject)

A program is rejected if the condition of `if constexpr` is not a constant
expression after contextual conversion to `bool`. Conversions that lose
information are allowed here, so `if constexpr (2)` and a pointer to a
variable with static storage duration are both valid conditions, as is a
class with a `constexpr explicit operator bool`.

Tests: statements/if/constexpr-condition-not-constant.cpp, statements/if/constexpr-condition-narrowing.cpp, statements/if/constexpr-condition-pointer.cpp, statements/if/constexpr-condition-explicit-bool.cpp, statements/if/constexpr-with-init.cpp

### IF-17 (reject)

A program is rejected if a `goto`, or a `case` or `default` label, transfers
control from outside an `if constexpr` or `if consteval` statement into one of
its branches.

Tests: statements/if/constexpr-goto-into.cpp, statements/if/constexpr-case-into.cpp, statements/if/consteval-goto-into.cpp

### IF-18 (required)

`if consteval` runs its then branch when the statement is evaluated as part of
[constant evaluation](../glossary.md#constant-evaluation), and its else branch
(if any) otherwise. `if !consteval` does the reverse. Inside a `consteval`
function every evaluation is constant evaluation, so `if consteval` always
takes the then branch there.

    constexpr int where() {
      if consteval { return 1; } else { return 2; }
    }
    static_assert(where() == 1);   // at run time, where() returns 2

Tests: statements/if/consteval-branches.cpp, statements/if/consteval-in-consteval.cpp

### IF-19 (required)

The branch that runs during constant evaluation (the then branch of
`if consteval`, the else branch of `if !consteval`) is an
[immediate function context](../glossary.md#immediate-function-context). It
may call a `consteval` function with arguments that are not constants. The
other branch is not an immediate function context, and such a call there is
rejected.

Tests: statements/if/consteval-immediate-context.cpp, statements/if/not-consteval-else-immediate.cpp, statements/if/consteval-immediate-outside.cpp

### IF-20 (reject)

A program is rejected if

- a branch of `if consteval` or `if !consteval` is not a compound statement
  (a block in braces),
- `if consteval` is followed by a parenthesized condition, or
- `!` appears after `if` and is followed by anything other than `consteval`.

Tests: statements/if/consteval-requires-braces.cpp, statements/if/consteval-else-requires-braces.cpp, statements/if/consteval-no-condition.cpp, statements/if/not-constexpr.cpp

## Evidence

Every rule in this section is `required` or `reject`. Clang 22.1.8 and GCC
16.2.1 agreed on all 56 tests.

## Divergences

None found.
