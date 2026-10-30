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
