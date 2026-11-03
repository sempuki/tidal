# Keywords and reserved names

A keyword is a word that the language reserves for its own syntax. This
page lists the keywords, the words that have a special meaning only in
certain places, and the names that a program must leave to the
implementation.

Depends on: primary expressions; declarators.

## Principles

:::{principle} KW-P1
:rules: KW-1 KW-2 KW-3 KW-4

A keyword cannot name anything, so the parser can read it without
lookup. Words added to the language late, such as `override` and
`final`, have a special meaning only where a keyword could not appear,
so existing programs that use them as names stay valid. The parser
recognizes such a word by its position alone, with no lookup
({rule}`KW-3`). Elsewhere the language uses lookup to decide how to
read a token. An example is `<` after a name, which opens a {term}`template argument`
list depending on what lookup finds ({rule}`TARG-23`). The
implementation reserves some names for its own declarations. A program
that uses them can clash with the implementation, though no compiler
reports it.
:::

## Constraints

:::{rule} KW-1
:kind: reject
:tests: foundations/keywords/keyword-alignas.cpp foundations/keywords/keyword-alignof.cpp foundations/keywords/keyword-and.cpp foundations/keywords/keyword-and-eq.cpp foundations/keywords/keyword-asm.cpp foundations/keywords/keyword-auto.cpp foundations/keywords/keyword-bitand.cpp foundations/keywords/keyword-bitor.cpp foundations/keywords/keyword-bool.cpp foundations/keywords/keyword-break.cpp foundations/keywords/keyword-case.cpp foundations/keywords/keyword-catch.cpp foundations/keywords/keyword-char.cpp foundations/keywords/keyword-char16-t.cpp foundations/keywords/keyword-char32-t.cpp foundations/keywords/keyword-char8-t.cpp foundations/keywords/keyword-class.cpp foundations/keywords/keyword-co-await.cpp foundations/keywords/keyword-co-return.cpp foundations/keywords/keyword-co-yield.cpp foundations/keywords/keyword-compl.cpp foundations/keywords/keyword-concept.cpp foundations/keywords/keyword-const.cpp foundations/keywords/keyword-const-cast.cpp foundations/keywords/keyword-consteval.cpp foundations/keywords/keyword-constexpr.cpp foundations/keywords/keyword-constinit.cpp foundations/keywords/keyword-continue.cpp foundations/keywords/keyword-decltype.cpp foundations/keywords/keyword-default.cpp foundations/keywords/keyword-delete.cpp foundations/keywords/keyword-do.cpp foundations/keywords/keyword-double.cpp foundations/keywords/keyword-dynamic-cast.cpp foundations/keywords/keyword-else.cpp foundations/keywords/keyword-enum.cpp foundations/keywords/keyword-explicit.cpp foundations/keywords/keyword-export.cpp foundations/keywords/keyword-extern.cpp foundations/keywords/keyword-false.cpp foundations/keywords/keyword-float.cpp foundations/keywords/keyword-for.cpp foundations/keywords/keyword-friend.cpp foundations/keywords/keyword-goto.cpp foundations/keywords/keyword-if.cpp foundations/keywords/keyword-inline.cpp foundations/keywords/keyword-int.cpp foundations/keywords/keyword-long.cpp foundations/keywords/keyword-mutable.cpp foundations/keywords/keyword-namespace.cpp foundations/keywords/keyword-new.cpp foundations/keywords/keyword-noexcept.cpp foundations/keywords/keyword-not.cpp foundations/keywords/keyword-not-eq.cpp foundations/keywords/keyword-nullptr.cpp foundations/keywords/keyword-operator.cpp foundations/keywords/keyword-or.cpp foundations/keywords/keyword-or-eq.cpp foundations/keywords/keyword-private.cpp foundations/keywords/keyword-protected.cpp foundations/keywords/keyword-public.cpp foundations/keywords/keyword-register.cpp foundations/keywords/keyword-reinterpret-cast.cpp foundations/keywords/keyword-requires.cpp foundations/keywords/keyword-return.cpp foundations/keywords/keyword-short.cpp foundations/keywords/keyword-signed.cpp foundations/keywords/keyword-sizeof.cpp foundations/keywords/keyword-static.cpp foundations/keywords/keyword-static-assert.cpp foundations/keywords/keyword-static-cast.cpp foundations/keywords/keyword-struct.cpp foundations/keywords/keyword-switch.cpp foundations/keywords/keyword-template.cpp foundations/keywords/keyword-this.cpp foundations/keywords/keyword-thread-local.cpp foundations/keywords/keyword-throw.cpp foundations/keywords/keyword-true.cpp foundations/keywords/keyword-try.cpp foundations/keywords/keyword-typedef.cpp foundations/keywords/keyword-typeid.cpp foundations/keywords/keyword-typename.cpp foundations/keywords/keyword-union.cpp foundations/keywords/keyword-unsigned.cpp foundations/keywords/keyword-using.cpp foundations/keywords/keyword-virtual.cpp foundations/keywords/keyword-void.cpp foundations/keywords/keyword-volatile.cpp foundations/keywords/keyword-wchar-t.cpp foundations/keywords/keyword-while.cpp foundations/keywords/keyword-xor.cpp foundations/keywords/keyword-xor-eq.cpp

These words are keywords. A program must not use a keyword as a name.
The grammar derives no name from a keyword ({rule}`TR-12`).

| Group | Keywords |
|---|---|
| Fundamental types | `void`, `bool`, `char`, `char8_t`, `char16_t`, `char32_t`, `wchar_t`, `short`, `int`, `long`, `signed`, `unsigned`, `float`, `double` |
| Placeholders and type queries | `auto`, `decltype` |
| Specifiers | `const`, `volatile`, `static`, `extern`, `thread_local`, `register`, `mutable`, `inline`, `constexpr`, `consteval`, `constinit`, `typedef`, `friend`, `virtual`, `explicit` |
| Classes and access | `class`, `struct`, `union`, `enum`, `public`, `protected`, `private`, `operator`, `this` |
| Other declarations | `namespace`, `using`, `static_assert`, `alignas`, `asm`, `export`, `template`, `typename`, `concept`, `requires` |
| Statements | `if`, `else`, `switch`, `case`, `default`, `for`, `while`, `do`, `break`, `continue`, `return`, `goto`, `try`, `catch` |
| Expressions | `new`, `delete`, `sizeof`, `alignof`, `noexcept`, `typeid`, `throw`, `nullptr`, `true`, `false`, `static_cast`, `dynamic_cast`, `const_cast`, `reinterpret_cast` |
| Coroutines (out of scope) | `co_await`, `co_yield`, `co_return` |
| Alternative operator spellings (KW-2) | `and`, `or`, `not`, `bitand`, `bitor`, `xor`, `compl`, `and_eq`, `or_eq`, `xor_eq`, `not_eq` |
:::

:::{rule} KW-2
:kind: required
:tests: foundations/keywords/alternative-tokens.cpp

Each alternative spelling means the operator it names:

| Spelling | Operator |
|---|---|
| `and`, `or`, `not` | `&&`, `\|\|`, `!` |
| `bitand`, `bitor`, `xor`, `compl` | `&`, `\|`, `^`, `~` |
| `and_eq`, `or_eq`, `xor_eq`, `not_eq` | `&=`, `\|=`, `^=`, `!=` |
:::

:::{rule} KW-3
:kind: required
:tests: foundations/keywords/contextual-names.cpp foundations/keywords/contextual-final-class.cpp

`override`, `final`, `import`, and `module` are identifiers. They can
name variables, functions, and classes. `override` and `final` have
their special meaning only after the {term}`declarator` of a member function or
after the name of a class ({rule}`VIRT-3`).
:::

## Implementation Requirements

:::{rule} KW-4
:kind: no-diagnostic
:tests: foundations/keywords/reserved-double-underscore.cpp

These names are reserved for the implementation, and a program must not
declare them:

- a name that contains two underscores in a row,
- a name that starts with an underscore and an uppercase letter,
- in the global namespace, a name that starts with an underscore.

An implementation need not diagnose a use of a reserved name.
:::

## Examples

```cpp
int final = 0;                     // fine: final is an identifier
struct Leaf final {};              // final has its special meaning
int class = 0;                     // rejected: class is a keyword
bool both = true and false;        // the same as true && false
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on all
96 tests.

- KW-1: each probe declares a variable named by one keyword. The
  compilers give different messages, mostly that a name was expected,
  so the probes carry no error pattern.
- KW-1: the coroutine keywords are keywords although coroutines are
  out of scope.
- KW-4: Clang, GCC, and EDG accept the reserved names without a
  warning in the default mode.
