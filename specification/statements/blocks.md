# Expression statements and blocks

This page covers three kinds of statement:

- An expression statement evaluates an expression for its effects.
- A null statement does nothing.
- A compound statement, or *block*, groups statements and opens a
  scope.

## Principles

:::{principle} STMT-P1
:rules: STMT-2 STMT-3 STMT-5 STMT-8 STMT-9 STMT-10

A block is a sequence and a scope. The statements of a block execute in
the order written, and a declaration is one of the statements, so a name
becomes visible at its declaration and stays visible to the closing
brace ({principle}`NAM-P1`). A name declared twice in one block has no
single meaning, so the second declaration is rejected, while a nested
block can hide the outer name ({principle}`NAM-P2`). Two declarations
of one entity, such as two declarations of one function, give the name
one meaning and are valid. A class name and a variable name can also
share a block, and then the variable hides the class. A label marks a
position, and the end of a block is a position, so a label can stand
there.
:::

:::{principle} STMT-P2
:rules: STMT-1 STMT-7 STMT-4

An expression statement exists for what its expression does, and the
value of the expression is thrown away. A read of a volatile object is
something the expression does, so a statement that only names such an
object reads the object.
A statement that neither changes anything nor reads anything observable
is almost always a mistake, so an implementation should say so.
:::

:::{principle} STMT-P3
:rules: STMT-6

Some text can be read as a declaration or as an expression. The language
reads such text as a declaration and decides by the whole statement, so
a statement that starts like a declaration and cannot finish as one is
an expression statement.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   expression_stmt: `expression` ";"
   null_stmt: ";"
   compound_stmt: "{" {`statement`} {`label`} "}"
   labeled_stmt: `label` `statement`
   label: {`attribute_spec`} `identifier` ":"
        : | {`attribute_spec`} "case" `constant_expr` ":"
        : | {`attribute_spec`} "default" ":"
```

## Constraints

:::{rule} STMT-2
:kind: required
:tests: statements/blocks/block-scope.cpp statements/blocks/block-hides-outer.cpp statements/blocks/declaration-anywhere.cpp statements/blocks/nested-blocks-hide.cpp

A block opens a scope. In a block:

- a declaration can occur between statements,
- a declared name is visible from its declaration to the end of the block,
- a nested block can declare a name again, which hides the outer name
  from that declaration to the end of the nested block.

STO-1 and STO-2 give when the variables of a block are created and
initialized.
:::

:::{rule} STMT-8
:kind: reject
:tests: statements/blocks/redeclare-same-block.cpp statements/blocks/redeclare-different-type.cpp statements/blocks/redeclare-typedef-different-type.cpp statements/blocks/redeclare-variable-as-function.cpp statements/blocks/redeclare-same-entity-ok.cpp

In a block, a name must not be declared twice (STMT-2), except by
declarations of one entity ({rule}`NAM-3`). These pairs in one block
are valid:

- two declarations of one function,
- two `extern` declarations of one variable,
- two `typedef` declarations that name the same type.

STMT-9 allows a class and a variable with one name.
:::

:::{rule} STMT-9
:kind: required
:tests: statements/blocks/class-and-variable-same-name.cpp

A block can declare a class or an enumeration and a variable or a
function with the same name. The variable or the function hides the
class name from its declaration to the end of the block.
:::

:::{rule} STMT-10
:kind: reject
:tests: statements/blocks/class-hidden-by-variable.cpp

Where a variable or a function hides a class name (STMT-9), the name
alone must not name the class. The program must name the class with
`struct`, `class`, `union`, or `enum`.
:::

:::{rule} STMT-5
:kind: required
:tests: statements/blocks/label-at-end-of-block.cpp statements/blocks/case-label-ends-block.cpp statements/blocks/default-label-ends-block.cpp statements/blocks/label-before-declaration.cpp

A label can be the last item of a block, including a `case` or `default`
label in the body of a `switch` ({question}`Q-0096`). A label can also
come before a declaration.
:::

:::{rule} STMT-6
:kind: required
:tests: statements/blocks/ambiguous-declaration.cpp statements/blocks/disambiguation-rest-not-declaration.cpp statements/blocks/disambiguation-declarator-forms.cpp

If the grammar ({rule}`TR-12`) derives a statement as an expression
statement and as a declaration, the statement is a declaration. The
whole statement decides. If the start of a statement has the form of a declaration and
the rest cannot belong to a declaration, the statement is an expression
statement.
:::

## Runtime Behavior

:::{rule} STMT-1
:kind: required
:tests: statements/blocks/expression-statement.cpp statements/blocks/null-statement.cpp statements/blocks/empty-and-nested-blocks.cpp

An expression statement evaluates its expression and discards the value.
A null statement does nothing.
:::

:::{rule} STMT-3
:kind: required
:tests: statements/blocks/statements-in-order.cpp statements/blocks/declaration-runs-in-order.cpp statements/blocks/empty-and-nested-blocks.cpp

The statements of a block execute in the order in which they occur. A
declaration is a statement, so its initializer runs when control reaches
the declaration.
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

void h() {
  struct Point { int x; };
  int Point = 3;         // valid: the variable hides the class
  struct Point p{4};     // valid: struct names the class
  Point q{5};            // rejected: Point is the variable here
}
```

## Evidence

All eight compilers agree on 24 of 26 tests. The results are in
`tests/RESULTS.md`.

- STMT-4: Clang warns "expression result unused" and GCC warns "statement
  has no effect" under `-Wunused-value`. EDG skips these tests.
- STMT-5: GCC rejects a `case` or `default` label as the last item of a
  block. Clang and EDG accept it ({question}`Q-0096`).
- STMT-7: the test shows that the statement is valid. The test cannot
  observe the read.
- STMT-10: Clang says that the class is hidden by a non-type
  declaration. GCC and EDG report a syntax error at the name after the
  class name.

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema, edg-
statements.
