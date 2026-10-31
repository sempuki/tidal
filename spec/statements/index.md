# Statements

A statement is the unit of execution in a function body. These pages give the rules for each kind of statement.

## Syntax

```{eval-rst}
.. productionlist:: tidal
   statement: `labeled_stmt` | `block_decl` | {`attribute_spec`} `plain_stmt`
   plain_stmt: `expression_stmt` | `null_stmt` | `compound_stmt`
             : | `if_stmt` | `switch_stmt` | `while_stmt` | `do_stmt`
             : | `for_stmt` | `range_for_stmt` | `jump_stmt` | `try_block`
```

A declaration that a block can hold is a statement. STMT-6 decides
between a declaration and an expression statement.

:::{tidal-pages}
blocks
if
switch
loops
range-for
jumps
:::

:::{toctree}
:hidden:
:maxdepth: 2

blocks
if
switch
loops
range-for
jumps
:::
