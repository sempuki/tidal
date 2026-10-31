# The range-based for statement

A range-based `for` statement executes its body once for each element of
a range. A range is one of these:

- an array,
- a braced list,
- an object of class type with `begin` and `end`.

## Principles

:::{principle} RFOR-P1
:rules: RFOR-1 RFOR-2 RFOR-3 RFOR-4 RFOR-7

A range-based `for` is an ordinary loop that the language writes for
you.

1. It evaluates the range one time and keeps it, so a temporary in the
   range expression lives as long as the loop.
2. It finds `begin` and `end` one time. It finds them as members, if
   the class has both names as members of any kind, and otherwise as
   free functions found through the argument's namespace
   ({principle}`NAM-P3`).
3. It then advances with `++`, stops when the iterator compares equal
   to `end`, and initializes the loop variable from each element.

The loop variable is not yet declared when
the range expression is evaluated, so a name there means the outer
entity.
:::

:::{principle} RFOR-P2
:rules: RFOR-5 RFOR-6

The loop variable is a fresh automatic variable of the body's scope in
each iteration ({principle}`LOOP-P2`). A storage class would contradict
that, and a redeclaration at the top level of the body would give the
name two meanings, so both are rejected.
:::

## Syntax

```{eval-rst}
.. productionlist:: stmt_range_for
   range_for: "for" "(" [`for_init`] `for_range_decl` ":" `for_range_init` ")"
            :    `statement`
   for_init: `expression_stmt` | `simple_declaration` | `alias_declaration`
   for_range_decl: `decl_specifiers` `declarator`
                 : `decl_specifiers` [`ref_qualifier`] "[" `identifier_list` "]"
   for_range_init: `expression` | `braced_init`
```

## Constraints

:::{rule} RFOR-2
:kind: required
:tests: statements/range-for/member-begin-end.cpp statements/range-for/free-begin-end.cpp statements/range-for/sentinel-end.cpp statements/range-for/data-member-named-begin.cpp statements/range-for/equality-only-iterator.cpp statements/range-for/member-begin-only-uses-free.cpp statements/range-for/callable-data-members.cpp

For a range of class type, `begin` and `end` are found as follows:

- If the class has a member named `begin` and a member named `end`, of
  any kind, the loop calls `range.begin()` and `range.end()`. A data
  member counts, so a data member of a callable type works
  ({question}`Q-0098`).
- If only one of the names is a member, or neither, both are free
  functions that argument-dependent lookup finds
  *(lookup referenced)*.

`begin` and `end` can return different types. The loop advances the
iterator with prefix `++`. It compares the iterator with `end` by `!=`,
which can come from `==`.
:::

:::{rule} RFOR-7
:kind: reject
:tests: statements/range-for/not-a-range.cpp statements/range-for/member-begin-only.cpp statements/range-for/incomplete-element.cpp statements/range-for/begin-end-not-adl.cpp statements/range-for/postfix-only-iterator.cpp statements/range-for/data-member-begin-counts.cpp

These conditions apply to the loop of RFOR-2:

- The range must have a complete type, and it must have both `begin`
  and `end`.
- A data member `begin` or `end` of type `int` makes the loop not valid
  ({question}`Q-0098`).
- Free functions that argument-dependent lookup does not find are not
  used.
- The iterator must have prefix `++`.
:::

:::{rule} RFOR-5
:kind: reject
:tests: statements/range-for/storage-class.cpp statements/range-for/static-thread-local-declaration.cpp statements/range-for/extern-loop-variable.cpp

The loop variable must not be any of these ({question}`Q-0026`):

- `static`,
- `thread_local`,
- `extern`.
:::

:::{rule} RFOR-6
:kind: reject
:tests: statements/range-for/redeclare-loop-variable.cpp

The name of the loop variable must not be declared again in the
outermost block of the body.
:::

## Runtime Behavior

:::{rule} RFOR-1
:kind: required
:tests: statements/range-for/array.cpp statements/range-for/braced-list.cpp

The body executes once for each element of the range, in order. Each
element initializes the loop variable. A braced list gives a
`std::initializer_list` *(library referenced)*.
:::

:::{rule} RFOR-3
:kind: required
:tests: statements/range-for/range-evaluated-once.cpp statements/range-for/init-statement.cpp statements/range-for/structured-binding.cpp statements/range-for/end-computed-once.cpp statements/range-for/loop-name-not-in-range.cpp

The range expression is evaluated one time, before the first iteration.
The loop variable is not yet in scope there, so a name in the range
expression refers to the outer entity, also when the loop variable has
the same name.
`begin` and `end` are also called one time, so a change to the range in
the body does not move the end. An initializer statement can come before
the loop variable. The loop variable can be a structured binding.
:::

:::{rule} RFOR-4
:kind: required
:tests: statements/range-for/temporary-range-lives.cpp statements/range-for/temporary-in-range-expression.cpp

Each temporary object that the range expression creates lives until the
loop ends.
:::

## Examples

```cpp
int a[] = {1, 2, 3};
for (int v : a) sum += v;            // 1, then 2, then 3
for (int& v : a) v *= 2;             // changes the elements of a

struct Range {
  int data[3];
  int* begin() { return data; }
  int* end() { return data + 3; }
};
for (int v : Range{}) use(v);        // uses the member functions

for (auto [x, y] : points) use(x, y);

for (int v : make().items()) use(v); // the result of make() lives
                                     // until the loop ends
```

## Evidence

The results are in `tests/RESULTS.md`.

- RFOR-4: GCC 14 and Clang 18 destroy temporaries in the range expression
  before the body executes. Newer versions keep them until the loop ends,
  from the change in C++23 ({question}`Q-0020`).
- RFOR-2: a data member named `begin` with no member `end` does not stop
  the use of free `begin` and `end` on any compiler. With members of both
  names, Clang and GCC use the member form for data members too. EDG
  uses it only for member functions ({question}`Q-0098`).
- RFOR-5: GCC 14 accepts a storage class on the loop variable. GCC 16,
  Clang, and EDG reject it ({question}`Q-0026`).

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, edg-
statements.
