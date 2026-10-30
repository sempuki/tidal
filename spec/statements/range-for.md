# The range-based for statement

A range-based `for` statement executes its body once for each element of
a range. A range is an array, a braced list, or an object of class type
with `begin` and `end`.

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

## Legality Rules

:::{rule} RFOR-2
:kind: required
:tests: statements/range-for/member-begin-end.cpp statements/range-for/free-begin-end.cpp statements/range-for/sentinel-end.cpp statements/range-for/not-a-range.cpp statements/range-for/member-begin-only.cpp statements/range-for/incomplete-element.cpp

For a range of class type, `begin` and `end` are found as follows:

- If the class declares a member `begin` or a member `end`, both are
  member functions.
- If not, both are free functions that argument-dependent lookup finds
  *(referenced)*.

`begin` and `end` can return different types. The range must have a
complete type, and it must have both `begin` and `end`.
:::

:::{rule} RFOR-5
:kind: reject
:tests: statements/range-for/storage-class.cpp statements/range-for/static-thread-local-declaration.cpp

The loop variable must not be `static` or `thread_local`
({question}`Q-0026`).
:::

## Dynamic Semantics

:::{rule} RFOR-1
:kind: required
:tests: statements/range-for/array.cpp statements/range-for/braced-list.cpp

The body executes once for each element of the range, in order. Each
element initializes the loop variable. A braced list gives a
`std::initializer_list` *(library referenced)*.
:::

:::{rule} RFOR-3
:kind: required
:tests: statements/range-for/range-evaluated-once.cpp statements/range-for/init-statement.cpp statements/range-for/structured-binding.cpp

The range expression is evaluated one time, before the first iteration.
An initializer statement can come before the loop variable. The loop
variable can be a structured binding.
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
- RFOR-5: GCC 14 accepts a storage class on the loop variable. GCC 16,
  Clang, and EDG reject it ({question}`Q-0026`).

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, gcc-cp-parser,
gcc-cp-semantics, edg-statements.
