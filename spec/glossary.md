# Glossary

Definitions of terms used across Tidal sections. Terms are listed
alphabetically. Each entry gives Tidal's own definition.

Terms marked *(referenced)* belong to topics outside the current scope.
Their entries describe enough to read the in-scope sections and will be
replaced when those topics are specified.

## Constant evaluation

*(referenced)* Evaluation of an expression by the translator, while the
program is being compiled, instead of at run time. It happens wherever a
value is needed during translation (an array bound, a `static_assert`, a
template argument, the condition of `if constexpr`, the initializer of a
`constexpr` variable) and for every call to a `consteval` function. A
program is rejected if constant evaluation is required and reaches an
operation with undefined behavior.

## Contextual conversion to bool

The conversion applied to a value that must be read as true or false, such
as an `if` condition. The value `e` is converted as if by the declaration
`bool t(e);`. That form permits `explicit` conversion functions, which an
ordinary implicit conversion would skip. Arithmetic values convert to
`false` when zero and `true` otherwise. Pointers convert to `false` when null
and `true` otherwise.

## Discarded statement

The branch of an `if constexpr` statement that its condition did not select.
Code in a discarded statement is never run, does not take part in return type
deduction, and does not create a need for any function or variable to be
defined. In a template it is also not instantiated.

## Immediate function context

A place where a call to a `consteval` function does not have to be a
constant expression by itself, because the surrounding code is already
guaranteed to run only during constant evaluation. The body of a `consteval`
function is one. The branch of `if consteval` that runs during constant
evaluation is another.

## Integer promotion

A conversion applied to an integer operand whose type is narrower than `int`
(`bool`, the character types, `short`, and their signed and unsigned
variants). The value becomes an `int` if `int` can hold every value of the
original type, and an `unsigned int` otherwise. Unscoped enumerations and
bit-fields promote as well. Their exact rules belong to those topics
*(referenced)*.

## Linkage

Whether a name declared in one scope can refer to the same entity as the
same name declared in another. *External* linkage connects names across
translation units. *Internal* linkage connects names within one translation
unit only. A name with no linkage refers only to the entity its own
declaration introduces.

## Placeholder type

`auto` or `decltype(auto)` written where a type is expected. The actual
type is worked out later, from an initializer or a return statement. A
concept name may come before the placeholder to constrain the type.

## Storage duration

How long the storage for an object exists. *Automatic* storage lasts from
the object's declaration to the end of its block. *Static* storage lasts
for the whole program run. *Thread* storage lasts for the life of a thread,
with one object per thread. *Dynamic* storage is obtained and released with
`new` and `delete` *(referenced)*.

## Translation unit

One source file after preprocessing, together with everything it includes.
Translation units are compiled separately and then linked into a program.

## Unsequenced

Two evaluations are unsequenced when the language does not say which
happens first and allows them to overlap. The operands of most binary
operators are unsequenced with respect to each other. A function call in
one operand is not interleaved with work in the other operand, but the
two may still run in either order.

## Usual arithmetic conversions

The conversions that bring the two operands of a binary arithmetic operator
to a common type:

1. If either operand has floating-point type, the other is converted to the
   floating-point type that has the greater range, in the order `float`,
   `double`, `long double`. Extended floating-point types *(referenced)* have
   their own ordering.
2. Otherwise both operands undergo [integer promotion](#integer-promotion).
   If the types now match, that is the common type.
3. If both are signed or both are unsigned, the narrower one is converted to
   the wider one.
4. If the unsigned operand's type is at least as wide as the signed one's,
   the signed operand is converted to the unsigned type.
5. If the signed type can hold every value of the unsigned type, the unsigned
   operand is converted to the signed type.
6. Otherwise both are converted to the unsigned type with the same width as
   the signed type.

Width means the type's conversion rank *(referenced)*, which for the
standard integer types follows `char`, `short`, `int`, `long`, `long long`.

## Value category

Every expression is one of three kinds. An *lvalue* names an object or
function that has an identity and can have its address taken. An *xvalue*
names an object whose resources may be reused, such as the result of
`std::move`. A *prvalue* computes a value, such as `a * b` or `42`, and has
no identity of its own. The full rules belong to the expressions overview
*(referenced)*.
