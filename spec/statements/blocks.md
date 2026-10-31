# Expression statements and blocks

An expression statement evaluates an expression for its effects. A null
statement does nothing. A compound statement, or *block*, groups
statements and opens a scope.

## Syntax

```{eval-rst}
.. productionlist:: stmt_blocks
   expression_stmt: `expression` ";"
   null_stmt: ";"
   compound_stmt: "{" `block_item`* `label`* "}"
   block_item: `statement` | `declaration`
   label: `identifier` ":"
```

## Constraints

:::{rule} STMT-2
:kind: required
:tests: statements/blocks/block-scope.cpp statements/blocks/block-hides-outer.cpp statements/blocks/declaration-anywhere.cpp statements/blocks/redeclare-same-block.cpp

A block opens a scope. In a block:

- a declaration can occur between statements,
- a declared name is visible from its declaration to the end of the block,
- a name must not be declared twice,
- a nested block can declare a name again, which hides the outer name.

STO-1 and STO-2 give when the variables of a block are created and
initialized.
:::

:::{rule} STMT-5
:kind: required
:tests: statements/blocks/label-at-end-of-block.cpp statements/blocks/case-label-ends-block.cpp statements/blocks/default-label-ends-block.cpp

A label can be the last item of a block. This includes a `case` or
`default` label in the body of a `switch` ({question}`Q-0096`).
:::

:::{rule} STMT-6
:kind: required
:tests: statements/blocks/ambiguous-declaration.cpp statements/blocks/disambiguation-rest-not-declaration.cpp

If a statement can be read as an expression statement or as a
declaration, it is a declaration. The whole statement decides. If the
start of a statement has the form of a declaration and the rest cannot
belong to one, the statement is an expression statement.
:::

## Dynamic Semantics

:::{rule} STMT-1
:kind: required
:tests: statements/blocks/expression-statement.cpp statements/blocks/null-statement.cpp

An expression statement evaluates its expression and discards the value.
A null statement does nothing.
:::

:::{rule} STMT-3
:kind: required
:tests: statements/blocks/statements-in-order.cpp

The statements of a block execute in the order in which they occur.
:::

:::{rule} STMT-7
:kind: required
:tests: statements/blocks/volatile-name-statement.cpp

An expression statement that only names a volatile object reads the
object (UNEV-2). The read is an observable effect, so the implementation
performs it.
:::

## Recommended Practice

:::{rule} STMT-4
:kind: recommended
:tests: statements/blocks/unused-value-warns.cpp statements/blocks/void-cast-silences.cpp

An implementation should warn about an expression statement that has no
effect and whose value is unused. A cast to `void` should silence the
warning.
:::

## Examples

```cpp
void f(int a) {
  a + 1;                 // warning: the value is unused
  (void)(a + 1);         // no warning
  ;                      // null statement
  int x = 1;
  {
    int x = 2;           // hides the outer x
  }
  int x = 3;             // rejected: x is already declared in this block
}

void g(bool b) {
  if (b) goto end;
  b = !b;
end:
}                        // a label can end a block
```

## Evidence

All seven compilers agree on all tests in this page, except where a
question is noted. The results are in
`tests/RESULTS.md`.

- STMT-4: Clang warns "expression result unused" and GCC warns "statement
  has no effect" under `-Wunused-value`. EDG skips these tests.
- STMT-5: GCC rejects a `case` or `default` label as the last item of a
  block. Clang and EDG accept it ({question}`Q-0096`).
- STMT-7: the test shows that the statement is valid. It cannot observe
  the read.

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, edg-
statements.
