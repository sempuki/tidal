# Expressions

An expression computes a value, or causes an effect, or both. These
pages give the rules for each family of operators.

## Syntax

Each level of the expression grammar is defined on the page of its
operators. A level is made of operands from the level below it, so the
first row binds most tightly.

| Nonterminal | Operators | Page |
|---|---|---|
| `primary` | literals, names, `this`, `( )`, lambdas | {doc}`primary` |
| `postfix_expr` | `[]`, `()`, `.`, `->`, postfix `++` and `--`, `T(e)`, named casts | {doc}`postfix` |
| `unary_expr` | `+`, `-`, `*`, `&`, `!`, `~`, prefix `++` and `--`, `sizeof`, `alignof`, `noexcept`, `new`, `delete` | {doc}`unary` |
| `cast_expr` | `(T)e` | {doc}`casts` |
| `pm_expr` | `.*`, `->*` | {doc}`member-pointers` |
| `mul_expr` | `*`, `/`, `%` | {doc}`multiplicative` |
| `add_expr` | `+`, `-` | {doc}`additive` |
| `shift_expr` | `<<`, `>>` | {doc}`shift` |
| `three_way_expr` | `<=>` | {doc}`comparison` |
| `relational_expr` | `<`, `>`, `<=`, `>=` | {doc}`comparison` |
| `equality_expr` | `==`, `!=` | {doc}`comparison` |
| `and_expr` | `&` | {doc}`bitwise` |
| `xor_expr` | `^` | {doc}`bitwise` |
| `or_expr` | `\|` | {doc}`bitwise` |
| `logical_and_expr` | `&&` | {doc}`logical` |
| `logical_or_expr` | `\|\|` | {doc}`logical` |
| `conditional_expr` | `? :` | {doc}`conditional` |
| `assignment_expr` | `=` and the compound assignments, `throw` | {doc}`assignment` |
| `expression` | `,` | {doc}`comma` |

`new_expr` and `delete_expr` are on {doc}`new-delete`, `lambda_expr` on
{doc}`lambdas`, and `decltype_spec` on {doc}`decltype`. `constant_expr`
is on {doc}`../foundations/constant-evaluation`, and `throw_expr` on
{doc}`../exceptions/throw`.

:::{tidal-pages}
primary
decltype
multiplicative
additive
shift
comparison
bitwise
logical
conditional
assignment
comma
unary
member-pointers
postfix
casts
new-delete
lambdas
:::

:::{toctree}
:hidden:
:maxdepth: 2

primary
decltype
multiplicative
additive
shift
comparison
bitwise
logical
conditional
assignment
comma
unary
member-pointers
postfix
casts
new-delete
lambdas
:::
