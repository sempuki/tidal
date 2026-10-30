# Shift operators

The operators `<<` and `>>` shift the bits of an integer value to the left
or to the right.

## Syntax

```{eval-rst}
.. productionlist:: expr_shift
   shift_expr: `add_expr` { ("<<" | ">>") `add_expr` }
```

## Legality Rules

:::{rule} SHF-1
:kind: required
:tests: expressions/shift/grouping.cpp

`<<` and `>>` have the same precedence. They group from left to right.
They bind less tightly than `+` and `-`, and more tightly than `<`.
:::

:::{rule} SHF-2
:kind: required
:tests: expressions/shift/operands-integral.cpp

Both operands can have integer type or unscoped enumeration type. This
includes `bool` and the character types.
:::

:::{rule} SHF-3
:kind: reject
:tests: expressions/shift/operand-floating.cpp expressions/shift/count-floating.cpp expressions/shift/operand-scoped-enum.cpp expressions/shift/operand-pointer.cpp

An operand must not have floating-point type, pointer type, or scoped
enumeration type.
:::

:::{rule} SHF-4
:kind: required
:tests: expressions/shift/result-type.cpp expressions/shift/result-prvalue.cpp

Each operand goes through {term}`integer promotion` on its own. The
result has the promoted type of the left operand. The type of the right
operand has no effect on it. The result is a prvalue.
:::

## Dynamic Semantics

:::{rule} SHF-5
:kind: required
:tests: expressions/shift/left-shift-values.cpp expressions/shift/left-shift-negative.cpp expressions/shift/left-shift-overflow-signed.cpp

`a << n` is `a` times two to the power `n`, reduced modulo two to the
power of the width of the result type. This applies also to negative
values and to results that change the sign.
:::

:::{rule} SHF-6
:kind: required
:tests: expressions/shift/right-shift-values.cpp expressions/shift/right-shift-negative.cpp

`a >> n` is `a` divided by two to the power `n`, rounded toward negative
infinity. For a negative `a`, the sign stays negative.
:::

:::{rule} SHF-8
:kind: required
:tests: expressions/shift/operand-order.cpp

The left operand is evaluated before the right operand.
:::

## Undefined Behavior

:::{rule} SHF-7
:kind: undefined
:tests: expressions/shift/count-negative.cpp expressions/shift/count-too-large.cpp expressions/shift/count-too-large-right.cpp expressions/shift/count-width-of-promoted.cpp

A negative shift count gives undefined behavior. So does a shift count
equal to or greater than the bit width of the promoted left operand.
:::

## Recommended Practice

:::{rule} SHF-9
:kind: recommended
:tests: expressions/shift/count-constant-warns.cpp expressions/shift/count-negative-warns.cpp

An implementation should warn if a constant shift count is negative or is
at least the width of the promoted left operand.
:::

## Examples

```cpp
1 << 1 + 1;          // 1 << 2 == 4
3 << 2;              // 12
-1 << 3;             // -8
-7 >> 1;             // -4: rounded toward negative infinity
1 << 31;             // the most negative 32-bit int

char c = 1;
c << 20;             // fine: c is promoted to int first
1 << 32;             // undefined where int has 32 bits
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on all 20
tests.

- SHF-5, SHF-6: all compilers accept left shifts of negative values and
  overflowing left shifts during {term}`constant evaluation`. The results
  match the rules.
- SHF-7: all compilers reject these shift counts during constant
  evaluation.
- SHF-9: Clang and GCC warn by default (`-Wshift-count-overflow`,
  `-Wshift-count-negative`). The tests use `-Werror`. EDG skips them.

Inputs: clang-sema-expr, clang-diag-sema, gcc-cp-typeck.
