# Primary expressions

Primary expressions are the simplest expressions. They are literals,
`this`, names, and an expression in parentheses. This page covers the
types and values of literals, the meaning of `this`, the value category
of names, and qualified names.

Lambda expressions, fold expressions, and requires expressions are
primary expressions too. Lambdas have their own page. Fold and requires
expressions belong with templates *(referenced)*.

Depends on: value categories, standard conversions.

## Principles

:::{principle} PRIM-P1
:rules: PRIM-1 PRIM-2 PRIM-3 PRIM-16 PRIM-4 PRIM-17 PRIM-5 PRIM-18 PRIM-10

A literal is a value written in the source. Its spelling alone decides
its type and its value, and no context changes them. The base and the
suffix of an integer literal, the point and exponent of a floating
literal, and the prefix of a character literal are the whole story. An
integer literal gets the first type in a fixed list that can hold it, so
a literal never loses part of its value in silence. A literal that no
type in its list can hold is rejected for the same reason. A spelling
that fits none of the forms is rejected, because a literal has no other
meaning to fall back on.
:::

:::{principle} PRIM-P2
:rules: PRIM-7 PRIM-19 PRIM-8 PRIM-9

A string literal is unlike the other literals. It denotes an array
object that lives for the whole program, so it is an lvalue, with an
address and a size that counts the terminating null. Its characters are
constant, and writing to them is undefined. Adjacent literals join into
one array because they spell one value. Whether two equal spellings
share one object is left to the implementation, because a program has
no reason to ask.
:::

:::{principle} PRIM-P3
:rules: PRIM-11 PRIM-20 PRIM-12 PRIM-14

A name is not a value. It denotes the entity that lookup finds
({principle}`NAM-P2`), and its value category follows from what that
entity is ({principle}`VC-P1`): a variable or function is an lvalue, an
enumerator a prvalue. Inside a member function, a bare member name means
the member of the current object, and `this` is a prvalue pointer to
that object with the qualifiers of the function. Where there is no
current object, there is no `this`. Parentheses group and change nothing
else.
:::

:::{principle} PRIM-P4
:rules: PRIM-6 PRIM-15 PRIM-21 PRIM-22 PRIM-23 PRIM-13

The character sets and the floating-point formats of a target belong to
the implementation, so the parts of a literal that depend on them are
implementation-defined. These are the value of a multi-character
literal, the signedness of `char`, the rounding of a floating value with
no exact representation, a floating value outside its type's range, the
extended floating suffixes, and a character that needs more than one
code unit. An implementation should warn where such a literal is likely
a mistake.
:::

## Syntax

```{eval-rst}
.. productionlist:: prim
   primary: `literal` | "this" | "(" `expression` ")" | `id_expression`
          : `lambda_expression`
   integer_literal: `integer_digits` [`integer_suffix`]
   integer_digits: `decimal` | "0" `octal_digits` | ("0x" | "0X") `hex_digits` | ("0b" | "0B") `binary_digits`
   integer_suffix: ["u" | "U"] ["l" | "L" | "ll" | "LL" | "z" | "Z"]
                 : ("l" | "L" | "ll" | "LL" | "z" | "Z") ("u" | "U")
   char_prefix: "u8" | "u" | "U" | "L"
   string_literal: [`char_prefix`] '"' {`s_char`} '"'
                 : [`char_prefix`] 'R"' `delimiter` "(" {`raw_char`} ")" `delimiter` '"'
   id_expression: [`nested_name`] `identifier`
   nested_name: "::" | `name` "::" | `nested_name` `name` "::"
```

A single quote `'` can separate digits in any numeric literal. It must
occur between two digits. A *raw string* has no escape sequences.

## Constraints

:::{rule} PRIM-1
:kind: required
:tests: expressions/primary/int-decimal-types.cpp expressions/primary/int-hex-fits-unsigned-ll.cpp expressions/primary/int-hex-types.cpp expressions/primary/int-negative-is-unary.cpp expressions/primary/int-size-suffix.cpp expressions/primary/int-suffix-types.cpp

An integer literal has the first type from its candidate list that can
hold its value. The list is built in four steps:

1. Start with the signed types `int`, `long`, and `long long`, in that
   order.
2. For an octal, hexadecimal, or binary literal, add the unsigned form of
   each type directly after it.
3. With a `u` suffix, use only the unsigned forms of the types.
4. An `l` suffix removes the `int` forms. An `ll` suffix keeps only the
   `long long` forms.

A `z` suffix gives the signed type that matches `std::size_t`, and `uz`
gives `std::size_t`. A minus sign is a unary operator. It is not part of
the literal.
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
:tests: expressions/primary/int-forms.cpp

`0x` starts a hexadecimal literal, `0b` a binary literal, and `0` followed
by digits an octal literal. Suffix letters can be in either case, and `u`
can come before or after the length suffix.
:::

:::{rule} PRIM-16
:kind: reject
:tests: expressions/primary/int-bad-digit-binary.cpp expressions/primary/int-bad-digit-octal.cpp expressions/primary/int-bad-suffix.cpp expressions/primary/int-separator-leading.cpp expressions/primary/int-separator-trailing.cpp expressions/primary/int-prefix-no-digits-hex.cpp expressions/primary/int-prefix-no-digits-binary.cpp

A base prefix must have at least one digit after it. Each digit must be
valid for the base of the literal. A digit separator
must have a digit on each side. A suffix must be one of the forms in the
syntax.
:::

:::{rule} PRIM-4
:kind: required
:tests: expressions/primary/float-forms.cpp expressions/primary/float-types.cpp

A floating literal has type `double`. The suffix `f` gives `float` and
`l` gives `long double`. A decimal floating literal has a decimal point,
an exponent `e`, or both. A hexadecimal floating literal starts with `0x`
and has a binary exponent `p`, which is a power of 2.
:::

:::{rule} PRIM-17
:kind: reject
:tests: expressions/primary/float-exponent-no-digits.cpp expressions/primary/float-exponent-sign-no-digits.cpp expressions/primary/float-hex-needs-exponent.cpp expressions/primary/float-integer-suffix.cpp

A hexadecimal floating literal must have an exponent. An exponent must
have digits. A floating literal must not have an integer suffix.
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

An escape sequence gives one code unit or character. The forms are simple
escapes such as `\n`, octal `\101` or `\o{101}`, hexadecimal `\x41` or
`\x{41}`, and universal character names `\u00e9` and `\U0001F600`. A
universal character name can name a basic or a control character. A
named escape `\N{...}` gives the character with that Unicode name.
:::

:::{rule} PRIM-18
:kind: reject
:tests: expressions/primary/char-delimited-empty.cpp expressions/primary/char-empty.cpp expressions/primary/char-escape-out-of-range.cpp expressions/primary/char-hex-no-digits.cpp expressions/primary/char-octal-too-large.cpp expressions/primary/char-u8-not-single-byte.cpp expressions/primary/char-ucn-surrogate.cpp expressions/primary/char-unicode-multichar.cpp expressions/primary/char-unknown-escape.cpp expressions/primary/char16-not-bmp.cpp expressions/primary/char-named-escape-unknown.cpp expressions/primary/char-prefixed-not-single-unit.cpp

A character literal must contain at least one character. It must not
have:

- an unknown escape sequence, or `\x` or `\x{}` with no digits,
- an octal or hexadecimal escape too large for the type,
- a universal character name for a surrogate,
- more than one character with a `u8`, `u`, `U`, or `L` prefix,
- a prefix and a character that does not fit in one code unit of its
  type,
- a named escape with a name that Unicode does not define.
:::

:::{rule} PRIM-7
:kind: required
:tests: expressions/primary/string-concat.cpp expressions/primary/string-escape-in-string.cpp expressions/primary/string-index-constant.cpp expressions/primary/string-lvalue.cpp expressions/primary/string-raw-long-delimiter.cpp expressions/primary/string-raw.cpp expressions/primary/string-type.cpp

A string literal is an lvalue of type array of `N` constant characters.
`N` counts a terminating null character. The prefix sets the character
type as for character literals. Adjacent string literals are joined into
one. If one has a prefix, the result has that prefix. A raw string keeps
each character as written.
:::

:::{rule} PRIM-19
:kind: reject
:tests: expressions/primary/string-concat-mixed.cpp expressions/primary/string-raw-bad-delimiter.cpp expressions/primary/string-to-nonconst-pointer.cpp expressions/primary/string-raw-delimiter-too-long.cpp expressions/primary/string-raw-delimiter-backslash.cpp expressions/primary/string-concat-u8-u.cpp expressions/primary/string-raw-delimiter-paren.cpp expressions/primary/string-raw-delimiter-tab.cpp

If two adjacent string literals both have a prefix, the prefixes must be
the same. A string literal does not convert to a pointer to non-const
characters. A raw string delimiter must have at most 16 characters and
must not contain a space, a parenthesis, a backslash, or a control
character such as a tab.
:::

:::{rule} PRIM-10
:kind: required
:tests: expressions/primary/bool-nullptr.cpp expressions/primary/nullptr-member-pointer.cpp expressions/primary/nullptr-to-bool-copy.cpp expressions/primary/nullptr-to-int.cpp

`true` and `false` have type `bool`. `nullptr` has type
`std::nullptr_t`. It converts to every pointer and pointer-to-member
type. It converts to `bool` only by direct initialization, and it does
not convert to an integer.
:::

:::{rule} PRIM-11
:kind: required
:tests: expressions/primary/this-const-member-modify.cpp expressions/primary/this-default-member-init.cpp expressions/primary/this-prvalue.cpp expressions/primary/this-type.cpp expressions/primary/this-trailing-return.cpp

In a member function of class `C`, `this` is a prvalue that points to
the object. Its type is pointer to `C` with the cv-qualifiers of the
member function. A program can also use `this` in a default member
initializer and in the trailing return type of a member function.
:::

:::{rule} PRIM-20
:kind: reject
:tests: expressions/primary/this-namespace.cpp expressions/primary/this-static.cpp expressions/primary/this-explicit-object.cpp

A program must not use `this` in a static member function, in a member
function with an explicit object parameter, or outside a class.
:::

:::{rule} PRIM-12
:kind: required
:tests: expressions/primary/name-enumerator-prvalue.cpp expressions/primary/name-function-lvalue.cpp expressions/primary/name-member-in-static.cpp expressions/primary/name-member-outside-object.cpp expressions/primary/name-member-unevaluated.cpp expressions/primary/name-parameter-in-scope.cpp expressions/primary/name-qualified-class-static.cpp expressions/primary/name-qualified-global.cpp expressions/primary/name-undeclared.cpp expressions/primary/name-variable-lvalue.cpp expressions/primary/this-implicit-member.cpp expressions/primary/member-mutable-in-const.cpp expressions/primary/member-const-in-const-function.cpp

A name refers to the entity that lookup finds *(referenced)*. Its value
category is:

- an lvalue for a variable, a parameter, or a function,
- a prvalue for an enumerator.

In a member function, the name of a non-static member means `this->m`.
A data member then has the cv-qualifiers of the object, unless it is
`mutable` or a reference.
Elsewhere, such a name is valid only in an unevaluated operand. `::x`
names `x` in the global namespace. `N::x` names `x` in the namespace or
class `N`.
:::

:::{rule} PRIM-14
:kind: required
:tests: expressions/primary/paren-function-call.cpp expressions/primary/paren-lvalue.cpp expressions/primary/paren-same-type.cpp

An expression in parentheses has the same type, value, and value
category as the expression.
:::

## Undefined Behavior

:::{rule} PRIM-8
:kind: undefined
:tests: expressions/primary/string-modify-constant.cpp

The characters of a string literal are constant. If a program writes to
one of them, for example through a pointer from `const_cast`, the
behavior is undefined.
:::

## Implementation Requirements

:::{rule} PRIM-6
:kind: impl-defined
:tests: expressions/primary/char-multichar.cpp expressions/primary/char-plain-signedness.cpp

The value of a character literal with more than one character and no
prefix is implementation-defined. Its type is `int`. If plain `char` is
signed is implementation-defined. On the x86-64 Linux targets in the
matrix, `'ab'` is `0x6162` and `char` is signed.
:::

:::{rule} PRIM-9
:kind: unspecified
:tests: expressions/primary/string-compare-constant.cpp expressions/primary/string-same-object-either.cpp expressions/primary/string-same-literal-two-evaluations.cpp

It is unspecified if two string literals with the same characters are
the same object. This also applies to two evaluations of one string
literal. Constant evaluation rejects a comparison of their addresses.
:::

:::{rule} PRIM-15
:kind: impl-defined
:tests: expressions/primary/float-overflow.cpp

The value of a floating literal outside the range of its type is
implementation-defined. An implementation can reject it
({question}`Q-0063`).
:::

:::{rule} PRIM-21
:kind: impl-defined
:tests: expressions/primary/float-nearest-value.cpp

If the value of a floating literal is in the range of its type but has no
exact representation, the result is one of the two nearest values that
the type can represent. The implementation chooses which one. On the
x86-64 Linux targets in the matrix, it is the nearer value.
:::

:::{rule} PRIM-22
:kind: impl-defined
:tests: expressions/primary/float-extended-suffix.cpp

The suffixes `f16`, `f32`, `f64`, `f128`, and `bf16`, in either case,
give the extended floating-point types *(referenced)*. Their support is
conditional. GCC and EDG accept them. Clang rejects them.
:::

:::{rule} PRIM-23
:kind: impl-defined
:tests: expressions/primary/char-unprefixed-not-single-unit.cpp

A character literal without a prefix that contains one character with
no single code unit in the ordinary encoding is conditionally supported.
If an implementation accepts it, its type is `int` and its value is
implementation-defined. No implementation in the matrix accepts it.
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

struct Counter {
  int n = 0;
  void bump() { ++n; }    // means ++this->n
};
```

## Evidence

The results are in `tests/RESULTS.md`. All seven compilers agree on 91 of 94 tests.

- PRIM-2: Clang and GCC reject a decimal literal that fits only an
  unsigned type under `-pedantic-errors`. EDG accepts it
  ({question}`Q-0064`).
- PRIM-15: Clang and GCC accept `1e999f` with a warning and give
  infinity. EDG rejects it ({question}`Q-0063`).
- PRIM-22: GCC and EDG accept the extended floating suffixes. Clang
  rejects them. Support is conditional.

Inputs: clang-lex-literal, clang-diag-lex, clang-sema-expr,
clang-diag-sema, edg-decls.
