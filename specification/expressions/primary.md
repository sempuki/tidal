# Primary expressions

Primary expressions are the simplest expressions. The primary
expressions are these:

- literals,
- `this`,
- names,
- an expression in parentheses.

This page covers:

- the types and values of literals,
- the meaning of `this`,
- the value category of names,
- qualified names,
- fold expressions.

Lambda expressions and requires expressions are primary expressions too.
Lambdas have their own page. Requires expressions are on
{doc}`/templates/concepts`.

Depends on: value categories, standard conversions.

## Principles

:::{principle} PRIM-P1
:rules: PRIM-1 PRIM-2 PRIM-3 PRIM-16 PRIM-10 PRIM-24 PRIM-26

A literal is a value written in the source. The spelling alone decides
the type and the value of a literal, and no context changes either. For
an integer literal, the base and the suffix fix a list of types, and the
literal gets the first type in that list that can hold it. So a literal
never loses part of its value in silence, and a literal that no type in
its list can hold is rejected for the same reason. A spelling that fits
none of the forms is rejected, because a literal has no other meaning to
fall back on. `true`, `false`, and `nullptr` are literals of their own
types.
:::

:::{principle} PRIM-P5
:rules: PRIM-4 PRIM-17 PRIM-5 PRIM-18 PRIM-30 PRIM-31 PRIM-33

A floating literal is `double` unless a suffix says otherwise, and a
hexadecimal one needs an exponent, because without the exponent the
spelling could be an integer. A character literal's prefix sets its
type, and the literal must hold one character of that type. A literal
with no character, or with one that does not fit a single code unit, is
rejected.

The prefix also names an encoding. The value of a one-character literal
is the code unit of its character in that encoding. Every encoding
gives the ten digits consecutive values, so `c - '0'` is the value of a
digit `c` on every implementation. The values of letters and other
characters depend on the encoding.
:::

:::{principle} PRIM-P2
:rules: PRIM-7 PRIM-19 PRIM-8 PRIM-9

Unlike the other literals, a string literal denotes an array object that
lives for the whole program. So a string literal is an lvalue, with an
address and a size that counts the terminating null. The characters of a
string literal are constant, and writing to them is undefined. Adjacent
literals join into one array because they spell one value. The
implementation decides whether two equal spellings share one object,
because a program has no reason to ask.
:::

:::{principle} PRIM-P3
:rules: PRIM-11 PRIM-20 PRIM-12 PRIM-14 PRIM-25

A name denotes the entity that lookup finds ({principle}`NAM-P2`), and
the {term}`value category` of the name follows from what that entity is
({principle}`VC-P1`). A variable or function is an lvalue, and an
enumerator is a prvalue. Inside a member function, a bare member name
means the member of the current object, and `this` is a prvalue pointer
to that object with the qualifiers of the function. Where there is no
current object, there is no `this`. Parentheses group and change nothing
else.
:::

:::{principle} PRIM-P4
:rules: PRIM-32 PRIM-6 PRIM-15 PRIM-21 PRIM-22 PRIM-23 PRIM-13

The character sets and the floating-point formats of a target belong to
the implementation, so the parts of a literal that depend on them are
implementation-defined. The implementation-defined parts are these:

- the ordinary encoding and the wide encoding,
- the value of a multi-character literal,
- the signedness of `char`,
- the rounding of a floating value with no exact representation,
- a floating value outside its type's range,
- the extended floating suffixes,
- a character that needs more than one code unit.

An implementation should warn where such a literal is likely a mistake.
:::

:::{principle} PRIM-P6
:rules: PRIM-27 PRIM-28 PRIM-29

A fold expression applies one binary operator across the elements of a
pack ({term}`template parameter pack`). The fold stands for the
expression that a program would write out by hand. The side of the
`...` sets how the parentheses nest. For a pack `p` with the elements
`p1`, `p2`, and `p3`:

```text
(p - ...)        p1 - (p2 - p3)            right fold
(... - p)        (p1 - p2) - p3            left fold
(p - ... - 10)   p1 - (p2 - (p3 - 10))     right fold with a last operand
(10 - ... - p)   ((10 - p1) - p2) - p3     left fold with a first operand
```

The written-out expression follows the rules of its operator, also for
the order of evaluation. A fold over `&&` or `||` stops as early as the
written-out expression does. An empty pack has a value only where one
value is the obvious answer, so a fold of an empty pack needs `&&`,
`||`, the comma, or an extra operand.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   primary: `literal` | "this" | "(" `expression` ")" | `id_expression`
          : | `lambda_expr` | `fold_expr` | `requires_expression`
   fold_expr: "(" `cast_expr` `fold_op` "..." ")"
            : | "(" "..." `fold_op` `cast_expr` ")"
            : | "(" `cast_expr` `fold_op` "..." `fold_op` `cast_expr` ")"
   fold_op: "+" | "-" | "*" | "/" | "%" | "^" | "&" | "|" | "<<" | ">>"
          : | "+=" | "-=" | "*=" | "/=" | "%=" | "^=" | "&=" | "|=" | "<<=" | ">>=" | "="
          : | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" | "," | ".*" | "->*"
   id_expression: `unqualified_id` | `qualified_name`
   unqualified_id: `identifier` | "~" `class_name`
                 : | `operator_function_id` | `literal_operator_id`
                 : | `conversion_function_id`
                 : | `template_id`
   qualified_name: `nested_name` ["template"] `unqualified_id`
   nested_name: "::"
              : | [`nested_name`] `identifier` "::"
              : | [`nested_name`] ["template"] `template_id` "::"
              : | `decltype_spec` "::"
   identifier: `id_start` {`id_start` | `digit`}
   id_start: `letter` | "_" | `ucn`
           : | a character outside the basic character set that Unicode lets start an identifier
   letter: one of the 26 Latin letters, in lower case or upper case
   literal: `integer_literal` | `floating_literal` | `char_literal`
          : | `string_literal` {`string_literal`}
          : | "true" | "false" | "nullptr"
   integer_literal: `integer_digits` [`integer_suffix` | `ud_suffix`]
   integer_digits: `decimal_digits` | `octal_digits` | `hex_digits` | `binary_digits`
   decimal_digits: `nonzero_digit` {["'"] `digit`}
   octal_digits: "0" {["'"] `octal_digit`}
   hex_digits: ("0x" | "0X") `hex_digit` {["'"] `hex_digit`}
   binary_digits: ("0b" | "0B") `binary_digit` {["'"] `binary_digit`}
   integer_suffix: `unsigned_suffix` [`length_suffix`]
                 : | `length_suffix` [`unsigned_suffix`]
   unsigned_suffix: "u" | "U"
   length_suffix: "l" | "L" | "ll" | "LL" | "z" | "Z"
   digit: "0" | `nonzero_digit`
   nonzero_digit: `nonzero_octal` | "8" | "9"
   octal_digit: "0" | `nonzero_octal`
   nonzero_octal: "1" | "2" | "3" | "4" | "5" | "6" | "7"
   binary_digit: "0" | "1"
   hex_digit: `digit` | `hex_letter`
   hex_letter: one of the letters a to f, in lower case or upper case
   digit_seq: `digit` {["'"] `digit`}
   hex_digit_seq: `hex_digit` {["'"] `hex_digit`}
   floating_literal: (`decimal_float` | `hex_float`) [`float_suffix` | `ud_suffix`]
   decimal_float: `digit_seq` "." [`digit_seq`] [`exponent`]
                : | "." `digit_seq` [`exponent`]
                : | `digit_seq` `exponent`
   exponent: ("e" | "E") ["+" | "-"] `digit_seq`
   hex_float: ("0x" | "0X") `hex_digit_seq` ["." [`hex_digit_seq`]] `binary_exponent`
            : | ("0x" | "0X") "." `hex_digit_seq` `binary_exponent`
   binary_exponent: ("p" | "P") ["+" | "-"] `digit_seq`
   float_suffix: "f" | "F" | "l" | "L"
               : | "f16" | "F16" | "f32" | "F32" | "f64" | "F64" | "f128" | "F128"
               : | "bf16" | "BF16"
   char_prefix: "u8" | "u" | "U" | "L"
   char_literal: [`char_prefix`] "'" `c_char` {`c_char`} "'" [`ud_suffix`]
   c_char: `escape`
         : | a character other than "'", "\", or a new-line
   string_literal: [`char_prefix`] '"' {`s_char`} '"' [`ud_suffix`]
                 : | [`char_prefix`] 'R"' `delimiter` "(" {`raw_char`} ")" `delimiter` '"' [`ud_suffix`]
   s_char: `escape`
         : | a character other than '"', "\", or a new-line
   raw_char: a character, including a new-line, that does not end the raw string
   delimiter: {`d_char`}
   d_char: a basic character other than a space, "(", ")", "\", or a control character
   escape: "\" `simple_escape`
         : | "\" `octal_digit` [`octal_digit` [`octal_digit`]]
         : | "\o{" `octal_digit` {`octal_digit`} "}"
         : | "\x" `hex_digit` {`hex_digit`}
         : | "\x{" `hex_digit` {`hex_digit`} "}"
         : | `ucn`
         : | "\N{" `char_name` "}"
   simple_escape: "'" | '"' | "?" | "\" | "a" | "b" | "f" | "n" | "r" | "t" | "v"
   ucn: "\u" `hex_quad` | "\U" `hex_quad` `hex_quad`
      : | "\u{" `hex_digit` {`hex_digit`} "}"
   hex_quad: `hex_digit` `hex_digit` `hex_digit` `hex_digit`
   char_name: the name of a Unicode character
   ud_suffix: `identifier`
```

A single quote `'` can separate digits in any numeric literal. The
separator must occur between two digits. A *raw string* has no escape
sequences.

## Constraints

:::{rule} PRIM-1
:kind: required
:tests: expressions/primary/int-decimal-types.cpp expressions/primary/int-hex-fits-unsigned-ll.cpp expressions/primary/int-hex-types.cpp expressions/primary/int-negative-is-unary.cpp expressions/primary/int-size-suffix.cpp expressions/primary/int-suffix-types.cpp

An integer literal has the first type from its candidate list that can
hold its value. The list is built in four steps:

1. Start with the signed types `int`, `long`, and `long long`, in that
   order.
2. For an octal, hexadecimal, or binary literal, add the unsigned form
   of each type directly after it.
3. With a `u` suffix, use only the unsigned forms of the types.
4. An `l` suffix removes the `int` forms. An `ll` suffix keeps only the
   `long long` forms.

A `z` suffix gives the signed type that matches `std::size_t`, and `uz`
gives `std::size_t`. A minus sign is a unary operator and is not part
of the literal.
:::

:::{rule} PRIM-2
:kind: reject
:tests: expressions/primary/int-too-large-signed-decimal.cpp expressions/primary/int-too-large.cpp

An integer literal must have a type in its list. A decimal literal with
no `u` suffix that only an unsigned type can hold is not valid
({question}`Q-0064`).
:::

:::{rule} PRIM-3
:kind: required
:tests: expressions/primary/int-forms.cpp expressions/primary/int-forms-suffixes.cpp

`0x` starts a hexadecimal literal, `0b` a binary literal, and `0`
followed by digits an octal literal. Suffix letters can be in either
case, and `u` can come before or after the length suffix.
:::

:::{rule} PRIM-16
:kind: reject
:tests: expressions/primary/int-bad-digit-binary.cpp expressions/primary/int-bad-digit-octal.cpp expressions/primary/int-bad-suffix.cpp expressions/primary/int-separator-leading.cpp expressions/primary/int-separator-trailing.cpp expressions/primary/int-prefix-no-digits-hex.cpp expressions/primary/int-prefix-no-digits-binary.cpp

These requirements apply to an integer literal:

- A base prefix must have at least one digit after it.
- Each digit must be valid for the base of the literal.
- A suffix must be one of the forms in the syntax, or the suffix of a
  literal operator ({rule}`OPR-8`).

In an integer or floating literal, a digit separator must have a digit
on each side.
:::

:::{rule} PRIM-26
:kind: reject
:tests: expressions/primary/number-extends-over-sign.cpp expressions/primary/number-ends-at-space.cpp expressions/primary/number-runs-over-letters.cpp expressions/primary/number-runs-over-dots.cpp expressions/primary/number-runs-over-underscore.cpp

A numeric literal runs over each following character that the table
allows:

| After | The literal takes |
|---|---|
| any character of the literal | a digit, a letter, `_`, `'`, or `.` |
| `e`, `E`, `p`, or `P` | also a `+` or `-` |

The whole run must form one valid literal. So `1u8` is one literal
with the suffix `u8`, and `1.2.3` and `0xe+1` are not valid.
:::

:::{rule} PRIM-4
:kind: required
:tests: expressions/primary/float-forms.cpp expressions/primary/float-types.cpp

A floating literal has type `double`, or `float` with the suffix `f`,
or `long double` with the suffix `l`. A decimal floating literal has a
decimal point, an exponent `e`, or both. A hexadecimal floating literal
starts with `0x` and has a binary exponent `p`, which is a power of 2.
:::

:::{rule} PRIM-17
:kind: reject
:tests: expressions/primary/float-exponent-no-digits.cpp expressions/primary/float-exponent-sign-no-digits.cpp expressions/primary/float-hex-needs-exponent.cpp expressions/primary/float-integer-suffix.cpp

These requirements apply:

- A hexadecimal floating literal must have an exponent.
- An exponent must have digits.
- A floating literal must not have an integer suffix.
:::

:::{rule} PRIM-5
:kind: required
:tests: expressions/primary/char-escapes.cpp expressions/primary/char-types.cpp expressions/primary/char-ucn-basic.cpp expressions/primary/char-ucn-control.cpp expressions/primary/char-named-escape.cpp

The prefix of a character literal sets its type:

| Prefix | Type |
|---|---|
| none | `char` |
| `L` | `wchar_t` |
| `u8` | `char8_t` |
| `u` | `char16_t` |
| `U` | `char32_t` |

An escape sequence gives one code unit or character. The forms are
these:

- simple escapes such as `\n`,
- octal `\101` or `\o{101}`,
- hexadecimal `\x41` or `\x{41}`,
- universal character names `\u00e9` and `\U0001F600`.

A universal character name can name a basic or a control character. A
named escape `\N{...}` gives the character with that Unicode name.
:::

:::{rule} PRIM-18
:kind: reject
:tests: expressions/primary/char-delimited-empty.cpp expressions/primary/char-empty.cpp expressions/primary/char-escape-out-of-range.cpp expressions/primary/char-hex-no-digits.cpp expressions/primary/char-octal-too-large.cpp expressions/primary/char-u8-not-single-byte.cpp expressions/primary/char-ucn-surrogate.cpp expressions/primary/char-unicode-multichar.cpp expressions/primary/char-unknown-escape.cpp expressions/primary/char16-not-bmp.cpp expressions/primary/char-named-escape-unknown.cpp

A character literal must contain at least one character. The literal
must not have:

- an unknown escape sequence, or `\x` or `\x{}` with no digits,
- an octal or hexadecimal escape too large for the type,
- a universal character name for a surrogate,
- more than one character with a `u8`, `u`, `U`, or `L` prefix,
- a prefix and a character that does not fit in one code unit of its
  type,
- a named escape with a name that Unicode does not define.
:::

:::{rule} PRIM-30
:kind: required
:tests: expressions/primary/char-value-code-unit.cpp expressions/primary/char-value-escapes-prefixes.cpp

A character literal with one character has the value of that
character's code unit in the encoding that the prefix names:

| Prefix | Encoding |
|---|---|
| none | the {term}`ordinary encoding` |
| `L` | the wide encoding (PRIM-32) |
| `u8` | UTF-8 |
| `u` | UTF-16 |
| `U` | UTF-32 |

An octal or hexadecimal escape gives its number as the code unit. The
number converts to the type of the literal ({rule}`CONV-2`).
:::

:::{rule} PRIM-31
:kind: required
:tests: expressions/primary/char-digits-consecutive.cpp expressions/primary/char-digits-parse.cpp

In each encoding of PRIM-30, the digits `0` to `9` have consecutive code
units in increasing order.
:::

:::{rule} PRIM-33
:kind: required
:tests: expressions/primary/char-simple-escape-values.cpp expressions/primary/string-simple-escape-values.cpp

Each simple escape stands for one character, and its value is the code
unit of that character (PRIM-30):

| Escape | Character | Escape | Character |
|---|---|---|---|
| `\'` | single quote | `\a` | alert |
| `\"` | double quote | `\b` | backspace |
| `\?` | question mark | `\f` | form feed |
| `\\` | backslash | `\n` | new-line |
| | | `\r` | carriage return |
| | | `\t` | horizontal tab |
| | | `\v` | vertical tab |
:::

:::{rule} PRIM-7
:kind: required
:tests: expressions/primary/string-concat.cpp expressions/primary/string-escape-in-string.cpp expressions/primary/string-index-constant.cpp expressions/primary/string-lvalue.cpp expressions/primary/string-raw-long-delimiter.cpp expressions/primary/string-raw.cpp expressions/primary/string-type.cpp

A string literal is an lvalue of type array of `N` constant characters.
`N` counts a terminating null character. String literals also follow
these rules:

- The prefix sets the character type as for character literals.
- Adjacent string literals are joined into one. If one of the literals
  has a prefix, the result has that prefix.
- A raw string keeps each character as written.
:::

:::{rule} PRIM-19
:kind: reject
:tests: expressions/primary/string-concat-mixed.cpp expressions/primary/string-raw-bad-delimiter.cpp expressions/primary/string-to-nonconst-pointer.cpp expressions/primary/string-raw-delimiter-too-long.cpp expressions/primary/string-raw-delimiter-backslash.cpp expressions/primary/string-concat-u8-u.cpp expressions/primary/string-raw-delimiter-paren.cpp expressions/primary/string-raw-delimiter-tab.cpp

These requirements apply:

- If two adjacent string literals both have a prefix, the prefixes must
  be the same.
- A string literal does not convert to a pointer to non-const
  characters.
- A raw string delimiter must have at most 16 characters. The delimiter
  must not contain a space, a parenthesis, a backslash, or a control
  character such as a tab.
:::

:::{rule} PRIM-10
:kind: required
:tests: expressions/primary/bool-nullptr.cpp expressions/primary/nullptr-member-pointer.cpp

`true` and `false` have type `bool`. `nullptr` has type
`std::nullptr_t`. `nullptr` converts to every pointer and
pointer-to-member type, and to `bool` by direct initialization.
:::

:::{rule} PRIM-24
:kind: reject
:tests: expressions/primary/nullptr-to-bool-copy.cpp expressions/primary/nullptr-to-int.cpp

`nullptr` converts to `bool` only by direct initialization (PRIM-10).
`nullptr` does not convert to an integer.
:::

:::{rule} PRIM-11
:kind: required
:tests: expressions/primary/this-const-member-modify.cpp expressions/primary/this-default-member-init.cpp expressions/primary/this-prvalue.cpp expressions/primary/this-type.cpp expressions/primary/this-trailing-return.cpp

In a member function of class `C`, `this` is a prvalue that points to
the object. The type of `this` is pointer to `C` with the {term}`cv-qualifiers <cv-qualifier>`
of the member function. A program can also use `this` in a default
member initializer and in the trailing return type of a member function.
:::

:::{rule} PRIM-20
:kind: reject
:tests: expressions/primary/this-namespace.cpp expressions/primary/this-static.cpp expressions/primary/this-explicit-object.cpp

A program must not use `this` in these places:

- a static member function,
- a member function with an explicit object parameter,
- outside a class.
:::

:::{rule} PRIM-12
:kind: required
:tests: expressions/primary/name-enumerator-prvalue.cpp expressions/primary/name-function-lvalue.cpp expressions/primary/name-member-unevaluated.cpp expressions/primary/name-parameter-in-scope.cpp expressions/primary/name-qualified-class-static.cpp expressions/primary/name-qualified-global.cpp expressions/primary/name-undeclared.cpp expressions/primary/name-variable-lvalue.cpp expressions/primary/this-implicit-member.cpp expressions/primary/member-mutable-in-const.cpp expressions/primary/member-const-in-const-function.cpp

A name refers to the entity that lookup finds ({rule}`NAM-8`). The
value category of the name is:

- an lvalue for a variable, a parameter, or a function,
- a prvalue for an enumerator.

In a member function, the name of a non-static member means `this->m`. A
data member then has the cv-qualifiers of the object, unless the member
is `mutable` or a reference. `::x` names `x` in the global namespace.
`N::x` names `x` in the namespace or class `N`, or the enumerator `x`
of the enumeration `N` ({rule}`NAM-19`).
:::

:::{rule} PRIM-25
:kind: reject
:tests: expressions/primary/name-member-in-static.cpp expressions/primary/name-member-outside-object.cpp expressions/decltype/nested-class-member-evaluated.cpp

Inside a member function that has `this` (PRIM-20), the name of any
non-static member of the class or a base is valid (PRIM-12). Elsewhere,
the name is valid only in an unevaluated operand ({rule}`UNEV-1`).
These places are elsewhere:

- a static member function,
- a member function of a nested class, for a member of the enclosing
  class.
:::

:::{rule} PRIM-14
:kind: required
:tests: expressions/primary/paren-function-call.cpp expressions/primary/paren-lvalue.cpp expressions/primary/paren-same-type.cpp

An expression in parentheses has the same type, value, and value
category as the expression.
:::

:::{rule} PRIM-27
:kind: required
:tests: expressions/primary/fold-expansion.cpp expressions/primary/fold-shift-and-comma.cpp

A fold expression stands for an expression with one copy of the pack
operand for each element, in the order of the pack. A right fold, with
`...` after the pack operand, nests to the right. A left fold nests to
the left, and an extra operand goes at the outer end.
:::

:::{rule} PRIM-28
:kind: required
:tests: expressions/primary/fold-expansion.cpp expressions/primary/fold-order.cpp

A fold expression is evaluated as the expression that PRIM-27 gives, so
the rules of its operator give the order of evaluation ({rule}`LOG-5`,
{rule}`COMMA-1`, {rule}`SEQ-8`). If the pack is empty, the fold has
these values:

- `true` for `&&`, and `false` for `||`,
- a `void` expression for the comma,
- the extra operand, if the fold has one.
:::

:::{rule} PRIM-29
:kind: reject
:tests: expressions/primary/fold-empty-pack.cpp expressions/primary/fold-operand-binary.cpp expressions/primary/fold-mismatched-operators.cpp expressions/primary/fold-two-packs.cpp expressions/primary/fold-no-pack.cpp expressions/primary/fold-without-parentheses.cpp expressions/primary/fold-three-way-operator.cpp

A fold expression must obey these rules:

- The fold is in parentheses, and its operator is a `fold_op`.
- An operand is a cast expression. A binary operator in an operand needs
  its own parentheses ({question}`Q-0194`).
- Exactly one operand names a pack that is not yet expanded.
- In a fold with an extra operand, the two operators are the same.
- A fold without an extra operand over an operator other than `&&`,
  `||`, and the comma has a pack with at least one element.
:::

## Undefined Behavior

:::{rule} PRIM-8
:kind: undefined
:tests: expressions/primary/string-modify-constant.cpp expressions/primary/string-modify-sanitizer.cpp

The characters of a string literal are constant. If a program writes to
one of them, for example through a pointer from `const_cast`, the
behavior is undefined.
:::

## Implementation Requirements

:::{rule} PRIM-32
:kind: impl-defined
:tests: expressions/primary/char-encoding-lp64.cpp expressions/primary/char-encoding-ilp32.cpp expressions/primary/char-encoding-ascii.cpp

The ordinary encoding and the wide encoding are implementation-defined.
The matrix targets choose these encodings:

| Encoding | x86-64 Linux | `-m32` |
|---|---|---|
| ordinary, for `char` | UTF-8 | UTF-8 |
| wide, for `wchar_t` | UTF-32 | UTF-32 |

In both encodings, each ASCII character has one code unit with its ASCII
value. Each row of the table gives characters with consecutive values,
in the order shown:

| Characters | Values |
|---|---|
| null | 0 |
| alert, backspace, horizontal tab, new-line, vertical tab, form feed, carriage return | 7 to 13 |
| space | 32 |
| `!` `"` `#` `$` `%` `&` `'` `(` `)` `*` `+` `,` `-` `.` `/` | 33 to 47 |
| `0` to `9` | 48 to 57 |
| `:` `;` `<` `=` `>` `?` `@` | 58 to 64 |
| `A` to `Z` | 65 to 90 |
| `[` `\` `]` `^` `_` `` ` `` | 91 to 96 |
| `a` to `z` | 97 to 122 |
| `{` `\|` `}` `~` | 123 to 126 |
:::

:::{rule} PRIM-6
:kind: impl-defined
:tests: expressions/primary/char-multichar.cpp expressions/primary/char-plain-signedness.cpp

The value of a character literal with more than one character and no
prefix is implementation-defined. The type of such a literal is `int`.
The signedness of plain `char` is implementation-defined. On the x86-64
Linux targets in the matrix, `'ab'` is `0x6162` and `char` is signed.
:::

:::{rule} PRIM-9
:kind: unspecified
:tests: expressions/primary/string-compare-constant.cpp expressions/primary/string-same-object-either.cpp expressions/primary/string-same-literal-two-evaluations.cpp

Whether two string literals with the same characters are the same
object is unspecified. The same rule applies to two evaluations of one
string literal. {term}`Constant evaluation <constant evaluation>` rejects a comparison of the
addresses of such literals ({question}`Q-0033`).
:::

:::{rule} PRIM-15
:kind: conditionally-supported
:tests: expressions/primary/float-overflow.cpp expressions/primary/double-overflow.cpp

A floating literal outside the range of its type is conditionally
supported. An implementation that accepts such a literal documents the
value ({question}`Q-0063`).
:::

:::{rule} PRIM-21
:kind: impl-defined
:tests: expressions/primary/float-nearest-value.cpp expressions/primary/float-nearest-value-float.cpp

If the value of a floating literal is in the range of its type but has
no exact representation, the result is one of the two nearest values
that the type can represent. The implementation chooses which one. On
the x86-64 Linux targets in the matrix, the result is the nearer value.
:::

:::{rule} PRIM-22
:kind: conditionally-supported
:tests: expressions/primary/float-extended-suffix.cpp expressions/primary/float-extended-suffix-sixteen.cpp

The suffixes `f16`, `f32`, `f64`, `f128`, and `bf16`, in either case,
give the extended floating-point types *(extended floating-point types
referenced)*. Support for the suffixes is conditional. GCC and EDG
accept every suffix. Clang accepts `f16` and rejects the others.
:::

:::{rule} PRIM-23
:kind: conditionally-supported
:tests: expressions/primary/char-unprefixed-not-single-unit.cpp expressions/primary/char-unprefixed-utf8-source.cpp

A character literal without a prefix that contains one character with
no single code unit in the ordinary encoding is conditionally supported.
If an implementation accepts such a literal, the literal has type `int`
and an implementation-defined value. No implementation in the matrix
accepts such a literal.
:::

## Recommended Practice

:::{rule} PRIM-13
:kind: recommended
:tests: expressions/primary/char-multichar-warns.cpp expressions/primary/float-overflow-warns.cpp

An implementation should warn about a floating literal outside the range
of its type and about a multi-character literal.
:::

## Examples

```cpp
auto a = 4'294'967'295;   // long: too large for int
auto b = 0xFFFF'FFFF;     // unsigned int
auto c = 10uz;            // std::size_t
auto d = 0x1.8p3;         // 12.0
auto e = u8'x';           // char8_t
auto f = u"ab" "cd";      // const char16_t[5]
auto g = R"(C:\path)";    // backslash kept
int* p = nullptr;

constexpr int digit(char c) { return c - '0'; }
static_assert(digit('7') == 7);   // every implementation (PRIM-31)
static_assert('a' == 97);         // the matrix targets (PRIM-32)
char h = '\xff';                  // -1 where char is signed

struct Counter {
  int n = 0;
  void bump() { ++n; }    // means ++this->n
  static int get() { return n; }       // rejected: no object (PRIM-25)
  static int size() { return sizeof n; }  // valid: unevaluated
  struct Part {
    int get() { return n; }            // rejected: no Counter object
  };
};

template <class... T> int total(T... v) { return (0 + ... + v); }
template <class... T> bool all(T... v) { return (v && ...); }
total();                  // 0
all();                    // true
total(1, 2, 3);           // ((0 + 1) + 2) + 3
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 115
of 124 tests.

- PRIM-2: Clang and GCC reject a decimal literal that fits only an
  unsigned type under `-pedantic-errors`. EDG accepts it
  ({question}`Q-0064`).
- PRIM-15: Clang and GCC accept `1e999f` with a warning and give
  infinity. EDG rejects it ({question}`Q-0063`).
- PRIM-22: GCC and EDG accept the extended floating suffixes. Clang
  accepts `f16` and `F16` and rejects the others. Support is
  conditional.
- PRIM-29: Clang and GCC reject a fold whose operand holds a binary
  operator without parentheses. EDG accepts it ({question}`Q-0194`).
- PRIM-32: EDG, as the matrix configures it, rejects `"é"` because one
  `char` cannot hold the character. Its ordinary encoding is not UTF-8,
  which PRIM-32 allows. EDG agrees on every ASCII value. Two tests show
  the difference, one for each EDG target. EDG reads the source file as
  UTF-8 and accepts a character outside ASCII in a literal with a `u8`,
  `u`, `U`, or `L` prefix, such as `u'€'` and `L'é'`.
- PRIM-28: every compiler evaluates a comma fold from the first element
  to the last, and stops a fold over `&&` or `||` at the first operand
  that decides the result.
- PRIM-30 to PRIM-33: GCC and Clang give every ASCII character its
  ASCII value in `char` and `wchar_t` literals, on both targets. A
  string literal with U+00E9 has two `char` code units and one
  `wchar_t` code unit.
- PRIM-31, PRIM-32: GCC can choose another ordinary encoding with
  `-fexec-charset`. With the EBCDIC code page IBM1047, the digits keep
  consecutive values from 0xF0 to 0xF9, and `'j' - 'a'` is 16. So only
  the digits are consecutive in every encoding, and the order of the
  letters belongs to the encoding. Clang accepts only UTF-8.
- PRIM-13: Clang and GCC warn with no flag about a floating literal
  outside the range of its type (Clang `-Wliteral-range`, GCC
  `-Woverflow`) and about a multi-character literal (`-Wmultichar`).
  So `-Werror` alone rejects both. EDG warns about a multi-character
  literal and rejects `1e999f` (PRIM-15).
- PRIM-18: Clang and GCC give a warning for an unknown escape such as
  `'\q'`, and `-pedantic-errors` makes the warning an error. EDG in
  strict mode gives a warning and accepts the program
  ({question}`Q-0330`).
- PRIM-8: the matrix targets keep string literals in memory that the
  program cannot write. So a write to a literal faults at run time, and
  `-fsanitize=address` reports the fault as SEGV.
- PRIM-9: Clang rejects a comparison of the addresses of two string
  literals during constant evaluation. GCC and EDG accept it and fold it
  ({question}`Q-0033`).

Inputs: clang-lex-literal, clang-diag-lex, clang-sema-expr,
clang-diag-sema, edg-decls.
